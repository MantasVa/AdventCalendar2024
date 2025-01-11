use std::{collections::{HashMap, HashSet}, fs, str::FromStr, vec};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Object {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Default, Clone)]
struct Map {
    mapsize: (i32, i32),
    robot: (i32, i32),
    robot_dirs: Vec<(i32, i32)>,
    objects: HashMap<(i32, i32), Object>,

    wide_objects: HashMap<(i32, i32), Object>,
    wide_robot: (i32, i32),
    wide_mapsize: (i32, i32),
}

impl Map {
    fn walk(&mut self) {
        for dir in self.robot_dirs.clone().iter() {
            let next_pos = (self.robot.0 + dir.0, self.robot.1 + dir.1);
            if self.can_move(next_pos, *dir){
                self.robot = next_pos;
            }
        }
    }

    fn can_move (&mut self, pos: (i32, i32), dir: (i32, i32)) -> bool {
        match self.objects.get(&pos) {
           None => true,
           Some(Object::Wall) => false,
           Some(Object::Box) => {
               let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
               let can_move = self.can_move(next_pos, dir);
               if can_move {
                   self.objects.remove(&pos);
                   self.objects.insert(next_pos, Object::Box);
                   true
               } else {
                   false
               }
           },
           _ => panic!()
        }
   }

    fn walk_wide(&mut self) {
        for dir in self.robot_dirs.clone().iter() {
            let next_pos = (self.wide_robot.0 + dir.0, self.wide_robot.1 + dir.1);
            if self.can_move_wide(&next_pos, dir){
                self.wide_robot = next_pos;
            }
        }
    }

    fn can_move_wide (&mut self, pos: &(i32, i32), dir: &(i32, i32)) -> bool {
        if dir.0 == 0 {
            let result = match self.wide_objects.get(pos) {
                None => true,
                Some(Object::Wall) => false,
                Some(Object::BoxLeft | Object::BoxRight) => {
                    let nextpos = (pos.0 + dir.0, pos.1 + dir.1);
                    if self.can_move_wide(&nextpos, dir) {
                        let i = self.wide_objects.remove(pos).unwrap();
                        self.wide_objects.insert(nextpos, i);
                        true
                    } else {
                        false
                    }
                }
                _ => panic!("wrong function for moves"),
            };
            result
        } else {
            let (allowed, mut movelist) = match self.wide_objects.get(pos) {
                None => return true, // the robot just wants to move up/down
                Some(item) => match item {
                    Object::Wall => return false, // don't move the robot into a wall
                    Object::Box => unreachable!(),
                    Object::BoxLeft => self.get_move_list(pos, dir),
                    Object::BoxRight => self.get_move_list(&(pos.0, pos.1 - 1), dir),
                },
            };
            // println!("movelist: {movelist:?}");

            if allowed {
                while !movelist.is_empty() {
                    let moveset = movelist.drain(..).collect::<HashSet<(i32, i32)>>();
                    for m in moveset {
                        let p1 = (m.0 + dir.0, m.1 + dir.1);
                        let p2 = (m.0 + dir.0, m.1 + dir.1 + 1);
                        if self.wide_objects.contains_key(&p1) || self.wide_objects.contains_key(&p2) {
                            movelist.push(m);
                        } else {
                            self.wide_objects.remove(&m);
                            self.wide_objects.remove(&(m.0, m.1 + 1));
                            self.wide_objects.insert(p1, Object::BoxLeft);
                            self.wide_objects.insert(p2, Object::BoxRight);
                        }
                    }
                }
            }
            allowed
        }
   }

