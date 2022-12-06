use std::collections::LinkedList;

static EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let (crates_str, moves_str): (&str, &str) = data.split_once("\n\n").unwrap();
    let mut stacks = Vec::<LinkedList<char>>::new();
    let crates = crates_str.split("\n").collect::<Vec<&str>>();
    let indices = crates.last().unwrap();

    for (x, c) in indices.chars().enumerate() {
        if c.is_numeric() {
            let mut stack = LinkedList::<char>::new();
            for y in 1..crates.len() {
                let crate_denotation = crates[crates.len() - 1 - y].chars().nth(x).unwrap_or(' ');
                if crate_denotation.is_alphabetic() {
                    stack.push_back(crate_denotation)
                }
            }
            stacks.push(stack);
        }
    }

    for move_str in moves_str.trim().split("\n") {
        let m_words = move_str
            .split(" ")
            .map(|p| p.parse::<i32>().unwrap_or(-1))
            .collect::<Vec<i32>>();
        let (count, from_stack, to_stack) = (m_words[1], m_words[3] - 1, m_words[5] - 1);

        if simple {
            for _ in 0..count {
                let v = stacks[from_stack as usize].pop_back().unwrap();
                stacks[to_stack as usize].push_back(v)
            }
        } else {
            let mut intermediary = LinkedList::<char>::new();
            for _ in 0..count {
                let v = stacks[from_stack as usize].pop_back().unwrap();
                intermediary.push_back(v);
            }
            for _ in 0..count {
                let v = intermediary.pop_back().unwrap();
                stacks[to_stack as usize].push_back(v);
            }
        }
    }
    let tops = stacks
        .iter()
        .map(|s| s.back().unwrap_or(&' '))
        .collect::<String>();

    format!("\n{}", tops)
}
