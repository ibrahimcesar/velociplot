//! SVG vector graphics backend

use crate::core::{Bounds, Canvas, Point2D};
use crate::error::{Error, Result};
use std::fmt::Write;

/// Canvas implementation for SVG vector graphics
pub struct SvgCanvas {
    width: u32,
    height: u32,
    bounds: Bounds,
    elements: String,
    margin: (u32, u32, u32, u32), // left, top, right, bottom
}

impl SvgCanvas {
    /// Create a new SVG canvas with given dimensions
    ///
    /// # Examples
    ///
    /// ```
    /// use velociplot::backend::SvgCanvas;
    /// use velociplot::core::Bounds;
    ///
    /// let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
    /// let canvas = SvgCanvas::new(800, 600, bounds);
    /// ```
    #[must_use]
    pub fn new(width: u32, height: u32, bounds: Bounds) -> Self {
        Self {
            width,
            height,
            bounds,
            elements: String::new(),
            margin: (60, 40, 20, 40), // left, top, right, bottom
        }
    }

    /// Set margin (left, top, right, bottom) in pixels
    pub fn set_margin(&mut self, left: u32, top: u32, right: u32, bottom: u32) {
        self.margin = (left, top, right, bottom);
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

    /// Convert RGBA to SVG color string
    fn rgba_to_svg(rgba: &[u8; 4]) -> String {
        if rgba[3] == 255 {
            format!("rgb({},{},{})", rgba[0], rgba[1], rgba[2])
        } else {
            format!(
                "rgba({},{},{},{})",
                rgba[0],
                rgba[1],
                rgba[2],
                f32::from(rgba[3]) / 255.0
            )
        }
    }

    /// Encode to SVG string
    ///
    /// # Errors
    ///
    /// Returns an error if SVG generation fails
    pub fn to_svg(&self) -> Result<String> {
        let mut svg = String::new();

        // SVG header
        writeln!(&mut svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
            .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))?;

        writeln!(
            &mut svg,
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" version="1.1">"#,
            self.width, self.height
        )
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))?;

        // Add all elements
        write!(&mut svg, "{}", self.elements)
            .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))?;

        // Close SVG
        writeln!(&mut svg, "</svg>")
            .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))?;

        Ok(svg)
    }

    /// Save to SVG file
    ///
    /// # Errors
    ///
    /// Returns an error if file writing fails
    pub fn save_svg(&self, path: &str) -> Result<()> {
        let svg_content = self.to_svg()?;
        std::fs::write(path, svg_content)?;
        Ok(())
    }
}

impl Canvas for SvgCanvas {
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
        self.draw_line_pixels(x1, y1, x2, y2, color, width)
    }

    fn draw_circle(
        &mut self,
        center: &Point2D,
        radius: f32,
        color: &[u8; 4],
        filled: bool,
    ) -> Result<()> {
        let (cx, cy) = self.transform(center);
        let color_str = Self::rgba_to_svg(color);

        if filled {
            writeln!(
                &mut self.elements,
                r#"  <circle cx="{cx}" cy="{cy}" r="{radius}" fill="{color_str}" />"#
            )
        } else {
            writeln!(
                &mut self.elements,
                r#"  <circle cx="{cx}" cy="{cy}" r="{radius}" fill="none" stroke="{color_str}" stroke-width="1" />"#
            )
        }
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))
    }

    fn draw_rectangle(
        &mut self,
        top_left: &Point2D,
        width: f64,
        height: f64,
        color: &[u8; 4],
    ) -> Result<()> {
        let (x, y) = self.transform(top_left);
        let bottom_right = Point2D::new(top_left.x + width, top_left.y + height);
        let (x2, y2) = self.transform(&bottom_right);

        let rect_width = (x2 - x).abs();
        let rect_height = (y2 - y).abs();
        let color_str = Self::rgba_to_svg(color);

        writeln!(
            &mut self.elements,
            r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
            x.min(x2),
            y.min(y2),
            rect_width,
            rect_height,
            color_str
        )
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: &[u8; 4]) -> Result<()> {
        let (px, py) = self.transform(&Point2D::new(f64::from(x), f64::from(y)));
        self.draw_text_pixels(text, px, py, size, color)
    }

    fn fill_background(&mut self, color: &[u8; 4]) -> Result<()> {
        let color_str = Self::rgba_to_svg(color);
        writeln!(
            &mut self.elements,
            r#"  <rect width="{}" height="{}" fill="{}" />"#,
            self.width, self.height, color_str
        )
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))
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
        let color_str = Self::rgba_to_svg(color);
        writeln!(
            &mut self.elements,
            r#"  <line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color_str}" stroke-width="{width}" stroke-linecap="round" />"#
        )
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))
    }

    fn draw_text_pixels(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let color_str = Self::rgba_to_svg(color);
        // SVG text baseline is at the bottom, so we don't need to adjust y
        writeln!(
            &mut self.elements,
            r#"  <text x="{}" y="{}" font-family="monospace" font-size="{}px" fill="{}">{}</text>"#,
            x,
            y,
            size,
            color_str,
            Self::escape_xml(text)
        )
        .map_err(|e| Error::Rendering(format!("SVG write failed: {e}")))
    }

    fn calculate_corner_densities(
        &self,
        _legend_width: u32,
        _legend_height: u32,
    ) -> (f64, f64, f64, f64) {
        // SVG doesn't have pixel data to sample, so we can't calculate density
        // Default to all zeros (which will choose UpperRight by default)
        (0.0, 0.0, 0.0, 0.0)
    }
}

impl SvgCanvas {
    /// Escape XML special characters
    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_creation() {
        let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
        let canvas = SvgCanvas::new(800, 600, bounds);
        assert_eq!(canvas.dimensions(), (800, 600));
    }

    #[test]
    fn test_svg_output() {
        let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
        let mut canvas = SvgCanvas::new(800, 600, bounds);
        canvas.fill_background(&[255, 255, 255, 255]).unwrap();

        let svg = canvas.to_svg().unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("width=\"800\""));
        assert!(svg.contains("height=\"600\""));
    }

    #[test]
    fn test_rgba_conversion() {
        assert_eq!(SvgCanvas::rgba_to_svg(&[255, 0, 0, 255]), "rgb(255,0,0)");
        assert_eq!(
            SvgCanvas::rgba_to_svg(&[0, 128, 255, 128]),
            "rgba(0,128,255,0.5019608)"
        );
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(
            SvgCanvas::escape_xml("a < b & c > d"),
            "a &lt; b &amp; c &gt; d"
        );
        assert_eq!(SvgCanvas::escape_xml("\"test\""), "&quot;test&quot;");
    }
}
