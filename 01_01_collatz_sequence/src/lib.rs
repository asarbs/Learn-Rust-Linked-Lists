fn collatz_length(mut n: i32) -> u32 {
    let mut len: u32 = 1;
    while n > 1 {
      n = if n % 2 == 0 {
        n / 2
      } else {
        3 * n + 1
      };
      len += 1;
    }
    return len;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_length_11() {
        let result = collatz_length(11);
        assert_eq!(result, 15);
    }

    #[test]
    fn collatz_length_15() {
        let result = collatz_length(15);
        assert_eq!(result, 18);
    }

    #[test]
    fn collatz_length_27() {
        let result = collatz_length(27);
        assert_eq!(result, 112);
    }

}
