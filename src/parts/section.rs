use std::collections::BTreeSet;
use std::iter::FromIterator;

use prelude::*;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section<'a> {
    pub crawl_delay: Option<usize>,
    pub req_rate: Option<RequestRate>,
    pub rules: Vec<Rule<'a>>,
    pub sitemaps: BTreeSet<Url>,
    pub useragents: BTreeSet<Cow<'a, str>>,
    pub host: Option<Cow<'a, str>>,
}

impl <'a> Default for Section<'a> {
    fn default() -> Self {
        Section {
            crawl_delay: None,
            req_rate: None,
            rules: vec![ Rule::disallow("") ],
            sitemaps: BTreeSet::new(),
            useragents: BTreeSet::from_iter(Some(Cow::from("*")).into_iter()),
            host: None,
        }
    }
}

impl <'a> Render for Section<'a> {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        for ua in &self.useragents {
            writeln!(w, "User-agent: {}", ua)?;
        }
        for rule in &self.rules {
            rule.render_to(w)?;
        }
        if let Some(delay) = self.crawl_delay.as_ref() {
            writeln!(w, "Crawl-delay: {}", delay)?;
        }
        if let Some(rate) = self.req_rate.as_ref() {
            rate.render_to(w)?;
        }
        for url in &self.sitemaps {
            writeln!(w, "Sitemap: {}", url)?;
        }
        if let Some(host) = self.host.as_ref() {
            writeln!(w, "Host: {}", host)?;
        }
        writeln!(w)
    }
}

impl <'a> Section<'a> {
    pub fn empty() -> Self {
        Section {
            crawl_delay: None,
            req_rate: None,
            rules: Vec::new(),
            sitemaps: BTreeSet::new(),
            useragents: BTreeSet::new(),
            host: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.crawl_delay.is_none()
            && self.req_rate.is_none()
            && self.host.is_none()
            && self.rules.is_empty()
            && self.sitemaps.is_empty()
            && self.useragents.is_empty()
    }

    pub fn has_rules(&self) -> bool {
        !self.rules.is_empty()
            || self.crawl_delay.is_some()
            || self.req_rate.is_some()
            || self.host.is_some()
            || !self.sitemaps.is_empty()
    }

    pub fn is_default(&self) -> bool {
        self.useragents.contains("*")
    }

    pub fn merge(&mut self, mut other: Section<'a>) {
        if !self.is_default() {
            if other.is_default() {
                self.useragents = other.useragents;
            } else {
                self.useragents.append(&mut other.useragents);
            }
        }
        self.sitemaps.append(&mut other.sitemaps);
        self.rules.append(&mut other.rules);
        if other.crawl_delay.is_some() {
            self.crawl_delay = other.crawl_delay;
        }
        if other.req_rate.is_some() {
            self.req_rate = other.req_rate;
        }
    }

    pub fn push_ua<U>(&mut self, ua: U) where U: Into<Cow<'a, str>> {
        if self.is_default() {
            return;
        }
        let ua = ua.into();
        if ua == "*" {
            self.useragents.clear();
        }
        self.useragents.insert(ua);
    }

    pub fn push_rule(&mut self, rule: Rule<'a>) {
        self.rules.push(rule)
    }

    pub fn push_sitemap(&mut self, url: &str) -> Result<(), UrlParseError> {
        Url::parse(url)
            .map(|url| { self.sitemaps.insert(url); } )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!("User-agent: *\nDisallow:\n\n", Section::default().render().unwrap());
    }

    #[test]
    fn conditions() {
        let mut section = Section::empty();
        assert!(section.is_empty());
        assert!(!section.is_default());

        section.push_ua("bot");
        assert!(!section.is_empty());
        assert!(!section.is_default());

        let mut section = Section::empty();
        section.push_ua("*");
        assert!(!section.is_empty());
        assert!(section.is_default());

        let section = Section::default();
        assert!(!section.is_empty());
        assert!(section.is_default());
    }
}
