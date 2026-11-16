pub fn total(items: &[u32]) -> u32 {
    let mut sum = 0;
    for item in items {
        sum += item;
    }
    sum
}
