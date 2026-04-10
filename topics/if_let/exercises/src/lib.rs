pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    if let Some(value) = x {
        value
    } else {
        v
    }
}
