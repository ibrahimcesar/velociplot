//! Tick mark generation for axes

/// A single tick mark with position and label
#[derive(Debug, Clone)]
pub struct Tick {
    /// Value in data coordinates
    pub value: f64,
    /// Label to display
    pub label: String,
    /// Whether this is a major tick
    pub major: bool,
}

/// Tick generator for creating nice tick values
pub struct TickGenerator;

impl TickGenerator {
    /// Generate evenly spaced ticks using a linear scale
    ///
    /// # Examples
    ///
    /// ```
    /// use velociplot::axes::TickGenerator;
    ///
    /// let ticks = TickGenerator::linear(0.0, 100.0, 5);
    /// assert!(ticks.len() >= 2); // At least 2 ticks
    /// assert!(ticks.first().unwrap().value <= 0.0);
    /// assert!(ticks.last().unwrap().value >= 100.0);
    /// ```
    #[must_use]
    pub fn linear(min: f64, max: f64, count: usize) -> Vec<Tick> {
        let count = count.max(2); // At least 2 ticks (min and max)
        let range = max - min;
        let step = Self::nice_number(range / (count - 1) as f64, false);

        let tick_min = (min / step).floor() * step;
        let tick_max = (max / step).ceil() * step;

        let mut ticks = Vec::new();
        let mut value = tick_min;

        while value <= tick_max + step * 0.5 {
            if value >= min - step * 0.5 && value <= max + step * 0.5 {
                ticks.push(Tick {
                    value,
                    label: format!("{value:.2}"),
                    major: true,
                });
            }
            value += step;
        }

        ticks
    }

    /// Generate logarithmic ticks
    #[allow(dead_code)]
    #[must_use]
    pub fn logarithmic(min: f64, max: f64, base: f64) -> Vec<Tick> {
        let log_min = min.max(1e-10).log(base).floor();
        let log_max = max.log(base).ceil();

        let mut ticks = Vec::new();
        let mut exp = log_min;

        while exp <= log_max {
            let value = base.powf(exp);
            ticks.push(Tick {
                value,
                label: format!("{value:.1e}"),
                major: true,
            });
            exp += 1.0;
        }

        ticks
    }

    /// Find a "nice" number approximately equal to x
    fn nice_number(x: f64, round: bool) -> f64 {
        let exp = x.log10().floor();
        let f = x / 10_f64.powf(exp);

        let nice_f = if round {
            if f < 1.5 {
                1.0
            } else if f < 3.0 {
                2.0
            } else if f < 7.0 {
                5.0
            } else {
                10.0
            }
        } else if f <= 1.0 {
            1.0
        } else if f <= 2.0 {
            2.0
        } else if f <= 5.0 {
            5.0
        } else {
            10.0
        };

        nice_f * 10_f64.powf(exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_ticks() {
        let ticks = TickGenerator::linear(0.0, 100.0, 5);
        assert!(!ticks.is_empty());
        assert!(ticks.len() >= 2); // At least min and max ticks

        // Check that ticks are in ascending order
        for i in 1..ticks.len() {
            assert!(ticks[i].value > ticks[i - 1].value);
        }

        // Check that ticks span the range
        assert!(ticks.first().unwrap().value <= 0.0);
        assert!(ticks.last().unwrap().value >= 100.0);
    }

    #[test]
    fn test_nice_number() {
        let nice = TickGenerator::nice_number(0.73, false);
        assert!((nice - 1.0).abs() < 0.01 || (nice - 0.5).abs() < 0.01);
    }
}
