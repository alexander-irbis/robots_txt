pub mod robots;
pub mod section;

pub use self::robots::*;
pub use self::section::*;


#[cfg(test)]
mod tests {
    use prelude::*;

    static SAMPLE_1: &'static str = r#"
User-agent: cybermapper
Disallow:

User-agent: *
Disallow: /cyberworld/map/

"#;


    static SAMPLE_2: &'static str = r#"
User-agent: *
Disallow: /private
Disallow:
Crawl-delay: 4.5
Request-rate: 9/20
Sitemap: http://example.com/sitemap.xml

Host: example.com
"#;


    fn assert_eq(robots: Robots, sample: &str) {
        assert_eq!(robots.to_string(), sample.trim_left());
    }

    #[test]
    fn build_1_start_end_section() {
        let robots = Robots::start_build()
            .start_section_for("cybermapper")
            .disallow("")
            .end_section()
            .start_section()
            .useragent("*")
            .disallow("/cyberworld/map/")
            .end_section()
            .finalize();

        assert_eq(robots, SAMPLE_1);
    }

    #[test]
    fn build_1_with_section() {
        let robots = Robots::start_build()
            .with_section_for("cybermapper", |section| section.disallow(""))
            .with_section(|section| {
                section.useragent("*").disallow("/cyberworld/map/")
            })
            .finalize();

        assert_eq(robots, SAMPLE_1);
    }

    #[test]
    fn build_2_start_end_section() {
        let robots = Robots::start_build()
            .host("example.com")
            .start_section_for("*")
            .disallow("/private")
            .disallow("")
            .crawl_delay(4.5)
            .request_rate(9, 20)
            .sitemap("http://example.com/sitemap.xml".parse().unwrap())
            .end_section()
            .finalize();

        assert_eq(robots, SAMPLE_2);
    }

    #[test]
    fn build_2_with_section() {
        let robots = Robots::start_build()
            .host("example.com")
            .with_section_for("*", |section| {
                section
                    .disallow("/private")
                    .disallow("")
                    .crawl_delay(4.5)
                    .request_rate(9, 20)
                    .sitemap("http://example.com/sitemap.xml".parse().unwrap())
            })
            .finalize();

        assert_eq(robots, SAMPLE_2);
    }
}
