use std::io::Read;

const DATA_SIZE: usize = 300_000;

fn inc_dp(dp: usize) -> usize {
    if dp >= DATA_SIZE - 1 {
        return 0;
    }

    return dp + 1;
}

fn dec_dp(dp: usize) -> usize {
    if dp == 0 {
        return DATA_SIZE - 1;
    }

    return dp - 1;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Enter at least one argument!");
        return;
    }
    let mut data = [0u8; DATA_SIZE];

    let filename = &args[1];
    let instructions = if let Ok(f) = std::fs::read(filename) {
        f
    } else {
        println!("File not found: {filename}");
        return;
    };

    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    let mut output = String::new();

    // let stdin = std::io::stdin();
    let mut loop_stack;
    loop {
        if instruction_pointer >= instructions.len() {
            break;
        }
        let instruction = instructions[instruction_pointer] as char;

        if false && ['[', ']', '+', '-', '.', ',', '<', '>'].contains(&instruction) {
            println!("ip: {instruction_pointer}");
            println!("dp: {data_pointer}");
            println!("instruction: {instruction}");
            println!("data: {:?}\n", &data[0..100]);

            // stdin.read_line(&mut String::new()).unwrap();
        }

        match instruction {
            //  	Increment the data pointer by one
            '>' => {
                data_pointer = inc_dp(data_pointer);
            }
            // Decrement the data pointer by one
            '<' => {
                data_pointer = dec_dp(data_pointer);
            }
            // Increment the byte at the data pointer by one.
            '+' => {
                data[data_pointer] = data[data_pointer].wrapping_add(1);
            }
            // Decrement the byte at the data pointer by one.
            '-' => {
                data[data_pointer] = data[data_pointer].wrapping_sub(1);
            }
            // Output the byte at the data pointer.
            '.' => {
                output.push(data[data_pointer] as char);
            }
            // Accept one byte of input, storing its value in the byte at the data pointer.
            ',' => match std::io::stdin().bytes().next() {
                Some(Ok(b)) => data[data_pointer] = b,
                Some(Err(e)) => {
                    println!("Error: {e}");
                    return;
                }
                None => (),
            },
            /*
                If the byte at the data pointer is zero,
                then instead of moving the instruction pointer forward
                to the next command, jump it forward to the command
                after the matching ] command
            */
            '[' => {
                loop_stack = 0;
                let b = data[data_pointer];
                if b == 0 {
                    instruction_pointer += 1;
                    while instruction_pointer < instructions.len() {
                        let instruction = instructions[instruction_pointer] as char;

                        if instruction == '[' {
                            loop_stack += 1;
                        } else if instruction == ']' {
                            if loop_stack == 0 {
                                break;
                            }
                            loop_stack -= 1;
                        }
                        instruction_pointer += 1;
                    }
                }
            }
            /*
                If the byte at the data pointer is nonzero,
                then instead of moving the instruction pointer forward
                to the next command, jump it back to the command
                after the matching [ command.
            */
            ']' => {
                loop_stack = 0;
                let b = data[data_pointer];
                if b != 0 {
                    instruction_pointer -= 1;
                    loop {
                        let instruction = instructions[instruction_pointer] as char;

                        if instruction == ']' {
                            loop_stack += 1;
                        } else if instruction == '[' {
                            if loop_stack == 0 {
                                break;
                            }
                            loop_stack -= 1;
                        }
                        instruction_pointer -= 1;
                    }
                }
            }
            _ => {}
        };
        instruction_pointer += 1;
    }

    println!("{output}");
}
