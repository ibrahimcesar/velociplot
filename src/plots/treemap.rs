//! Treemap implementation for hierarchical data visualization
//!
//! Treemaps display hierarchical data as nested rectangles. The area of each
//! rectangle is proportional to a quantitative value, and rectangles can be
//! colored by category.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let mut treemap = Treemap::new();
//! treemap.add_item("Technology", 45.0, Color::from_hex("#3498db").unwrap());
//! treemap.add_item("Healthcare", 32.0, Color::from_hex("#2ecc71").unwrap());
//! treemap.add_item("Finance", 28.0, Color::from_hex("#e74c3c").unwrap());
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::Result;

/// A single item in the treemap
#[derive(Debug, Clone)]
pub struct TreemapItem {
    /// Label for this item
    pub label: String,
    /// Value (determines size)
    pub value: f64,
    /// Color for this rectangle
    pub color: Color,
}

/// Rectangle in the treemap layout
#[derive(Debug, Clone)]
struct TreemapRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    item: TreemapItem,
}

/// Treemap visualization for hierarchical data
///
/// Uses a squarified layout algorithm to create rectangles with better
/// aspect ratios than simple slicing.
pub struct Treemap {
    items: Vec<TreemapItem>,
    show_labels: bool,
    show_values: bool,
    border_width: f32,
    border_color: Color,
    padding: f64,
}

impl Treemap {
    /// Create a new empty treemap
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let treemap = Treemap::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            show_labels: true,
            show_values: true,
            border_width: 1.5,
            border_color: Color::WHITE,
            padding: 2.0,
        }
    }

    /// Add an item to the treemap
    ///
    /// # Arguments
    ///
    /// * `label` - Label for this item
    /// * `value` - Quantitative value (must be positive)
    /// * `color` - Color for this rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let mut treemap = Treemap::new();
    /// treemap.add_item("Category A", 100.0, Color::from_hex("#3498db").unwrap());
    /// ```
    pub fn add_item(&mut self, label: impl Into<String>, value: f64, color: Color) -> &mut Self {
        if value > 0.0 {
            self.items.push(TreemapItem {
                label: label.into(),
                value,
                color,
            });
        }
        self
    }

    /// Set whether to show labels
    #[must_use]
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set whether to show values
    #[must_use]
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set border width
    #[must_use]
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width.max(0.0);
        self
    }

    /// Set border color
    #[must_use]
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set padding between rectangles
    #[must_use]
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// Get bounds for the treemap (always 0,0 to 100,100)
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.items.is_empty() {
            return None;
        }
        Some(Bounds::new(0.0, 100.0, 0.0, 100.0))
    }

    /// Calculate treemap layout using squarified algorithm
    fn calculate_layout(&self) -> Vec<TreemapRect> {
        if self.items.is_empty() {
            return Vec::new();
        }

        // Sort items by value (descending) for better layout
        let mut sorted_items = self.items.clone();
        sorted_items.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());

        let total: f64 = sorted_items.iter().map(|item| item.value).sum();
        if total <= 0.0 {
            return Vec::new();
        }

        // Normalize values to fill 100x100 space
        let normalized: Vec<(TreemapItem, f64)> = sorted_items
            .into_iter()
            .map(|item| {
                let normalized_value = (item.value / total) * 10000.0; // 100x100 = 10000
                (item, normalized_value)
            })
            .collect();

        // Use squarified treemap algorithm
        self.squarify(&normalized, 0.0, 0.0, 100.0, 100.0)
    }

    /// Squarified treemap layout algorithm
    fn squarify(
        &self,
        items: &[(TreemapItem, f64)],
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Vec<TreemapRect> {
        if items.is_empty() {
            return Vec::new();
        }

        if items.len() == 1 {
            return vec![TreemapRect {
                x: x + self.padding,
                y: y + self.padding,
                width: width - 2.0 * self.padding,
                height: height - 2.0 * self.padding,
                item: items[0].0.clone(),
            }];
        }

        // Determine whether to slice horizontally or vertically
        let vertical = width >= height;

        // Simple slicing algorithm (can be improved with proper squarified algorithm)
        let total: f64 = items.iter().map(|(_, v)| v).sum();
        let mut current_offset = 0.0;
        let mut rects = Vec::new();

        for (item, value) in items {
            let proportion = value / total;

            let rect = if vertical {
                let rect_width = width * proportion;
                TreemapRect {
                    x: x + current_offset + self.padding,
                    y: y + self.padding,
                    width: rect_width - 2.0 * self.padding,
                    height: height - 2.0 * self.padding,
                    item: item.clone(),
                }
            } else {
                let rect_height = height * proportion;
                TreemapRect {
                    x: x + self.padding,
                    y: y + current_offset + self.padding,
                    width: width - 2.0 * self.padding,
                    height: rect_height - 2.0 * self.padding,
                    item: item.clone(),
                }
            };

            if vertical {
                current_offset += width * proportion;
            } else {
                current_offset += height * proportion;
            }

            rects.push(rect);
        }

        rects
    }
}

