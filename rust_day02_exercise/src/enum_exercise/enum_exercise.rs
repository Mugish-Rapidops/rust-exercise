#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum ConfigValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
}

pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn find_user(users: &Vec<User>, traget_id: u32) -> Option<User> {
    let user = users.iter().find(|&user| user.id == traget_id);
    user.cloned()
}

pub fn parse_config_value(value: &str) -> Option<ConfigValue> {
    if let Ok(val) = value.parse::<i32>() {
        return Some(ConfigValue::Int(val));
    }
    if let Ok(val) = value.parse::<f64>() {
        return Some(ConfigValue::Float(val));
    }
    if let Ok(val) = value.parse::<bool>() {
        return Some(ConfigValue::Bool(val));
    }
    if value.is_empty() || value.contains('_') {
        return None;
    } else {
        return Some(ConfigValue::String(value.to_string()));
    }
}
