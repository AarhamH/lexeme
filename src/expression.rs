use crate::{utils, value::Value};

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn create(num_string:&str) -> (&str,Self){  
        let(num_string, number) = utils::extract_nums(num_string);
        (num_string, Self(number.parse().unwrap()))
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
    pub fn create(op_string:&str) -> (&str,Self) {
        let (op_string, op) = utils::extract_op(op_string);
        let op = match op {
            "+" => Self(OperatorEnum::Add),
            "-" => Self(OperatorEnum::Subtract),
            "/" => Self(OperatorEnum::Divide),
            "*" => Self(OperatorEnum::Multiply),
            _ => unreachable!(),

        };

        (op_string,op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub left: Number,
    pub right: Number,
    pub op: Operator
}

impl Expression {
    pub fn create(s: &str) -> (&str, Self) {
        let (s, left) = Number::create(s); 
        let (s,_) = utils::extract_whitespace(s);

        let (s, op) = Operator::create(s);
        let (s,_) = utils::extract_whitespace(s);

        let (s, right) = Number::create(s);
        
        (s, Self { left, right, op })
    }

    pub(crate) fn evaluate(&self) -> Value {
        let Number(left) = self.left;
        let Number(right) = self.right;

        let res = match self.op {
            Operator(OperatorEnum::Add) => left + right,
            Operator(OperatorEnum::Subtract) => left - right,
            Operator(OperatorEnum::Multiply) => left * right,
            Operator(OperatorEnum::Divide) => left / right,
        };

        Value::Number(res)
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::create("123"),("",Number(123)));
    }

    #[test]
    fn parse_operator_plus() {
        assert_eq!(Operator::create("+"),("",Operator(OperatorEnum::Add)));
    }

    #[test]

    fn parse_operator_minus() { 
        assert_eq!(Operator::create("-"),("",Operator(OperatorEnum::Subtract)));
    }

    #[test]
    fn parse_operator_div() {
        assert_eq!(Operator::create("/"),("", Operator(OperatorEnum::Divide)));
    }

    #[test]
    fn parse_operator_mul() {
        assert_eq!(Operator::create("*"),("", Operator(OperatorEnum::Multiply)));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expression::create("1+2"),
            (
                "",
                Expression {
                    left: Number(1),
                    right: Number(2),
                    op: Operator(OperatorEnum::Add),
                },
            ),
        );
    }

    #[test]
    fn parse_one_plus_two_with_spaces() {
        assert_eq!(
            Expression::create("1       +  2"),
            (
                "",
                Expression {
                    left: Number(1),
                    right: Number(2),
                    op: Operator(OperatorEnum::Add),
                },
            ),
        );
    }
        #[test]
    fn evaluate_add() {
        assert_eq!(
            Expression {
                left: Number(10),
                right: Number(10),
                op: Operator(OperatorEnum::Add),
            }
            .evaluate(),
            Value::Number(20),
        );
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(
            Expression {
                left: Number(1),
                right: Number(5),
                op: Operator(OperatorEnum::Subtract)
            }
            .evaluate(),
            Value::Number(-4),
        );
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(
            Expression {
                left: Number(5),
                right: Number(6),
                op: Operator(OperatorEnum::Multiply),
            }
            .evaluate(),
            Value::Number(30),
        );
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(
            Expression {
                left: Number(200),
                right: Number(20),
                op: Operator(OperatorEnum::Divide),
            }
            .evaluate(),
            Value::Number(10),
        );
    }
}

