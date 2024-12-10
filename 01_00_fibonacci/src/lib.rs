fn fib(n: u32) -> u32 {
    if n < 2  {
        return n;
    } else {
        return fib(n - 1) + fib(n -2);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_0() {
        let result = fib(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn fib_1() {
        let result = fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn fib_19() {
        let res = fib(19);
        assert_eq!(res, 4181);
    }
}
