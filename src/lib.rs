//! The `theseus` crate provides a function that adds two to a number.
//!
//! # Examples
//! ```
//! assert_eq!(7, theseus::add_two(5));
//! ```

/// This function adds two to its argument.
/// # Examples
/// ```
/// use theseus::add_two;
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

struct Cell {
    row: i32,
    column: i32,
    north: Box<Option<Cell>>,
    south: Box<Option<Cell>>,
    east: Box<Option<Cell>>,
    west: Box<Option<Cell>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(10, add_two(8));
    }
}
