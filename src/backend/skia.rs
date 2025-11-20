//! tiny-skia rendering backend

use crate::core::{Bounds, Canvas, Point2D};
use crate::error::{Error, Result};
use fontdue::{Font, FontSettings};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Canvas implementation using tiny-skia
pub struct SkiaCanvas {
    pixmap: Pixmap,
    bounds: Bounds,
    width: u32,
    height: u32,
    /// Margin in pixels (left, top, right, bottom)
    margin: (u32, u32, u32, u32),
    /// Font for text rendering
    font: Font,
}

impl SkiaCanvas {
    /// Create a new canvas with given dimensions
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "raster")]
    /// # {
    /// use velociplot::backend::SkiaCanvas;
    /// use velociplot::core::Bounds;
    ///
    /// let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
    /// let canvas = SkiaCanvas::new(800, 600, bounds).unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the pixmap cannot be created
    pub fn new(width: u32, height: u32, bounds: Bounds) -> Result<Self> {
        let pixmap = Pixmap::new(width, height)
            .ok_or_else(|| Error::Rendering("Failed to create pixmap".into()))?;

        // Load embedded font (JetBrains Mono Regular)
        let font = Font::from_bytes(
            include_bytes!("../../assets/JetBrainsMono-Regular.ttf") as &[u8],
            FontSettings::default(),
        )
        .map_err(|e| Error::Rendering(format!("Failed to load font: {e}")))?;

        Ok(Self {
            pixmap,
            bounds,
            width,
            height,
            margin: (60, 40, 20, 40), // left, top, right, bottom
            font,
        })
    }

    /// Set margin (left, top, right, bottom) in pixels
    pub fn set_margin(&mut self, left: u32, top: u32, right: u32, bottom: u32) {
        self.margin = (left, top, right, bottom);
    }

    /// Load a custom font from bytes
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use velociplot::backend::SkiaCanvas;
    /// # use velociplot::core::Bounds;
    /// # let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
    /// # let mut canvas = SkiaCanvas::new(800, 600, bounds).unwrap();
    /// let font_bytes = std::fs::read("path/to/font.ttf").unwrap();
    /// canvas.load_font_from_bytes(&font_bytes).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the font data is invalid
    pub fn load_font_from_bytes(&mut self, font_data: &[u8]) -> Result<()> {
        self.font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| Error::Rendering(format!("Failed to load font: {e}")))?;
        Ok(())
    }

    /// Load a custom font from a file path
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use velociplot::backend::SkiaCanvas;
    /// # use velociplot::core::Bounds;
    /// # let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
    /// # let mut canvas = SkiaCanvas::new(800, 600, bounds).unwrap();
    /// canvas.load_font_from_file("path/to/font.ttf").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or the font data is invalid
    #[cfg(feature = "raster")]
    pub fn load_font_from_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let font_data = std::fs::read(path).map_err(Error::Io)?;
        self.load_font_from_bytes(&font_data)
    }

    /// Get the plotting area (excluding margins)
    #[allow(clippy::cast_precision_loss)]
    fn plot_area(&self) -> (f32, f32, f32, f32) {
        let (ml, mt, mr, mb) = self.margin;
        (
            ml as f32,
            mt as f32,
            (self.width - mr) as f32,
            (self.height - mb) as f32,
        )
    }

    /// Encode to PNG bytes
    ///
    /// # Errors
    ///
    /// Returns an error if PNG encoding fails
    pub fn encode_png(&self) -> Result<Vec<u8>> {
        self.pixmap
            .encode_png()
            .map_err(|e| Error::Rendering(format!("PNG encoding failed: {e}")))
    }

    /// Save to PNG file
    ///
    /// # Errors
    ///
    /// Returns an error if file writing fails
    pub fn save_png(&self, path: &str) -> Result<()> {
        let data = self.encode_png()?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Calculate "density" (non-white pixels) in a rectangular region
    /// Returns a value between 0.0 (all white) and 1.0 (all non-white)
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn calculate_region_density(&self, x: u32, y: u32, width: u32, height: u32) -> f64 {
        let pixels = self.pixmap.pixels();
        let mut non_white_count = 0;
        let mut total_samples = 0;

        // Sample every 4th pixel for performance (still gives good results)
        for py in (y..(y + height).min(self.height)).step_by(4) {
            for px in (x..(x + width).min(self.width)).step_by(4) {
                let idx = (py * self.width + px) as usize;
                if let Some(pixel) = pixels.get(idx) {
                    let r = pixel.red();
                    let g = pixel.green();
                    let b = pixel.blue();

                    // Consider a pixel "white" if all RGB components are > 250
                    if r < 250 || g < 250 || b < 250 {
                        non_white_count += 1;
                    }
                    total_samples += 1;
                }
            }
        }

        if total_samples == 0 {
            0.0
        } else {
            f64::from(non_white_count) / f64::from(total_samples)
        }
    }

    /// Calculate density in the four corners and return the best position
    /// Returns (`UpperRight`, `UpperLeft`, `LowerRight`, `LowerLeft`) densities
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn calculate_corner_densities(
        &self,
        legend_width: u32,
        legend_height: u32,
    ) -> (f64, f64, f64, f64) {
        let margin = 20u32;
        let (ml, mt, mr, mb) = self.margin;

        // Upper right
        let ur_x = self.width.saturating_sub(legend_width + margin + mr);
        let ur_y = mt + margin;
        let ur_density = self.calculate_region_density(ur_x, ur_y, legend_width, legend_height);

        // Upper left
        let ul_x = ml + margin;
        let ul_y = mt + margin;
        let ul_density = self.calculate_region_density(ul_x, ul_y, legend_width, legend_height);

        // Lower right
        let lr_x = self.width.saturating_sub(legend_width + margin + mr);
        let lr_y = self.height.saturating_sub(legend_height + margin + mb);
        let lr_density = self.calculate_region_density(lr_x, lr_y, legend_width, legend_height);

        // Lower left
        let ll_x = ml + margin;
        let ll_y = self.height.saturating_sub(legend_height + margin + mb);
        let ll_density = self.calculate_region_density(ll_x, ll_y, legend_width, legend_height);

        (ur_density, ul_density, lr_density, ll_density)
    }

    /// Convert RGBA u8 array to tiny-skia Color
    fn color_from_rgba(rgba: &[u8; 4]) -> Color {
        Color::from_rgba8(rgba[0], rgba[1], rgba[2], rgba[3])
    }

    /// Combine this canvas with another canvas vertically (other below this one)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use velociplot::prelude::*;
    /// # fn example() -> Result<()> {
    /// // Create plot canvas
    /// let bounds = Bounds::new(0.0, 0.0, 10.0, 10.0);
    /// let mut plot_canvas = SkiaCanvas::new(800, 600, bounds)?;
    /// // ... draw plot ...
    ///
    /// // Create legend canvas
    /// let mut legend_canvas = SkiaCanvas::new(800, 100, bounds)?;
    /// // ... draw legend ...
    ///
    /// // Combine them
    /// let combined = plot_canvas.combine_vertical(&legend_canvas)?;
    /// combined.save_png("combined.png")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if canvas creation or pixel copying fails
    pub fn combine_vertical(&self, other: &Self) -> Result<Self> {
        if self.width != other.width {
            return Err(Error::Rendering(
                "Cannot combine canvases with different widths".into(),
            ));
        }

        let combined_height = self.height + other.height;
        let mut combined_pixmap = Pixmap::new(self.width, combined_height)
            .ok_or_else(|| Error::Rendering("Failed to create combined pixmap".into()))?;

        // Copy pixels from first canvas
        let self_pixels = self.pixmap.pixels();
        let combined_pixels = combined_pixmap.pixels_mut();

        for y in 0..self.height {
            for x in 0..self.width {
                let src_idx = (y * self.width + x) as usize;
                let dst_idx = (y * self.width + x) as usize;
                if let Some(pixel) = self_pixels.get(src_idx) {
                    combined_pixels[dst_idx] = *pixel;
                }
            }
        }

        // Copy pixels from second canvas below the first
        let other_pixels = other.pixmap.pixels();
        for y in 0..other.height {
            for x in 0..other.width {
                let src_idx = (y * other.width + x) as usize;
                let dst_y = self.height + y;
                let dst_idx = (dst_y * self.width + x) as usize;
                if let Some(pixel) = other_pixels.get(src_idx) {
                    combined_pixels[dst_idx] = *pixel;
                }
            }
        }

        Ok(Self {
            pixmap: combined_pixmap,
            bounds: self.bounds,
            width: self.width,
            height: combined_height,
            margin: self.margin,
            font: self.font.clone(),
        })
    }

    /// Create a canvas suitable for rendering a legend only
    ///
    /// This creates a canvas with minimal margins, suitable for rendering
    /// just a legend that can be saved separately or combined with a plot canvas.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use velociplot::prelude::*;
    /// # fn example() -> Result<()> {
    /// let bounds = Bounds::new(0.0, 0.0, 10.0, 10.0);
    /// let mut legend_canvas = SkiaCanvas::new_legend_canvas(800, 100, bounds)?;
    /// legend_canvas.fill_background(&Color::WHITE.to_rgba())?;
    ///
    /// let legend = Legend::new()
    ///     .position(LegendPosition::Custom(400, 50));
    /// legend.draw(&mut legend_canvas)?;
    /// legend_canvas.save_png("legend_only.png")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if canvas creation fails
    pub fn new_legend_canvas(width: u32, height: u32, bounds: Bounds) -> Result<Self> {
        let mut canvas = Self::new(width, height, bounds)?;
        // Set minimal margins for legend-only canvas
        canvas.margin = (10, 10, 10, 10);
        Ok(canvas)
    }
}

