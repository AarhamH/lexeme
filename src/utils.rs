

pub(crate) fn master_func(accept: impl Fn(char) -> bool, s:&str) -> (&str,&str) {
    let end = s.char_indices().find_map(|(i,c)| if accept(c) {None} else {Some(i)}).unwrap_or_else(|| s.len());
    let extracted = &s[..end];
    let remainder = &s[end..];
    (remainder,extracted)
}

pub(crate) fn extract_nums(s: &str) -> (&str, &str) {
    master_func(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s:&str) -> (&str, &str) {
    master_func(|c| c == ' ', s)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_1() {
        assert_eq!(extract_nums("1+2"), ("+2","1"));
    }

    #[test]
    fn extract_2432() {
        assert_eq!(extract_nums("2432+4442"), ("+4442","2432"));
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

    #[test]
    fn extract_ws() {
        assert_eq!(extract_whitespace("   23"), ("23", "   "));
    }
}