use std::collections::btree_map::Values;

pub fn basic_function(name: &str) -> String {
    let greeting = format!("Hello, {}", name);
    greeting
}

// multiple_parameters
pub fn calculate_area(width: f64, height: f64) -> f64 {
    if width < 0 as f64 || height < 0 as f64 {
        return 0.0;
    } else {
        width * height
    }
}

pub fn is_enven(num: i32) -> bool {
    num % 2 == 0
}

pub fn factorial(num: u32) -> u32 {
    if num == 0 {
        return 1;
    } else {
        return num * factorial(num - 1);
    }
}

pub fn find_max<T: Ord>(value: &[T]) -> Option<&T> {
    let ans = value.iter().max();
    let ans = match ans {
        Some(max) => Some(max),
        None => None,
    };
    ans
}

pub fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
