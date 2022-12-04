use std::collections::{HashMap, HashSet, LinkedList};

static EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let mut letter_values = HashMap::<char, u32>::new();
    let ascii_lower = "abcdefghijklmnopqrstuvwxyz".chars();
    let ascii_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    for (i, c) in ascii_lower.enumerate() {
        letter_values.insert(c, (i as u32) + 1);
    }
    for (i, c) in ascii_upper.enumerate() {
        letter_values.insert(c, (i as u32) + 27);
    }

    let lines = data.trim().split("\n");
    if simple {
        let priorities = lines
            .map(|line: &str| {
                (
                    line[..line.len() / 2].chars().collect::<HashSet<char>>(),
                    line[line.len() / 2..].chars().collect::<HashSet<char>>(),
                )
            })
            .map(|(s1, s2)| {
                s1.into_iter()
                    .filter(|c| s2.contains(c))
                    .collect::<HashSet<char>>()
            })
            .map(|chars| chars.iter().map(|c| letter_values[c]).sum::<u32>())
            .sum::<u32>();

        format!("{}", priorities)
    } else {
        let mut priorities: u32 = 0;

        let mut group: LinkedList<HashSet<char>> = LinkedList::new();
        for (i, line) in lines.enumerate() {
            if i % 3 == 2 && i > 0 {
                priorities += line
                    .chars()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .filter(|c| group.iter().all(|s| s.contains(c)))
                    .map(|l| letter_values[&l])
                    .sum::<u32>();
                group.clear();
            } else {
                group.push_back(line.chars().collect());
            }
        }

        format!("{}", priorities)
    }
}
