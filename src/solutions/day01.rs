static EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));
    let calories_per_elf = data.trim().split("\n\n").map(|chunk| {
        chunk
            .split("\n")
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
    });

    if simple {
        format!("{}", calories_per_elf.max().unwrap())
    } else {
        let mut calories_per_elf_vec = calories_per_elf.collect::<Vec<i32>>();
        calories_per_elf_vec.sort_by(|a, b| b.cmp(a));
        format!("{}", calories_per_elf_vec[..3].iter().sum::<i32>())
    }
}
