use prelude::*;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section<'a> {
    pub crawl_delay: Option<Duration>,
    pub req_rate: Option<RequestRate>,
    pub rules: Vec<Rule<'a>>,
    pub sitemaps: Vec<Url>,
    pub useragents: Vec<Cow<'a, str>>,
}

impl <'a> Default for Section<'a> {
    fn default() -> Self {
        Section {
            crawl_delay: None,
            req_rate: None,
            rules: vec![ Rule::disallow("") ],
            sitemaps: Vec::new(),
            useragents: vec![ Cow::from("*") ],
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
            writeln!(w, "Crawl-delay: {}", delay.as_secs())?;
        }
        if let Some(rate) = self.req_rate.as_ref() {
            rate.render_to(w)?;
        }
        for url in &self.sitemaps {
            writeln!(w, "Sitemap: {}", url)?;
        }
        writeln!(w)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!("User-agent: *\nDisallow: \n\n", Section::default().render().unwrap());
    }
}
