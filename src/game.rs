use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Null,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Team {
    Null,
    Red,
    Green,
    Blue,
}

impl FromStr for Team {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cr" | "ur" => Ok(Self::Red),
            "cg" | "ug" => Ok(Self::Green),
            "cb" | "ub" => Ok(Self::Blue),
            "ux" => Ok(Self::Null),
            other => Err(format!("unknown team {:?}", other)),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Player {
    pub team: Team,
    pub name: String,
}

#[derive(Debug)]
pub struct Square {
    pub controlled_by: Team,
    pub occupied_by_player: Option<String>,
}

impl Square {
    pub fn new(team: Team) -> Self {
        Self {
            controlled_by: team,
            occupied_by_player: None,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub squares: Vec<Vec<Square>>,
}

#[derive(Debug)]
pub struct Game {
    pub us: Option<String>,
    pub players: HashMap<String, Player>,
    pub timeline: Vec<Map>,
}
