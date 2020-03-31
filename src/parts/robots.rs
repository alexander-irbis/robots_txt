use std::{borrow::Cow, fmt};

use unicase::UniCase;

use crate::{builder::*, parse::*, parts::*};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Robots<'a> {
    pub default_section: Section<'a>,
    pub sections: Vec<Section<'a>>,
    pub host: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for Robots<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for section in &self.sections {
            section.fmt(f)?;
        }
        self.default_section.fmt(f)?;
        if let Some(host) = self.host.as_ref() {
            writeln!(f, "Host: {}", host)?;
        }
        Ok(())
    }
}

impl<'a> Robots<'a> {
    pub fn builder() -> RobotsBuilder<'a> {
        RobotsBuilder::new()
    }

    pub fn from_str_lossy(input: &'a str) -> Robots<'a> {
        let mut robots = Constructor::default();

        for line in input.lines() {
            let (line, comment) = split_comment(line);
            match split_kv(line) {
                // Comment line, just skip
                None if comment.is_some() => {}
                // Empty line
                None => robots.end_section(),
                // Some statement
                Some((k, v)) => {
                    match UniCase::new(k) {
                        k if k == UniCase::new("user-agent") => {
                            if robots.section.has_rules() {
                                robots.end_section();
                            }
                            robots.section.push_ua(v);
                        }
                        k if k == UniCase::new("disallow") => {
                            robots.section.push_rule(Rule::disallow(v));
                        }
                        k if k == UniCase::new("allow") => {
                            robots.section.push_rule(Rule::allow(v));
                        }
                        k if k == UniCase::new("sitemap") => {
                            robots.section.push_sitemap(v).ok();
                        }
                        k if k == UniCase::new("host") => robots.set_host(v),
                        k if k == UniCase::new("crawl-delay") => {
                            v.parse().map(|v| robots.section.crawl_delay = Some(v)).ok();
                        }
                        k if k == UniCase::new("request-rate") => {
                            if let Some((r, s)) = split_rr(v) {
                                r.parse()
                                    .and_then(|r| {
                                        s.parse().map(|s| {
                                            robots.section.req_rate = Some(RequestRate::new(r, s))
                                        })
                                    })
                                    .ok();
                            }
                        }

                        // "Unrecognised headers are ignored"
                        _ => {}
                    }
                }
            }
        }

        robots.build()
    }

    pub fn choose_section<U>(&self, ua: U) -> &Section<'a>
    where
        U: AsRef<str>,
    {
        let ua: &str = ua.as_ref();
        if !ua.is_empty() {
            for section in &self.sections {
                for ua2 in &section.useragents {
                    if ua2.len() > ua.len() {
                        continue;
                    }
                    let matches = (0..=ua.len() - ua2.len())
                        .map(|i| &ua[i..i + ua2.len()])
                        .any(|s: &str| UniCase::new(s) == UniCase::new(ua2));

                    if matches {
                        return section;
                    }
                }
            }
        }
        &self.default_section
    }
}

struct Constructor<'a> {
    pub default_section: Option<Section<'a>>,
    pub sections: Vec<Section<'a>>,
    pub section: Section<'a>,
    pub host: Option<Cow<'a, str>>,
}

impl<'a> Default for Constructor<'a> {
    fn default() -> Self {
        Constructor {
            default_section: None,
            sections: Vec::new(),
            section: Section::empty(),
            host: None,
        }
    }
}

impl<'a> Constructor<'a> {
    pub fn set_host<H>(&mut self, host: H)
    where
        H: Into<Cow<'a, str>>,
    {
        // Take into account only the first `Host` directive.
        if self.host.is_none() {
            self.host = Some(host.into())
        }
    }

    pub fn end_section(&mut self) {
        if self.section.is_empty() {
            return;
        }
        let section = ::std::mem::replace(&mut self.section, Section::empty());
        if section.is_default() {
            match self.default_section {
                Some(ref mut default_section) => default_section.merge(section),
                None => self.default_section = Some(section),
            }
        } else {
            self.sections.push(section)
        }
    }

    pub fn build(mut self) -> Robots<'a> {
        self.end_section();
        Robots {
            default_section: self.default_section.unwrap_or_default(),
            sections: self.sections,
            host: self.host,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!(
            "User-agent: *\nDisallow:\n\n",
            Robots::default().to_string()
        );
    }

    static ROBOTS1: &str = r#"
# robots.txt for http://www.site.com
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
Disallow: /tmp/ # these will soon disappear
"#;

    static RESULT1: &str = r#"
User-agent: *
Disallow: /cyberworld/map/
Disallow: /tmp/

"#;

    static ROBOTS2: &str = r#"
# robots.txt for http://www.site.com
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
# Cybermapper knows where to go
User-Agent: cybermapper
Disallow:
"#;

    static RESULT2: &str = r#"
User-agent: cybermapper
Disallow:

User-agent: *
Disallow: /cyberworld/map/

"#;

    static ROBOTS3: &str = r#"
# robots.txt for http://www.site.com
User-Agent: *
Disallow: /
"#;

    static RESULT3: &str = r#"
User-agent: *
Disallow: /

"#;

    static ROBOTS4: &str = r#"
User-Agent: *
Disallow:
Disallow: /private
Crawl-delay: 4.5
Request-Rate: 9/20
Sitemap: http://example.com/sitemap.xml
Host: example.com
"#;

    static RESULT4: &str = r#"
User-agent: *
Disallow:
Disallow: /private
Crawl-delay: 4.5
Request-rate: 9/20
Sitemap: http://example.com/sitemap.xml

Host: example.com
"#;

    #[test]
    fn parse() {
        let test = |robots, sample| {
            let robots = Robots::from_str_lossy(robots);
            assert_eq!((sample as &str).trim_start(), robots.to_string());
        };

        test(ROBOTS1, RESULT1);
        test(ROBOTS2, RESULT2);
        test(ROBOTS3, RESULT3);
        test(ROBOTS4, RESULT4);
    }
}
