// string concatenation
pub fn combine_names(first_name: &str, last_name: &str) -> String {
    let full_name = first_name.to_string() + " " + last_name;
    full_name
}

// Count the number of vowels (a,e,i, o,u) in a string (case insensitive)
pub fn count_vowels(text: &str) -> usize {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let text = text.to_lowercase();
    let mut count: usize = 0;
    for character in text.chars() {
        let vowel = vowels.iter().find(|&&vowel| character == vowel);
        if let Some(_) = vowel {
            count += 1;
        }
    }
    count
}

// Convert a string to title case (first letter of each word capitalized)
pub fn to_title_case(s: &str) -> String {
    let vec_s = s.split(" ");
    let mut title_case = String::new();
    for word in vec_s {
        let mut c = word.chars();
        let first = c.next().unwrap();
        title_case.push_str(&first.to_uppercase().to_string());
        title_case.push_str(c.as_str());
        title_case.push_str(" ");
    }
    title_case
}

// Truncate a string to the given length and add "..." if it was longer than length
// pub fn smart_truncate(text: &str, max_length: usize) -> String {
//     let mut truncate_string = vec![];
//     let mut flag = 0;
//     for char in text.chars() {
//         truncate_string.push(char);
//         flag += 1;
//         if flag == max_length {
//             truncate_string.push('.');
//             truncate_string.push('.');
//             truncate_string.push('.');
//             break;
//         }
//     }
//     truncate_string.iter().collect()
// }

// optimized version
pub fn smart_truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        return text.to_string();
    }

    let mut result = String::with_capacity(max_length + 3);
    result.extend(text.chars().take(max_length));
    result.push_str("...");
    result
}

// password strenght checker
pub fn is_strong_password(password: &str) -> bool {
    let mut validation_errors: Vec<String> = vec![];
    if password.len() < 8 {
        validation_errors.push("password must be 8 character long".to_string());
    }
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    if !has_uppercase {
        validation_errors.push("password must contain one uppercase letter".to_string());
    }
    if !has_lowercase {
        validation_errors.push("password must contain one lower letter".to_string());
    }
    if !has_digit {
        validation_errors.push("password must contain one digit".to_string());
    }
    if validation_errors.len() > 0 {
        return false;
    } else {
        return true;
    }
}
