pub(crate) fn extract_nums(s: &str) -> (&str, &str) {
    let mut num_end = 0;

    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() {
            num_end = i + 1;
        } else {
            break;
        }
    }

    let digits = &s[..num_end];
    let remainder = &s[num_end..];
    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(extract_nums("1+2"), ("+2","1"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-22"),("22","-"));
    }

     #[test]
    fn extract_div() {
        assert_eq!(extract_op("/2"),("2","/"));
    }

     #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+32"),("32","+"))
    }

     #[test]
    fn extract_mul() {
        assert_eq!(extract_op("*25"),("25","*"));
    }

 
}