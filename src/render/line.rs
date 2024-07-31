use bevy::prelude::{Component,Vec3};

#[derive(Component,Clone)]
pub struct Line{
    pub from: Vec3,
    pub to: Vec3
}
