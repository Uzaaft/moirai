use std::ops::{Add, Mul};

use crate::area::Area;

#[derive(PartialEq)]
pub struct Length {
    /// Value of the Length
    Value: f64,
}

impl Length {
    pub fn get(&self) -> f64 {
        self.Value
    }
}

#[derive(PartialEq, Eq)]
pub enum MetricUnit {
    /// Conversion factor for feet to meter
    Feet,
    /// Conversion factor for meter to meter
    Meter,
}

impl From<MetricUnit> for f64 {
    fn from(unit: MetricUnit) -> f64 {
        match unit {
            MetricUnit::Feet => 0.3048,
            MetricUnit::Meter => 1.0,
        }
    }
}

impl Mul<Length> for Length {
    type Output = Area;

    fn mul(self, rhs: Length) -> Area {
        Area {
            Value: self.Value * rhs.Value,
        }
    }
}

impl Add<Length> for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Length {
        Length {
            Value: self.Value + rhs.Value,
        }
    }
}

/// Impl new for Length, with the generic B which is the type MetricUnit, and which created a
/// Length Struct with the given value multiplied by the enum
impl Length {
    /// Creates a new [`Length`].
    /// Pass on the given unit Enum as a generic, like this:
    /// /// ```rust
    /// use moirai;
    ///
    ///
    /// fn main(){
    ///    let metric_length = moirai::Length::new::<{moirai::MetricUnit::Meter}>(1.0)
    ///    let imperial_lengt = Length::new::<{ MetricUnit::Feet }>(1.0);
    /// }
    /// ```
    pub fn new<const B: MetricUnit>(value: f64) -> Length {
        Length {
            Value: value * f64::from(B),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        let a = Length::new::<{ MetricUnit::Meter }>(1.0);
        let b = Length::new::<{ MetricUnit::Feet }>(1.0);
        let answer = Length::new::<{ MetricUnit::Meter }>(1.3048);
        approx::assert_abs_diff_eq!((a + b).get(), answer.get());
    }
}
