pub fn ownership_move() {
    let city = String::from("Mumbai");
    let _location = city.to_string();
    // `to_string()` creates a clone of the string, so `city` remains valid
    println!("City: {}", city);
}

pub fn function_takes_ownership(msg: &String) {
    println!("Message: {}", msg);
}

pub fn function_return_ownership() -> String {
    String::from("Tejas")
}

pub fn copy_vs_move() {
    let x = 10;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Copy trait allows primitive types to be duplicated

    let s1 = String::from("Ownership");
    let s2 = s1.to_owned();
    // Cloning heap data avoids ownership transfer
    println!("s1: {}, s2: {}", s1, s2);
}
