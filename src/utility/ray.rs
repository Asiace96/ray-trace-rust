use crate::utility::vec3::Vec3;
use crate::utility::vec3::Point3;

#[derive(Default, Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {

    pub fn new(orig: Point3, dir: Vec3 ) -> Self {
        Ray {orig: orig,
             dir: dir 
        }
    }

    pub fn origin(&self) -> Point3 {
        return self.orig;
    }

    pub fn direction(&self) -> Vec3 {
        return self.dir;
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t*self.dir;
    }
}

