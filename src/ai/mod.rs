use crate::*;

const ZONE_SIZE: i32 = 7;
const DEFEND_RANGE: i32 = 14;

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
    if vector.x == 0 && vector.y == 0 {
        return 0.0;
    }
    loop {
        pos = pos + vector;
        if pos.x >= ZONE_SIZE || pos.y >= ZONE_SIZE {
            break;
        }
        if let Some(square) = map.square(pos) {
            if square.occupied_by_player.is_none() {
                if square.controlled_by == Team::Null {
                    score += value * 0.8;
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

fn find_target_square(game: &Game) -> Option<(Position, String)> {
    let starting = game.our_position();
    if starting.x > ZONE_SIZE || starting.y > ZONE_SIZE {
        let target = Position::new(
            if starting.x > ZONE_SIZE {
                ZONE_SIZE
            } else {
                starting.x
            },
            if starting.y > ZONE_SIZE {
                ZONE_SIZE
            } else {
                starting.y
            },
        );
        Some((target, "enter zone".into()))
    } else {
        let mut threats = Vec::new();
        let map = game.map();
        let our_pos = game.our_position();
        for (name, position) in &game.map().players {
            let their_team = game.players[name].team;
            let they_own_left_of_them =
                map.controller_of(*position + Position::new(-1, 0)) == their_team;
            let they_own_top_of_them =
                map.controller_of(*position + Position::new(0, -1)) == their_team;
            let dist_from_us = our_pos.distance(*position) - DEFEND_RANGE;
            if their_team == game.our_team() || position.x == 0 || position.y == 0 {
                // Hack until non-active players are kicked
                continue;
            } else if position.x > ZONE_SIZE && position.y <= ZONE_SIZE && !they_own_left_of_them {
                let dist = dist_from_us + position.x - ZONE_SIZE;
                threats.push((dist, Position::new(ZONE_SIZE, position.y), name));
            } else if position.x <= ZONE_SIZE && position.y > ZONE_SIZE && !they_own_top_of_them {
                let dist = dist_from_us + position.y - ZONE_SIZE;
                threats.push((dist, Position::new(position.x, ZONE_SIZE), name));
            } else if position.x > ZONE_SIZE
                && position.y > ZONE_SIZE
                && (!they_own_left_of_them || !they_own_top_of_them)
            {
                let dist = dist_from_us + (position.x - ZONE_SIZE) + (position.y - ZONE_SIZE);
                threats.push((dist, Position::new(ZONE_SIZE, ZONE_SIZE), name));
            }
        }
        let biggest_threat = threats.iter().min_by_key(|threat| threat.0);
        if let Some(threat) = biggest_threat {
            if threat.0 < DEFEND_RANGE {
                Some((
                    threat.1,
                    format!("defend against {} at {}", threat.2, map.players[threat.2]),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn run(game: &Game) -> Direction {
    let pos = game.our_position();
    eprintln!("We are at {}", pos);
    let map = game.map();
    let team = game.our_team();
    let target = match find_target_square(game) {
        Some((target, reason)) => {
            eprintln!("Moving toward {} to {}", target, reason);
            Some(target)
        }
        None => {
            eprintln!("No enemies around");
            None
        }
    };
    // let target = find_enimy_square(map, pos, team);
    let directions: Vec<(f32, Direction)> = vec![
        (Position::new(0, 0), Direction::Null),
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
    .map(|(score, direction)| {
        if let Some(target) = target {
            let correct_way = match direction {
                Direction::Null => target == pos,
                Direction::Left => target.x < pos.x,
                Direction::Right => target.x > pos.x,
                Direction::Up => target.y < pos.y,
                Direction::Down => target.y > pos.y,
            };
            let score = score + if correct_way { 2.0 } else { 0.0 };
            (score, direction)
        } else {
            (score, direction)
        }
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
