use std::io::{self, Read};

pub fn basic_loop_with_counter() {
    let mut counter = 0;
    loop {
        if counter == 10 {
            break;
        }
        println!("{}", counter);
        counter += 2;
    }
}

pub fn while_loop_with_condition() {
    let mut counter = 5;
    while counter > 0 {
        println!("{}", counter);
        if counter == 1 {
            println!("liftoff!");
        }
        counter -= 1;
    }
}

pub fn for_loop_with_range() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}

pub fn nested_loops_with_labels() {
    'outer: for i in 1..=3 {
        if i == 3 {
            break 'outer;
        }
        let ascii = 96;
        'inner: for j in 1..=3 {
            if i == 2 && j == 3 {
                break 'inner;
            }
            let ascii = ascii + j as u8;
            println!("{} {}", i, ascii as char)
        }
    }
}

pub fn loop_with_pattern_matching() {
    loop {
        println!("Enter a number:");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line");
            continue;
        }
        let input = match input.trim().parse::<i32>() {
            Ok(input) => input,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        match input {
            0 => break,
            input if input % 2 == 0 => println!("Even"),
            input if input % 2 != 0 => println!("Odd"),
            _ => println!("Please enter a number"),
        }
    }
}
