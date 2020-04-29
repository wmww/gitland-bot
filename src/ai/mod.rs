use crate::*;
use std::collections::HashSet;

fn count_good_squares_in_direction(
    map: &Map,
    starting: Position,
    vector: Position,
    our_team: Team,
    falloff: f32,
) -> f32 {
    let mut pos = starting;
    let mut score = 0.0;
    let mut value = 1.0;
    loop {
        pos = pos + vector;
        if let Some(square) = map.square(pos) {
            if square.occupied_by_player.is_none() {
                if square.controlled_by == Team::Null {
                    score += value * 1.2;
                } else if square.controlled_by != our_team {
                    score += value;
                } else {
                    score += value * 0.01;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        value *= falloff;
    }
    score
}

fn find_enimy_square(map: &Map, starting: Position, our_team: Team) -> Position {
    for range in 1..100 {
        eprintln!("looking within {} square of position", range);
        for dx in -range..range + 1 {
            for dy in -range..range + 1 {
                if i32::abs(dx) + i32::abs(dy) <= range {
                    let pos = starting + Position::new(dx, dy);
                    eprintln!("scanning {}", pos);
                    if let Some(square) = map.square(pos) {
                        if square.controlled_by != our_team && square.occupied_by_player.is_none() {
                            return pos;
                        }
                    }
                }
            }
        }
    }
    Position::new(2, 2)
}

pub fn run(game: &Game) -> Direction {
    let pos = game.our_position();
    eprintln!("We are at {}", pos);
    let map = game.map();
    let team = game.our_team();
    // let target = find_enimy_square(map, pos, team);
    let directions: Vec<(f32, Direction)> = vec![
        (Position::new(-1, 0), Direction::Left),
        (Position::new(1, 0), Direction::Right),
        (Position::new(0, -1), Direction::Up),
        (Position::new(0, 1), Direction::Down),
    ]
    .iter()
    .map(|(vector, direction)| {
        (
            count_good_squares_in_direction(map, pos, *vector, team, 0.7),
            *direction,
        )
    })
    .collect();
    for (score, direciton) in &directions {
        eprintln!("{:?} score is {}", direciton, score);
    }
    let (_score, best) = directions
        .iter()
        .fold((0.0, Direction::Left), |best, current| {
            if current.0 > best.0 {
                *current
            } else {
                best
            }
        });
    best
}
