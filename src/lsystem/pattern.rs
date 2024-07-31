use serde::Deserialize;
use rand::Rng;

#[derive(Deserialize,Default,Clone)]
pub struct LsystemPattern{
    pub name:String,
    pub rules:Vec<LsystemRules>,
    pub angle:f32,
    pub first_angle:f32,
    pub axiom:String
}

impl LsystemPattern{
    pub fn init_random_pattern()->LsystemPattern{
        let input_fn =include_str!("./patterns.json");
        let deserialized: Vec<LsystemPattern> = serde_json::from_str(&input_fn).unwrap();
        let mut rng = rand::thread_rng();
        let index:usize = rng.gen_range(0..deserialized.len()); 
        deserialized[index].clone()
    }
}

#[derive(Deserialize,Default,Clone)]
pub struct LsystemRules{
    pub from:String,
    pub to:String
}