use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Null,
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "idle"),
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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
    pub players: HashMap<String, Position>,
    pub squares: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(
        players: HashMap<String, Position>,
        squares: Vec<Vec<Square>>,
    ) -> Result<Self, String> {
        let height = squares.len();
        if height == 0 {
            return Err("no squares".to_owned());
        }
        let width = squares[0].len();
        for (i, row) in squares.iter().enumerate() {
            if row.len() != width {
                return Err(format!(
                    "First row is {} wide but row {} is {} wide",
                    width,
                    i,
                    row.len()
                ));
            }
        }
        let mut map = Self {
            players,
            squares,
            width,
            height,
        };
        for (name, pos) in &map.players {
            let mut square = &mut map.squares[pos.y as usize][pos.x as usize];
            if let Some(player) = &square.occupied_by_player {
                return Err(format!(
                    "{} is occupied by both {} and {}",
                    pos, player, name
                ));
            }
            square.occupied_by_player = Some(name.to_string());
        }
        Ok(map)
    }

    pub fn is_inside(&self, pos: Position) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.width && pos.y >= 0 && (pos.y as usize) < self.height
    }

    pub fn square(&self, pos: Position) -> Option<&Square> {
        if self.is_inside(pos) {
            Some(&self.squares[pos.y as usize][pos.x as usize])
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
