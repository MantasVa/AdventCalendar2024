use std::{collections::{HashMap, HashSet}, fs, str::FromStr};

use pathfinding::prelude::astar_bag_collect;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq)]
enum Type {
    Wall,
    Path,
}

#[derive(Default)]
struct Maze {
    map: HashMap<(i32, i32), Type>,
    start: (i32, i32),
    end: (i32, i32),
    dir: (i32, i32),
}

impl FromStr for Maze {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = Maze::default();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let coords = (row as i32, col as i32);
    
                match ch {
                    '#' => _ = map.map.insert(coords, Type::Wall),
                    '.' => _ = map.map.insert(coords, Type::Path),
                    'S' => {
                        map.start = coords;
                        map.map.insert(coords, Type::Path);
                        map.dir = (0, 1);
                    },
                    'E' => {
                        map.end = coords;
                        map.map.insert(coords, Type::Path);
                    },
                    _ => unreachable!()
                }
            }
        }

        Ok(map)
    }
}

impl Maze {
    fn find_paths_to_end(&self) -> (Vec<Vec<((i32, i32), (i32, i32))>>, u32) {
        astar_bag_collect(
            &(self.start, self.dir),
            |(curr_pos, curr_dir)| {
                let mut successors = Vec::new();
    
                let nextpos = (curr_pos.0 + curr_dir.0, curr_pos.1 + curr_dir.1);
                if let Some(t) = self.map.get(&nextpos) {
                    if *t == Type::Path
                    {
                        successors.push(((nextpos, *curr_dir), 1));
                    }
                }
    
                let next_dir_cl = Maze::turn_clockwise(*curr_dir);
                successors.push(((*curr_pos, next_dir_cl), 1000));
        
                let next_dir_ant = Maze::turn_anti_clockwise(*curr_dir);
                successors.push(((*curr_pos, next_dir_ant), 1000));
    
                successors.into_iter()
            },
            |(curr_pos, _)| {
                curr_pos.0.abs_diff(self.end.0) + curr_pos.1.abs_diff(self.end.1)
            },
            |(curr_pos, _)| *curr_pos == self.end).unwrap_or_default()
    }

    fn turn_clockwise(dir: (i32, i32)) -> (i32, i32) {
        match dir {
            //North -> East
            (-1, 0) => (0, 1),
            // East -> South
            (0, 1) => (1, 0),
            // South -> West
            (1, 0) => (0, -1),
            // West -> North
            (0, -1) => (-1, 0),
            _ => unreachable!(),
        }
    }

    fn turn_anti_clockwise(dir: (i32, i32)) -> (i32, i32) {
        match dir {
            // East -> North
            (0, 1) => (-1, 0),
            // North -> West
            (-1, 0) => (0, -1),
            // West -> South
            (0, -1) => (1, 0),
            // South -> East
            (1, 0) => (0, 1),
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let maze = Maze::from_str(input.as_str())?;
    let result = maze.find_paths_to_end();

    println!("Part 1 answer: {}", &result.1);
    println!("Part 2 answer: {}", &result.0.iter().flat_map(|x| x.iter().map(|x| x.0)).collect::<HashSet<(i32, i32)>>().len());

    Ok(())
}

/*
--- Day 16: Reindeer Maze ---
It's time again for the Reindeer Olympics! This year, the big event is the Reindeer Maze, where the Reindeer compete for the lowest score.

You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?

The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile (marked E). They can move forward one tile at a time (increasing their score by 1 point), but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by 1000 points).

To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:

###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
There are many paths through this maze, but taking any of the best paths would incur a score of only 7036. This can be achieved by taking a total of 36 steps forward and turning 90 degrees a total of 7 times:


###############
#.......#....E#
#.#.###.#.###^#
#.....#.#...#^#
#.###.#####.#^#
#.#.#.......#^#
#.#.#####.###^#
#..>>>>>>>>v#^#
###^#.#####v#^#
#>>^#.....#v#^#
#^#.#.###.#v#^#
#^....#...#v#^#
#^###.#.#.#v#^#
#S..#.....#>>^#
###############
Here's a second example:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
In this maze, the best paths cost 11048 points; following one such path would look like this:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################
Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.

Analyze your map carefully. What is the lowest score a Reindeer could possibly get?

--- Part Two ---
Now that you know what the best paths look like, you can figure out the best spot to sit.

Every non-wall tile (S, ., or E) is equipped with places to sit along the edges of the tile. While determining which of these tiles would be the best spot to sit depends on a whole bunch of factors (how comfortable the seats are, how far away the bathrooms are, whether there's a pillar blocking your view, etc.), the most important factor is whether the tile is on one of the best paths through the maze. If you sit somewhere else, you'd miss all the action!

So, you'll need to determine which tiles are part of any best path through the maze, including the S and E tiles.

In the first example, there are 45 tiles (marked O) that are part of at least one of the various best paths through the maze:

###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############
In the second example, there are 64 tiles that are part of at least one of the best paths:

#################
#...#...#...#..O#
#.#.#.#.#.#.#.#O#
#.#.#.#...#...#O#
#.#.#.#.###.#.#O#
#OOO#.#.#.....#O#
#O#O#.#.#.#####O#
#O#O..#.#.#OOOOO#
#O#O#####.#O###O#
#O#O#..OOOOO#OOO#
#O#O###O#####O###
#O#O#OOO#..OOO#.#
#O#O#O#####O###.#
#O#O#OOOOOOO..#.#
#O#O#O#########.#
#O#OOO..........#
#################
Analyze your map further. How many tiles are part of at least one of the best paths through the maze?
*/