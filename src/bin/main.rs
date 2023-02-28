// Copyright {{param "copyright"}} Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", {{param "name" | replace "-" "_"}}::say_hello(None));
    Ok(())
}
