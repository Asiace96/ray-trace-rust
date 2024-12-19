use std::ops::{Add, AddAssign, Neg, Sub, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Formatter, Result};

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
      return f64::sqrt(self.length_squared());
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
