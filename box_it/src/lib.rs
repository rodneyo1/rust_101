// lib.rs or in box_it module
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let vec: Vec<u32> = s
        .split_whitespace()
        .filter_map(|part| {
            if part.ends_with('k') || part.ends_with('K') {
                let number = &part[..part.len() - 1];
                number.parse::<f32>().ok().map(|n| (n * 1000.0) as u32)
            } else {
                part.parse::<u32>().ok()
            }
        })
        .collect();

    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a // dereferencing the Box to move out the Vec<u32>
}
