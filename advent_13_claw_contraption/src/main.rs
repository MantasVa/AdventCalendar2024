use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Default)]
struct Machine {
        button_a: (i64, i64),
        button_b: (i64, i64),
        target: (i64, i64),
}

impl Machine {
    const BUTTON_A_COST: i64 = 3;
    const BUTTON_B_COST: i64 = 1;

    fn get_with_target_delta(&self, delta: i64) -> Machine {
        Machine { 
            button_a: self.button_a, 
            button_b: self.button_b, 
            target: (self.target.0 + delta, self.target.1 + delta) 
        }
    }

    fn get_optimal_win_cost(&self) -> Option<i64> {
        let a_det = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        let a2_det = self.button_a.0 * self.target.1 - self.button_a.1 * self.target.0;

        let but_b_count = a2_det / a_det;
        let but_a_count = (self.target.0 - self.button_b.0 * but_b_count ) / self.button_a.0;

        
        if self.button_a.0 * but_a_count + self.button_b.0 * but_b_count == self.target.0 &&
            self.button_a.1 * but_a_count + self.button_b.1 * but_b_count == self.target.1
            {
        return Some(but_a_count * Machine::BUTTON_A_COST + but_b_count * Machine::BUTTON_B_COST);
            }

            None

    }
}

fn main() -> Result<()> {
    let machines = parse_machines()?;

    _ = part1(machines.clone());
    _ = part2(machines);

    Ok(())
}

fn part1(machines: Vec<Machine>) -> Result<()> {
    let mut result = 0;

    for machine in machines {
        if let Some(cost) = machine.get_optimal_win_cost(){
            result += cost;
        }
    }

    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(machines: Vec<Machine>) -> Result<()> {
    let mut result = 0;

    for machine in machines {
        if let Some(cost) = machine.get_with_target_delta(1_000_000_000_0000).get_optimal_win_cost(){
            result += cost;
        }
    }

    println!("Part 2 answer: {}", result);
    return Ok(());
}

fn parse_machines() -> Result<Vec<Machine>> {
    let input = fs::read_to_string("input.txt")?;

    let mut machines = Vec::new();
    let mut machine = Machine::default();
    for line in input.lines() {
        if line.starts_with("Button A") {
            let (left, right) = line.split_once(',').unwrap();
            let x_add = left.split_once('+').unwrap().1.parse::<i64>().unwrap();
            let y_add = right.split_once('+').unwrap().1.parse::<i64>().unwrap();

            machine.button_a = (x_add, y_add);
        } else if line.starts_with("Button B") {
            let (left, right) = line.split_once(',').unwrap();
            let x_add = left.split_once('+').unwrap().1.parse::<i64>().unwrap();
            let y_add = right.split_once('+').unwrap().1.parse::<i64>().unwrap();

            machine.button_b = (x_add, y_add);
        } else if line.starts_with("Prize"){
            let (left, right) = line.split_once(',').unwrap();
            let target_x = left.split_once("X=").unwrap().1.parse::<i64>().unwrap();
            let target_y = right.split_once("Y=").unwrap().1.parse::<i64>().unwrap();
            machine.target = (target_x, target_y);

            machines.push(machine);
            machine = Machine::default();
        }
    }

    Ok(machines)
}

/*
--- Day 13: Claw Contraption ---
Next up: the lobby of a resort on a tropical island. The Historians take a moment to admire the hexagonal floor tiles before spreading out.

Fortunately, it looks like the resort has a new arcade! Maybe you can win some prizes from the claw machines?

The claw machines here are a little unusual. Instead of a joystick or directional buttons to control the claw, these machines have two buttons labeled A and B. Worse, you can't just put in a token and play; it costs 3 tokens to push the A button and 1 token to push the B button.

With a little experimentation, you figure out that each machine's buttons are configured to move the claw a specific amount to the right (along the X axis) and a specific amount forward (along the Y axis) each time that button is pressed.

Each machine contains one prize; to win the prize, the claw must be positioned exactly above the prize on both the X and Y axes.

You wonder: what is the smallest number of tokens you would have to spend to win as many prizes as possible? You assemble a list of every machine's button behavior and prize location (your puzzle input). For example:

Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
This list describes the button configuration and prize location of four different claw machines.

For now, consider just the first claw machine in the list:

Pushing the machine's A button would move the claw 94 units along the X axis and 34 units along the Y axis.
Pushing the B button would move the claw 22 units along the X axis and 67 units along the Y axis.
The prize is located at X=8400, Y=5400; this means that from the claw's initial position, it would need to move exactly 8400 units along the X axis and exactly 5400 units along the Y axis to be perfectly aligned with the prize in this machine.
The cheapest way to win the prize is by pushing the A button 80 times and the B button 40 times. This would line up the claw along the X axis (because 80*94 + 40*22 = 8400) and along the Y axis (because 80*34 + 40*67 = 5400). Doing this would cost 80*3 tokens for the A presses and 40*1 for the B presses, a total of 280 tokens.

For the second and fourth claw machines, there is no combination of A and B presses that will ever win a prize.

For the third claw machine, the cheapest way to win the prize is by pushing the A button 38 times and the B button 86 times. Doing this would cost a total of 200 tokens.

So, the most prizes you could possibly win is two; the minimum tokens you would have to spend to win all (two) prizes is 480.

You estimate that each button would need to be pressed no more than 100 times to win a prize. How else would someone be expected to play?

Figure out how to win as many prizes as possible. What is the fewest tokens you would have to spend to win all possible prizes?

--- Part Two ---
As you go to win the first prize, you discover that the claw is nowhere near where you expected it would be. Due to a unit conversion error in your measurements, the position of every prize is actually 10000000000000 higher on both the X and Y axis!

Add 10000000000000 to the X and Y position of every prize. After making this change, the example above would now look like this:

Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
Now, it is only possible to win a prize on the second and fourth claw machines. Unfortunately, it will take many more than 100 presses to do so.

Using the corrected prize coordinates, figure out how to win as many prizes as possible. What is the fewest tokens you would have to spend to win all possible prizes?

 */
