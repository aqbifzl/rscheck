pub fn is_camel_case(str: &str) -> bool {
    let mut lower_case_found = false;
    let mut upper_case_found = false;
    let mut was_previous_upper = false;

    for ch in str.chars() {
        if ch.is_lowercase() {
            was_previous_upper = false;
            lower_case_found = true;
        } else if ch.is_uppercase() {
            if was_previous_upper {
                return false;
            }
            was_previous_upper = true;
            upper_case_found = true;
        } else if !ch.is_ascii_digit() {
            return false;
        }
    }

    lower_case_found && upper_case_found
}

pub fn is_snake_case(str: &str) -> bool {
    str.contains('_')
        && str.chars().any(|ch| ch.is_alphabetic())
        && !str.starts_with('_')
        && !str.ends_with('_')
}

pub fn parse_camel_case(str: &str) -> Option<Vec<String>> {
    if !is_camel_case(str) {
        return None;
    }
    let mut result: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for ch in str.chars() {
        if ch.is_uppercase() && !buffer.is_empty() {
            result.push(buffer.to_lowercase());
            buffer.clear();
        }
        buffer.push(ch);
    }

    if !buffer.is_empty() {
        result.push(buffer.to_lowercase());
    }

    Some(result)
}

pub fn parse_snake_case(str: &str) -> Option<Vec<String>> {
    if !is_snake_case(str) {
        return None;
    }
    Some(str.trim().split('_').map(|x| x.to_lowercase()).collect())
}

#[cfg(test)]
mod tests {
    use crate::spell_checker::parse_variables::is_camel_case;
    use crate::spell_checker::parse_variables::is_snake_case;

    #[test]
    fn check_snake_case_checker() {
        assert!(!is_snake_case(""));
        assert!(!is_snake_case("3"));
        assert!(!is_snake_case("2_3"));
        assert!(is_snake_case("my_function"));
        assert!(is_snake_case("MY_FUNCTION"));
        assert!(is_snake_case("mY_funcTion"));
        assert!(!is_snake_case("____"));
    }

    #[test]
    fn check_camel_case_checker() {
        assert!(!is_camel_case(""));
        assert!(!is_camel_case("3"));
        assert!(!is_camel_case("2_3"));
        assert!(!is_camel_case("my_function"));
        assert!(!is_camel_case("MY_FUNCTION"));
        assert!(!is_camel_case("mY_funcTion"));
        assert!(!is_camel_case("____"));
        assert!(is_camel_case("myFunction"));
        assert!(is_camel_case("MyFunction"));
        assert!(is_camel_case("thisIsMyFunction"));
        assert!(is_camel_case("ThisIsMyFunction"));
        assert!(!is_camel_case("THisIsMyFunction"));
    }
}
