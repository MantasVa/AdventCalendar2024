use std::{fs, str::FromStr};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Map {
    robots: Vec<Robot>,
    cols: i32,
    rows: i32,
}

#[derive(Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let robots = s.lines().map(| line | {
            let (left, right) = line.split_once(' ').unwrap();

            let (p_col, p_row) = left.split_once('=').unwrap().1.split_once(',').unwrap();
            let pos = (p_col.parse::<i32>().unwrap(), p_row.parse::<i32>().unwrap());

            let (v_col, v_row) = right.split_once('=').unwrap().1.split_once(',').unwrap();
            let velocity = (v_col.parse::<i32>().unwrap(), v_row.parse::<i32>().unwrap());

            Robot { pos, velocity }
        }).collect();

        Ok(Map { robots, cols: 11, rows: 7 })
    }
}

impl Map {
    fn move_x(mut self, seconds: u32, print: bool) -> usize {
        
        for s in 0..seconds {

            for (idx, robot) in self.robots.clone().iter().enumerate() {

                self.robots[idx] = self.move_robot(*robot);
            }

            if print {
                self.display_grid(s);
            }
        }

        self.get_safety_factor()
    }

    fn display_grid(&self, s: u32) {
        println!("Seconds {}", s);
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.robots.iter().find(|r| r.pos == (col, row)).is_some() {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    
        println!();
    }
    
    fn move_robot(&self, mut robot: Robot) -> Robot {

        let new_pos = (robot.pos.0 + robot.velocity.0, robot.pos.1 + robot.velocity.1); 

        if self.pos_is_out_of_bounds(new_pos) {
            let mut col = new_pos.0;
            let mut row = new_pos.1;

            if col < 0 {
                col = self.cols + col;
            }

            if col >= self.cols {
                col = col - self.cols;
            }

            if row < 0 {
                row = self.rows + row;
            }

            if row >= self.rows {
                row = row - self.rows;
            }

            robot.pos = (col, row);
        } else {
            robot.pos = new_pos;
        }

        robot
    }

    fn pos_is_out_of_bounds(&self, pos: (i32, i32)) -> bool {
        if pos.0 < 0 || pos.0 >= self.cols ||
           pos.1 < 0 || pos.1 >= self.rows {
            return true;
           }

        false
    }

    fn get_safety_factor(&self) -> usize {

        let row_mid = self.rows / 2;
        let col_mid = self.cols / 2;

        let q1 = self.robots.iter().filter(|r| r.pos.0 < col_mid && r.pos.1 < row_mid).count();
        let q2 = self.robots.iter().filter(|r| r.pos.0 > col_mid && r.pos.1 < row_mid).count();
        let q3 = self.robots.iter().filter(|r| r.pos.0 < col_mid && r.pos.1 > row_mid).count();
        let q4 = self.robots.iter().filter(|r| r.pos.0 > col_mid && r.pos.1 > row_mid).count();

        q1 * q2 * q3 * q4
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let map = Map::from_str(&input)?;

    _ = part1(map);

    Ok(())
}

fn part1(map: Map) -> Result<()> {
    let factor= map.move_x(100, false);

    println!("Part 1 answer: {}", factor);
    return Ok(());
}

/*
--- Day 14: Restroom Redoubt ---
One of The Historians needs to use the bathroom; fortunately, you know there's a bathroom near an unvisited location on their list, and so you're all quickly teleported directly to the lobby of Easter Bunny Headquarters.

Unfortunately, EBHQ seems to have "improved" bathroom security again after your last visit. The area outside the bathroom is swarming with robots!

To get The Historian safely to the bathroom, you'll need a way to predict where the robots will be in the future. Fortunately, they all seem to be moving on the tile floor in predictable straight lines.

You make a list (your puzzle input) of all of the robots' current positions (p) and velocities (v), one robot per line. For example:

p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
Each robot's position is given as p=x,y where x represents the number of tiles the robot is from the left wall and y represents the number of tiles from the top wall (when viewed from above). So, a position of p=0,0 means the robot is all the way in the top-left corner.

Each robot's velocity is given as v=x,y where x and y are given in tiles per second. Positive x means the robot is moving to the right, and positive y means the robot is moving down. So, a velocity of v=1,-2 means that each second, the robot moves 1 tile to the right and 2 tiles up.

The robots outside the actual bathroom are in a space which is 101 tiles wide and 103 tiles tall (when viewed from above). However, in this example, the robots are in a space which is only 11 tiles wide and 7 tiles tall.

The robots are good at navigating over/under each other (due to a combination of springs, extendable legs, and quadcopters), so they can share the same tile and don't interact with each other. Visually, the number of robots on each tile in this example looks like this:

1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1...
These robots have a unique feature for maximum bathroom security: they can teleport. When a robot would run into an edge of the space they're in, they instead teleport to the other side, effectively wrapping around the edges. Here is what robot p=2,4 v=2,-3 does for the first few seconds:

Initial state:
...........
...........
...........
...........
..1........
...........
...........

After 1 second:
...........
....1......
...........
...........
...........
...........
...........

After 2 seconds:
...........
...........
...........
...........
...........
......1....
...........

After 3 seconds:
...........
...........
........1..
...........
...........
...........
...........

After 4 seconds:
...........
...........
...........
...........
...........
...........
..........1

After 5 seconds:
...........
...........
...........
.1.........
...........
...........
...........
The Historian can't wait much longer, so you don't have to simulate the robots for very long. Where will the robots be after 100 seconds?

In the above example, the number of robots on each tile after 100 seconds has elapsed looks like this:

......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....
To determine the safest area, count the number of robots in each quadrant after 100 seconds. Robots that are exactly in the middle (horizontally or vertically) don't count as being in any quadrant, so the only relevant robots are:

..... 2..1.
..... .....
1.... .....
           
..... .....
...12 .....
.1... 1....
In this example, the quadrants contain 1, 3, 4, and 1 robot. Multiplying these together gives a total safety factor of 12.

Predict the motion of the robots in your list within a space which is 101 tiles wide and 103 tiles tall. What will the safety factor be after exactly 100 seconds have elapsed?

--- Part Two ---
During the bathroom break, someone notices that these robots seem awfully similar to ones built and used at the North Pole. If they're the same type of robots, they should have a hard-coded Easter egg: very rarely, most of the robots should arrange themselves into a picture of a Christmas tree.

What is the fewest number of seconds that must elapse for the robots to display the Easter egg?*/