impl Canvas for SkiaCanvas {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn bounds(&self) -> Bounds {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
    }

    fn transform(&self, point: &Point2D) -> (f32, f32) {
        let (x_min, y_min, x_max, y_max) = self.plot_area();

        // Map data coordinates to pixel coordinates
        let x_range = self.bounds.width();
        let y_range = self.bounds.height();
        let pixel_width = x_max - x_min;
        let pixel_height = y_max - y_min;

        let x_normalized = (point.x - self.bounds.x_min) / x_range;
        let y_normalized = (point.y - self.bounds.y_min) / y_range;

        // Convert to pixel coordinates (flip y-axis for screen coordinates)
        #[allow(clippy::cast_possible_truncation)]
        let x_pixel = x_min + x_normalized as f32 * pixel_width;
        #[allow(clippy::cast_possible_truncation)]
        let y_pixel = y_max - y_normalized as f32 * pixel_height; // flip y

        (x_pixel, y_pixel)
    }

    fn draw_line(
        &mut self,
        from: &Point2D,
        to: &Point2D,
        color: &[u8; 4],
        width: f32,
    ) -> Result<()> {
        let (x1, y1) = self.transform(from);
        let (x2, y2) = self.transform(to);

        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        let path = pb
            .finish()
            .ok_or_else(|| Error::Rendering("Failed to build path".into()))?;

        let mut paint = Paint::default();
        paint.set_color(Self::color_from_rgba(color));
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = width;

        self.pixmap
            .stroke_path(&path, &paint, &stroke, Transform::identity(), None);

        Ok(())
    }

