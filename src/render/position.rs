use bevy::prelude::Vec3;

#[derive(Default,Clone,Copy)]
pub struct Position{
    pub x:f32,
    pub y:f32
}

impl Position{
    pub fn to_vec3(&self)->Vec3{
        Vec3::new(self.x,self.y,0.0)
    }
}
