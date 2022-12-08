use std::collections::HashMap;

static EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));

    let mut dir_sizes = HashMap::<String, u32>::new();
    let mut current_path = Vec::new();

    for line in data.trim().split("\n") {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", "/"] => {
                current_path.clear();
                current_path.push("");
            }
            ["$", "cd", ".."] => {
                current_path.pop();
            }
            ["$", "cd", d] => {
                current_path.push(*d);
            }
            ["$", ..] => continue,
            ["dir", ..] => continue,
            [filesize_str, _] => {
                let filesize = filesize_str.parse::<u32>().unwrap_or(0);
                let mut path_segment_stack = current_path.clone();
                while !path_segment_stack.is_empty() {
                    let current_dir_path = path_segment_stack.join("/");
                    dir_sizes
                        .entry(current_dir_path)
                        .and_modify(|ds| *ds += filesize)
                        .or_insert(filesize);
                    path_segment_stack.pop();
                }
            }
            _ => println!("Unknown pattern: {}", line),
        }
    }

    return if simple {
        let mut cleanable_dir_sizes = 0;
        for (_, s) in dir_sizes.iter() {
            if *s < 100000 {
                cleanable_dir_sizes += *s;
            }
        }
        format!("\n{}", cleanable_dir_sizes)
    } else {
        let current_free = 70000000 - dir_sizes.get("").unwrap();
        let need_to_delete = 30000000 - current_free;
        let min_size = dir_sizes
            .iter()
            .filter_map(|(_, s)| if *s >= need_to_delete { Some(*s) } else { None })
            .collect::<Vec<u32>>()
            .into_iter()
            .min()
            .unwrap();
        format!("{}", min_size)
    };
}
