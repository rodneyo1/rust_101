pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (index, &value) in array.iter().enumerate() {
        if value == key {
            return Some(index);
        }
    }
    None
}