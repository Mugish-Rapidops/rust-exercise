use rust_day02_exercise::{
    enum_exercise::enum_exercise::{
        find_user, first_char, parse_config_value, safe_divide, ConfigValue, User
    },
    match_exercise::match_exercise::{
        file_type, get_permissions, light_duration, move_robot, process_payment, role_name, Direction, PaymentMethod, Position, TrafficLight, UserRole
    },
    ownership_exercise::ownership_exercise::{
        copy_vs_move, function_return_ownership, function_takes_ownership, ownership_move,
    },
    result_exercise::result_exercise::{
        divide_strings, parse_even_number, read_file_contents, sum_results, validate_user, MathError
    },
};

fn main() {
    let division_result = safe_divide(10.0, 10.0);
    if let Some(result) = division_result {
        println!("division result :{}", result);
    }

    let s = "hhh";
    let first_char = first_char(s);
    assert_eq!(first_char, Some('h'));

    let mut users = Vec::new();
    for i in 0..=10 {
        users.push(User {
            id: i,
            name: format!("{}th user", i).to_string(),
        });
    }
    assert_eq!(find_user(&users, 10).unwrap().name, "10th user".to_string());
    assert_eq!(find_user(&users, 102), None);

    assert_eq!(parse_config_value("42"), Some(ConfigValue::Int(42)));
    assert_eq!(parse_config_value("true"), Some(ConfigValue::Bool(true)));
    assert_eq!(parse_config_value("3.14"), Some(ConfigValue::Float(3.14)));
    assert_eq!(
        parse_config_value("hello"),
        Some(ConfigValue::String("hello".to_string()))
    );
    assert_eq!(parse_config_value("not_a_number"), None);

    ownership_move();
    let msg = String::from("Rust is fun!");
    function_takes_ownership(&msg);
    println!("Again: {}", msg);
    let name: String;
    name = function_return_ownership();
    println!("Name: {}", name);
    copy_vs_move();

    let exists = read_file_contents("Cargo.toml");
    println!("{:?}", exists);
    let missing = read_file_contents("nonexistent.txt");
    println!("{:?}", missing);

    assert_eq!(parse_even_number("42"), Ok(42));
    assert_eq!(
        parse_even_number("43"),
        Err("Number must be even".to_string())
    );
    assert_eq!(
        parse_even_number("abc"),
        Err("Invalid number format".to_string())
    );
    let valid = validate_user("jone_doe1712".to_string(), 25);
    println!("valid {:?}", valid);
    let invalid = validate_user("x".to_string(), 250);
    println!("invalid {:?}", invalid);
    assert_eq!(sum_results(Ok(1), Ok(2)), Ok(3));
    assert_eq!(
        sum_results(Ok(1), Err("error".to_string())),
        Err("error".to_string())
    );
    assert_eq!(divide_strings("4", "2"), Ok(2.0));
    // assert_eq!(divide_strings("a", "2"), Err(MathError::ParseError(_)));
    match divide_strings("a", "2") {
        Err(MathError::ParseError(_)) => assert!(true),
        _ => assert!(false, "Expected ParseError variant"),
    }
    assert_eq!(divide_strings("4", "0"), Err(MathError::DivisionByZero));

    //  match exercise 1: Traffic Light Controller
    light_duration(TrafficLight::Red);
    light_duration(TrafficLight::Yellow);
    light_duration(TrafficLight::Green);

    //  match exercise 2: File Extension Analyzer
    let extension = "txt";
    file_type(extension);

    //  match exercise 3: Payment Processor
    let payment_msg_for_cash = process_payment(PaymentMethod::Cash, 1000.00);
    println!("{}", payment_msg_for_cash);

    let payment_msg_for_cc = process_payment(
        PaymentMethod::CreditCard("number".to_string(), 1, 2030),
        1000.00,
    );
    println!("{}", payment_msg_for_cc);

    let payment_msg_for_pp = process_payment(
        PaymentMethod::PayPal("tempmail@gmail.com".to_string()),
        1000.00,
    );
    println!("{}", payment_msg_for_pp);

    let payment_msg_for_crypto =
        process_payment(PaymentMethod::Crypto("wallet address".to_string()), 1000.00);
    println!("{}", payment_msg_for_crypto);

    //  match exercise 4: Robot Direction Controller
    let current = Position { x: 0, y: 0 };
    let new_position = move_robot(current, Direction::Up);
    println!("new position: {:?}", new_position);

    //  match exercise 5: User Role Permissions
    let roles = vec![
        UserRole::Guest,
        UserRole::User,
        UserRole::Moderator,
        UserRole::Admin,
    ];

    for role in roles {
        let permissions = get_permissions(&role);
        println!("{:?} permissions: {:?}", role_name(role), permissions);
    }
}
