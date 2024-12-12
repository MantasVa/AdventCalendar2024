use std::{collections::{HashMap, HashSet}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    const MOVES:[(i32, i32); 8] = [(1,0), (-1, 0), (0, -1),  (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
}

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    const SECRET_WORD: &str = "XMAS";
    let word_occ = find_occurances(input, SECRET_WORD);

    println!("Part 1 answer: {}", word_occ.len());
    return Ok(());
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    const SECRET_WORD: &str = "MAS";

    let word_occ = find_occurances(input, SECRET_WORD);
    
    // leave out only diagonal words
    let filtered = word_occ.iter().filter(|set| 
        set[0].0 != set[1].0 && set[0].0 != set[2].0 && set[1].0 != set[2].0 &&
        set[0].1 != set[1].1 && set[0].1 != set[2].1 && set[1].1 != set[2].1);

    // group diagonal word occurances by middle char A coordinates.
    let mut map = HashMap::new();    
    for occ in filtered {

        if map.contains_key(&(occ[1].0, occ[1].1)){
            *map.get_mut(&(occ[1].0, occ[1].1)).unwrap() += 1;
        } else {
            map.insert((occ[1].0, occ[1].1), 1);
        }
    }

    // count occurances that have two diagonal words with same middle points.
    let result = map.iter().filter(|(_, value)| **value == 2).count();

    println!("Part 2 answer: {}", result);
    return Ok(());
}

fn find_occurances(input: String, search_word: &str) -> HashSet<Vec<(i32, i32)>> {
    let mut map = HashMap::new();
    let mut leads = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), ch);

            if search_word.starts_with(ch) {
                leads.push((x as i32, y as i32));
            }
        }
    }

    let mut word_occ: HashSet<Vec<(i32, i32)>> = HashSet::new();
    for (x, y) in leads {
        let mut occ = [true; 8];
        for i in 1..search_word.len() {
            for (idx, (pos_x, pos_y)) in Coord::MOVES.map(|(x1, y1)| (x + (x1 * i as i32), y + (y1 * i as i32))).iter().enumerate() {

                if !map.contains_key(&(*pos_x, *pos_y)) || map[&(*pos_x, *pos_y)] != search_word.chars().nth(i).unwrap() {
                    occ[idx] = false;
                }
            }
        }

        for set in occ.iter().enumerate().filter(|(_, seq_correct)| **seq_correct)
        .map(|(idx, _)| (0..search_word.len()).map(|i| {
            if i == 0 {
                (x, y)
            } else {
                (x + (Coord::MOVES[idx].0 * i as i32), y + (Coord::MOVES[idx].1 * i as i32))
            }
        }).collect()) {
            word_occ.insert(set);
        }
    }

    word_occ
}

/*
--- Day 4: Ceres Search ---
"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


..X...
.SAMX.
.A..A.
XMAS.S
.X....
The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
Take a look at the little Elf's word search. How many times does XMAS appear?

--- Part Two ---
The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

M.S
.A.
M.S
Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear? */