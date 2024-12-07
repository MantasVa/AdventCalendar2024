use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();

        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let sum = left.iter().zip(right.iter()).fold(0, |acc, (l, r)| acc + (l - r).abs());

    println!("Part 1 answer: {}", sum);
    return Ok(());
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();

        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    let sum = left.iter().map(|val| *val * right.iter().filter(|x| *x == val).count() as u32).sum::<u32>();

    println!("Part 2 answer: {}", sum);
    return Ok(());
}
