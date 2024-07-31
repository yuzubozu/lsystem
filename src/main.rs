use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy::{
    prelude::*,
    window::PresentMode,
};
use std::collections::HashMap;
use bevy_lsystem::lsystem::pattern::LsystemPattern;
use bevy_lsystem::lsystem::evolve::evolve;
use bevy_lsystem::render::calculation::*;
use bevy_lsystem::render::line::Line;
use bevy_lsystem::render::execute::execute_generation;
const WINDOW_WIDTH:f32=500.;
const WINDOW_HEIGHT:f32=500.;
const GRAPH_MARGIN_RATE:f32=0.8;
const GENERATION_MAX:i32=5;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "lsystem".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                canvas: Some("#mygame-canvas".into()),
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins(GenerationSetupPlugin)
        .add_systems(Startup,(setup,spawn_limit_text,spawn_explanation_text))
        .add_systems(Update,(
            l_system,
            render_line,
            change_limit_text,
            reset,
            change_explanation_text
        ))
        .run();
}

pub struct GenerationSetupPlugin;

impl Plugin for GenerationSetupPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GenerationData::init_random_pattern());

        app.insert_resource(Correction{
            vec: Vec3{
                x:0.0,
                y:0.0,
                z:0.0
            },
            length_modifier: 1.0
        });
    }
}

#[derive(Resource)]
struct GenerationData{
    name: String,
    times: i32,
    limit: i32,
    generated: String,
    angle:f32,
    rules:HashMap<String,String>,
    axiom:String,
    first_angle: f32
}

impl GenerationData{
    fn init_from_pattern(pattern:LsystemPattern)->GenerationData{
        let mut rules = HashMap::new();

        for  rule in pattern.rules{
            rules.insert(rule.from,rule.to);
        }

        GenerationData{
            name: pattern.name,
            rules: rules,
            angle: pattern.angle,
            generated:pattern.axiom.clone(),
            times: 0,
            limit: GENERATION_MAX,
            axiom:pattern.axiom.clone(),
            first_angle: pattern.first_angle,
        }
    }

    fn init_random_pattern()->GenerationData{
        let pattern=LsystemPattern::init_random_pattern();
        Self::init_from_pattern(pattern)
    }

    fn reset_random_pattern(&mut self){
        let data = Self::init_random_pattern();
        self.name = data.name;
        self.rules = data.rules;
        self.angle = data.angle;
        self.generated = data.generated;
        self.times = data.times;
        self.limit = data.limit;
        self.first_angle = data.first_angle;
        self.axiom = data.axiom;
    }
}

//Component to identify text
#[derive(Component)]
struct ExplainText;

#[derive(Component)]
struct GenerationLimitText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });
}

/**
 * functions to render limit text
 */

fn spawn_limit_text(
    mut commands: Commands,
    generation_data: ResMut<GenerationData>
){
    let string = String::from("generation rest:") + &(generation_data.limit-generation_data.times).to_string();
    commands.spawn(
        (
            TextBundle::from_section(
                string,
                TextStyle {..default() },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            }),
            GenerationLimitText
        )
    );
}

/**
 * functions to render texts
 */
fn spawn_explanation_text(
    mut commands: Commands,
    generation_data: ResMut<GenerationData>
){
    let string = generate_explanation_text(generation_data);
    commands.spawn(
        (
            TextBundle::from_section(
                string,
                TextStyle { ..default() },
            ),
            ExplainText
        )
    );
}

fn change_explanation_text(mut query: Query<&mut Text,&ExplainText>,generation_data: ResMut<GenerationData>) {
    let string = generate_explanation_text(generation_data);
    for mut text in query.iter_mut() {
        text.sections[0].value=string.clone();
    }
}

fn generate_explanation_text(generation_data: ResMut<GenerationData>)->String{
    let mut string = String::from("Push [Enter] to generate lsystem graph.");

    string = string+"\nPush [r] to reset lsystem graph.";
    string = string+"\n";
    string = string+"\rname:"+&generation_data.name;
    string = string+"\nangle:"+&generation_data.angle.to_string();
    string = string+"\naxiom:"+&generation_data.axiom;
    string = string+"\nrules:";
    for (from,to) in &generation_data.rules{
        string = string+"\n from:"+from+"  to:"+to;
    }

    string
}


fn change_limit_text(mut query: Query<&mut Text,&GenerationLimitText>,generation_data: ResMut<GenerationData>) {
    let string = String::from("generation rest:") + &(generation_data.limit-generation_data.times).to_string();
    for mut text in query.iter_mut() {
        text.sections[0].value=string.clone();
    }
}

/**
 * support functions to render line.
 */
fn render_line(
    mut draw_line: ResMut<DebugLines>,
    correction: ResMut<Correction>,
    query: Query<&Line, With<Line>>
){
    for line_data in &query {
        draw_line.line(
            (line_data.from - correction.vec)*correction.length_modifier,
            (line_data.to - correction.vec)*correction.length_modifier,
            0.0
        );
    }
}

/**
 * functions for reset
 */
fn reset(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut generation_data: ResMut<GenerationData>,
    query: Query<Entity, With<Line>>,
){
    if keys.just_pressed(KeyCode::R){
        for line_entity in &query {
            commands.entity(line_entity).despawn();
        }
        generation_data.reset_random_pattern();
    }
}


/**
 * functions for l_system
 */

fn l_system(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut generation_data: ResMut<GenerationData>,
    mut correction: ResMut<Correction>,
    query: Query<Entity, With<Line>>,
    ){

    if keys.just_pressed(KeyCode::Return){
        let mut lines = Vec::new();
        if generation_data.times< generation_data.limit{
            generation_data.generated = evolve(generation_data.generated.clone(),generation_data.rules.clone());
            lines = execute_generation(generation_data.generated.clone(),generation_data.angle,generation_data.first_angle);
            generation_data.times += 1;
    
            for line_entity in &query {
                commands.entity(line_entity).despawn();
            }
            let next_correction = calc_correction(&lines,Vec3::new(WINDOW_WIDTH,WINDOW_HEIGHT,0.),GRAPH_MARGIN_RATE);
            correction.vec = next_correction.vec;
            correction.length_modifier = next_correction.length_modifier;
        }
        
        for line in lines{
            commands.spawn(line);
        }
    }
}