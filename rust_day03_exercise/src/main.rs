use rust_day03_exercise::string_exercise::string_exercise::{
    combine_names, count_vowels, is_strong_password, smart_truncate, to_title_case,
};

fn main() {
    let full_name = combine_names("Mugish", "Beldar");
    println!("Full name: {}", full_name);

    let vowels = count_vowels("VowEl");
    println!("vowels: {}", vowels);

    let title_case = to_title_case("Convert in title case");
    println!("title case: {}", title_case);

    let smart_truncate = smart_truncate("smart truncate", 8);
    println!("smart truncate:---{}", smart_truncate);

    let strong_psw = is_strong_password("Str0ng P@s$w0rd");
    println!("is strong psw: {}", strong_psw);
}
