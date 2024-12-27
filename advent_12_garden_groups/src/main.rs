use std::{collections::{HashMap, HashSet}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Garden {
    map: HashMap<(i32, i32), char>
}


fn main() -> Result<()> {
    let garden = get_garden_map()?;

    _ = part1(&garden);
    _ = part2(&garden);

    Ok(())
}

fn part1(garden: &Garden) -> Result<()> {
    let mut garden = Garden { map: garden.map.clone() };

    let mut result= 0;
    while let Some(coord) = garden.map.keys().copied().next() {
        let ch = *garden.map.get(&coord).unwrap();
        let mut stack = vec![coord];
        stack.push(coord);
        _ = garden.map.remove(&coord);

        let mut visited = HashSet::new();
        while let Some(pos) = stack.pop() {
            if visited.insert(pos) {
                for (delta_x, delta_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let new = (pos.0 + delta_x, pos.1 + delta_y);

                    if ch == *garden.map.get(&new).unwrap_or(&'.') {
                        _ = garden.map.remove(&new);
                        stack.push(new);
                    }
                }
            }
        }

        let area = visited.len();
        let mut perimeter = 0;
        for (row, col) in &visited {
            for (delta_x, delta_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (row + delta_x, col + delta_y);

                if !visited.contains(&n) {
                    perimeter += 1;
                }
            }
        }

        result += area * perimeter;
    }

    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(garden: &Garden) -> Result<()> {
    let mut garden = Garden { map: garden.map.clone() };

    let mut result= 0;
    while let Some(coord) = garden.map.keys().copied().next() {
        let ch = *garden.map.get(&coord).unwrap();
        let mut stack = vec![coord];
        stack.push(coord);
        _ = garden.map.remove(&coord);

        let mut visited = HashSet::new();
        while let Some(pos) = stack.pop() {
            if visited.insert(pos) {
                for (delta_x, delta_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let new = (pos.0 + delta_x, pos.1 + delta_y);

                    if ch == *garden.map.get(&new).unwrap_or(&'.') {
                        _ = garden.map.remove(&new);
                        stack.push(new);
                    }
                }
            }
        }

        let area = visited.len();
        let mut edges = HashSet::new();
        for (row, col) in &visited {
            for (delta_x, delta_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (row + delta_x, col + delta_y);

                if !visited.contains(&n) {
                    edges.insert(((*row, *col), n));
                }
            }
        }

        let mut edges_count = 0;
        while let Some(edge) = edges.iter().copied().next() {
            let is_horizontal_edge = edge.0.1 == edge.1.1;

            if is_horizontal_edge {
                let mut new_col = edge.0.1;
                loop {
                    new_col -= 1;

                    let edge_pos = ((edge.0.0, new_col), (edge.1.0, new_col));
                    if !edges.remove(&edge_pos) {
                        break;
                    }
                }

                new_col = edge.0.1;
                loop {
                    new_col += 1;
                    let edge_pos = ((edge.0.0, new_col), (edge.1.0, new_col));
                    if !edges.remove(&edge_pos) {
                        break;
                    }
                }

            } else {
                let mut new_row = edge.0.0;
                loop {
                    new_row -= 1;

                    let edge_pos = ((new_row, edge.0.1), (new_row, edge.1.1));
                    if !edges.remove(&edge_pos) {
                        break;
                    }
                }

                new_row = edge.0.0;
                loop {
                    new_row += 1;
                    let edge_pos = ((new_row, edge.0.1), (new_row, edge.1.1));
                    if !edges.remove(&edge_pos) {
                        break;
                    }
                }
            }
            edges.remove(&edge);
            edges_count += 1;
        }

        result += area * edges_count;
    }

    println!("Part 2 answer: {}", result);
    return Ok(());
}

fn get_garden_map() -> Result<Garden> {
    let input = fs::read_to_string("input.txt")?;
    let mut map: HashMap<(i32, i32), char>= HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            _ = map.entry((row as i32, col as i32)).or_insert(ch);
        }
    }
    Ok(Garden { map })
}


