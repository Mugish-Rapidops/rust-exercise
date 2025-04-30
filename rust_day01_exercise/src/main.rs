// 1. Declare a constant MAX_POINTS at the global level (outside main) with value 100,000 (using underscore separator).
const MAX_POINTS: u32 = 100_000;

use core::num;
use std::vec;

use rust_day01_exercise::{
    closure_exercise::closure_exercise::{
        apply_operation, basic_closure, bonus_fnonece_fnmut_fn, closure_environment_capture,
        clouser_in_iterator,
    }, function_exercise::function_exercise::{
        apply_twice, basic_function, calculate_area, factorial, find_max, is_enven,
    }, loop_exercise::loop_exercise::{basic_loop_with_counter, for_loop_with_range, loop_with_pattern_matching, nested_loops_with_labels, while_loop_with_condition}, variable_exercise::variable_exercise::{
        basic_variable, bonus_challenge, scope_and_shadowing, type_annotations, variable_shadowing,
    }
};
fn main() {
    basic_variable();
    variable_shadowing();

    // Question 3: Constants
    // 2. Print this constant inside main.
    println!("The maximum points are: {}", MAX_POINTS); // Print the constant

    // 3. Try to change the constant's value - what happens?
    // MAX_POINTS = 200_000; // uncommenting this line will cause a compile-time error

    // 4. Create another constant MIN_POINTS inside main with value 10 and print it.
    const MIN_POINTS: u32 = 10;
    println!("The minimum points are: {}", MIN_POINTS);

    // 5. Can you shadow the global MAX_POINTS constant with a let binding inside main?
    // let MAX_POINTS = 50_000; // we can't shadowing or change constant variable
    // println!("Shadowed MAX_POINTS: {}", MAX_POINTS);

    type_annotations();
    scope_and_shadowing();
    bonus_challenge();

    basic_closure();
    closure_environment_capture();
    let add_five = |x: i32| x + 5;
    let double = |x: i32| x * 2;
    println!("{}", apply_operation(add_five, 10));
    println!("{}", apply_operation(double, 10));
    clouser_in_iterator();
    bonus_fnonece_fnmut_fn();

    let greet = basic_function("Alice");
    println!("{}", greet);
    println!("{}", calculate_area(-1.0, 5.0));
    println!("{}", calculate_area(3.5, 4.2));
    println!("is 4 even? {}", is_enven(4));
    println!("is 7 even? {}", is_enven(7));
    println!("5! = {}", factorial(5));
    println!("0! = {}", factorial(0));
    let numbers = vec![32, 12, 78, 3];
    println!("Max number: {:?}", find_max(&numbers));
    let numbers: Vec<i32> = vec![];
    println!("Max number: {:?}", find_max(&numbers));
    let words = vec!["apple", "banana", "cherry"];
    println!("Max word: {:?}", find_max(&words));
    let increament = |x| x * 2;
    println!("{}", apply_twice(increament, 5));

    basic_loop_with_counter();
    while_loop_with_condition();
    for_loop_with_range();
    nested_loops_with_labels();
    loop_with_pattern_matching();
}
