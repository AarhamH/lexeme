#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn create(num_string:&str) -> Self{
        Self(num_string.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn parse_number() {
        assert_eq!(Number::create("123"),Number(123));
    }
}