    fn draw_circle(
        &mut self,
        center: &Point2D,
        radius: f32,
        color: &[u8; 4],
        filled: bool,
    ) -> Result<()> {
        let (cx, cy) = self.transform(center);

        let mut pb = PathBuilder::new();
        pb.push_circle(cx, cy, radius);
        let path = pb
            .finish()
            .ok_or_else(|| Error::Rendering("Failed to build circle path".into()))?;

        let mut paint = Paint::default();
        paint.set_color(Self::color_from_rgba(color));
        paint.anti_alias = true;

        if filled {
            self.pixmap.fill_path(
                &path,
                &paint,
                tiny_skia::FillRule::Winding,
                Transform::identity(),
                None,
            );
        } else {
            let stroke = Stroke::default();
            self.pixmap
                .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        top_left: &Point2D,
        width: f64,
        height: f64,
        color: &[u8; 4],
    ) -> Result<()> {
        // Transform corners to pixel coordinates
        let (x1, y1) = self.transform(top_left);
        let bottom_right = Point2D::new(top_left.x + width, top_left.y + height);
        let (x2, y2) = self.transform(&bottom_right);

        // Build rectangle path
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y1);
        pb.line_to(x2, y2);
        pb.line_to(x1, y2);
        pb.close();
        let path = pb
            .finish()
            .ok_or_else(|| Error::Rendering("Failed to build rectangle path".into()))?;

        // Fill the rectangle
        let mut paint = Paint::default();
        paint.set_color(Self::color_from_rgba(color));
        paint.anti_alias = false; // Sharp edges for heatmap cells

        self.pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::Winding,
            Transform::identity(),
            None,
        );

