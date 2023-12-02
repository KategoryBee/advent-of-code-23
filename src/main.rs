use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 2286, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i32 {
    let input = read_lines(path).unwrap();

    let mut solution = 0;

    for line in input {
        let line = line.unwrap();
        let (_game_id, reveals) = line.split_once(':').unwrap();

        let mut most_red = 0;
        let mut most_green = 0;
        let mut most_blue = 0;

        for reveal in reveals.split(';') {
            for colour in reveal.split(',') {
                let (amount, colour) = colour.trim().split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();

                let bucket = match colour {
                    "red" => &mut most_red,
                    "green" => &mut most_green,
                    "blue" => &mut most_blue,
                    _ => panic!("unknown colour{}", colour),
                };

                if *bucket < amount {
                    *bucket = amount;
                }
            }
        }

        let cube_power = most_red * most_green * most_blue;
        solution += cube_power;
    }

    solution
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
