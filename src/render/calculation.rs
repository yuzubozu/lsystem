use bevy::prelude::{Resource,Vec3};
use super::line::Line;

#[derive(Resource)]
pub struct Correction{
    pub vec: Vec3,
    pub length_modifier: f32 
}

pub fn calc_correction(lines:&Vec<Line>,max:Vec3,rate:f32)->Correction{
    let correction_vec = calc_correction_vec(lines);
    let modifier = calc_correction_modifier(lines,max,rate);
    Correction{
        vec:correction_vec,
        length_modifier:modifier
    }
}

pub fn calc_correction_vec(lines:&Vec<Line>)->Vec3{
    let vec_sum = lines.into_iter()
        .fold(
            Vec3{x:0.0,y:0.0,z:0.0},
            |v,l| v+(l.from+l.to)/2.0);
    vec_sum / (lines.len() as f32)
}

pub fn calc_correction_modifier(lines:&Vec<Line>,max:Vec3,rate:f32)->f32{
    let (x_max,y_max) = lines.into_iter().fold((f32::MIN,f32::MIN),|(xi,yi),l|(xi.max(l.from.x).max(l.to.x),yi.max(l.from.y).max(l.to.y)));
    let (x_min,y_min) = lines.into_iter().fold((f32::MAX,f32::MAX),|(xi,yi),l|(xi.min(l.from.x).min(l.to.x),yi.min(l.from.y).min(l.to.y)));
    let (x_dif,y_dif) = (x_max-x_min,y_max-y_min);
    (max.x*rate/x_dif).min(max.y*rate/y_dif)
}