
![](https://img.shields.io/crates/l/robots_txt.svg)
[![crates.io](https://img.shields.io/crates/v/robots_txt.svg)](https://crates.io/crates/robots_txt)
[![Build Status](https://travis-ci.org/alexander-irbis/robots_txt.svg)](https://travis-ci.org/alexander-irbis/robots_txt)


# robots_txt

**robots_txt is a lightweight robots.txt parser and generator written in Rust.**

Nothing extra.
 
* [Documentation](https://docs.rs/robots_txt)


### Unstable

The implementation is WIP.


## Installation

Robots_txt is [available on crates.io](https://crates.io/crates/robots_txt) and can be included in your Cargo enabled project like this:

Cargo.toml:
```toml
[dependencies]
robots_txt = "0.4"
```

### Parsing & matching paths against rules

main.rs:
```rust
extern crate robots_txt;

use robots_txt::Robots;

static ROBOTS: &'static str = r#"

# robots.txt for http://www.site.com
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
# Cybermapper knows where to go
User-Agent: cybermapper
Disallow:

"#;

fn main() {
    let robots = Robots::from_str(ROBOTS);

    let matcher = SimpleMatcher::new(&robots.choose_section("NoName Bot").rules);
    assert!(matcher.check_path("/some/page"));
    assert!(matcher.check_path("/cyberworld/welcome.html"));
    assert!(!matcher.check_path("/cyberworld/map/object.html"));

    let matcher = SimpleMatcher::new(&robots.choose_section("Mozilla/5.0; CyberMapper v. 3.14").rules);
    assert!(matcher.check_path("/some/page"));
    assert!(matcher.check_path("/cyberworld/welcome.html"));
    assert!(matcher.check_path("/cyberworld/map/object.html"));
}
```


### Building & rendering

main.rs:
```rust
extern crate robots_txt;

use robots_txt::Robots;

fn main() {
    let robots1 = Robots::start_build()
        .start_section_for("cybermapper")
            .disallow("")
            .end_section()
        .start_section_for("*")
            .disallow("/cyberworld/map/")
            .end_section()
        .finalize();

    let robots2 = Robots::start_build()
        .host("example.com")
        .start_section_for("*")
            .disallow("/private")
            .disallow("")
            .crawl_delay(5)
            .request_rate(1, 5)
            .sitemap("http://example.com/sitemap.xml".parse().unwrap())
            .end_section()
        .finalize();
        
    println!("# robots.txt for http://cyber.example.com/\n\n{}", robots1);
    println!("# robots.txt for http://example.com/\n\n{}", robots2);
}
```
As a result we get
```
# robots.txt for http://cyber.example.com/

User-agent: cybermapper
Disallow:

User-agent: *
Disallow: /cyberworld/map/


# robots.txt for http://example.com/

User-agent: *
Disallow: /private
Disallow:
Crawl-delay: 5
Request-rate: 1/5
Sitemap: http://example.com/sitemap.xml

Host: example.com

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
