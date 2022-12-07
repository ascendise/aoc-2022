#[cfg(test)]
mod tests;

pub fn get_start_index(data: &str, buffer_size: usize) -> usize {
    data.as_bytes()
        .windows(buffer_size)
        .position(|w| has_unique_elements(&w))
        .unwrap() + buffer_size
}

fn has_unique_elements(slice: &[u8]) -> bool {
    !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}