   fn get_move_list(&self, pos: &(i32, i32), dir: &(i32, i32)) -> (bool, Vec<(i32, i32)>) {
        let mut mpos = *pos;
        if Some(&Object::BoxRight) == self.wide_objects.get(&mpos) {
            mpos = (mpos.0, mpos.1 - 1);
        }
        let newpos = (mpos.0 + dir.0, mpos.1 + dir.1);
        match (
            self.wide_objects.get(&newpos),
            self.wide_objects.get(&(newpos.0, newpos.1 + 1)),
        ) {
            (None, None) => (true, vec![mpos]),
            (_, Some(Object::Wall)) => (false, vec![]),
            (Some(Object::Wall), _) => (false, vec![]),
            (left, right) => {
                let mut movelist = vec![mpos];
                let (motion, v) = match left {
                    None => (true, vec![]),
                    Some(Object::BoxRight) => self.get_move_list(&(newpos.0, newpos.1 - 1), dir),
                    Some(Object::BoxLeft) => self.get_move_list(&newpos, dir),
                    _ => unreachable!(),
                };
                if !motion {
                    return (false, v);
                }
                movelist.extend(v);
                let (motion, v) = match right {
                    None => (true, vec![]),
                    Some(Object::BoxRight) => (true, vec![]),
                    Some(Object::BoxLeft) => self.get_move_list(&(newpos.0, newpos.1 + 1), dir),
                    _ => unreachable!(),
                };
                if !motion {
                    return (false, v);
                }
                movelist.extend(v);
                (true, movelist)
            }
        }
    }

   fn print_wide(&self) {
    for row in 0..self.wide_mapsize.0 + 1 {
        for col in 0..self.wide_mapsize.1 + 1 {

            match self.wide_objects.get(&(row, col)) {
                Some(Object::Box) => print!("O"),
                Some(Object::Wall) => print!("#"),
                Some(Object::BoxLeft) => print!("["),
                Some(Object::BoxRight) => print!("]"),
                None if self.wide_robot == (row, col) => print!("@"),
                None => print!("."),
            }
        }
        println!();
    }
   }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut is_map_parse = true;
        let mut map = Map::default();

        for (row, line) in s.lines().enumerate() {
            if line.is_empty() {
                is_map_parse = false;
                continue;
            } 

            let mut col_offset = 0;
            for (col, ch) in line.chars().enumerate() {
                if is_map_parse {
                    let coord = (row as i32, col as i32);
                    if ch == '#' {
                        map.objects.insert(coord, Object::Wall);
                        map.wide_objects.insert((coord.0, coord.1 + col_offset), Object::Wall);
                        col_offset += 1;
                        map.wide_objects.insert((coord.0, coord.1 + col_offset ), Object::Wall);
                    } else if ch == 'O' {
                        map.objects.insert(coord, Object::Box);

                        map.wide_objects.insert((coord.0, coord.1 + col_offset), Object::BoxLeft);
                        col_offset += 1;
                        map.wide_objects.insert((coord.0, coord.1 + col_offset), Object::BoxRight);
                    } else if ch == '@' {
                        map.robot = coord;
                        map.wide_robot = (coord.0, coord.1 + col_offset);
                        col_offset += 1;
                    } else {
                        col_offset += 1;
                    }

                    map.mapsize = coord;
                    map.wide_mapsize = (coord.0, coord.1 + col_offset)
                } else {
                    let dir = match ch {
                        '^' => (-1, 0),
                        'v' => (1, 0),
                        '<' => (0, -1),
                        '>' => (0, 1),
                        _ => panic!()
                    };
                    map.robot_dirs.push(dir);
                }
            }
        }

        Ok(map)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let map = Map::from_str(&input)?;

    _ = part1(map.clone());
    _ = part2(map);

    Ok(())
}

