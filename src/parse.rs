pub fn strip_comment(input: &str) -> (&str, Option<&str>) {
    match input.find('#') {
        Some(pos) => (&input[..pos], Some(&input[pos + 1..])),
        None => (input, None),
    }
}

pub fn split_kv(input: &str) -> Option<(&str, &str)> {
    input.find(':').map(|pos| {
        ((&input[..pos]).trim(), (&input[pos + 1..]).trim())
    })
}

pub fn split_rr(input: &str) -> Option<(&str, &str)> {
    input.find('/').map(|pos| {
        ((&input[..pos]).trim(), (&input[pos + 1..]).trim())
    })
}
