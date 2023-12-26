#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn create(num_string:&str) -> Self{
        Self(num_string.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum OperatorEnum {
    Add,
    Subtract,
    Divide,
    Multiply
}

#[derive(Debug, PartialEq)]
pub struct Operator(pub OperatorEnum);

impl Operator {
    pub fn create(op_string:&str) -> Self {
        match op_string {
            "+" => Self(OperatorEnum::Add),
            "-" => Self(OperatorEnum::Subtract),
            "/" => Self(OperatorEnum::Divide),
            "*" => Self(OperatorEnum::Multiply),
            _ => panic!("Invalid Operator!")

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::create("123"),Number(123));
    }

    #[test]
    fn parse_operator_plus() {
        assert_eq!(Operator::create("+"),Operator(OperatorEnum::Add));
    }

    #[test]
    fn parse_operator_minus() {
        assert_eq!(Operator::create("-"),Operator(OperatorEnum::Subtract));
    }

    #[test]
    fn parse_operator_div() {
        assert_eq!(Operator::create("/"),Operator(OperatorEnum::Divide));
    }

    #[test]
    fn parse_operator_mul() {
        assert_eq!(Operator::create("*"),Operator(OperatorEnum::Multiply));
    }
}
