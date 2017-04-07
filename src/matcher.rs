use std::ascii::AsciiExt;

use prelude::*;


#[derive(Clone, Debug)]
pub enum SimpleMatcher<'a> {
    GlobalRule(bool),
    Rules(&'a[Rule<'a>]),
}

impl <'a> SimpleMatcher<'a> {
    pub fn new(rules: &'a[Rule<'a>]) -> Self {
        let mut global_rule: Option<bool> = None;
        let (mut has_allow, mut has_disallow) = (false, false);
        for rule in rules {
            if has_allow && has_disallow {
                break;
            }
            let rule: &Rule = rule;
            match (rule.allow, rule.path.as_ref()) {
                // FIXME this rule must be filtered in a section
                (true,  "")  => continue,
                (false, "")  => global_rule = Some(true),
                (true,  "/") => global_rule = Some(true),
                (false, "/") => global_rule = Some(false),
                (true,  _)   => has_allow = true,
                (false, _)   => has_disallow = true,
            }
            if let Some(global) = global_rule {
                if global && has_disallow || !global && has_allow {
                    global_rule = None
                }
                break;
            }
        }
        match global_rule {
            Some(rule) => SimpleMatcher::GlobalRule(rule),
            None => SimpleMatcher::Rules(rules)
        }
    }

    pub fn check_path(&self, path: &str) -> bool {
        match *self {
            SimpleMatcher::GlobalRule(rule) => rule,
            SimpleMatcher::Rules(rules) => {
                for rule in rules {
                    let rule: &Rule = rule;
                    if rule.path.is_empty() {
                        return true
                    }
                    if rule.path.len() > path.len() {
                        continue
                    }
                    let part: &str = &path[ .. rule.path.len()];
                    if part.eq_ignore_ascii_case(&rule.path) {
                        return rule.allow
                    }
                }
                true
            }
        }
    }

    pub fn has_rules(&self) -> bool {
        match *self {
            SimpleMatcher::Rules(_) => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static ROBOTS1: &'static str = r#"
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
Disallow: /tmp/ # these will soon disappear
"#;


    static ROBOTS2: &'static str = r#"
# robots.txt for http://www.site.com
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
# Cybermapper knows where to go
User-Agent: cybermapper
Disallow:
"#;

    #[test]
    fn matcher1() {
        let robots = Robots::from_str(ROBOTS1);
        let matcher = SimpleMatcher::new(&robots.choose_section("").rules);
        assert!(matcher.has_rules());
        assert!(matcher.check_path("/public"));
        assert!(matcher.check_path("/t"));
        // FIXME striped trailing "/"
        // assert!(!matcher.check_path("/tmp"));
        assert!(!matcher.check_path("/tmp/file1"));
    }

    #[test]
    fn matcher2() {
        let robots = Robots::from_str(ROBOTS2);

        let matcher = SimpleMatcher::new(&robots.choose_section("AnyBot").rules);
        assert!(matcher.has_rules());
        assert!(matcher.check_path("/some/page"));
        assert!(matcher.check_path("/cyberworld/welcome.html"));
        assert!(!matcher.check_path("/cyberworld/map/object.html"));

        let matcher = SimpleMatcher::new(&robots.choose_section("Mozilla/5.0; CyberMapper v. 3.14").rules);
        assert!(!matcher.has_rules());
        assert!(matcher.check_path("/some/page"));
        assert!(matcher.check_path("/cyberworld/welcome.html"));
        assert!(matcher.check_path("/cyberworld/map/object.html"));

    }
}
