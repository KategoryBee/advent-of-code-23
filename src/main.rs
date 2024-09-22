use std::{collections::HashMap, fmt::Debug, io};

use regex::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 6, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let input = Input::parse(path);

    let starting_nodes: Vec<&str> = input
        .network
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| n.as_str())
        .collect();
    assert!(!starting_nodes.is_empty());

    // In this loop we collect the initial node name, how long until we see the node ending with
    // 'Z'. In our input the distant to the 'Z' node (there's only 1!), is the same as the cycle
    // length to get back to it. I assume the input was constructed this way. But i dislike that
    // it wasn't specified in the problem text.
    //
    // AND MORE IMPORTANTLY IT ISN'T TRUE FOR THE EXAMPLE DATA. which sucks even more. but the
    // lowest common multiple will still work on that data set
    let mut cycles = Vec::new();

    for &start in starting_nodes.iter() {
        let mut step = 0;
        let mut dir_idx = 0;

        // list of nodes / states we've already seen.
        // key is (Node Name, index in 'directions').
        // value is how many 'steps' in we were when we saw it, so we can figure out the length from
        // the starting node to the beginning of the cycle.
        let mut visited: HashMap<(String, usize), i64> = HashMap::new();
        let mut current_pos = start;

        loop {
            // insert details for current node. the logic is a bit cleaner for the initial starting
            // point if we do this up front, then take a step
            let already_visited = visited.insert((current_pos.to_owned(), dir_idx), step);

            if let Some(steps_last_seen_at) = already_visited {
                // ...
                let cycle_length = step - steps_last_seen_at;
                dbg!(cycle_length);
                break;
            }

            step += 1;

            let take_direction = input.directions[dir_idx];
            dir_idx += 1;
            if dir_idx >= input.directions.len() {
                dir_idx = 0;
            }

            // do the step thing.
            let go_to = if take_direction == b'L' {
                input.network[current_pos].0.as_str()
            } else {
                input.network[current_pos].1.as_str()
            };
            current_pos = go_to;

            if current_pos.ends_with('Z') {
                println!(
                    "ending position for {} at {} after {} steps",
                    start, current_pos, step
                );
                cycles.push(step);
            }
        }
    }

    cycles.iter().fold(1, |x, y| num::integer::lcm(x, *y))
}

#[derive(Debug)]
struct Input {
    directions: Vec<u8>,
    network: HashMap<String, (String, String)>,
}

impl Input {
    fn parse(path: &str) -> Self {
        // no Result return type, since out input from AOC should always be valid
        let mut input = read_lines(path).unwrap();

        let node_style = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let steps = input.next().unwrap().unwrap();
        let mut network = HashMap::new();

        for line in input {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }

            let caps = node_style.captures(&line).unwrap();

            let name = caps[1].to_owned();
            let to_l = caps[2].to_owned();
            let to_r = caps[3].to_owned();

            network.insert(name, (to_l, to_r));
        }

        Input {
            directions: steps.into_bytes(),
            network,
        }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
