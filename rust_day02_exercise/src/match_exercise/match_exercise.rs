pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub enum PaymentMethod {
    CreditCard(String, u8, u16),
    PayPal(String),
    Crypto(String),
    Cash,
}

pub fn light_duration(light: TrafficLight) -> u8 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 4,
        TrafficLight::Green => 15,
    }
}

pub fn file_type(extension: &str) -> &'static str {
    match extension {
        "text" => "TEXT",
        "rs" => "RUST",
        "jpg" => "Image",
        _ => "UNKNOWN",
    }
}

pub fn process_payment(method: PaymentMethod, amount: f64) -> String {
    match method {
        PaymentMethod::CreditCard(_, _, _) => {
            format!("Processed credit card payment of ${}", amount)
        }
        PaymentMethod::PayPal(_) => {
            format!("Processed PayPal payment of ${}", amount)
        }
        PaymentMethod::Crypto(_) => {
            format!("Processed crypto payment of ${}", amount)
        }
        PaymentMethod::Cash => {
            format!("Processed cash payment of ${}", amount)
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn move_robot(current: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => Position {
            x: current.x,
            y: current.y + 1,
        },
        Direction::Down => Position {
            x: current.x,
            y: current.y - 1,
        },
        Direction::Left => Position {
            x: current.x - 1,
            y: current.y,
        },
        Direction::Right => Position {
            x: current.x + 1,
            y: current.y,
        },
    }
}

pub enum UserRole {
    Guest,
    User,
    Moderator,
    Admin,
}

pub fn get_permissions(role: &UserRole) -> Vec<&'static str> {
    match role {
        UserRole::Guest => vec!["read"],
        UserRole::User => vec!["read", "comment"],
        UserRole::Moderator => vec!["read", "comment", "delete"],
        UserRole::Admin => vec!["read", "comment", "delete", "manage"],
    }
}

pub fn role_name(role: UserRole) -> String {
    match role {
        UserRole::Guest => "Guest".to_string(),
        UserRole::User => "User".to_string(),
        UserRole::Moderator => "Moderator".to_string(),
        UserRole::Admin => "Admin".to_string(),
    }
}