        Ok(())
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: &[u8; 4]) -> Result<()> {
        // Transform from data coordinates to pixel coordinates
        let (px, py) = self.transform(&Point2D::new(f64::from(x), f64::from(y)));
        self.draw_text_pixels(text, px, py, size, color)
    }

    fn fill_background(&mut self, color: &[u8; 4]) -> Result<()> {
        let color = Self::color_from_rgba(color);
        self.pixmap.fill(color);
        Ok(())
    }

    fn draw_line_pixels(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: &[u8; 4],
        width: f32,
    ) -> Result<()> {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        let path = pb
            .finish()
            .ok_or_else(|| Error::Rendering("Failed to build path".into()))?;

        let mut paint = Paint::default();
        paint.set_color(Self::color_from_rgba(color));
        paint.anti_alias = true;

        let mut stroke = Stroke::default();
        stroke.width = width;

        self.pixmap
            .stroke_path(&path, &paint, &stroke, Transform::identity(), None);

        Ok(())
    }

    fn draw_text_pixels(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            ..LayoutSettings::default()
        });
        layout.append(&[&self.font], &TextStyle::new(text, size, 0));

        // Rasterize and draw each glyph
        for glyph in layout.glyphs() {
            let (metrics, bitmap) = self.font.rasterize_config(glyph.key);

            // Skip empty glyphs (like spaces)
            if metrics.width == 0 || metrics.height == 0 || bitmap.is_empty() {
                continue;
            }

            // Draw each pixel of the glyph
            for (bitmap_y, row) in bitmap.chunks(metrics.width).enumerate() {
                for (bitmap_x, &alpha) in row.iter().enumerate() {
                    if alpha > 0 {
                        let px = glyph.x as usize + bitmap_x;
                        let py = glyph.y as usize + bitmap_y;

                        if px < self.width as usize && py < self.height as usize {
                            // Blend the text color with the existing pixel
                            let pixel_idx = (py * self.width as usize + px) * 4;
                            let pixels = self.pixmap.data_mut();

                            // Alpha blending
                            let alpha_f = f32::from(alpha) / 255.0;
                            pixels[pixel_idx] = blend_channel(pixels[pixel_idx], color[0], alpha_f);
                            pixels[pixel_idx + 1] =
                                blend_channel(pixels[pixel_idx + 1], color[1], alpha_f);
                            pixels[pixel_idx + 2] =
                                blend_channel(pixels[pixel_idx + 2], color[2], alpha_f);
                            pixels[pixel_idx + 3] = 255; // Keep alpha at full opacity
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn calculate_corner_densities(
        &self,
        legend_width: u32,
        legend_height: u32,
    ) -> (f64, f64, f64, f64) {
        self.calculate_corner_densities(legend_width, legend_height)
    }
}

/// Blend a color channel using alpha
#[inline]
fn blend_channel(bg: u8, fg: u8, alpha: f32) -> u8 {
    let bg_f = f32::from(bg) / 255.0;
    let fg_f = f32::from(fg) / 255.0;
    let blended = bg_f * (1.0 - alpha) + fg_f * alpha;
    (blended * 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_canvas_creation() {
        let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
        let canvas = SkiaCanvas::new(800, 600, bounds).unwrap();
        assert_eq!(canvas.dimensions(), (800, 600));
    }

    #[test]
    fn test_coordinate_transform() {
        let bounds = Bounds::new(0.0, 10.0, 0.0, 10.0);
        let canvas = SkiaCanvas::new(800, 600, bounds).unwrap();

        // Test corner points
        let (x, y) = canvas.transform(&Point2D::new(0.0, 0.0));
        assert_relative_eq!(x, 60.0, epsilon = 1.0); // left margin
        assert_relative_eq!(y, 560.0, epsilon = 1.0); // bottom (flipped y)

        let (x, y) = canvas.transform(&Point2D::new(10.0, 10.0));
        assert_relative_eq!(x, 780.0, epsilon = 1.0); // right side
        assert_relative_eq!(y, 40.0, epsilon = 1.0); // top
    }
}
