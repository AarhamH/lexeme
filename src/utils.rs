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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(extract_nums("1+2"), ("+2","1"));
    }
}