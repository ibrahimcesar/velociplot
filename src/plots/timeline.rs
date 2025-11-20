//! Timeline implementation for chronological event visualization
//!
//! Timelines display events along a temporal axis with dates, labels, and
//! optional descriptions. Supports both horizontal and vertical orientations.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let mut timeline = Timeline::new();
//! timeline.add_event(2020.0, "Launch", Some("Product launched".to_string()));
//! timeline.add_event(2021.0, "Series A", Some("Funding round".to_string()));
//! timeline.add_event(2022.0, "Expansion", Some("International expansion".to_string()));
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::Result;

/// A single event on the timeline
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    /// Position on the timeline (typically a year or timestamp)
    pub position: f64,
    /// Main label for the event
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Color for this event's marker
    pub color: Color,
    /// Marker size
    pub marker_size: f32,
}

/// Timeline orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimelineOrientation {
    /// Horizontal timeline (time flows left to right)
    Horizontal,
    /// Vertical timeline (time flows bottom to top)
    Vertical,
}

/// Timeline visualization for chronological events
pub struct Timeline {
    events: Vec<TimelineEvent>,
    orientation: TimelineOrientation,
    line_color: Color,
    line_width: f32,
    marker_default_color: Color,
    marker_default_size: f32,
    show_descriptions: bool,
    label_offset: f64,
    alternating_sides: bool,
}

