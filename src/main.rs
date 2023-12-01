use std::io;

fn main() {
    let input = read_lines("input.txt").unwrap();
    let mut sum = 0;
    for line in input {
        let (first, last) = first_last_digit(&line.unwrap());

        let this_line = first * 10 + last;
        sum += this_line;
    }

    println!("Result: {sum}");
}

fn first_last_digit(mut input: &str) -> (i32, i32) {
    const TESTS: [(i32, &str, &str); 10] = [
        (0, "0", "zero"),
        (1, "1", "one"),
        (2, "2", "two"),
        (3, "3", "three"),
        (4, "4", "four"),
        (5, "5", "five"),
        (6, "6", "six"),
        (7, "7", "seven"),
        (8, "8", "eight"),
        (9, "9", "nine"),
    ];

    let mut first = None;
    let mut last = None;

    while !input.is_empty() {
        for (val, dig_test, str_test) in TESTS {
            if input.starts_with(dig_test) || input.starts_with(str_test) {
                last = Some(val);
                if first.is_none() {
                    first = last;
                }
            }
        }

        input = &input[1..];
    }

    (first.unwrap(), last.unwrap())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
