use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

enum Sign {
    Plus,
    Minus
}

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut valid_records = 0;
    for line in input.lines() {
        let nums = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        
        let is_valid = is_record_valid(&nums);

        if is_valid {
            valid_records += 1;
        }
    }

    println!("Part 1 answer: {}", valid_records);
    return Ok(());
}


fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut valid_records = 0;
    for line in input.lines() {
        let nums = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        
        let mut is_valid = is_record_valid(&nums);

        if !is_valid {
            let mut i = 0;
            while i < nums.len() && !is_valid {
                let slice = nums.iter().enumerate().filter(|(idx, _)| *idx != i).map(|(_, val)| *val).collect::<Vec<i32>>();

                is_valid = is_record_valid(&slice);
                i += 1;
            }
        }

        if is_valid {
            valid_records += 1;
        }
    }

    println!("Part 2 answer: {}", valid_records);
    return Ok(());
}

fn is_record_valid(nums: &Vec<i32>) -> bool {
    let max_diff = 3;

    let mut prev_opt: Option<i32> = None;
    let mut sign: Option<Sign> = None;
    let mut valid = true;
    for num in nums {

        if prev_opt.is_some() {
            let prev = prev_opt.unwrap();
            if sign.is_none() {
                if *num > prev {
                    sign = Some(Sign::Plus)
                } else {
                    sign = Some(Sign::Minus);
                }
            }

            valid = match sign {
                Some(Sign::Plus) if *num < prev => false,
                Some(Sign::Plus) => num - prev > 0 && num - prev <= max_diff,
                Some(Sign::Minus) if *num > prev => false,
                Some(Sign::Minus) => prev - num > 0 && prev - num <= max_diff,
                _ => false,
            };

            if !valid {
                return false;
            }
        }
        prev_opt = Some(*num);
    }

    return valid;
}
