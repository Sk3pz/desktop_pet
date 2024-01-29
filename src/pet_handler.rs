use std::fmt::Display;
use crate::config::PetConfigSection;
use crate::gif::Gif;

#[derive(Clone)]
pub enum PetType {
    Dog,
}

impl PetType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "dog" => Some(PetType::Dog),
            _ => None,
        }
    }

    pub fn valid_pets() -> Vec<String> {
        vec![
            Self::Dog.to_string(),
        ]
    }
}

impl Display for PetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PetType::Dog => write!(f, "dog"),
        }
    }
}

#[derive(Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    // todo: more directions with better animations
    Stationary,
}

impl Direction {
    pub fn to_animation_name(&self, pet_type: PetType) -> String {
        match self {
            Direction::Left => format!("{}_run_left.gif", pet_type),
            Direction::Right => format!("{}_run_right.gif", pet_type),

            Direction::Stationary => format!("{}_stationary.gif", pet_type),
        }
    }
}

pub struct PetState {
    pub name: String,
    pub pet_type: PetType,
    pub pos: Pos,
    pub direction: Direction,
}

impl PetState {
    pub fn new(pet: PetConfigSection, initial_pos: Pos) -> Self {
        let pet_type = PetType::from_str(&pet.pet_type).unwrap();
        Self {
            name: pet.name,
            pet_type,
            pos: initial_pos,
            direction: Direction::Stationary,
        }
    }

    pub fn update_from_config(&mut self, pet: PetConfigSection) {
        self.name = pet.name;
        self.pet_type = PetType::from_str(&pet.pet_type).unwrap();
    }

    pub fn get_gif(&self) -> Gif {
        Gif::new(format!("./data/assets/{}", self.direction.to_animation_name(self.pet_type.clone())))
    }

    pub fn update(&mut self) {
        todo!()
    }

}
