#![deny(clippy::pedantic)]
#![warn(missing_docs)]

//! A very simple trait that overwrites errors.

/// Import this trait to get the `set_error` method on `Result`s and `Option`s
///
/// # Examples
/// ```
/// use set_error::ChangeError;
/// let foo: Option<u32> = None;
/// assert_eq!(Err(String::from("erorr")), foo.set_error("erorr"));
/// ```
pub trait ChangeError<T> {
    /// If an error is present it is replaced with the string passed into this method.
    fn set_error(self, s: &str) -> Result<T, String>;
}

impl<T, E> ChangeError<T> for Result<T, E> {
    fn set_error(self, s: &str) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(s.to_string()),
        }
    }
}
impl<T> ChangeError<T> for Option<T> {
    fn set_error(self, s: &str) -> Result<T, String> {
        match self {
            Some(t) => Ok(t),
            None => Err(s.to_string()),
        }
    }
}
