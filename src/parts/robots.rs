use prelude::*;


#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Robots<'a> {
    pub default_section: Section<'a>,
    pub sections: Vec<Section<'a>>,
    pub host: Option<Cow<'a, str>>,
}

impl <'a> Render for Robots<'a> {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        for section in &self.sections {
            section.render_to(w)?;
        }
        self.default_section.render_to(w)?;
        if let Some(host) = self.host.as_ref() {
            writeln!(w, "Host: {}", host)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!("User-agent: *\nDisallow: \n\n", Robots::default().render().unwrap());
    }
}
