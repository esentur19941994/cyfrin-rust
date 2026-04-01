pub fn zeros() -> [u32; 100] {
    let a = [0; 100];
    a
}

pub fn first_3(s: &[u32]) -> &[u32] {
    let first = &s[0..3];
    first
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let last = &s[s.len() - 3..];
    last
}