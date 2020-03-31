pub mod robots;
pub mod section;

pub use self::robots::*;
pub use self::section::*;

#[cfg(test)]
mod tests {
    use crate::parts::*;

    static SAMPLE_1: &str = r#"
User-agent: cybermapper
Disallow:

User-agent: *
Disallow: /cyberworld/map/

"#;

    static SAMPLE_2: &str = r#"
User-agent: *
Disallow: /private
Disallow:
Crawl-delay: 4.5
Request-rate: 9/20
Sitemap: http://example.com/sitemap.xml

Host: example.com
"#;

    fn assert_eq(robots: &Robots, sample: &str) {
        assert_eq!(robots.to_string(), sample.trim_start());
    }

    #[test]
    fn build_1_start_end_section() {
        let robots = Robots::builder()
            .start_section("cybermapper")
            .disallow("")
            .end_section()
            .start_section("*")
            .disallow("/cyberworld/map/")
            .end_section()
            .build();

        assert_eq(&robots, SAMPLE_1);
    }

    #[test]
    fn build_1_with_section() {
        let robots = Robots::builder()
            .with_section("cybermapper", |section| section.disallow(""))
            .with_section("*", |section| section.disallow("/cyberworld/map/"))
            .build();

        assert_eq(&robots, SAMPLE_1);
    }

    #[test]
    fn build_2_start_end_section() {
        let robots = Robots::builder()
            .host("example.com")
            .start_section("*")
            .disallow("/private")
            .disallow("")
            .crawl_delay(4.5)
            .request_rate(9, 20)
            .sitemap("http://example.com/sitemap.xml".parse().unwrap())
            .end_section()
            .build();

        assert_eq(&robots, SAMPLE_2);
    }

    #[test]
    fn build_2_with_section() {
        let robots = Robots::builder()
            .host("example.com")
            .with_section("*", |section| {
                section
                    .disallow("/private")
                    .disallow("")
                    .crawl_delay(4.5)
                    .request_rate(9, 20)
                    .sitemap("http://example.com/sitemap.xml".parse().unwrap())
            })
            .build();

        assert_eq(&robots, SAMPLE_2);
    }
}
