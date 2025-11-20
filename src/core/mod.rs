//! Core abstractions for plotting

mod bounds;
mod canvas;
mod point;
mod series;

pub use bounds::Bounds;
pub use canvas::{Canvas, Drawable};
pub use point::Point2D;
pub use series::{DataSeries, Series};

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_point_creation() {
        let p = Point2D::new(1.0, 2.0);
        assert_relative_eq!(p.x, 1.0);
        assert_relative_eq!(p.y, 2.0);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
        assert!(bounds.contains(&Point2D::new(5.0, 2.5)));
        assert!(!bounds.contains(&Point2D::new(11.0, 2.5)));
    }
}
