use std::collections::HashSet;

static EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let ws = match simple {
        true => 4,
        false => 14,
    };
    for i in 0..(data.len() - ws) {
        let window: Vec<char> = data.chars().skip(i).take(ws).collect();
        if HashSet::<char>::from_iter(window.into_iter()).len() == ws {
            return format!("{}", i + ws);
        }
    }

    format!("\n{}", "solution not found!")
}
