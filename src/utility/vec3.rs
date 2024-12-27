use std::ops::{Add, AddAssign, Neg, Sub, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Formatter, Result};
use crate::utility::common;

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {

  pub fn new(x: f64, y: f64, z: f64) -> Self {
       Vec3 {
           x: x,
           y: y,
           z: z, 
       }
   }

  pub fn from_float(value: f64) -> Self {
       Vec3 {
           x: value,
           y: value,
           z: value, 
       }
   }

  pub fn length_squared(&self) -> f64 {
      return self.x*self.x + self.y*self.y + self.z*self.z;
  }

  pub fn length(&self) -> f64 {
      return self.length_squared().sqrt();
  }

  pub fn near_zero(&self) -> bool {
      // Return true if the vector is close to zero in all dimentions
      let s = 1.0e-8;
      return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
  }


  pub fn random() -> Self {
      Vec3 {
        x: common::random_double(),
        y: common::random_double(), 
        z: common::random_double()
      }
  }

  pub fn random_range(min: f64, max: f64) -> Self {
      Vec3 {
        x: common::random_double_range(min, max),
        y: common::random_double_range(min, max),
        z: common::random_double_range(min, max)
      }
  }

//   pub fn x(self) -> f32 {
//       self.x
//   }

//   pub fn y(self) -> f32 {
//       self.y
//   }

//   pub fn z(self) -> f32 {
//       self.z
//   }

}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3)  {
        *self = Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
         Vec3::new(self*v.x, self*v.y, self*v.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self  {
         Vec3::new(self.x*t, self.y*t, self.z*t)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64)  {
        *self = Vec3::new(self.x*t, self.y*t, self.z*t);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64)  {
        *self = Vec3::new(self.x/t, self.y/t, self.z/t);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self  {
        Vec3::new(self.x/t, self.y/t, self.z/t)
    }
}



pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.x*v.x + u.y*v.y + u.z*v.z;
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(u: Vec3) -> Vec3 {
    return u / u.length();
}

pub fn random_unit_vector() -> Vec3 {
    // Return a random vector inside the unit sphere
    loop {
        let p = Vec3::random_range(-1.0,1.0);
        if f64::MIN_POSITIVE <= p.length_squared() && p.length_squared() <= 1.0 {
            return unit_vector(p);
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    match dot(on_unit_sphere, *normal) > 0.0 { // In the same direction as the normal
        true => on_unit_sphere,
        false => -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0*dot(v,n)*n;
}

