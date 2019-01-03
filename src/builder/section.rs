use std::{borrow::Cow, collections::BTreeSet};

use url::Url;

use crate::{builder::*, parts::*};

#[derive(Clone, Debug)]
pub struct SectionBuilder<'a> {
    robots: RobotsBuilder<'a>,
    rules: Vec<Rule<'a>>,
    useragents: BTreeSet<Cow<'a, str>>,
    sitemaps: BTreeSet<Url>,
    crawl_delay: Option<f64>,
    req_rate: Option<RequestRate>,
}

impl<'a> SectionBuilder<'a> {
    pub fn build(robots: RobotsBuilder<'a>) -> Self {
        SectionBuilder {
            robots,
            rules: Default::default(),
            useragents: Default::default(),
            sitemaps: Default::default(),
            crawl_delay: None,
            req_rate: None,
        }
    }

    pub fn useragent<U>(mut self, ua: U) -> Self
    where
        U: Into<Cow<'static, str>>,
    {
        self.useragents.insert(ua.into());
        self
    }

    pub fn disallow<P>(mut self, path: P) -> Self
    where
        P: Into<Cow<'static, str>>,
    {
        self.rules.push(Rule::disallow(path));
        self
    }

    pub fn allow<P>(mut self, path: P) -> Self
    where
        P: Into<Cow<'static, str>>,
    {
        self.rules.push(Rule::allow(path));
        self
    }

    pub fn sitemap(mut self, url: Url) -> Self {
        self.sitemaps.insert(url);
        self
    }

    pub fn crawl_delay(mut self, delay: f64) -> Self {
        self.crawl_delay = Some(delay);
        self
    }

    pub fn request_rate(mut self, requests: usize, seconds: usize) -> Self {
        self.req_rate = Some(RequestRate::new(requests, seconds));
        self
    }

    pub fn end_section(self) -> RobotsBuilder<'a> {
        self.robots.section(Section {
            crawl_delay: self.crawl_delay,
            req_rate: self.req_rate,
            rules: self.rules,
            sitemaps: self.sitemaps,
            useragents: self.useragents,
        })
    }
}
