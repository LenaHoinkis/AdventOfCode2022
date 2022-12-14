use std::fs;

fn main() {
    //Read Stacks
    let input = fs::read_to_string("input.txt").unwrap();
    //(crate,order,char)
    let emptyVec = Vec::new();
    let mut stacks: Vec<Vec<_>> = vec![emptyVec; 9];
    let mut start_of_commands = 0;

    //fill stack
    for (i, line) in input.lines().enumerate() {
        let x: Vec<(usize, char)> = line
            .char_indices()
            .filter(|c| c.1.is_ascii_alphabetic())
            .map(|c| ((c.0 + 2) / 4, c.1))
            .collect();

        if x.is_empty() {
            start_of_commands = i + 1;
            break;
        }
        for c in x {
            stacks[c.0].push(c.1)
        }
    }
    stacks.iter_mut().for_each(|x| x.reverse());
    let mut stacks_copy = stacks.clone();

    for (i, line) in input.lines().enumerate() {
        if i >= start_of_commands + 1 {
            // amount, old, new
            let command: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|c| c.to_string().parse::<usize>())
                .filter(|c| !c.is_err())
                .map(|c| c.unwrap())
                .collect();

            //part1
            for _ in 0..command[0] {
                let popped = stacks_copy[command[1] - 1].pop();
                match popped {
                    Some(i) => stacks_copy[command[2] - 1].push(i),
                    _ => unreachable!(),
                }
            }
            //part2
            let tail = stacks[command[1] - 1].len() - command[0];
            let mut drained: Vec<_> = stacks[command[1] - 1].drain(tail..).collect();
            stacks[command[2] - 1].append(&mut drained);
        }
    }
    println!("{}", get_code(stacks));
    println!("{}", get_code(stacks_copy));
}

fn get_code(vec: Vec<Vec<char>>) -> String {
    let mut buf = Vec::new();
    for ele in vec {
        buf.push(ele.last().unwrap().clone());
    }
    buf.into_iter().collect()
}
