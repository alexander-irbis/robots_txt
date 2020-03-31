use std::{borrow::Cow, fmt};

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

impl<'a> fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.allow {
            true => "Allow:",
            false => "Disallow:",
        })?;
        if !self.path.is_empty() {
            write!(f, " {}", self.path)?;
        };
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!("Allow: /\n", Rule::allow("/").to_string());
        assert_eq!("Disallow: /\n", Rule::disallow("/").to_string());
    }
}
