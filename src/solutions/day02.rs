use std::collections::HashMap;

static EXAMPLE: &str = "A Y
B X
C Z";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));
    let rounds = data.trim().split("\n");

    let points = match simple {
        true => HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]),
        false => HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 2), ("Y", 0), ("Z", 1)]),
    };

    let mut score = 0;
    for round in rounds {
        let choices = round.split(" ").map(|c| points[c]).collect::<Vec<i32>>();
        let p1 = choices[0];
        let p2 = choices[1];

        if simple {
            let i = ((3 + p2 - p1) % 3) as usize;
            score += p2;
            score += [3, 6, 0][i];
        } else {
            let mut c = (p2 + p1) % 3;
            if c == 0 {
                c = 3
            }
            score += c;
            score += [3, 6, 0][p2 as usize];
        }
    }

    format!("{}", score)
}
