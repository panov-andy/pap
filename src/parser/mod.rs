#[derive(Debug)]
pub enum Status {
    Ok,
    Info(String),
    Err(String),
}

#[derive(Debug)]
pub struct MResult<IN, OUT> {
    input: IN,
    result: OUT,
    status: Status,
}

impl<IN, OUT> MResult<IN, OUT> {
    fn ok(input: IN, out: OUT) -> MResult<IN, OUT> {
        return MResult { input, result: out, status: Status::Ok };
    }
}

type Parser = dyn Fn(&str) -> MResult<&str, &str>;

pub fn parser_start_with<'i, 't>(target: &'t str) -> impl Fn(&'i str) -> MResult<&'i str, &'i str> + 't {
    return move |input| {
        if input.len() == 0 {
            return MResult { input: "", result: "", status: Status::Err("empty string".to_string()) };
        } else if input.starts_with(&target) {
            return MResult::ok(&input[target.len()..], &input[..target.len()]);
        }
        let got = &input[..target.len()];
        let message = format!("expected {}, got {}", &target, &got);
        return MResult { input: &input, result: "", status: Status::Err(message) };
    };
}

pub fn parser_ends_with(target: &str) -> Parser {
    return move |input| {
        if input.len() == 0 {
            return MResult { input: "", result: "", status: Status::Err("empty string".to_string()) };
        } else if input.ends_with(&target) {
            return MResult::ok(&input[target.len()..], &input[..target.len()]);
        }
        let got = &input[target.len()..];
        let message = format!("expected {}, got {}", &target, &got);
        return MResult { input: &input, result: "", status: Status::Err(message) };
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::{parser_ends_with, parser_start_with};

    #[test]
    fn test_start_with() {
        let start_with_ab = parser_start_with("ab");
        let start_with_ra = parser_start_with("ra");

        let result = start_with_ab("abracadabra");
        println!("{:?}", result)
    }
    #[test]
    fn test_ends_with() {
        let ends_with_ra = parser_ends_with("ra");

        let result = start_with_ab("abracadabra");
        println!("{:?}", result)
    }
}