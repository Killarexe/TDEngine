#[derive(Clone, Copy, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vector2<f32> {
    pub fn round(&self) -> Vector2<i32> {
        Vector2::new(self.x.round() as i32, self.y.round() as i32)
    }
}

impl<T> Into<Vector2<T>> for (T, T) {
    fn into(self) -> Vector2<T> {
        Vector2::new(self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z}
    }
}

impl Vector3<f32> {
    pub fn projected(&self, fov: f32) -> Vector2<f32> {
        let mut z_fov: f32 = self.z + fov;
        if z_fov == 0.0 {
            z_fov = 0.001;
        }
        Vector2::new((self.x * fov) / z_fov, (self.y * fov) / z_fov)
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let (sin, cos): (f32, f32) = angle.sin_cos();
        Self {
            x: self.x,
            y: cos * self.y - sin * self.z,
            z: sin * self.y + cos * self.z
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let (sin, cos): (f32, f32) = angle.sin_cos();
        Self {
            x: cos * self.x - sin * self.z,
            y: self.y,
            z: sin * self.x + cos * self.z
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let (sin, cos): (f32, f32) = angle.sin_cos();
        Self {
            x: cos * self.x - sin * self.y,
            y: sin * self.x + cos * self.y,
            z: self.z
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Self { x: self.x * scale, y: self.y * scale, z: self.z * scale }
    }
}

impl<T> Into<Vector3<T>> for (T, T, T) {
    fn into(self) -> Vector3<T> {
        Vector3::new(self.0, self.1, self.2)
    }
}
