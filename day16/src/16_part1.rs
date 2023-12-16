use std::collections::HashSet;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinates {
    y: usize,
    x: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

fn main() {

    let input:&str = include_str!("../input/16.txt");
    let map: Vec<&str> = input.lines().collect();
    let mut visited_states = Vec::new();

    path(Direction::East, &Coordinates{x:0,y:0}, &map, &mut visited_states);

    let unique_coordinates: HashSet<Coordinates> = visited_states
        .iter()
        .cloned()
        .map(|(coordinates, _)| coordinates)
        .collect();

    let count_unique_coordinates = unique_coordinates.len();

    println!("{}", count_unique_coordinates);
}

fn path(direction: Direction, coordinates: &Coordinates, map: &Vec<&str>, visited: &mut Vec<(Coordinates, Direction)>) {
    if let Some(curr_spot) = map.get(coordinates.y).and_then(|line| line.chars().nth(coordinates.x)) {
        if !visited.contains(&(coordinates.clone(), direction))  {
            visited.push((coordinates.clone(), direction));
            match direction {
                Direction::North => {
                    match curr_spot {
                        '-' => {
                            if let Some(result) = coordinates.x.checked_sub(1) {
                                path(Direction::West, &Coordinates { y: coordinates.y, x: result }, map, visited);
                            }
                            if coordinates.x + 1 < map.len(){
                                path(Direction::East, &Coordinates { y: coordinates.y,  x: coordinates.x +1 }, map, visited);
                            }
                        },

                        '\\' => {
                            if let Some(result) = coordinates.x.checked_sub(1) {
                                path(Direction::West, &Coordinates { y: coordinates.y, x: result }, map, visited);
                            }
                        },
                        '/' => {
                            if coordinates.x + 1 < map.len(){
                                path(Direction::East, &Coordinates { y: coordinates.y,  x: coordinates.x +1 }, map, visited);
                            }
                        },
                        _ => {
                            if let Some(result) = coordinates.y.checked_sub(1) {
                                path(Direction::North, &Coordinates { y: result, x: coordinates.x }, map, visited);
                            }
                        }
                    }

                }
                Direction::East => {
                    match curr_spot {
                        '|' => {
                            if let Some(result) = coordinates.y.checked_sub(1) {
                                path(Direction::North, &Coordinates { y: result, x: coordinates.x }, map, visited);
                            }
                            if coordinates.y + 1 < map.len(){
                                path(Direction::South, &Coordinates { y: coordinates.y + 1, x: coordinates.x }, map, visited);
                            }
                        },

                        '\\' => {
                            if coordinates.y + 1 < map.len(){
                                path(Direction::South, &Coordinates { y: coordinates.y + 1, x: coordinates.x }, map, visited);
                            }
                        },
                        '/' => {
                            if let Some(result) = coordinates.y.checked_sub(1) {
                                path(Direction::North, &Coordinates { y: result, x: coordinates.x }, map, visited);
                            }
                        },
                        _ => {
                            if coordinates.x + 1 < map.len(){
                                        path(Direction::East, &Coordinates { y: coordinates.y, x: coordinates.x + 1 }, map, visited);
                            }
                        }
                    }

                }
                Direction::South => {
                    match curr_spot {
                        '-' => {
                            if let Some(result) = coordinates.x.checked_sub(1) {
                                path(Direction::West, &Coordinates { y: coordinates.y, x: result }, map, visited);
                            }
                            if coordinates.x + 1 < map.len(){
                                path(Direction::East, &Coordinates { y: coordinates.y,  x: coordinates.x +1 }, map, visited);
                            }
                        },

                        '\\' => {
                            if coordinates.x + 1 < map.len(){
                                path(Direction::East, &Coordinates { y: coordinates.y,  x: coordinates.x +1 }, map, visited);
                            }
                        },
                        '/' => {
                            if let Some(result) = coordinates.x.checked_sub(1) {
                                path(Direction::West, &Coordinates { y: coordinates.y, x: result }, map, visited);
                            }
                        },
                        _ => {
                            if coordinates.y + 1 < map.len(){
                                path(Direction::South, &Coordinates { y: coordinates.y+1, x: coordinates.x }, map, visited);
                            }
                        }
                    }
                }
                Direction::West => {
                    match curr_spot {
                        '|' => {
                            if let Some(result) = coordinates.y.checked_sub(1) {
                                path(Direction::North, &Coordinates { y: result, x: coordinates.x }, map, visited);
                            }
                            if coordinates.y + 1 < map.len(){
                                path(Direction::South, &Coordinates { y: coordinates.y + 1, x: coordinates.x }, map, visited);
                            }
                        },

                        '\\' => {
                            if let Some(result) = coordinates.y.checked_sub(1) {
                                path(Direction::North, &Coordinates { y: result, x: coordinates.x }, map, visited);
                            }
                        },
                        '/' => {
                            if coordinates.y + 1 < map.len(){
                                path(Direction::South, &Coordinates { y: coordinates.y + 1, x: coordinates.x }, map, visited);
                            }
                        },
                        _ => {
                            if let Some(result) = coordinates.x.checked_sub(1) {
                                path(Direction::West, &Coordinates { y: coordinates.y, x: result }, map, visited);
                            }
                        }
                    }
                }
            }
        }
    }
}