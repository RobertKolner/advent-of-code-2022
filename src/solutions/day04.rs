static EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

pub fn solve(input_data: Option<String>, simple: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE));
    let lines = data.trim().split("\n");

    let mut included_assignments: u32 = 0;
    for line in lines {
        let elves = line.split(",").collect::<Vec<&str>>();
        assert_eq!(elves.len(), 2);

        let e1 = elves[0];
        let e2 = elves[1];

        let convert_range = |v: &str| v.parse::<u32>().unwrap();
        let e1r = e1.split("-").map(convert_range).collect::<Vec<u32>>();
        let e2r = e2.split("-").map(convert_range).collect::<Vec<u32>>();

        assert_eq!(e1r.len(), 2);
        assert_eq!(e2r.len(), 2);

        let range_contained_in = match simple {
            true => |r1: (u32, u32), r2: (u32, u32)| r1.0 >= r2.0 && r1.1 <= r2.1,
            false => |r1: (u32, u32), r2: (u32, u32)| {
                (r1.0 >= r2.0 && r1.0 <= r2.1) || (r1.1 >= r2.0 && r1.1 <= r2.1)
            },
        };
        if range_contained_in((e1r[0], e1r[1]), (e2r[0], e2r[1]))
            || range_contained_in((e2r[0], e2r[1]), (e1r[0], e1r[1]))
        {
            included_assignments += 1
        }
    }

    format!("{}", included_assignments)
}
