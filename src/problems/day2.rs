/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/

use regex::Regex;
use std::io::Read;

pub struct Policy<'a> {
    pub min: i32,
    pub max: i32,
    pub letter: char,
    pub value: &'a str,
}

pub fn policy1<'a>(policy: &'a Policy) -> bool {
    let count: i32 = policy.value.chars().filter(|x| x == &policy.letter).count() as i32;
    count >= policy.min && count <= policy.max
}

pub fn policy2<'a>(policy: &'a Policy) -> bool {
    let exists_at_min = policy
        .value
        .chars()
        .nth((policy.min - 1) as usize)
        .expect("failed to find nth")
        == policy.letter;
    let exists_at_max = policy
        .value
        .chars()
        .nth((policy.max - 1) as usize)
        .expect("failed to find nth")
        == policy.letter;

    return exists_at_min != exists_at_max;
}

#[allow(unused_mut)]
pub fn run() {
    let mut file = std::fs::File::open("res/2/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut policy1_valid_count = 0;
    let mut policy2_valid_count = 0;
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let min = cap[1].parse::<i32>().unwrap();
            let max = cap[2].parse::<i32>().unwrap();
            let letter = cap[3].chars().nth(0).unwrap();
            let value = &cap[4];
            let mut policy = Policy {
                min,
                max,
                letter,
                value,
            };
            if policy1(&policy) {
                policy1_valid_count += 1;
            }
            if policy2(&policy) {
                policy2_valid_count += 1;
            }
        }
    }

    println!("policy 1: {}", policy1_valid_count);
    println!("policy 2: {}", policy2_valid_count);
}
