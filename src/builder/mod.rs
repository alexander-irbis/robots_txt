pub mod robots;
pub mod section;

pub use self::robots::*;
pub use self::section::*;


#[cfg(test)]
mod tests {
    use prelude::*;

    static RESULT1: &'static str = r#"
User-agent: cybermapper
Disallow:

User-agent: *
Disallow: /cyberworld/map/

"#;


    static RESULT2: &'static str = r#"
User-agent: *
Disallow: /private
Disallow:
Crawl-delay: 5
Request-rate: 1/5
Sitemap: http://example.com/sitemap.xml
Host: example.com

"#;


    #[test]
    fn build() {
        let test = |robots: Robots, sample| {
            assert_eq!((sample as &str).trim_left(), format!("{}", robots));
        };

        // ------------------------

        let robots = Robots::start_build()
            .start_section_for("cybermapper")
                .disallow("")
                .end_section()
            .start_section_for("*")
                .disallow("/cyberworld/map/")
                .end_section()
            .finalize();

        test(robots, RESULT1);

        // ------------------------

        let robots = Robots::start_build()
            .start_section_for("*")
                .disallow("/private")
                .disallow("")
                .crawl_delay(5)
                .request_rate(1, 5)
                .sitemap("http://example.com/sitemap.xml".parse().unwrap())
                .host("example.com")
                .end_section()
            .finalize();

        test(robots, RESULT2);
    }
}
