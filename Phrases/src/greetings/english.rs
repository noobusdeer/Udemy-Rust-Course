//! This module contains English phrases.
//! 
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!",
//!     phrases::greetings::english::hello(),
//!     username);
//! ```
//! 

/// Applies code that follows it.
/// In this case, it's out `hello()` function.
pub fn hello() -> String { "hello".to_string() }
/// In this case, it's out `goodbye()` function.
pub fn goodbye() -> String { "goodbye".to_string() }