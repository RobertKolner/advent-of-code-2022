use std::collections::HashSet;

static EXAMPLE: &str = "30373
25512
65332
33549
35390";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let land: Vec<Vec<i32>> = data
        .trim()
        .split("\n")
        .map(|l| {
            l.split("")
                .filter(|c| c.len() > 0)
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    let y_size = land.len();
    let x_size = land[0].len();

    if simple {
        let mut tree_set = HashSet::<(usize, usize)>::new();
        let mut current_highest = [-1, -1, -1, -1];
        let mut update_highest = |direction: usize, (x, y): (usize, usize)| {
            if x == usize::MAX && y == usize::MAX {
                current_highest[direction] = -1;
            } else if land[y][x] > current_highest[direction] {
                current_highest[direction] = land[y][x];
                tree_set.insert((x, y));
            };
        };

        for y in 0..y_size {
            for x in 0..x_size {
                update_highest(0, (x, y));
                update_highest(1, (x_size - x - 1, y));
            }
            update_highest(0, (usize::MAX, usize::MAX));
            update_highest(1, (usize::MAX, usize::MAX));
        }
        for x in 0..x_size {
            for y in 0..y_size {
                update_highest(2, (x, y));
                update_highest(3, (x, y_size - y - 1));
            }
            update_highest(2, (usize::MAX, usize::MAX));
            update_highest(3, (usize::MAX, usize::MAX));
        }

        format!("{}", tree_set.len())
    } else {
        let mut max_scenic_score = 0;
        for y in 1..y_size - 1 {
            for x in 1..x_size - 1 {
                let mut scenic_score = 1;
                let tree = land[y][x];

                let mut dx = 1;
                scenic_score *= loop {
                    if x + dx >= x_size - 1 || land[y][x + dx] >= tree {
                        break dx;
                    }
                    dx += 1;
                };

                dx = 1;
                scenic_score *= loop {
                    if dx >= x || land[y][x - dx] >= tree {
                        break dx;
                    }
                    dx += 1;
                };

                let mut dy = 1;
                scenic_score *= loop {
                    if y + dy >= y_size - 1 || land[y + dy][x] >= tree {
                        break dy;
                    }
                    dy += 1;
                };

                dy = 1;
                scenic_score *= loop {
                    if dy >= y || land[y - dy][x] >= tree {
                        break dy;
                    }
                    dy += 1;
                };

                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }
        format!("{}", max_scenic_score)
    }
}
