use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Number {
    Float(f64),
    Integer(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Parsed<'a> {
    Token(&'a str),
    Str(&'a str),
    Number(Number),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    InvalidString(usize),
    InvalidNumber(usize),
    ExpectedWhitespace(usize),
}

pub type ParseResult<'a, T> = Result<(usize, T, &'a str), ParseError>;

pub fn parse_token(src: &str) -> ParseResult<&str> {
    let mut t_start = None;
    let mut t_end = None;
    let mut end = None;
    for (i, c) in src.chars().enumerate() {
        if t_start.is_none() {
            if !c.is_whitespace() {
                t_start = Some(i);
            }
            continue;
        }
        if t_end.is_none() {
            if c.is_whitespace() {
                t_end = Some(i);
            }
            continue;
        }
        if !c.is_whitespace() {
            end = Some(i);
            break;
        }
    }
    if t_start.is_none() && t_end.is_none() {
        return Err(ParseError::UnexpectedEof);
    }

    let t_start = t_start.unwrap();
    if t_end.is_none() {
        Ok((t_start, &src[t_start..], ""))
    } else if end.is_none() {
        Ok((t_start, &src[t_start..t_end.unwrap()], ""))
    } else {
        Ok((t_start, &src[t_start..t_end.unwrap()], &src[end.unwrap()..]))
    }
}

fn parse_delimited(
    src: &str,
    start_char: char,
    end_char: char,
    escape_char: char,
) -> ParseResult<&str> {
    let mut s_start = None;
    let mut s_end = None;
    let mut end = None;
    let mut was_start_char = false;
    let mut was_end_char = false;
    let mut was_escape_char = false;
    for (i, c) in src.chars().enumerate() {
        if s_start.is_none() {
            if was_start_char {
                s_start = Some(i);
                was_start_char = false;
            } else if c == start_char {
                was_start_char = true;
                continue;
            } else if c.is_whitespace() {
                continue;
            } else {
                return Err(ParseError::InvalidString(i));
            }
        }
        if s_end.is_none() {
            if !was_escape_char && c == end_char {
                s_end = Some(i);
                was_end_char = true;
            } else if c == escape_char {
                was_escape_char = true;
                continue;
            }
            was_escape_char = false;
            continue;
        }
        if !c.is_whitespace() {
            if was_end_char {
                return Err(ParseError::ExpectedWhitespace(i));
            }
            end = Some(i);
            break;
        }
        was_end_char = false;
    }

    if s_start.is_none() || s_end.is_none() {
        return Err(ParseError::UnexpectedEof);
    }

    let s_start = s_start.unwrap();
    let s_end = s_end.unwrap();
    if end.is_none() {
        Ok((s_start, &src[s_start..s_end], ""))
    } else {
        Ok((s_start, &src[s_start..s_end], &src[end.unwrap()..]))
    }
}

#[inline]
pub fn parse_string(src: &str) -> ParseResult<&str> {
    parse_delimited(src, '`', '`', '\\')
}

pub fn parse_number(src: &str) -> ParseResult<Number> {
    let (index, token, rest) = parse_token(src)?;
    if let Ok(num) = token.parse() {
        Ok((index, Number::Integer(num), rest))
    } else if let Ok(num) = token.parse() {
        Ok((index, Number::Float(num), rest))
    } else {
        return Err(ParseError::InvalidNumber(index));
    }
}

pub fn parse_next(src: &str) -> ParseResult<Parsed> {
    if let Ok((index, string, rest)) = parse_string(src) {
        Ok((index, Parsed::Str(string), rest))
    } else if let Ok((index, num, rest)) = parse_number(src) {
        Ok((index, Parsed::Number(num), rest))
    } else {
        parse_token(src).map(|(index, token, rest)| (index, Parsed::Token(token), rest))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEof => f.write_str("unexpected end of file"),
            ParseError::InvalidString(i) => {
                f.write_fmt(format_args!("invalid string at character {}", i + 1))
            }
            ParseError::InvalidNumber(i) => {
                f.write_fmt(format_args!("invalid number at character {}", i + 1))
            }
            ParseError::ExpectedWhitespace(i) => {
                f.write_fmt(format_args!("expected whitespace at character {}", i + 1))
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl Eq for Number {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() -> Result<(), ParseError> {
        assert_eq!((0, "a", ""), parse_token("a")?);
        assert_eq!((0, "ab", ""), parse_token("ab")?);
        assert_eq!((0, "the", ""), parse_token("the")?);
        assert_eq!((1, "the", ""), parse_token(" the")?);
        assert_eq!((0, "the", ""), parse_token("the ")?);
        assert_eq!((1, "the", ""), parse_token(" the ")?);
        assert_eq!((3, "the", ""), parse_token("   the")?);
        assert_eq!((0, "the", ""), parse_token("the   ")?);
        assert_eq!((3, "the", ""), parse_token("   the   ")?);
        assert_eq!((0, "the", "list"), parse_token("the list")?);
        assert_eq!((1, "the", "list"), parse_token(" the list")?);
        assert_eq!((0, "the", "list"), parse_token("the   list")?);
        assert_eq!((3, "the", "list"), parse_token("   the   list")?);
        assert_eq!((0, "the", "list "), parse_token("the list ")?);
        assert_eq!((1, "the", "list "), parse_token(" the list ")?);
        assert_eq!((0, "the", "list "), parse_token("the   list ")?);
        assert_eq!((3, "the", "list "), parse_token("   the   list ")?);
        assert!(matches!(parse_token(""), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_token(" "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_token("   "), Err(ParseError::UnexpectedEof)));
        Ok(())
    }

    #[test]
    fn parse_strings() -> Result<(), ParseError> {
        assert_eq!((1, "", ""), parse_string("``")?);
        assert_eq!((1, "a", ""), parse_string("`a`")?);
        assert_eq!((1, " ", ""), parse_string("` `")?);
        assert_eq!((1, "hello, world", ""), parse_string("`hello, world`")?);
        assert_eq!((2, "hello, world", ""), parse_string(" `hello, world`")?);
        assert_eq!((1, "hello, world", ""), parse_string("`hello, world` ")?);
        assert_eq!((2, "hello, world", ""), parse_string(" `hello, world` ")?);
        assert_eq!((4, "hello, world", ""), parse_string("   `hello, world`")?);
        assert_eq!((1, "hello, world", ""), parse_string("`hello, world`   ")?);
        assert_eq!(
            (4, "hello, world", ""),
            parse_string("   `hello, world`   ")?
        );

        assert_eq!(
            (1, "hello, world", "token"),
            parse_string("`hello, world` token")?
        );
        assert_eq!(
            (2, "hello, world", "token"),
            parse_string(" `hello, world` token")?
        );
        assert_eq!(
            (1, "hello, world", "token"),
            parse_string("`hello, world`   token")?
        );
        assert_eq!(
            (4, "hello, world", "token"),
            parse_string("   `hello, world`   token")?
        );

        assert_eq!(
            (1, "hello, world", "token "),
            parse_string("`hello, world` token ")?
        );
        assert_eq!(
            (2, "hello, world", "token "),
            parse_string(" `hello, world` token ")?
        );
        assert_eq!(
            (1, "hello, world", "token "),
            parse_string("`hello, world`   token ")?
        );
        assert_eq!(
            (4, "hello, world", "token "),
            parse_string("   `hello, world`   token ")?
        );

        assert_eq!((1, "hello", "`world`"), parse_string("`hello` `world`")?);

        assert!(matches!(parse_string(""), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string(" "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(
            parse_string("   "),
            Err(ParseError::UnexpectedEof)
        ));

        assert!(matches!(parse_string("`"), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string("` "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string(" `"), Err(ParseError::UnexpectedEof)));
        assert!(matches!(
            parse_string(" ` "),
            Err(ParseError::UnexpectedEof)
        ));

        assert!(matches!(
            parse_string("a"),
            Err(ParseError::InvalidString(0))
        ));
        assert!(matches!(
            parse_string("a "),
            Err(ParseError::InvalidString(0))
        ));
        assert!(matches!(
            parse_string(" a"),
            Err(ParseError::InvalidString(1))
        ));
        assert!(matches!(
            parse_string(" a "),
            Err(ParseError::InvalidString(1))
        ));

        assert_eq!((1, r#"\`"#, ""), parse_string(r#"`\``"#)?);
        assert_eq!((1, r#"escaped\`"#, ""), parse_string(r#"`escaped\``"#)?);
        assert_eq!(
            (1, r#"escaped\`text"#, ""),
            parse_string(r#"`escaped\`text`"#)?
        );
        assert_eq!((1, r#" \`"#, ""), parse_string(r#"` \``"#)?);
        assert_eq!((1, r#"\` "#, ""), parse_string(r#"`\` `"#)?);
        assert_eq!((2, r#"\`"#, ""), parse_string(r#" `\``"#)?);
        assert_eq!((1, r#"\`"#, ""), parse_string(r#"`\`` "#)?);
        assert_eq!((2, r#"\`"#, ""), parse_string(r#" `\`` "#)?);

        assert!(matches!(
            parse_string("``a"),
            Err(ParseError::ExpectedWhitespace(2))
        ));
        assert!(matches!(
            parse_string("`hello`world"),
            Err(ParseError::ExpectedWhitespace(7))
        ));
        assert!(matches!(
            parse_string("`hello`world`"),
            Err(ParseError::ExpectedWhitespace(7))
        ));

        Ok(())
    }

    #[test]
    fn parse_numbers() -> Result<(), ParseError> {
        assert_eq!((0, Number::Integer(0), ""), parse_number("0")?);
        assert_eq!((0, Number::Integer(1), ""), parse_number("1")?);
        assert_eq!((0, Number::Integer(-1), ""), parse_number("-1")?);
        assert_eq!((0, Number::Float(0.), ""), parse_number("0.0")?);
        assert_eq!((0, Number::Float(1.), ""), parse_number("1.0")?);
        assert_eq!((0, Number::Float(-1.), ""), parse_number("-1.0")?);
        assert_eq!((0, Number::Integer(123), ""), parse_number("123")?);
        assert_eq!((0, Number::Integer(-123), ""), parse_number("-123")?);
        assert_eq!((0, Number::Float(123.123), ""), parse_number("123.123")?);
        assert_eq!((0, Number::Float(-123.123), ""), parse_number("-123.123")?);

        assert_eq!((1, Number::Integer(1), ""), parse_number(" 1")?);
        assert_eq!((0, Number::Integer(1), ""), parse_number("1 ")?);
        assert_eq!((1, Number::Integer(1), ""), parse_number(" 1 ")?);
        assert_eq!((1, Number::Float(1.), ""), parse_number(" 1.0")?);
        assert_eq!((0, Number::Float(1.), ""), parse_number("1.0 ")?);
        assert_eq!((1, Number::Float(1.), ""), parse_number(" 1.0 ")?);

        assert_eq!((0, Number::Integer(1), "token"), parse_number("1 token")?);
        assert_eq!((0, Number::Float(1.), "token"), parse_number("1.0 token")?);

        assert!(matches!(
            parse_number("a"),
            Err(ParseError::InvalidNumber(0))
        ));
        assert!(matches!(
            parse_number("a "),
            Err(ParseError::InvalidNumber(0))
        ));
        assert!(matches!(
            parse_number(" a"),
            Err(ParseError::InvalidNumber(1))
        ));
        assert!(matches!(
            parse_number(" a "),
            Err(ParseError::InvalidNumber(1))
        ));

        Ok(())
    }

    #[test]
    fn parse_nexts() -> Result<(), ParseError> {
        assert_eq!(
            (0, Parsed::Number(Number::Integer(0)), ""),
            parse_next("0")?
        );
        assert_eq!(
            (0, Parsed::Number(Number::Float(1.)), ""),
            parse_next("1.0")?
        );
        assert_eq!(
            (1, Parsed::Str("hello, world"), ""),
            parse_next("`hello, world`")?
        );
        assert_eq!((0, Parsed::Token("token"), ""), parse_next("token")?);

        assert_eq!(
            (0, Parsed::Number(Number::Integer(0)), "token"),
            parse_next("0 token")?
        );
        assert_eq!(
            (0, Parsed::Number(Number::Float(1.)), "token"),
            parse_next("1.0 token")?
        );
        assert_eq!(
            (1, Parsed::Str("hello, world"), "token"),
            parse_next("`hello, world` token")?
        );
        assert_eq!(
            (0, Parsed::Token("token"), "token2"),
            parse_next("token token2")?
        );

        Ok(())
    }
}
