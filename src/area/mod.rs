use std::ops::{Add, Mul};

#[derive(PartialEq)]
pub struct Area {
    /// Value of the area
    pub value: f64,
}

#[derive(PartialEq, Eq)]
pub enum AreaUnit {
    /// Conversion factor for $m^2$ to ${ft}^2$
    MeterSquared,
    /// Conversion factor for meter to meter
    FeetSquared,
}

impl From<AreaUnit> for f64 {
    fn from(unit: AreaUnit) -> f64 {
        match unit {
            AreaUnit::MeterSquared => 1.0,
            AreaUnit::FeetSquared => 1.0 / 10.7639,
        }
    }
}

impl Area {
    pub fn get(&self) -> f64 {
        self.value
    }
}

/// Impl new for Area, with the generic B which is the type AreaUnit, and which created a
/// Area Struct with the given value multiplied by the enum
impl Area {
    /// Creates a new [`Area`].
    /// Pass on the given unit Enum as a generic, like this:
    /// /// ```rust
    /// use moirai;
    ///
    ///
    /// fn main(){
    ///    let metric_length = moirai::Area::new::<{moirai::AreaUnit::Meter}>(1.0)
    ///    let imperial_lengt = Area::new::<{ AreaUnit::Feet }>(1.0);
    /// }
    /// ```
    pub fn new<const B: AreaUnit>(value: f64) -> Area {
        Area {
            value: value * f64::from(B),
        }
    }
}

impl Add<Area> for Area {
    type Output = Area;

    fn add(self, rhs: Area) -> Area {
        Area {
            value: self.value + rhs.value,
        }
    }
}
