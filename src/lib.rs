pub mod expression;
mod utils;

use crate::expression::Expression;

#[derive(Debug,PartialEq)]
pub struct BindingDef {
    name: String,
    value: Expression
}

impl BindingDef {
    pub fn create(line_string:&str) -> (&str,Self) {

        // check for declarative string, if exists, strip it off the og string
        let line_string = if line_string.starts_with("let") {
            &line_string[3..]
        }
        else {
            panic!("Expected line to start with let")
        };
        let (line_string, _) = utils::extract_whitespace(line_string);

        let (line_string, name) = utils::extract_identifier(line_string);
        let (line_string, _) = utils::extract_whitespace(line_string);

        let line_string = if line_string.starts_with('=') {
            &line_string[1..]
        }
        else {
            panic!("No equals sign")
        };

        let (line_string,_) = utils::extract_whitespace(line_string);

        let (line_string, value) = Expression::new(line_string);

        (
            line_string,
            Self {
                name: name.to_string(),
                value,
            }
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Number,Operator, OperatorEnum};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::create("let a = 10/2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    value: Expression {
                        left: Number(10),
                        right: Number(2),
                        op: Operator(OperatorEnum::Divide)
                    }
                }
            )
        )
    }
    
}

