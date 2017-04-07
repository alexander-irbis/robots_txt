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
        match section.is_default() {
            false => self.sections.push(section),
            true => match self.default_section {
                None => self.default_section = Some(section),
                Some(ref mut default_section) => default_section.merge(section),
            }
        };
        self
    }

    pub fn start_section(self) -> SectionBuilder<'a> {
        SectionBuilder::build(self)
    }

    pub fn start_section_for<U>(self, ua: U) -> SectionBuilder<'a> where U: Into<Cow<'static, str>> {
        SectionBuilder::build(self)
            .useragent(ua)
    }

    pub fn host<U>(mut self, host: U) -> Self where U: Into<Cow<'static, str>> {
        self.host = Some(host.into());
        self
    }

    pub fn finalize(self) -> Robots<'a> {
        let default_section = match self.default_section {
            Some(default_section) => default_section,
            None => Section::default(),
        };
        Robots {
            default_section: default_section,
            sections: self.sections,
            host: self.host,
        }
    }
}
