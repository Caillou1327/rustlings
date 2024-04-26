// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.




use std::str::FromStr;

// Define a custom error type for parsing
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    EmptyName,
    InvalidFormat,
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Implement the FromStr trait for Person
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // Step 1: Check if the length of the provided string is 0
        if s.is_empty() {
            return Ok(Person::default());
        }
        
        // Step 2: Split the string on commas
        let mut parts = s.split(',');
        
        // Step 3: Extract the name from the first part
        if let Some(name) = parts.next() {
            let name = name.trim().to_string();
            
            // Step 4: If the name is empty, return an error
            if name.is_empty() {
                return Err(ParsePersonError::EmptyName);
            }
            
            // Step 5: Extract and parse the age from the second part
            if let Some(age_str) = parts.next() {
                if let Ok(age) = age_str.trim().parse::<usize>() {
                    return Ok(Person { name, age });
                }
            }
        }
        
        // If parsing fails or no age provided, return an error
        Err(ParsePersonError::InvalidFormat)
    }
}

fn main() {
    let p1 = "Mark,20".parse::<Person>();
    let p2 = "Gerald,70".parse::<Person>();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_empty_string() {
        // Test parsing an empty string
        let result = "".parse::<Person>();
        assert_eq!(result, Ok(Person::default()));
    }

    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let result = "Mark,20".parse::<Person>();
        assert_eq!(result, Ok(Person { name: "Mark".to_string(), age: 20 }));
    }

    #[test]
    fn test_bad_convert() {
        // Test parsing an invalid string
        let result = "InvalidString".parse::<Person>();
        assert_eq!(result, Err(ParsePersonError::InvalidFormat));
    }

    #[test]
    fn test_missing_name() {
        // Test parsing a string with missing name
        let result = ",20".parse::<Person>();
        assert_eq!(result, Err(ParsePersonError::EmptyName));
    }

    #[test]
    fn test_missing_age() {
        // Test parsing a string with missing age
        let result = "Mark,".parse::<Person>();
        assert_eq!(result, Err(ParsePersonError::InvalidFormat));
    }

    #[test]
    fn test_invalid_age() {
        // Test parsing a string with invalid age
        let result = "Mark,twenty".parse::<Person>();
        assert_eq!(result, Err(ParsePersonError::InvalidFormat));
    }
}
