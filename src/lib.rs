// Copyright {{param "copyright"}} Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

{{if (param "error" true "Do you want an error template?") -}}
mod error;
pub use error::*;
{{- else -}}
{{deleteFile "src/error.rs"}}
{{- end }}

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
