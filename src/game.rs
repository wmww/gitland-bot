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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug)]
pub struct Player {
    pub team: Team,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Square {
    pub controlled_by: Team,
    //pub occupied_by_player: Option<String>,
}

impl Square {
    pub fn new(team: Team) -> Self {
        Self {
            controlled_by: team,
            //occupied_by_player: None,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub players: HashMap<String, Position>,
    pub squares: Vec<Vec<Square>>,
}

impl Map {
    pub fn square(&self, pos: Position) -> Option<Square> {
        if pos.y >= 0 && (pos.y as usize) < self.squares.len() {
            if pos.x >= 0 && (pos.x as usize) < self.squares[pos.y as usize].len() {
                Some(self.squares[pos.y as usize][pos.x as usize].clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub us: Option<String>,
    pub players: HashMap<String, Player>,
    pub timeline: Vec<Map>,
}

impl Game {
    pub fn our_position(&self) -> Position {
        let map = self.map();
        let us = map
            .players
            .get(self.us.as_ref().expect("we do not have a player"))
            .expect("we are not on the map");
        *us
    }

    pub fn our_team(&self) -> Team {
        let us = self
            .players
            .get(self.us.as_ref().expect("we do not have a player"))
            .expect("we are not in the game");
        us.team
    }

    pub fn map(&self) -> &Map {
        self.timeline.last().expect("timeline empty")
    }
}
