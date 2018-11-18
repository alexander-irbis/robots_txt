use prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Rule<'a> {
    pub allow: bool,
    pub path: Cow<'a, str>,
}

impl<'a> Rule<'a> {
    pub fn new<P>(allow: bool, path: P) -> Self
    where
        P: Into<Cow<'a, str>>,
    {
        let path = path.into();
        Rule { allow, path }
    }

    pub fn allow<P>(path: P) -> Self
    where
        P: Into<Cow<'a, str>>,
    {
        Rule::new(true, path)
    }

    pub fn disallow<P>(path: P) -> Self
    where
        P: Into<Cow<'a, str>>,
    {
        Rule::new(false, path)
    }
}

impl<'a> Render for Rule<'a> {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self.allow {
            true => write!(w, "Allow:")?,
            false => write!(w, "Disallow:")?,
        };
        if !self.path.is_empty() {
            write!(w, " {}", self.path)?;
        };
        writeln!(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!("Allow: /\n", Rule::allow("/").render().unwrap());
        assert_eq!("Disallow: /\n", Rule::disallow("/").render().unwrap());
    }
}
