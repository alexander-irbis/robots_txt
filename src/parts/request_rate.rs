use std::{fmt, time::Duration};

use crate::render::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RequestRate {
    pub requests: usize,
    pub seconds: usize,
}

impl RequestRate {
    pub fn new(requests: usize, seconds: usize) -> Self {
        RequestRate { requests, seconds }
    }

    pub fn into_duration(self) -> Duration {
        if self.seconds == 0 {
            Duration::new(0, 0)
        } else {
            let rate = self.seconds as f64 / self.requests as f64;
            Duration::new(rate.abs() as u64, (rate.fract() * 1_000_000_000.) as u32)
        }
    }
}

impl Render for RequestRate {
    fn render_to<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        writeln!(w, "Request-rate: {}/{}", self.requests, self.seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        assert_eq!(
            "Request-rate: 3/10\n",
            RequestRate::new(3, 10).render().unwrap()
        );
    }
}
