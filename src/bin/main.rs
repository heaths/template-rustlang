// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", template::say_hello(None));
    Ok(())
}
