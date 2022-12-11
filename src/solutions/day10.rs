use std::collections::{BinaryHeap, HashMap};

static EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));
    let mut signal_strength_sum = 0;
    let mut display = Vec::<Vec<char>>::with_capacity(6);
    for _ in 0..6 {
        display.push(vec!['.'; 40]);
    }

    let mut cycle: i32 = 0;
    let mut reg_x = 1;
    let mut instruction_queue = BinaryHeap::<(i32, &str, i32)>::new();

    let instruction_lengths = HashMap::from([("addx", 2)]);
    let mut lines_iter = data.trim().split("\n").into_iter();
    loop {
        cycle -= 1; // BinaryHeap sorts descending, thus this saves work

        // Part 1
        if ((-cycle + 20) % 40) == 0 {
            let signal_strength = -cycle * reg_x;
            signal_strength_sum += signal_strength;
        }

        // Part 2
        if -cycle <= 240 {
            let display_y = (-cycle - 1) / 40;
            let display_x = (-cycle - 1) % 40;
            for x in (reg_x - 1)..(reg_x + 2) {
                if x == display_x {
                    display[display_y as usize][display_x as usize] = '#';
                }
            }
        }

        if instruction_queue.is_empty() {
            let line = match lines_iter.next() {
                Some(l) => l,
                _ => break,
            };
            match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
                ["noop"] => (),
                [instruction, value_str] if instruction_lengths.contains_key(instruction) => {
                    let instruction_length = instruction_lengths.get(instruction).unwrap();
                    let value = value_str.parse::<i32>().unwrap();
                    instruction_queue.push((cycle - instruction_length + 1, instruction, value));
                }
                v => panic!("Invalid command {:?}", v),
            };
        }

        match instruction_queue.peek() {
            Some((c, _, _)) => {
                if *c == cycle {
                    let (_, instruction, value) = instruction_queue.pop().unwrap();
                    match instruction {
                        "addx" => reg_x += value,
                        _ => (),
                    };
                }
            }
            _ => (),
        }
    }

    if !simple {
        for line in display {
            println!("{}", line.iter().collect::<String>());
        }
    }

    format!("{}", signal_strength_sum)
}
