use chrono::Utc;
use rust_day03_exercise::{
    generic_types_and_traits::generic_types_and_traits::{
        NewsArticle, Payload, Summarize, SysOp, SystemCall, TraitBoundStruct, Tweet, Vehicle,
        file_reader, mock_api_call, process_item, smallest, track,
    },
    string_exercise::string_exercise::{
        combine_names, count_vowels, is_strong_password, smart_truncate, to_title_case,
    },
};

#[tokio::main]
async fn main() {
    // string exercise
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

    // generic types and traits exercise

    // exercise:1
    let small_num = smallest(10, 20);
    println!("Small num: {}", small_num);

    // exercise: 2
    let todo = mock_api_call(Payload { id: 1 }).await;

    match todo {
        Ok(todo) => println!("todo: {:?}", todo.data),
        Err(err) => println!("Error: {:?}", err),
    }

    // exercise: 3
    let vehicle = Vehicle {
        name: "Audi".to_string(),
    };
    track(vehicle);

    let sys = { SysOp::new() };
    println!("sys: {:?}", sys);

    // exercise: 4
    let trait_bound_struct = TraitBoundStruct {
        name: "trait bound struct".to_string(),
    };
    process_item(trait_bound_struct);

    // exercise: 5
    let article = NewsArticle::new(
        "Rust trait".to_string(),
        Utc::now(),
        "Rust trait".to_string(),
        "Graydon Hoare".to_string(),
    );
    let article_summary = article.summary();
    println!("article: {}", article_summary);

    let tweet = Tweet::new(
        Utc::now(),
        "Rust trait".to_string(),
        "tweet on Rust trait ".to_string(),
        "Jone Doe".to_string(),
    );
    let tweet_summary = article.summary();
    println!("tweet: {}", tweet_summary);

    // error handling exercise

    let data = file_reader(None);
    match data {
        Ok(data) => println!("file data: {}", data),
        Err(e) => println!("Got Error {}", e),
    };
}