/*
--- Day 12: Garden Groups ---
Why not search for the Chief Historian near the gardener and his massive farm? There's plenty of food, so The Historians grab something to eat while they search.

You're about to settle near a complex arrangement of garden plots when some Elves ask if you can lend a hand. They'd like to set up fences around each region of garden plots, but they can't figure out how much fence they need to order or how much it will cost. They hand you a map (your puzzle input) of the garden plots.

Each garden plot grows only a single type of plant and is indicated by a single letter on your map. When multiple garden plots are growing the same type of plant and are touching (horizontally or vertically), they form a region. For example:

AAAA
BBCD
BBCC
EEEC
This 4x4 arrangement includes garden plots growing five different types of plants (labeled A, B, C, D, and E), each grouped into their own region.

In order to accurately calculate the cost of the fence around a single region, you need to know that region's area and perimeter.

The area of a region is simply the number of garden plots the region contains. The above map's type A, B, and C plants are each in a region of area 4. The type E plants are in a region of area 3; the type D plants are in a region of area 1.

Each garden plot is a square and so has four sides. The perimeter of a region is the number of sides of garden plots in the region that do not touch another garden plot in the same region. The type A and C plants are each in a region with perimeter 10. The type B and E plants are each in a region with perimeter 8. The lone D plot forms its own region with perimeter 4.

Visually indicating the sides of plots in each region that contribute to the perimeter using - and |, the above map's regions' perimeters are measured as follows:

+-+-+-+-+
|A A A A|
+-+-+-+-+     +-+
              |D|
+-+-+   +-+   +-+
|B B|   |C|
+   +   + +-+
|B B|   |C C|
+-+-+   +-+ +
          |C|
+-+-+-+   +-+
|E E E|
+-+-+-+
Plants of the same type can appear in multiple separate regions, and regions can even appear within other regions. For example:

OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
The above map contains five regions, one containing all of the O garden plots, and the other four each containing a single X plot.

The four X regions each have area 1 and perimeter 4. The region containing 21 type O plants is more complicated; in addition to its outer edge contributing a perimeter of 20, its boundary with each X region contributes an additional 4 to its perimeter, for a total perimeter of 36.

Due to "modern" business practices, the price of fence required for a region is found by multiplying that region's area by its perimeter. The total price of fencing all regions on a map is found by adding together the price of fence for every region on the map.

In the first example, region A has price 4 * 10 = 40, region B has price 4 * 8 = 32, region C has price 4 * 10 = 40, region D has price 1 * 4 = 4, and region E has price 3 * 8 = 24. So, the total price for the first example is 140.

In the second example, the region with all of the O plants has price 21 * 36 = 756, and each of the four smaller X regions has price 1 * 4 = 4, for a total price of 772 (756 + 4 + 4 + 4 + 4).

Here's a larger example:

RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
It contains:

A region of R plants with price 12 * 18 = 216.
A region of I plants with price 4 * 8 = 32.
A region of C plants with price 14 * 28 = 392.
A region of F plants with price 10 * 18 = 180.
A region of V plants with price 13 * 20 = 260.
A region of J plants with price 11 * 20 = 220.
A region of C plants with price 1 * 4 = 4.
A region of E plants with price 13 * 18 = 234.
A region of I plants with price 14 * 22 = 308.
A region of M plants with price 5 * 12 = 60.
A region of S plants with price 3 * 8 = 24.
So, it has a total price of 1930.

What is the total price of fencing all regions on your map?

--- Part Two ---
Fortunately, the Elves are trying to order so much fence that they qualify for a bulk discount!

Under the bulk discount, instead of using the perimeter to calculate the price, you need to use the number of sides each region has. Each straight section of fence counts as a side, regardless of how long it is.

Consider this example again:

AAAA
BBCD
BBCC
EEEC
The region containing type A plants has 4 sides, as does each of the regions containing plants of type B, D, and E. However, the more complex region containing the plants of type C has 8 sides!

Using the new method of calculating the per-region price by multiplying the region's area by its number of sides, regions A through E have prices 16, 16, 32, 4, and 12, respectively, for a total price of 80.

The second example above (full of type X and O plants) would have a total price of 436.

Here's a map that includes an E-shaped region full of type E plants:

EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
The E-shaped region has an area of 17 and 12 sides for a price of 204. Including the two regions full of type X plants, this map has a total price of 236.

This map has a total price of 368:

AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
It includes two regions full of type B plants (each with 4 sides) and a single region full of type A plants (with 4 sides on the outside and 8 more sides on the inside, a total of 12 sides). Be especially careful when counting the fence around regions like the one full of type A plants; in particular, each section of fence has an in-side and an out-side, so the fence does not connect across the middle of the region (where the two B regions touch diagonally). (The Elves would have used the Möbius Fencing Company instead, but their contract terms were too one-sided.)

The larger example from before now has the following updated prices:

A region of R plants with price 12 * 10 = 120.
A region of I plants with price 4 * 4 = 16.
A region of C plants with price 14 * 22 = 308.
A region of F plants with price 10 * 12 = 120.
A region of V plants with price 13 * 10 = 130.
A region of J plants with price 11 * 12 = 132.
A region of C plants with price 1 * 4 = 4.
A region of E plants with price 13 * 8 = 104.
A region of I plants with price 14 * 16 = 224.
A region of M plants with price 5 * 6 = 30.
A region of S plants with price 3 * 6 = 18.
Adding these together produces its new total price of 1206.

What is the new total price of fencing all regions on your map?
*/