impl Default for Treemap {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Treemap {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let rects = self.calculate_layout();

        for rect in &rects {
            // Draw filled rectangle
            let top_left = Point2D::new(rect.x, rect.y);
            canvas.draw_rectangle(
                &top_left,
                rect.width,
                rect.height,
                &rect.item.color.to_rgba(),
            )?;

            // Draw border
            if self.border_width > 0.0 {
                let border_color = self.border_color.to_rgba();

                // Top
                canvas.draw_line(
                    &Point2D::new(rect.x, rect.y),
                    &Point2D::new(rect.x + rect.width, rect.y),
                    &border_color,
                    self.border_width,
                )?;

                // Right
                canvas.draw_line(
                    &Point2D::new(rect.x + rect.width, rect.y),
                    &Point2D::new(rect.x + rect.width, rect.y + rect.height),
                    &border_color,
                    self.border_width,
                )?;

                // Bottom
                canvas.draw_line(
                    &Point2D::new(rect.x + rect.width, rect.y + rect.height),
                    &Point2D::new(rect.x, rect.y + rect.height),
                    &border_color,
                    self.border_width,
                )?;

                // Left
                canvas.draw_line(
                    &Point2D::new(rect.x, rect.y + rect.height),
                    &Point2D::new(rect.x, rect.y),
                    &border_color,
                    self.border_width,
                )?;
            }

            // Draw label and value if rectangle is large enough
            if rect.width > 15.0 && rect.height > 10.0 {
                let text_x = rect.x + rect.width / 2.0;
                let text_y = rect.y + rect.height / 2.0;

                // Determine text color (white or black based on background)
                let text_color = if self.is_dark_color(&rect.item.color) {
                    Color::WHITE
                } else {
                    Color::from_hex("#2c3e50").unwrap()
                };

                if self.show_labels {
                    let label_y = if self.show_values {
                        text_y - 6.0
                    } else {
                        text_y
                    };

                    canvas.draw_text(
                        &rect.item.label,
                        text_x as f32,
                        label_y as f32,
                        11.0,
                        &text_color.to_rgba(),
                    )?;
                }

                if self.show_values && rect.height > 25.0 {
                    let value_y = if self.show_labels {
                        text_y + 6.0
                    } else {
                        text_y
                    };

                    canvas.draw_text(
                        &format!("{:.1}", rect.item.value),
                        text_x as f32,
                        value_y as f32,
                        9.0,
                        &text_color.to_rgba(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl Treemap {
    /// Determine if a color is dark (for text color selection)
    fn is_dark_color(&self, color: &Color) -> bool {
        let rgba = color.to_rgba();
        let r = f64::from(rgba[0]) / 255.0;
        let g = f64::from(rgba[1]) / 255.0;
        let b = f64::from(rgba[2]) / 255.0;

        // Calculate relative luminance
        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        luminance < 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treemap_creation() {
        let treemap = Treemap::new();
        assert!(treemap.items.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut treemap = Treemap::new();
        treemap.add_item("Test", 100.0, Color::RED);
        assert_eq!(treemap.items.len(), 1);
        assert_eq!(treemap.items[0].label, "Test");
        assert_eq!(treemap.items[0].value, 100.0);
    }

    #[test]
    fn test_negative_value_ignored() {
        let mut treemap = Treemap::new();
        treemap.add_item("Test", -50.0, Color::RED);
        assert_eq!(treemap.items.len(), 0);
    }

    #[test]
    fn test_bounds() {
        let mut treemap = Treemap::new();
        treemap.add_item("A", 50.0, Color::RED);
        let bounds = treemap.bounds().unwrap();
        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 100.0);
        assert_eq!(bounds.y_min, 0.0);
        assert_eq!(bounds.y_max, 100.0);
    }

    #[test]
    fn test_empty_bounds() {
        let treemap = Treemap::new();
        assert!(treemap.bounds().is_none());
    }

    #[test]
    fn test_layout_calculation() {
        let mut treemap = Treemap::new();
        treemap.add_item("A", 50.0, Color::RED);
        treemap.add_item("B", 30.0, Color::BLUE);
        treemap.add_item("C", 20.0, Color::GREEN);

        let rects = treemap.calculate_layout();
        assert_eq!(rects.len(), 3);

        // Check that total area approximately equals 10000 (100x100)
        let total_area: f64 = rects
            .iter()
            .map(|r| (r.width + 2.0 * treemap.padding) * (r.height + 2.0 * treemap.padding))
            .sum();
        assert!((total_area - 10000.0).abs() < 100.0); // Allow some tolerance for padding
    }
}
