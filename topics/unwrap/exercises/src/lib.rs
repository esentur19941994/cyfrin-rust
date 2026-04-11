pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let num_a = a.parse::<u32>().expect("Failed to parse variable");
    let num_b = b.parse::<u32>().expect("Failed to parse variable");
    num_a + num_b
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
}
