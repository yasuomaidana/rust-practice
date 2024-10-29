#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
}

pub struct Angle {
    value: f64,
    radians: bool,
}

impl Angle {
    pub(crate) fn new(value: f64) -> Angle {
        Angle { value, radians: false }
    }
    fn to_radians(&self) -> f64 {
        if self.radians {
            self.value
        } else {
            self.value.to_radians()
        }
    }

    fn to_degrees(&self) -> f64 {
        if self.radians {
            self.value.to_degrees()
        } else {
            self.value
        }
    }

    pub(crate) fn from_ratio(dx:f64, dy:f64) -> Angle {
        Angle { value: dy.atan2(dx), radians: true }
    }
}

impl Vector {
    pub(crate) fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn from_angle(magnitude: f64, angle: Angle) -> Vector {
        Vector {
            x: magnitude * angle.to_radians().cos(),
            y: magnitude * angle.to_radians().sin(),
        }
    }

    pub(crate) fn from_ratio(magnitude: f64, dx: f64, dy:f64) -> Vector {
        Self::from_angle(magnitude, Angle::from_ratio(dx, dy))
    }

    pub(crate) fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub(crate) fn angle(&self) -> f64 {
        Angle::from_ratio(self.x, self.y).to_degrees()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_from_ration() {
        let dx = 0.0;
        let dy = 4.0;
        let angle = Angle::from_ratio(dx, dy);
        assert_eq!(angle.value, std::f64::consts::FRAC_PI_2);

        let dy = -4.0;
        let angle = Angle::from_ratio(dx, dy);
        assert_eq!(angle.value, -std::f64::consts::FRAC_PI_2);

        let dx = 3.0;
        let dy = 0.0;
        let angle = Angle::from_ratio(dx, dy);
        assert_eq!(angle.value, 0.0);

        let dx = -3.0;
        let dy = -3.0;
        let angle = Angle::from_ratio(dx, dy);
        let expected_angle = -std::f64::consts::FRAC_PI_4 * 3.0;
        assert_eq!(angle.value, expected_angle);
    }
}