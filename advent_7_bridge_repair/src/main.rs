use std::{fs, str::FromStr};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Hash)]
struct Equation {
    target: usize,
    nums: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let target = left.parse::<usize>().unwrap();
        let nums = right.split(" ").map(|n| n.parse().unwrap()).collect();
        Ok(Self { target, nums })
    }
}

impl Equation {
    const OPERATIONS_1: [fn (usize, usize) -> usize; 2] = [
        |a, b| -> usize { a + b },
        |a, b| -> usize { a * b },
    ];

    const OPERATIONS_2: [fn (usize, usize) -> usize; 3] = [
        |a, b| -> usize { a + b },
        |a, b| -> usize { a * b },
        |a, b| -> usize { format!("{}{}", a, b).parse().unwrap()},
    ];

    fn can_solve(&self, ops: &[fn (usize, usize) -> usize]) -> Result<usize> {
        match Equation::eval(self.target, self.nums[0], self.nums[1..].to_vec(), ops).unwrap() {
            true => Ok(self.target),
            false => Ok(0),
        }
    }

    fn eval(target: usize, acc: usize, numbers: Vec<usize>, ops: &[fn (usize, usize) -> usize]) -> Result<bool> {
        if numbers.len() == 0 {
            return Ok(acc == target)
        }

        Ok(ops.iter().any(|op| 
            Equation::eval(target, op(acc, numbers[0]), numbers[1..].to_vec(), ops).unwrap()))
    }
}

fn main() -> Result<()> {
    let equations = get_equations()?;

    _ = part1(&equations);
    _ = part2(&equations);

    Ok(())
}

fn get_equations() -> Result<Vec<Equation>> {
    let input = fs::read_to_string("input.txt")?;
    let equations = input.lines().map(|s| Equation::from_str(s).unwrap()).collect();
    Ok(equations)
}

fn part1(equations: &Vec<Equation>) -> Result<()> {
    let result = 
        equations
        .iter()
        .map(|eq| eq.can_solve(&Equation::OPERATIONS_1)
        .unwrap())
        .sum::<usize>();

    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(equations: &Vec<Equation>) -> Result<()> {
    let result = 
        equations
        .iter()
        .map(|eq| eq.can_solve(&Equation::OPERATIONS_2)
        .unwrap())
        .sum::<usize>();

    println!("Part 2 answer: {}", result);
    return Ok(());
}

/*
--- Day 7: Bridge Repair ---
The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief isn't on this side of the bridge, though; maybe he's on the other side?

When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.

You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some young elephants were playing nearby and stole all the operators from their calibration equations! They could finish the calibrations if only someone could determine which test values could possibly be produced by placing any combination of operators into their calibration equations (your puzzle input).

For example:

190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
Each line represents a single equation. The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.

Operators are always evaluated left-to-right, not according to precedence rules. Furthermore, numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants holding two different types of operators: add (+) and multiply (*).

Only three of the above equations can be made true by inserting operators:

190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would give 29, but choosing * would give the test value (10 * 19 = 190).
3267: 81 40 27 has two positions for operators. Of the four possible configurations of the operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated left-to-right)!
292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
The engineers just need the total calibration result, which is the sum of the test values from just the equations that could possibly be true. In the above example, the sum of the test values for the three equations listed above is 3749.

Determine which equations could possibly be true. What is their total calibration result?

--- Part Two ---
The engineers seem concerned; the total calibration result you gave them is nowhere close to being within safety tolerances. Just then, you spot your mistake: some well-hidden elephants are holding a third type of operator.

The concatenation operator (||) combines the digits from its left and right inputs into a single number. For example, 12 || 345 would become 12345. All operators are still evaluated left-to-right.

Now, apart from the three equations that could be made true using only addition and multiplication, the above example has three more equations that can be made true by inserting operators:

156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
192: 17 8 14 can be made true using 17 || 8 + 14.
Adding up all six test values (the three that could be made before using only + and * plus the new three that can now be made by also using ||) produces the new total calibration result of 11387.

Using your new knowledge of elephant hiding spots, determine which equations could possibly be true. What is their total calibration result?
*/