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