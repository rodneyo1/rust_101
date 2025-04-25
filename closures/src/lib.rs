pub fn first_fifty_even_square() -> Vec<i32> {
     let mut v = Vec::new();
    let iseven = |i| i%2==0;

    for i in 1..101{
        if iseven(i){
            v.push(i*i)
        }
    }
    v
}