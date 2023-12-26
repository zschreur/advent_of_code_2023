use crate::grid::*;
use std::collections::{BTreeMap, BTreeSet};

/*
  Djikstra's algorithm
*/

/*
   1. Create a graph of the maze
   2. Find the shortest path from the start to the end
   3. Return the length of the path
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    point: Point,
    count: usize,
    direction: Direction,
}

impl Node {
    fn new(point: Point, direction: Direction, count: usize) -> Self {
        Self {
            point,
            direction,
            count,
        }
    }
}

fn djikstra(grid: &Grid<usize>) -> Option<usize> {
    let mut visited_nodes = BTreeSet::<Node>::new();
    let mut unvisited_nodes = BTreeMap::<usize, BTreeSet<Node>>::new();
    unvisited_nodes.insert(
        *grid.get(Point(0, 1)).unwrap(),
        BTreeSet::from_iter(vec![Node::new(Point(0, 1), Direction::Down, 1)]),
    );
    unvisited_nodes.insert(
        *grid.get(Point(1, 0)).unwrap(),
        BTreeSet::from_iter(vec![Node::new(Point(1, 0), Direction::Down, 1)]),
    );

    loop {
        let (current_node, current_cost) = {
            let mut min_value = match unvisited_nodes.first_entry() {
                Some(min_value) => min_value,
                None => {
                    break;
                }
            };
            let cost = *min_value.key();
            let node = match min_value.get_mut().pop_first() {
                Some(node) => node,
                _ => {
                    break;
                }
            };

            if min_value.get().is_empty() {
                min_value.remove_entry();
            }

            (node, cost)
        };
        if current_node.point.0 == grid.size() - 1 && current_node.point.1 == grid.size() - 1 {
            return Some(current_cost);
        }

        // Vec of nodes and the calculated cost to get there
        let neighbors = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|direction| match (**direction, current_node.direction) {
            (Direction::Up, Direction::Down) => false,
            (Direction::Down, Direction::Up) => false,
            (Direction::Left, Direction::Right) => false,
            (Direction::Right, Direction::Left) => false,
            _ => true,
        })
        .filter_map(|direction| {
            if direction == &current_node.direction {
                if current_node.count >= 3 {
                    None
                } else {
                    Some((*direction, current_node.count + 1))
                }
            } else {
                Some((*direction, 1))
            }
        })
        .filter_map(|(direction, count)| {
            current_node
                .point
                .move_direction(direction)
                .and_then(|point| {
                    grid.get(point).map(|v| {
                        (
                            Node {
                                point,
                                direction,
                                count,
                            },
                            v + current_cost,
                        )
                    })
                })
        })
        .collect::<Vec<_>>();

        neighbors
            .iter()
            .filter(|(node, _)| !visited_nodes.contains(node))
            .for_each(|(node, value)| {
                match unvisited_nodes.range_mut(value + 1..).find_map(|(c, v)| {
                    if v.contains(node) {
                        Some(*c)
                    } else {
                        None
                    }
                }) {
                    Some(v) => {
                        unvisited_nodes.get_mut(&v).unwrap().remove(node);
                        if unvisited_nodes.get(&v).unwrap().is_empty() {
                            unvisited_nodes.remove(&v);
                        }
                    }
                    None => (),
                }
                unvisited_nodes
                    .entry(*value)
                    .or_insert_with(BTreeSet::new)
                    .insert(*node);
            });

        visited_nodes.insert(current_node);
    }

    None
}

pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        let blocks = self
            .0
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::<_>::new(blocks.len(), blocks);
        let res = djikstra(&grid)
            .ok_or("No path found")
            .map(|v| super::AOCResult::ULong(v as u128))?;

        Ok(res)
    }

    fn run_part_two(&self) -> Result<super::AOCResult, Box<dyn std::error::Error>> {
        Err("Not implemented".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test() {
        let blocks = SAMPLE_INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::<_>::new(SAMPLE_INPUT.lines().count(), blocks);
        assert!(djikstra(&grid).is_some());
        assert_eq!(djikstra(&grid).unwrap(), 102);
    }
}