use prelude::*;

#[derive(Clone, Debug, Default)]
pub struct RobotsBuilder<'a> {
    default_section: Option<Section<'a>>,
    sections: Vec<Section<'a>>,
    host: Option<Cow<'a, str>>,
}

impl<'a> RobotsBuilder<'a> {
    pub fn build() -> Self {
        RobotsBuilder::default()
    }

    pub fn section(mut self, section: Section<'a>) -> Self {
        if section.is_default() {
            match self.default_section {
                None => self.default_section = Some(section),
                Some(ref mut default_section) => default_section.merge(section),
            }
        } else {
            self.sections.push(section);
        };
        self
    }

    pub fn start_section(self) -> SectionBuilder<'a> {
        SectionBuilder::build(self)
    }

    pub fn start_section_for<U>(self, ua: U) -> SectionBuilder<'a>
    where
        U: Into<Cow<'static, str>>,
    {
        SectionBuilder::build(self).useragent(ua)
    }

    pub fn with_section<F>(self, f: F) -> Self
    where
        F: FnOnce(SectionBuilder<'a>) -> SectionBuilder<'a>,
    {
        let section = SectionBuilder::build(self);
        let section = f(section);
        section.end_section()
    }

    pub fn with_section_for<F, U>(self, ua: U, f: F) -> Self
    where
        F: FnOnce(SectionBuilder<'a>) -> SectionBuilder<'a>,
        U: Into<Cow<'static, str>>,
    {
        let section = SectionBuilder::build(self).useragent(ua);
        let section = f(section);
        section.end_section()
    }

    pub fn host<U>(mut self, host: U) -> Self
    where
        U: Into<Cow<'a, str>>,
    {
        self.host = Some(host.into());
        self
    }

    pub fn finalize(self) -> Robots<'a> {
        let default_section = match self.default_section {
            Some(default_section) => default_section,
            None => Section::default(),
        };
        Robots {
            default_section,
            sections: self.sections,
            host: self.host,
        }
    }
}
