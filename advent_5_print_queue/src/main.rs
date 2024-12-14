use std::{collections::{hash_map::Entry, HashMap}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct RuleSet {
    page_rules: HashMap<u32, Vec<u32>>,
}

impl RuleSet {

    fn is_update_valid(&self, page_update: &HashMap<u32, u32> /* Key: Page number, Value: Position */) -> bool {
        let mut valid = true;
        for (number, position) in page_update {
            let page_rules = self.page_rules.get(&number);

            if page_rules.is_some() {
                for page_rule in page_rules.unwrap() {
                    let opt_page = page_update.get(page_rule);

                    if opt_page.is_some() && opt_page.unwrap() < position {
                        valid = false;
                        break;                            
                    }
                }
            } else {
                continue;
            }

            if !valid {
                break;
            }
        }

        valid
    }

    fn fix_update_for_ruleset(&self, page_update: &HashMap<u32, u32> /* Key: Page number, Value: Position */) -> HashMap<u32, u32> {
        let mut pos: u32 = 0;

        let mut updated = page_update.clone();

        while pos < page_update.len() as u32 {
            let copied = updated.clone();
            let (page, _) = copied.iter().find(|(_, p)| **p == pos).clone().unwrap();
            let page_rules = self.page_rules.get(&page);

            if page_rules.is_some() {
                for page_rule in page_rules.unwrap() {
                    let opt_page = updated.get(page_rule).copied();

                    if opt_page.is_some() {
                        let mut prev = opt_page.unwrap();
                        if prev < pos {
                            updated.iter_mut().for_each(|(key, k_pos)| {
                                if *key == *page_rule {
                                    *k_pos = pos;
                                } else if *key == *page {
                                    *k_pos = prev;
                                } else if *k_pos >= pos && k_pos < &mut prev {
                                    *k_pos += 1;
                                }
                            });
    
                            pos = 0;
                            continue;      
                        }
                    }
                }
            }

            pos += 1;
        }

        updated
    }
}

fn main() -> Result<()> {
    process_rulesets()
}

fn process_rulesets() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut valid_updates_result = 0;
    let mut fixed_invalid_updates_result = 0;

    let mut ruleset = RuleSet { page_rules: HashMap::new() };
    let mut parsing_ruleset = true;
    for line in input.lines() {

        if line.is_empty() {
            parsing_ruleset = false;
            continue;
        }

        if parsing_ruleset {
            let (left, right) = line.split_once("|").unwrap();

            let key = left.parse::<u32>().unwrap();
            let page_rule = right.parse::<u32>().unwrap();
    
            match ruleset.page_rules.entry(key) {
                Entry::Occupied(o) => {
                    let value: &mut Vec<u32> = o.into_mut();
                    value.push(page_rule);
                },
                Entry::Vacant(v) => {
                    v.insert(vec![ page_rule ]);
                },
            };
        } else {
            let page_update: HashMap<_, _> = line.split(",").enumerate().map(|(idx, x)| (x.parse::<u32>().unwrap(), idx as u32)).collect();
            let valid = ruleset.is_update_valid(&page_update);
            let middle_pos = (page_update.len() / 2) as u32;

            if valid {
                valid_updates_result += *(page_update.iter().find(|(_, v)| **v == middle_pos).unwrap().0);
            } else {
                let fixed = ruleset.fix_update_for_ruleset(&page_update);
                fixed_invalid_updates_result += *(fixed.iter().find(|(_, v)| **v == middle_pos).unwrap().0);
            }
        }
    }

    println!("Part 1 answer: {}", valid_updates_result);
    println!("Part 2 answer: {}", fixed_invalid_updates_result);
    return Ok(());
}

/*
--- Day 5: Print Queue ---
Satisfied with their search on Ceres, the squadron of scholars suggests subsequently scanning the stationery stacks of sub-basement 17.

The North Pole printing department is busier than ever this close to Christmas, and while The Historians continue their search of this historically significant facility, an Elf operating a very familiar printer beckons you over.

The Elf must recognize you, because they waste no time explaining that the new sleigh launch safety manual updates won't print correctly. Failure to update the safety manuals would be dire indeed, so you offer your services.

Safety protocols clearly indicate that new pages for the safety manuals must be printed in a very specific order. The notation X|Y means that if both page number X and page number Y are to be produced as part of an update, page number X must be printed at some point before page number Y.

The Elf has for you both the page ordering rules and the pages to produce in each update (your puzzle input), but can't figure out whether each update has the pages in the right order.

For example:

47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
The first section specifies the page ordering rules, one per line. The first rule, 47|53, means that if an update includes both page number 47 and page number 53, then page number 47 must be printed at some point before page number 53. (47 doesn't necessarily need to be immediately before 53; other pages are allowed to be between them.)

The second section specifies the page numbers of each update. Because most safety manuals are different, the pages needed in the updates are different too. The first update, 75,47,61,53,29, means that the update consists of page numbers 75, 47, 61, 53, and 29.

To get the printers going as soon as possible, start by identifying which updates are already in the right order.

In the above example, the first update (75,47,61,53,29) is in the right order:

75 is correctly first because there are rules that put each other page after it: 75|47, 75|61, 75|53, and 75|29.
47 is correctly second because 75 must be before it (75|47) and every other page must be after it according to 47|61, 47|53, and 47|29.
61 is correctly in the middle because 75 and 47 are before it (75|61 and 47|61) and 53 and 29 are after it (61|53 and 61|29).
53 is correctly fourth because it is before page number 29 (53|29).
29 is the only page left and so is correctly last.
Because the first update does not include some page numbers, the ordering rules involving those missing page numbers are ignored.

The second and third updates are also in the correct order according to the rules. Like the first update, they also do not include every page number, and so only some of the ordering rules apply - within each update, the ordering rules that involve missing page numbers are not used.

The fourth update, 75,97,47,61,53, is not in the correct order: it would print 75 before 97, which violates the rule 97|75.

The fifth update, 61,13,29, is also not in the correct order, since it breaks the rule 29|13.

The last update, 97,13,75,29,47, is not in the correct order due to breaking several rules.

For some reason, the Elves also need to know the middle page number of each update being printed. Because you are currently only printing the correctly-ordered updates, you will need to find the middle page number of each correctly-ordered update. In the above example, the correctly-ordered updates are:

75,47,61,53,29
97,61,53,29,13
75,29,13
These have middle page numbers of 61, 53, and 29 respectively. Adding these page numbers together gives 143.

Of course, you'll need to be careful: the actual list of page ordering rules is bigger and more complicated than the above example.

Determine which updates are already in the correct order. What do you get if you add up the middle page number from those correctly-ordered updates?

--- Part Two ---
While the Elves get to work printing the correctly-ordered updates, you have a little time to fix the rest of them.

For each of the incorrectly-ordered updates, use the page ordering rules to put the page numbers in the right order. For the above example, here are the three incorrectly-ordered updates and their correct orderings:

75,97,47,61,53 becomes 97,75,47,61,53.
61,13,29 becomes 61,29,13.
97,13,75,29,47 becomes 97,75,47,29,13.
After taking only the incorrectly-ordered updates and ordering them correctly, their middle page numbers are 47, 29, and 47. Adding these together produces 123.

Find the updates which are not in the correct order. What do you get if you add up the middle page numbers after correctly ordering just those updates?
*/