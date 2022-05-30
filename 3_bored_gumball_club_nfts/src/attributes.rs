use sbor::*;

// Color attributes
#[derive(TypeId, Encode, Decode, Describe)]
pub enum Color {
    Blue,
    Yellow,
    Red
}

// Hat attributes
#[derive(TypeId, Encode, Decode, Describe)]
pub enum Hat {
    Beanie,
    Cowboy,
    Party
}

// Eye attributes
#[derive(TypeId, Encode, Decode, Describe)]
pub enum Eyes {
    Laser,
    Sleepy,
    Eyepatch
}

// Functions allowing us to convert an integer to an attribute
impl Color {
    pub fn from(value: u32) -> Color {
        match value {
            1 => Color::Blue,
            2 => Color::Yellow,
            3 => Color::Red,
            _ => panic!("Invalid color value!")
        }
    }
}

impl Hat {
    pub fn from(value: u32) -> Hat {
        match value {
            1 => Hat::Beanie,
            2 => Hat::Cowboy,
            3 => Hat::Party,
            _ => panic!("Invalid hat value!")
        }
    }
}

impl Eyes {
    pub fn from(value: u32) -> Eyes {
        match value {
            1 => Eyes::Laser,
            2 => Eyes::Sleepy,
            3 => Eyes::Eyepatch,
            _ => panic!("Invalid hat value!")
        }
    }
}