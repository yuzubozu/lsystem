use super::state::State;
use super::line::Line;
use std::f32::consts::PI;

pub fn execute_generation(generation:String,angle:f32,first_angle:f32)->Vec<Line>{
    let mut state = State::default();
    state.angle += first_angle;
    let mut stack = Vec::new();
    let mut lines = Vec::new();

    for c in generation.chars(){
        state = match c {
            '['=>{
                stack.push(state);
                state
            },
            ']'=>{
                stack.pop().unwrap()
            }
            '+'=>execute_rotate(state,angle),
            '-'=>execute_rotate(state,-1.0 * angle),
            'F'=>{
                let new_state = execute_move(state,10.0);
                lines.push(Line{
                    from: state.position.to_vec3(),
                    to: new_state.position.to_vec3()
                });
                new_state
            }
            'G'=>execute_move(state,10.0),
            _ => state
        }
    }

    lines
}

fn execute_move(mut state:State,len: f32)->State{
    state.position.x += len * f32::cos(to_radian(state.angle));
    state.position.y += len * f32::sin(to_radian(state.angle));
    state
}

fn execute_rotate(mut state:State,deg:f32)->State{
    state.angle += deg;
    state
}

fn to_radian(degree:f32)->f32{
    degree/180.0 * PI
}