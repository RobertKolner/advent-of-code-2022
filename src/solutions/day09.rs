use std::collections::HashSet;

static EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));
    let rope_length = if simple { 2 } else { 10 };

    let mut visited_cells = HashSet::<(i32, i32)>::from([(0, 0)]);

    let mut rope = Vec::<(i32, i32)>::with_capacity(rope_length);
    for _ in 0..rope_length {
        rope.push((0, 0));
    }

    for line in data.trim().split("\n") {
        let (direction, amount_str) = line.split_once(" ").unwrap();
        let amount = amount_str.parse::<i32>().unwrap();

        for _ in 0..amount {
            let head = rope[0];
            rope[0] = match direction {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                d => panic!("Invalid direction {}", d),
            };

            for i in 0..rope.len() - 1 {
                let knot_front = rope[i];
                let knot_back = rope[i + 1];

                let diff = (knot_front.0 - knot_back.0, knot_front.1 - knot_back.1);
                rope[i + 1] = match diff {
                    (0, 0) => continue,
                    (dh, 0) if dh.abs() > 1 => (knot_back.0 + dh.clamp(-1, 1), knot_back.1),
                    (0, dv) if dv.abs() > 1 => (knot_back.0, knot_back.1 + dv.clamp(-1, 1)),
                    (dh, dv) if dh.abs() > 1 || dv.abs() > 1 => {
                        (knot_back.0 + dh.clamp(-1, 1), knot_back.1 + dv.clamp(-1, 1))
                    }
                    _ => continue,
                };
            }
            visited_cells.insert(*rope.last().unwrap());
        }
    }

    format!("{}", visited_cells.len())
}
