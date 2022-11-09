// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

pub fn say_hello(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_none() {
        assert_eq!("Hello, world!", say_hello(None));
    }

    #[test]
    fn hello_world_some() {
        assert_eq!("Hello, Rust!", say_hello(Some("Rust")));
    }
}
