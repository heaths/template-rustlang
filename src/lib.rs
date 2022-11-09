// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

pub fn say_hello(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
}
