pub fn sum(a: i32, b: i32) -> i32 {
    a.checked_add(b).expect("attempt to add with overflow")
}

pub fn diff(a: i32, b: i32) -> i32 {
    a.checked_sub(b).expect("attempt to subtract with overflow")
}

pub fn pro(a: i32, b: i32) -> i32 {
    a.checked_mul(b).expect("attempt to multiply with overflow")
}

pub fn quo(a: f64, b: f64) -> f64 {
    a / b
}

pub fn rem(a: f64, b: f64) -> f64 {
    a % b
}