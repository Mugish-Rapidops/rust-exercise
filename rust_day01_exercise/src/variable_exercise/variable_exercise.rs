pub fn basic_variable() {
    /*Question 1: Basic Variables */
    /*  1. Declare an immutable variable x with the value 5 and print it. */
    let x = 5;
    println!("x: {}", x);

    /*2. Try to change x to 10 - what happens? Comment out this line after you see the error. */
    // x = 10; // x is declare as a immutable varibale than we can't change it's value to do this we have to declare x as a mutable varible

    /*3. Declare a mutable variable y with the value 3 and print it. */
    let mut y = 3;
    println!("y: {}", y);

    /*4. Change y to 7 and print it again. */
    y = 10;
    println!("y: {}", y);

    /*5. Declare a new immutable variable z that contains the sum of x and y, then print it. */
    let mut z = 0;
    println!("z: {}", z);
    z = x + y;
    println!("z = x + y : {}", z);
}

pub fn variable_shadowing() {
    /*Question 2: Variable Shadowing */
    /*1. Declare a variable number with value 5 and print it. */
    let number = 5;
    println!("number: {}", number);

    /*2. Shadow the original number by multiplying it by 2 and print the new value*/
    let number = number * 2;
    println!("shadowed number: {}", number);

    /*3. Shadow number again by changing it to a string "five" and print it. */
    let number = "five".to_string();
    println!("shadowed number: {}", number);

    /*4. What happens if you try to use the string number in a mathematical operation like number + 10? */
    // number = number + 10; // since last we shadowed number with string than we can't do mathematical operation
}

pub fn type_annotations() {
    // Question 4: Type Annotations
    // 1. Declare variables with these explicit types and values:
    // • a : i32 with value 42
    let a: i32 = 42;
    // • b : f64 with value 3.14
    let b: f64 = 3.14;
    // • c: bool with value true
    let c: bool = true;
    // • d : char with value 'R'
    let d: char = 'R';

    // Print all four variables.
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);

    // 1. What happens if you try to assign 3.14 to an i32 variable?
    // let a: i32 = 3.14; // uncommenting this line will cause a compile-time error because 3.14 is a f64, not an i32.

    // 2. What happens when you assign 42 to an f64 variable?
    let _b: f64 = 42.0; // This is valid because 42 can be implicitly converted to f64.
}

// Question 5: Scope and Shadowing

pub fn scope_and_shadowing() {
    // Question 5: Scope and Shadowing
    // 1. Declare a variable outer with value 10 and print it.
    let outer = 10;
    println!("outer: {}", outer);

    // 2. Create a new block (with curly braces).
    {
        // Inside, declare a variable inner with value 5 and print it.
        let inner = 5;
        println!("inner: {}", inner);

        // Shadow outer by redeclaring it as a string "ten" and print it.
        let outer = "ten";
        println!("shadowed outer: {}", outer);
    }

    // 3. After the block, print outer again - what value does it have?
    println!("outer after block: {}", outer); // Print the original outer value (10)

    // 4. Try to access inner outside the block - what happens?
    // println!("inner outside block: {}", inner); // uncommenting this line will cause a compile-time error because inner is not accessible outside its block.
}

pub fn bonus_challenge() {
    // Bonus Challenge
    // Create a program that:
    // 1. Starts with an immutable variable count set to 0
    let count = 0;
    // 2. Uses shadowing to increment it three times (to 3)
    let count = count + 1;
    let count = count + 1;
    let count = count + 1;
    // 3. Then declares it as mutable and increments it two more times (to 5)
    let mut count = count;
    count += 1;
    count += 1;
    // 4. Finally shadows it one last time as a string "five" and prints it
    let count = "five".to_string();
    println!("count: {}", count);
}
