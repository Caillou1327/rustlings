// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct IntoColorError;

// Implement TryFrom for tuples
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        match tuple {
            (255, 0, 0) => Ok(Color::Red),
            (0, 255, 0) => Ok(Color::Green),
            (0, 0, 255) => Ok(Color::Blue),
            _ => Err(IntoColorError),
        }
    }
}

// Implement TryFrom for arrays
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        match arr {
            [255, 0, 0] => Ok(Color::Red),
            [0, 255, 0] => Ok(Color::Green),
            [0, 0, 255] => Ok(Color::Blue),
            _ => Err(IntoColorError),
        }
    }
}

// Implement TryFrom for slices
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError);
        }
        
        match slice {
            &[255, 0, 0] => Ok(Color::Red),
            &[0, 255, 0] => Ok(Color::Green),
            &[0, 0, 255] => Ok(Color::Blue),
            _ => Err(IntoColorError),
        }
    }
}

fn main() {
    // Example usage
    let tuple_color = Color::try_from((255, 0, 0));
    println!("{:?}", tuple_color); // Should print Ok(Red)
    
    let array_color = Color::try_from([0, 255, 0]);
    println!("{:?}", array_color); // Should print Ok(Green)
    
    let slice_color = Color::try_from(&[0, 0, 255][..]);
    println!("{:?}", slice_color); // Should print Ok(Blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_conversion() {
        assert_eq!(Color::try_from((255, 0, 0)), Ok(Color::Red));
        assert_eq!(Color::try_from((0, 255, 0)), Ok(Color::Green));
        assert_eq!(Color::try_from((0, 0, 255)), Ok(Color::Blue));
        assert_eq!(Color::try_from((0, 0, 0)), Err(IntoColorError));
    }

    #[test]
    fn test_array_conversion() {
        assert_eq!(Color::try_from([255, 0, 0]), Ok(Color::Red));
        assert_eq!(Color::try_from([0, 255, 0]), Ok(Color::Green));
        assert_eq!(Color::try_from([0, 0, 255]), Ok(Color::Blue));
        assert_eq!(Color::try_from([0, 0, 0]), Err(IntoColorError));
    }

    #[test]
    fn test_slice_conversion() {
        assert_eq!(Color::try_from(&[255, 0, 0][..]), Ok(Color::Red));
        assert_eq!(Color::try_from(&[0, 255, 0][..]), Ok(Color::Green));
        assert_eq!(Color::try_from(&[0, 0, 255][..]), Ok(Color::Blue));
        assert_eq!(Color::try_from(&[0, 0, 0][..]), Err(IntoColorError));
        assert_eq!(Color::try_from(&[0, 0, 0, 0][..]), Err(IntoColorError));
    }
}

