
![](https://img.shields.io/crates/l/robots_txt.svg) [![](https://img.shields.io/crates/v/robots_txt.svg)](https://crates.io/crates/robots_txt)

> robots_txt is a lightweight robots.txt parser and generator written in Rust.

Nothing extra.
 
* [Documentation](https://docs.rs/robots_txt)


### Unstable

The implementation is WIP.


## Installation

Robots_txt is [available on crates.io](https://crates.io/crates/robots_txt) and can be included in your Cargo enabled project like this:

Cargo.toml:
```toml
[dependencies]
robots_txt = "*"
```

main.rs:
```rust
#[macro_use]
extern crate robots_txt;

use robots_txt::Robots;

static ROBOTS: &'static str = r#"
    # TODO simple example
"#;

fn main() {
    let robots = Robots::parse(ROBOTS).unwrap();
    println!("{}", robots);
}
```

## Alternatives

 * [messense/robotparser-rs](https://github.com/messense/robotparser-rs)   robots.txt parser for Rust


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