impl Timeline {
    /// Create a new empty timeline
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let timeline = Timeline::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            orientation: TimelineOrientation::Horizontal,
            line_color: Color::from_hex("#34495e").unwrap(),
            line_width: 2.0,
            marker_default_color: Color::from_hex("#3498db").unwrap(),
            marker_default_size: 8.0,
            show_descriptions: true,
            label_offset: 20.0,
            alternating_sides: true,
        }
    }

    /// Add an event to the timeline
    ///
    /// # Arguments
    ///
    /// * `position` - Temporal position (e.g., year, timestamp)
    /// * `label` - Event label
    /// * `description` - Optional description
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let mut timeline = Timeline::new();
    /// timeline.add_event(2020.0, "Launch", Some("Product launched".to_string()));
    /// ```
    pub fn add_event(
        &mut self,
        position: f64,
        label: impl Into<String>,
        description: impl Into<Option<String>>,
    ) -> &mut Self {
        self.events.push(TimelineEvent {
            position,
            label: label.into(),
            description: description.into(),
            color: self.marker_default_color,
            marker_size: self.marker_default_size,
        });
        self
    }

    /// Add an event with custom color
    pub fn add_event_colored(
        &mut self,
        position: f64,
        label: impl Into<String>,
        description: impl Into<Option<String>>,
        color: Color,
    ) -> &mut Self {
        self.events.push(TimelineEvent {
            position,
            label: label.into(),
            description: description.into(),
            color,
            marker_size: self.marker_default_size,
        });
        self
    }

    /// Set timeline orientation
    #[must_use]
    pub fn orientation(mut self, orientation: TimelineOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the main timeline line color
    #[must_use]
    pub fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    /// Set the timeline line width
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width.max(0.0);
        self
    }

    /// Set the default marker color for events
    #[must_use]
    pub fn marker_color(mut self, color: Color) -> Self {
        self.marker_default_color = color;
        self
    }

    /// Set the default marker size
    #[must_use]
    pub fn marker_size(mut self, size: f32) -> Self {
        self.marker_default_size = size.max(1.0);
        self
    }

    /// Set whether to show event descriptions
    #[must_use]
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set label offset from timeline
    #[must_use]
    pub fn label_offset(mut self, offset: f64) -> Self {
        self.label_offset = offset.max(0.0);
        self
    }

    /// Set whether to alternate labels on both sides of the timeline
    #[must_use]
    pub fn alternating_sides(mut self, alternating: bool) -> Self {
        self.alternating_sides = alternating;
        self
    }

    /// Get bounds for the timeline
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.events.is_empty() {
            return None;
        }

        let positions: Vec<f64> = self.events.iter().map(|e| e.position).collect();
        let min_pos = positions.iter().copied().fold(f64::INFINITY, f64::min);
        let max_pos = positions.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        // Add some padding
        let padding = (max_pos - min_pos) * 0.1;

        match self.orientation {
            TimelineOrientation::Horizontal => {
                // X: timeline positions, Y: -50 to 50 (for labels above/below)
                Some(Bounds::new(
                    min_pos - padding,
                    max_pos + padding,
                    -50.0,
                    50.0,
                ))
            }
            TimelineOrientation::Vertical => {
                // X: -50 to 50 (for labels left/right), Y: timeline positions
                Some(Bounds::new(
                    -50.0,
                    50.0,
                    min_pos - padding,
                    max_pos + padding,
                ))
            }
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Timeline {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.events.is_empty() {
            return Ok(());
        }

        // Sort events by position
        let mut sorted_events = self.events.clone();
        sorted_events.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

        let bounds = canvas.bounds();

        match self.orientation {
            TimelineOrientation::Horizontal => {
                // Draw main horizontal timeline
                let timeline_y = 0.0;
                let start_x = bounds.x_min;
                let end_x = bounds.x_max;

                canvas.draw_line(
                    &Point2D::new(start_x, timeline_y),
                    &Point2D::new(end_x, timeline_y),
                    &self.line_color.to_rgba(),
                    self.line_width,
                )?;

                // Draw events
                for (idx, event) in sorted_events.iter().enumerate() {
                    let x = event.position;
                    let above = if self.alternating_sides {
                        idx % 2 == 0
                    } else {
                        true
                    };

                    // Draw connector line from timeline to label
                    let connector_start = Point2D::new(x, timeline_y);
                    let connector_end = Point2D::new(
                        x,
                        if above {
                            timeline_y + self.label_offset
                        } else {
                            timeline_y - self.label_offset
                        },
                    );
                    canvas.draw_line(
                        &connector_start,
                        &connector_end,
                        &event.color.to_rgba(),
                        1.5,
                    )?;

                    // Draw marker on timeline
                    canvas.draw_circle(
                        &Point2D::new(x, timeline_y),
                        event.marker_size,
                        &event.color.to_rgba(),
                        true,
                    )?;

                    // Draw label
                    let label_y = if above {
                        timeline_y + self.label_offset + 5.0
                    } else {
                        timeline_y - self.label_offset - 5.0
                    };

                    canvas.draw_text(
                        &event.label,
                        x as f32,
                        label_y as f32,
                        10.0,
                        &Color::from_hex("#2c3e50").unwrap().to_rgba(),
                    )?;

                    // Draw description if enabled
                    if self.show_descriptions {
                        if let Some(desc) = &event.description {
                            let desc_y = if above {
                                label_y + 10.0
                            } else {
                                label_y - 10.0
                            };
                            canvas.draw_text(
                                desc,
                                x as f32,
                                desc_y as f32,
                                8.0,
                                &Color::from_hex("#7f8c8d").unwrap().to_rgba(),
                            )?;
                        }
                    }

                    // Draw date/position below timeline
                    canvas.draw_text(
                        &format!("{:.0}", event.position),
                        x as f32,
                        (timeline_y - 8.0) as f32,
                        8.0,
                        &Color::from_hex("#95a5a6").unwrap().to_rgba(),
                    )?;
                }
            }
            TimelineOrientation::Vertical => {
                // Draw main vertical timeline
                let timeline_x = 0.0;
                let start_y = bounds.y_min;
                let end_y = bounds.y_max;

                canvas.draw_line(
                    &Point2D::new(timeline_x, start_y),
                    &Point2D::new(timeline_x, end_y),
                    &self.line_color.to_rgba(),
                    self.line_width,
                )?;

                // Draw events
                for (idx, event) in sorted_events.iter().enumerate() {
                    let y = event.position;
                    let right_side = if self.alternating_sides {
                        idx % 2 == 0
                    } else {
                        true
                    };

                    // Draw connector line from timeline to label
                    let connector_start = Point2D::new(timeline_x, y);
                    let connector_end = Point2D::new(
                        if right_side {
                            timeline_x + self.label_offset
                        } else {
                            timeline_x - self.label_offset
                        },
                        y,
                    );
                    canvas.draw_line(
                        &connector_start,
                        &connector_end,
                        &event.color.to_rgba(),
                        1.5,
                    )?;

                    // Draw marker on timeline
                    canvas.draw_circle(
                        &Point2D::new(timeline_x, y),
                        event.marker_size,
                        &event.color.to_rgba(),
                        true,
                    )?;

                    // Draw label
                    let label_x = if right_side {
                        timeline_x + self.label_offset + 5.0
                    } else {
                        timeline_x - self.label_offset - 5.0
                    };

                    canvas.draw_text(
                        &event.label,
                        label_x as f32,
                        y as f32,
                        10.0,
                        &Color::from_hex("#2c3e50").unwrap().to_rgba(),
                    )?;

                    // Draw description if enabled
                    if self.show_descriptions {
                        if let Some(desc) = &event.description {
                            canvas.draw_text(
                                desc,
                                label_x as f32,
                                (y + 10.0) as f32,
                                8.0,
                                &Color::from_hex("#7f8c8d").unwrap().to_rgba(),
                            )?;
                        }
                    }

                    // Draw date/position to the left of timeline
                    canvas.draw_text(
                        &format!("{:.0}", event.position),
                        (timeline_x - 15.0) as f32,
                        y as f32,
                        8.0,
                        &Color::from_hex("#95a5a6").unwrap().to_rgba(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new();
        assert_eq!(timeline.events.len(), 0);
    }

    #[test]
    fn test_add_event() {
        let mut timeline = Timeline::new();
        timeline.add_event(2020.0, "Event", Some("Description".to_string()));
        assert_eq!(timeline.events.len(), 1);
        assert_eq!(timeline.events[0].position, 2020.0);
        assert_eq!(timeline.events[0].label, "Event");
    }

    #[test]
    fn test_add_event_no_description() {
        let mut timeline = Timeline::new();
        timeline.add_event(2021.0, "Event", None);
        assert_eq!(timeline.events.len(), 1);
        assert!(timeline.events[0].description.is_none());
    }

    #[test]
    fn test_add_colored_event() {
        let mut timeline = Timeline::new();
        timeline.add_event_colored(2022.0, "Event", None, Color::RED);
        assert_eq!(timeline.events[0].color, Color::RED);
    }

    #[test]
    fn test_bounds_horizontal() {
        let mut timeline = Timeline::new();
        timeline.add_event(2020.0, "A", None);
        timeline.add_event(2025.0, "B", None);

        let bounds = timeline.bounds().unwrap();
        assert!(bounds.x_min < 2020.0);
        assert!(bounds.x_max > 2025.0);
        assert_eq!(bounds.y_min, -50.0);
        assert_eq!(bounds.y_max, 50.0);
    }

    #[test]
    fn test_bounds_vertical() {
        let mut timeline = Timeline::new();
        timeline
            .add_event(2020.0, "A", None)
            .add_event(2025.0, "B", None);

        let timeline = timeline.orientation(TimelineOrientation::Vertical);
        let bounds = timeline.bounds().unwrap();

        assert_eq!(bounds.x_min, -50.0);
        assert_eq!(bounds.x_max, 50.0);
        assert!(bounds.y_min < 2020.0);
        assert!(bounds.y_max > 2025.0);
    }

    #[test]
    fn test_empty_bounds() {
        let timeline = Timeline::new();
        assert!(timeline.bounds().is_none());
    }
}
