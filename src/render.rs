use std::fmt;

pub trait Render {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;

    fn render(&self) -> Result<String, fmt::Error> {
        let mut buffer = String::new();
        self.render_to(&mut buffer)?;
        Ok(buffer)
    }
}
