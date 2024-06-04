static CELCIUS_MUL_CONST: f32 = 9f32 / 5f32;
static CELCIUS_ADD_CONST: i32 = 32;
static FAHRENHEIT_MUL_CONST: f32 = 5f32 / 9f32;
static FAHRENHEIT_SUB_CONST: i32 = 32;

pub fn celcius2fahrenheit(celcius: i32) -> i32 {
    (celcius as f32 * CELCIUS_MUL_CONST) as i32 + CELCIUS_ADD_CONST
}

pub fn fahrenheit2celcius(fahrenheit: i32) -> i32 {
    ((fahrenheit - FAHRENHEIT_SUB_CONST) as f32 * FAHRENHEIT_MUL_CONST) as i32
}

pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        0u64
    } else if n == 1 {
        1u64
    } else {
        let mut res = 0u64;
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 2..n + 1 {
            res = a + b;
            a = b;
            b = res
        }
        res
    }
}

pub fn fibonacci_rec(n: u32) -> u64 {
    if n == 0 {
        0u64
    } else if n == 1 {
        1u64
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec( n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_celcius() {
        assert_eq!(celcius2fahrenheit(32), 89);
    }

    #[test]
    fn convert_fahrenheit() {
        assert_eq!(fahrenheit2celcius(32), 0);
    }

    #[test]
    fn fibonacci_loop_test() {
        assert_eq!(fibonacci_loop(0), 0);
        assert_eq!(fibonacci_loop(1), 1);
        assert_eq!(fibonacci_loop(2), 1);
        assert_eq!(fibonacci_loop(3), 2);
        assert_eq!(fibonacci_loop(4), 3);
        assert_eq!(fibonacci_loop(5), 5);
        assert_eq!(fibonacci_loop(6), 8);
    }
    
    #[test]
    fn fibonacci_rec_test() {
        assert_eq!(fibonacci_rec(0), 0);
        assert_eq!(fibonacci_rec(1), 1);
        assert_eq!(fibonacci_rec(2), 1);
        assert_eq!(fibonacci_rec(3), 2);
        assert_eq!(fibonacci_rec(4), 3);
        assert_eq!(fibonacci_rec(5), 5);
        assert_eq!(fibonacci_rec(6), 8);
    }
}
