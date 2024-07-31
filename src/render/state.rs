use super::position::Position;

#[derive(Default,Clone,Copy)]
pub struct State{
    pub position: Position,
    pub angle: f32
}