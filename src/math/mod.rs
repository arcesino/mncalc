/// Computes absolute value of a number
pub fn abs(number: i32) -> i32 {
    if number < 0 {
        number * -1
    } else {
        number
    }
}

/// Computes the GCD of 2 numbers using well-known Euclidean Algorithm. 
/// See: https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
       return a;
    }
    
    gcd(b, a % b)
}
