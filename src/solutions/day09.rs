use std::collections::HashSet;

static EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub fn solve(input_data: Option<String>, _simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let mut visited_cells = HashSet::<(i32, i32)>::from([(0, 0)]);

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut check_tail = |old_head: (i32, i32), new_head: (i32, i32)| {
        let tail_diff = ((tail.0 - new_head.0), (tail.1 - new_head.1));
        let new_tail_diff = (tail_diff.0.clamp(-1, 1), tail_diff.1.clamp(-1, 1));
        if tail_diff != new_tail_diff {
            tail = old_head;
            visited_cells.insert(tail);
        }
    };

    for line in data.trim().split("\n") {
        let (direction, amount_str) = line.split_once(" ").unwrap();
        let amount = amount_str.parse::<i32>().unwrap();

        for _ in 0..amount {
            let old_head = head.clone();
            head = match direction {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                d => panic!("Invalid direction {}", d),
            };
            check_tail(old_head, head);
        }
    }

    format!("{}", visited_cells.len())
}
