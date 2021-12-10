use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum ParseResult {
    Valid,
    Incomplete(String),
    Illegal(char),
}

fn parse_line(s: &String) -> ParseResult {
    let mut queue: Vec<char> = Vec::default();
    for c in s.chars() {
        match c {
            '[' => queue.push(']'),
            '(' => queue.push(')'),
            '{' => queue.push('}'),
            '<' => queue.push('>'),
            ch => match queue.pop() {
                Some(q_ch) if q_ch == ch => {}
                _ => return ParseResult::Illegal(ch),
            },
        }
    }
    if queue.len() == 0 {
        ParseResult::Valid
    } else {
        let s2: String = queue.iter().rev().collect();
        ParseResult::Incomplete(s2)
    }
}

fn main() -> Result<(), String> {
    let results = BufReader::new(std::io::stdin())
        .lines()
        .map(|r| r.map(|s| parse_line(&s)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    let mut total_syntax_error_score = 0;
    let mut incomplete_scores = Vec::default();
    for r in results {
        match r {
            ParseResult::Illegal(ch) => {
                total_syntax_error_score += match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                }
            }
            ParseResult::Incomplete(st) => {
                incomplete_scores.push(st.chars().fold(0_i64, |score, ch| {
                    score * 5
                        + match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                }))
            }
            _ => {}
        }
    }

    incomplete_scores.sort();
    let middle_score = incomplete_scores[(incomplete_scores.len()) / 2];

    println!("Total syntax error score: {:?}", total_syntax_error_score);
    println!("Middle score: {:?}", middle_score);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_line() {
        assert_eq!(
            parse_line(&"[({(<(())[]>[[{[]{<()<>>".to_owned()),
            ParseResult::Incomplete("}}]])})]".to_owned())
        );
        assert_eq!(
            parse_line(&"[(()[<>])]({[<{<<[]>>(".to_owned()),
            ParseResult::Incomplete(")}>]})".to_owned())
        );
        assert_eq!(
            parse_line(&"{([(<{}[<>[]}>{[]{[(<()>".to_owned()),
            ParseResult::Illegal('}')
        );
        assert_eq!(
            parse_line(&"(((({<>}<{<{<>}{[]{[]{}".to_owned()),
            ParseResult::Incomplete("}}>}>))))".to_owned())
        );
        assert_eq!(
            parse_line(&"[[<[([]))<([[{}[[()]]]".to_owned()),
            ParseResult::Illegal(')')
        );
        assert_eq!(
            parse_line(&"[{[{({}]{}}([{[{{{}}([]".to_owned()),
            ParseResult::Illegal(']')
        );
        assert_eq!(
            parse_line(&"{<[[]]>}<{[{[{[]{()[[[]".to_owned()),
            ParseResult::Incomplete("]]}}]}]}>".to_owned())
        );
        assert_eq!(
            parse_line(&"[<(<(<(<{}))><([]([]()".to_owned()),
            ParseResult::Illegal(')')
        );
        assert_eq!(
            parse_line(&"<{([([[(<>()){}]>(<<{{".to_owned()),
            ParseResult::Illegal('>')
        );
        assert_eq!(
            parse_line(&"<{([{{}}[<[[[<>{}]]]>[]]".to_owned()),
            ParseResult::Incomplete("])}>".to_owned())
        );
    }
}
