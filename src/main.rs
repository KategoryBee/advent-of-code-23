use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 8, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn limit_for_colour(colour: &str) -> Option<i32> {
    static LIMITS: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    for l in LIMITS {
        if l.0 == colour {
            return Some(l.1);
        }
    }

    None
}

fn solve(path: &str) -> i32 {
    let input = read_lines(path).unwrap();

    let mut solution = 0;

    for line in input {
        let line = line.unwrap();
        let (game_id, reveals) = line.split_once(':').unwrap();

        let game_id: i32 = game_id.strip_prefix("Game ").unwrap().parse().unwrap();

        let mut all_ok = true;

        for reveal in reveals.split(';') {
            for color in reveal.split(',') {
                let (amount, color) = color.trim().split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();

                let limit = limit_for_colour(color).unwrap();
                if amount > limit {
                    all_ok = false;
                }
            }
        }

        if all_ok {
            solution += game_id;
        }
    }

    solution
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