fn part1(mut map: Map) -> Result<()> {
    map.walk();

    let result = map.objects.iter().filter_map(|((row, col), obj)| {
        if obj == &Object::Box {
            return Some(row * 100 + col )
        }
        None
    }).sum::<i32>();

    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(mut map: Map) -> Result<()> {
    map.walk_wide();

    let result = map.wide_objects.iter().filter_map(|((row, col), obj)| {
        if obj == &Object::BoxLeft {
            return Some(row * 100 + col )
        }
        None
    }).sum::<i32>();
    map.print_wide();
    println!("Part 2 answer: {}", result);
    return Ok(());
}

/*
--- Day 15: Warehouse Woes ---
You appear back inside your own mini submarine! Each Historian drives their mini submarine in a different direction; maybe the Chief has his own submarine down here somewhere as well?

You look up to see a vast school of lanternfish swimming past you. On closer inspection, they seem quite anxious, so you drive your mini submarine over to see if you can help.

Because lanternfish populations grow rapidly, they need a lot of food, and that food needs to be stored somewhere. That's why these lanternfish have built elaborate warehouse complexes operated by robots!

These lanternfish seem so anxious because they have lost control of the robot that operates one of their most important warehouses! It is currently running amok, pushing around boxes in the warehouse with no regard for lanternfish logistics or lanternfish inventory management strategies.

Right now, none of the lanternfish are brave enough to swim up to an unpredictable robot so they could shut it off. However, if you could anticipate the robot's movements, maybe they could find a safe option.

The lanternfish already have a map of the warehouse and a list of movements the robot will attempt to make (your puzzle input). The problem is that the movements will sometimes fail as boxes are shifted around, making the actual movements of the robot difficult to predict.

For example:

##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
As the robot (@) attempts to move, if there are any boxes (O) in the way, the robot will also attempt to push those boxes. However, if this action would cause the robot or a box to move into a wall (#), nothing moves instead, including the robot. The initial positions of these are shown on the map at the top of the document the lanternfish gave you.

The rest of the document describes the moves (^ for up, v for down, < for left, > for right) that the robot will attempt to make, in order. (The moves form a single giant sequence; they are broken into multiple lines just to make copy-pasting easier. Newlines within the move sequence should be ignored.)

Here is a smaller example to get started:

########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
Were the robot to attempt the given sequence of moves, it would push around the boxes as follows:

Initial state:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move <:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
The larger example has many more moves; after the robot has finished those moves, the warehouse would look like this:

##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
The lanternfish use their own custom Goods Positioning System (GPS for short) to track the locations of the boxes. The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance from the left edge of the map. (This process does not stop at wall tiles; measure all the way to the edges of the map.)

So, the box shown below has a distance of 1 from the top edge of the map and 4 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 4 = 104.

#######
#...O..
#......
The lanternfish would like to know the sum of all boxes' GPS coordinates after the robot finishes moving. In the larger example, the sum of all boxes' GPS coordinates is 10092. In the smaller example, the sum is 2028.

Predict the motion of the robot and boxes in the warehouse. After the robot is finished moving, what is the sum of all boxes' GPS coordinates?

--- Part Two ---
The lanternfish use your information to find a safe moment to swim in and turn off the malfunctioning robot! Just as they start preparing a festival in your honor, reports start coming in that a second warehouse's robot is also malfunctioning.

This warehouse's layout is surprisingly similar to the one you just helped. There is one key difference: everything except the robot is twice as wide! The robot's list of movements doesn't change.

To get the wider warehouse's map, start with your original map and, for each tile, make the following changes:

If the tile is #, the new map contains ## instead.
If the tile is O, the new map contains [] instead.
If the tile is ., the new map contains .. instead.
If the tile is @, the new map contains @. instead.
This will produce a new warehouse map which is twice as wide and with wide boxes that are represented by []. (The robot does not change size.)

The larger example from before would now look like this:

####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
Because boxes are now twice as wide but the robot is still the same size and speed, boxes can be aligned such that they directly push two other boxes at once. For example, consider this situation:

#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
After appropriately resizing this map, the robot would push around these boxes as follows:

Initial state:
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

Move <:
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############

Move v:
##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############

Move v:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############

Move <:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############

Move <:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############

Move ^:
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############

Move ^:
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############

Move <:
##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############

Move <:
##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############

Move ^:
##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############

Move ^:
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
This warehouse also uses GPS to locate the boxes. For these larger boxes, distances are measured from the edge of the map to the closest edge of the box in question. So, the box shown below has a distance of 1 from the top edge of the map and 5 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 5 = 105.

##########
##...[]...
##........
In the scaled-up version of the larger example from above, after the robot has finished all of its moves, the warehouse would look like this:

####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
The sum of these boxes' GPS coordinates is 9021.

Predict the motion of the robot and boxes in this new, scaled-up warehouse. What is the sum of all boxes' final GPS coordinates?

*/