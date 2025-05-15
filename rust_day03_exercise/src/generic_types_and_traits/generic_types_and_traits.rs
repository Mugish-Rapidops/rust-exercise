use std::{
    fmt::{Debug, Display},
    fs::{self, File},
    io::{self, Write},
};

use chrono::{DateTime, Utc};
use reqwest;
use serde::Deserialize;

pub fn smallest<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        return y;
    } else {
        return x;
    }
}

#[derive(Deserialize, Debug)]
pub struct Todo {
    pub userId: u32,
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct Payload<IdType> {
    pub id: IdType,
}

#[derive(Debug)]
pub struct Response<DataType> {
    pub data: DataType,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    BadRequest,
    ServerError,
}

pub async fn mock_api_call<Identity: Display>(
    payload: Payload<Identity>,
) -> Result<Response<Todo>, ApiError> {
    let url = format!("https://jsonplaceholder.typicode.com/todos/{}", payload.id);
    let todo_response = reqwest::get(&url).await;

    match todo_response {
        Ok(response) => {
            if response.status().is_success() {
                let todo = response.json::<Todo>().await;
                match todo {
                    Ok(todo) => Ok(Response { data: todo }),
                    Err(_) => Err(ApiError::BadRequest),
                }
            } else {
                Err(ApiError::NotFound)
            }
        }
        Err(_) => Err(ApiError::ServerError),
    }
}

pub trait Trackable {}

#[derive(Debug)]
pub struct Vehicle {
    pub name: String,
}

impl Trackable for Vehicle {}

impl Display for Vehicle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn track<T: Trackable + Debug>(item: T) {
    println!("Tracking an item {:?}", item);
}

#[derive(Debug)]
pub struct SysOp {
    arch: String,
}
pub unsafe trait SystemCall {
    fn new() -> SysOp;
}

unsafe impl SystemCall for SysOp {
    fn new() -> SysOp {
        SysOp {
            arch: "amd64".to_string(),
        }
    }
}

pub struct SafeStruct {
    pub value: i32,
}

// Test if SafeStruct is Send
fn test_safe_struct() {
    is_send::<SafeStruct>();
}

// Define a struct containing raw pointers
pub struct UnsafeStruct {
    pub ptr: *const i32,
}

// Test if UnsafeStruct is Send
fn test_unsafe_struct() {
    // is_send::<UnsafeStruct>();
}

// Function to check if a type is Send
fn is_send<T: Send>() {}

pub trait Print {
    fn print(&self);
}

pub trait Create {
    fn new(name: String) -> Self;
}
#[derive(Debug, Clone)]
pub struct TraitBoundStruct<T> {
    pub name: T,
}

impl<T: Debug> Print for TraitBoundStruct<T> {
    fn print(&self) {
        print!("{:?}", self.name);
    }
}

impl Create for TraitBoundStruct<String> {
    fn new(name: String) -> Self {
        Self { name }
    }
}

pub fn process_item<T: Debug + Clone>(item: T) {
    println!("{:?}", item);
    let cloned_item = item.clone();
}

pub trait Summarize {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub title: String,
    pub date: DateTime<Utc>,
    pub conent: String,
    pub author: String,
}

pub struct Tweet {
    pub title: String,
    pub date: DateTime<Utc>,
    pub tweet: String,
    pub user_name: String,
}

impl Summarize for NewsArticle {
    fn summary(&self) -> String {
        format!(
            "{} {} {} {}",
            self.author, self.title, self.conent, self.date
        )
    }
}

impl NewsArticle {
    pub fn new(title: String, date: DateTime<Utc>, conent: String, author: String) -> Self {
        NewsArticle {
            title,
            date,
            conent,
            author,
        }
    }
}
impl Summarize for Tweet {
    fn summary(&self) -> String {
        format!(
            "{} {} {} {}",
            self.user_name, self.tweet, self.title, self.date
        )
    }
}
impl Tweet {
    pub fn new(date: DateTime<Utc>, title: String, tweet: String, user_name: String) -> Self {
        Tweet {
            date,
            title,
            tweet,
            user_name,
        }
    }
}

// error handling exercise
pub fn file_reader(file_name: Option<String>) -> Result<String, io::Error> {
    let name = file_name.unwrap_or_else(|| "example.txt".to_string());
    fs::read_to_string(name).or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            let mut created_file = File::create("example.txt")?;
            let insert_data = "data from example file".to_string();
            created_file.write_all(insert_data.as_bytes())?;
            fs::read_to_string("example.txt")
        } else {
            Err(e)
        }
    })
}
