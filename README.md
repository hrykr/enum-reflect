# Enum Reflection Rust

[<img alt="github" src="https://img.shields.io/badge/github-hrykr%2Fenum--reflect-blue?logo=github&label=github&link=https%3A%2F%2Fgithub.com%2Fhrykr%2Fenum-reflect" height="20">](https://github.com/hrykr/enum-reflect)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum_reflect?logo=rust" height="20">](https://crates.io/crates/enum_reflect)
[<img alt="crates.io" src="https://img.shields.io/docsrs/enum_reflect?logo=docs.rs&label=docs.rs" height="20">](https://docs.rs/enum_reflect/latest/enum_reflect/)


Rust library for enum reflection. Use it to get all fields in enum.

## Installation

`cargo add enum_reflect`
> or
```toml
[dependencies]
enum_reflect = "0.2.1"
```

## Dependencies

> ### [Enum Reflect Extern](https://crates.io/crates/enum_reflect_extern)
> Extern traits, structs and other for enum_reflect.

## Example

```rust
#[derive(EnumReflect)]
pub enum Example {
    Empty,
    Example1 {
        var1: String,
        var2: i32,
    },
    Example2 {
        var1: String,
        var2: i32,
        var3: bool,
    },
}

fn foo() {
    let exml = Example::Example1 { var1: "Hello, World!".to_string(), var2: 32 };
    
    for (name, value) in exml.get_named_fields() {
        println!("Field {} is {}", name, value.to_string());
    }
}

```
> ### Output:
> ```
> Field var1 is Hello, World!
> Field var2 is 32
> ```