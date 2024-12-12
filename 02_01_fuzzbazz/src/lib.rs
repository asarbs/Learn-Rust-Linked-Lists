

pub fn fizzbazz(n: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(n as usize);
    for x in 1..n {
        if x % 3 == 0 && x % 5 == 0 {
            out.push("Fizz Buzz".to_string());
            continue;
        } else if x % 3 == 0 {
            out.push("Fizz".to_string());
            continue;
        } else if x % 5 == 0 {
            out.push("Buzz".to_string());
            continue;
        } else {
            out.push(x.to_string());
        }
    }
    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let exp: Vec<String> = vec![
            "1".to_string(), "2".to_string(), "Fizz".to_string(), "4".to_string(), "Buzz".to_string(),
            "Fizz".to_string(), "7".to_string(), "8".to_string(), "Fizz".to_string(), "Buzz".to_string(),
            "11".to_string(), "Fizz".to_string(), "13".to_string(), "14".to_string(), "Fizz Buzz".to_string(),
            "16".to_string(), "17".to_string(), "Fizz".to_string(), "19".to_string(), "Buzz".to_string(),
            "Fizz".to_string(), "22".to_string(), "23".to_string(), "Fizz".to_string(), "Buzz".to_string(),
            "26".to_string(), "Fizz".to_string(), "28".to_string(), "29".to_string(), "Fizz Buzz".to_string(),
            "31".to_string(), "32".to_string(), "Fizz".to_string(), "34".to_string(), "Buzz".to_string(),
            "Fizz".to_string()
        ];
        let res = fizzbazz(37);
        assert_eq!(res,exp);
    }
}
