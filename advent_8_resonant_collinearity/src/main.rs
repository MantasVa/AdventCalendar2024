use std::{collections::{HashMap, HashSet}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Map {
    freq_antennas: HashMap<char, HashSet<(i32, i32)>>,
    size: (i32, i32),
}

impl Map {
    fn get_antidotes(&self, part_2: bool) -> HashSet<(i32, i32)> {
        let mut antidotes = HashSet::new();
        for (_, antennas) in &self.freq_antennas {
            for ant_1 in antennas{
                for ant_2 in antennas {
                    if ant_1 == ant_2 {
                        continue;
                    }
    
                    if !part_2 {
                        antidotes.insert(*ant_1);
                        antidotes.insert(*ant_2);
                    }
    
                    let row_dist = ant_1.0 - ant_2.0;
                    let col_dist = ant_1.1 - ant_2.1;
    
                    let mut iteration = 1;
                    loop {
                        let it_row_dist = row_dist * iteration;
                        let it_col_dist = col_dist * iteration;
        
                        let a1 = (ant_1.0 + it_row_dist, ant_1.1 + it_col_dist);
                        let a2 = (ant_2.0 - it_row_dist, ant_2.1 - it_col_dist);
        
                        if self.in_bounds(&a1) {
                            antidotes.insert(a1);
                        }
        
                        if self.in_bounds(&a2) {
                            antidotes.insert(a2);
                        }
    
                        if !self.in_bounds(&a1) && 
                           !self.in_bounds(&a2) {
                            break;
                           }
                        
                        if part_2 {
                            break;
                        } else {
                            iteration += 1;
                        }
                    }
                }
            }
        }

        antidotes
    }

    fn in_bounds(&self, coords: &(i32, i32)) -> bool {
        coords.0 >= 0 && coords.0 < self.size.0 && coords.1 >= 0 && coords.1 < self.size.1
    }
}

fn main() -> Result<()> {
    let map = get_map()?;

    _ = part1(&map);
    _ = part2(&map);

    Ok(())
}

fn get_map() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;

    let mut map = Map { freq_antennas: HashMap::new(), size: (-1, -1) };
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                let entry = map.freq_antennas.entry(ch).or_default();
                entry.insert((row as i32, col as i32));
            }

            if cols == 0 {
                cols = line.len();
            }
        }
        rows = row + 1;
    }
    map.size = (rows as i32, cols as i32);

    Ok(map)
}

fn part1(map: &Map) -> Result<()> {
    println!("Part 1 answer: {}", map.get_antidotes(true).len());
    return Ok(());
}

fn part2(map: &Map) -> Result<()> {
    println!("Part 1 answer: {}", map.get_antidotes(false).len());
    return Ok(());
}

/*
--- Day 8: Resonant Collinearity ---
You find yourselves on the roof of a top-secret Easter Bunny installation.

While The Historians do their thing, you take a look at the familiar huge antenna. Much to your surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!

Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map (your puzzle input) of these antennas. For example:

............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas. In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other. This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.

So, for these two antennas with frequency a, they create the two antinodes marked with #:

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
Adding a third antenna with the same frequency creates several more antinodes. It would ideally add four antinodes, but two are off the right side of the map, so instead it adds only two:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........
Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its location:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........
The first example has antennas with two different frequencies, so the antinodes they create look like this, plus an antinode overlapping the topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations that contain an antinode within the bounds of the map.

Calculate the impact of the signal. How many unique locations within the bounds of the map contain an antinode?

--- Part Two ---
Watching over your shoulder as you work, one of The Historians asks if you took the effects of resonant harmonics into your calculations.

Whoops!

After updating your model, it turns out that an antinode occurs at any grid position exactly in line with at least two antennas of the same frequency, regardless of distance. This means that some of the new antinodes will occur at the position of each antenna (unless that antenna is the only one of its frequency).

So, these three T-frequency antennas now create many antinodes:

T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
In fact, the three T-frequency antennas are all exactly in line with two antennas, so they are all also antinodes! This brings the total number of antinodes in the above example to 9.

The original example now has 34 antinodes, including the antinodes that appear on every antenna:

##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##
Calculate the impact of the signal using this updated model. How many unique locations within the bounds of the map contain an antinode?
*/