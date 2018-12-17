pub fn abs(number: i32) -> i32 {
    if number < 0 {
        number * -1
    } else {
        number
    }
}

// Implementation of well-known Euclidean Algorithm. See; https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
       return a;
    }
    
    gcd(b, a % b)
}
