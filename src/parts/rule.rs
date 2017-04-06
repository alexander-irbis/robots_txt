use prelude::*;


#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Rule<'a> {
    pub allowance: bool,
    pub path: Cow<'a, str>,
}

impl<'a> Rule<'a> {
    pub fn new<P>(allowanse: bool, path: P) -> Self
        where P: Into<Cow<'a, str>>
    {
        Rule {
            allowance: allowanse,
            path: path.into(),
        }
    }

    pub fn allow<P>(path: P) -> Self
        where P: Into<Cow<'a, str>>
    {
        Rule::new(true, path)
    }

    pub fn disallow<P>(path: P) -> Self
        where P: Into<Cow<'a, str>>
    {
        Rule::new(false, path)
    }
}

impl <'a> Render for Rule<'a> {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self.allowance {
            true => writeln!(w, "Allow: {}", self.path),
            false => writeln!(w, "Disallow: {}", self.path),
        }
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
