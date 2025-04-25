// This function returns a closure that adds a fixed value `x` to its argument.
pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// This function takes a closure `f` and returns a new closure that applies `f` twice.
pub fn twice( f: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| f(f(x)))
}
