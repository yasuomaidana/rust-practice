use std::ops::Sub;

pub fn function_evaluator<F, T>(f: F, a: T, b: T) -> T
where
    F: Fn(T) -> T,
    T: Sub<Output = T>,
{
    f(b) - f(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f = |x: f64| 2.0 * x;
        assert_eq!(function_evaluator(f, 1.0, 3.0), 4.0);
    }
}
