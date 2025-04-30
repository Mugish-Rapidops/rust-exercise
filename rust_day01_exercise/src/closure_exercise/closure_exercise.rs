pub fn basic_closure() {
    let triple = |x: i32| x * 3;
    println!("5 tripled: {}", triple(5));
    println!("-2 tripled: {}", triple(-2));
}

pub fn closure_environment_capture() {
    let n = 10;
    let add_n = |x: i32| x + n;
    println!("5 + {} = {}", n, add_n(5));
    let n = -3;
    let add_n = |x: i32| x + n;
    println!("5 + {} = {}", n, add_n(5));
}

pub fn apply_operation(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

pub fn clouser_in_iterator() {
    let result = 1..=20;
    let result: i32 = result
        .map(|val: i32| if val % 2 == 0 { val * val } else { 0 })
        .sum::<i32>();
    println!("Sum of squared of even numbers: {}", result);
}

pub fn bonus_fnonece_fnmut_fn() {
    let name = "Alice".to_string();
    let greet_once = move || {
        println!("Hello {}", name);
        name
    };
    greet_once();

    let mut counter = 0;
    let mut increatment = || {
        counter += 1;
        println!("Counter: {}", counter);
    };
    increatment();
    increatment();

    let read_only = |x: i32| x + counter;
    println!("Read: {}", read_only(5));
}
