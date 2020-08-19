#[derive(Debug)]
enum ParseError {
    UnexpectedEof,
    UnexpectedChar {
        index: usize,
        expected: char,
        got: char,
    },
}

fn parse_token(src: &str) -> Result<(&str, &str), ParseError> {
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
        Err(ParseError::UnexpectedEof)
    } else if t_end.is_none() {
        Ok((&src[t_start.unwrap()..], ""))
    } else if end.is_none() {
        Ok((&src[t_start.unwrap()..t_end.unwrap()], ""))
    } else {
        Ok((&src[t_start.unwrap()..t_end.unwrap()], &src[end.unwrap()..]))
    }
}

fn parse_delimited(
    src: &str,
    start_char: char,
    end_char: char,
    escape_char: char,
) -> Result<(&str, &str), ParseError> {
    let mut s_start = None;
    let mut s_end = None;
    let mut end = None;
    let mut was_start_char = false;
    let mut was_escape_char = false;
    for (i, c) in src.chars().enumerate() {
        println!(
            "{} {} {:?} {:?} {:?} {:?} {:?}",
            i, c, was_start_char, was_escape_char, s_start, s_end, end
        );
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
                return Err(ParseError::UnexpectedChar {
                    index: i,
                    expected: start_char,
                    got: c,
                });
            }
        }
        if s_end.is_none() {
            if !was_escape_char && c == end_char {
                s_end = Some(i);
            } else if c == escape_char {
                was_escape_char = true;
                continue;
            }
            was_escape_char = false;
            continue;
        }
        if !c.is_whitespace() {
            end = Some(i);
            break;
        }
    }
    if s_start.is_none() || s_end.is_none() {
        Err(ParseError::UnexpectedEof)
    } else if s_end.is_none() {
        Ok((&src[s_start.unwrap()..], ""))
    } else if end.is_none() {
        Ok((&src[s_start.unwrap()..s_end.unwrap()], ""))
    } else {
        Ok((&src[s_start.unwrap()..s_end.unwrap()], &src[end.unwrap()..]))
    }
}

#[inline]
fn parse_string(src: &str) -> Result<(&str, &str), ParseError> {
    parse_delimited(src, '\'', '\'', '\\')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() -> Result<(), ParseError> {
        assert_eq!(("a", ""), parse_token("a")?);
        assert_eq!(("ab", ""), parse_token("ab")?);
        assert_eq!(("the", ""), parse_token("the")?);
        assert_eq!(("the", ""), parse_token(" the")?);
        assert_eq!(("the", ""), parse_token("the ")?);
        assert_eq!(("the", ""), parse_token(" the ")?);
        assert_eq!(("the", ""), parse_token("   the")?);
        assert_eq!(("the", ""), parse_token("the   ")?);
        assert_eq!(("the", ""), parse_token("   the   ")?);
        assert_eq!(("the", "list"), parse_token("the list")?);
        assert_eq!(("the", "list"), parse_token(" the list")?);
        assert_eq!(("the", "list"), parse_token("the   list")?);
        assert_eq!(("the", "list"), parse_token("   the   list")?);
        assert_eq!(("the", "list "), parse_token("the list ")?);
        assert_eq!(("the", "list "), parse_token(" the list ")?);
        assert_eq!(("the", "list "), parse_token("the   list ")?);
        assert_eq!(("the", "list "), parse_token("   the   list ")?);
        assert!(matches!(parse_token(""), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_token(" "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_token("   "), Err(ParseError::UnexpectedEof)));
        Ok(())
    }

    #[test]
    fn parse_strings() -> Result<(), ParseError> {
        assert_eq!(("", ""), parse_string("''")?);
        assert_eq!(("a", ""), parse_string("'a'")?);
        assert_eq!((" ", ""), parse_string("' '")?);
        assert_eq!(("hello, world", ""), parse_string("'hello, world'")?);
        assert_eq!(("hello, world", ""), parse_string(" 'hello, world'")?);
        assert_eq!(("hello, world", ""), parse_string("'hello, world' ")?);
        assert_eq!(("hello, world", ""), parse_string(" 'hello, world' ")?);
        assert_eq!(("hello, world", ""), parse_string("   'hello, world'")?);
        assert_eq!(("hello, world", ""), parse_string("'hello, world'   ")?);
        assert_eq!(("hello, world", ""), parse_string("   'hello, world'   ")?);

        assert_eq!(
            ("hello, world", "token"),
            parse_string("'hello, world' token")?
        );
        assert_eq!(
            ("hello, world", "token"),
            parse_string(" 'hello, world' token")?
        );
        assert_eq!(
            ("hello, world", "token"),
            parse_string("'hello, world'   token")?
        );
        assert_eq!(
            ("hello, world", "token"),
            parse_string("   'hello, world'   token")?
        );

        assert_eq!(
            ("hello, world", "token "),
            parse_string("'hello, world' token ")?
        );
        assert_eq!(
            ("hello, world", "token "),
            parse_string(" 'hello, world' token ")?
        );
        assert_eq!(
            ("hello, world", "token "),
            parse_string("'hello, world'   token ")?
        );
        assert_eq!(
            ("hello, world", "token "),
            parse_string("   'hello, world'   token ")?
        );

        assert!(matches!(parse_string(""), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string(" "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(
            parse_string("   "),
            Err(ParseError::UnexpectedEof)
        ));

        assert!(matches!(parse_string("'"), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string("' "), Err(ParseError::UnexpectedEof)));
        assert!(matches!(parse_string(" '"), Err(ParseError::UnexpectedEof)));
        assert!(matches!(
            parse_string(" ' "),
            Err(ParseError::UnexpectedEof)
        ));

        assert!(matches!(
            parse_string("a"),
            Err(ParseError::UnexpectedChar {
                index: 0,
                expected: '\'',
                got: 'a'
            })
        ));
        assert!(matches!(
            parse_string("a "),
            Err(ParseError::UnexpectedChar {
                index: 0,
                expected: '\'',
                got: 'a'
            })
        ));
        assert!(matches!(
            parse_string(" a"),
            Err(ParseError::UnexpectedChar {
                index: 1,
                expected: '\'',
                got: 'a'
            })
        ));
        assert!(matches!(
            parse_string(" a "),
            Err(ParseError::UnexpectedChar {
                index: 1,
                expected: '\'',
                got: 'a'
            })
        ));

        assert_eq!((r#"\'"#, ""), parse_string(r#"'\''"#)?);
        assert_eq!((r#"escaped\'"#, ""), parse_string(r#"'escaped\''"#)?);
        assert_eq!(
            (r#"escaped\'text"#, ""),
            parse_string(r#"'escaped\'text'"#)?
        );
        assert_eq!((r#" \'"#, ""), parse_string(r#"' \''"#)?);
        assert_eq!((r#"\' "#, ""), parse_string(r#"'\' '"#)?);
        assert_eq!((r#"\'"#, ""), parse_string(r#" '\''"#)?);
        assert_eq!((r#"\'"#, ""), parse_string(r#"'\'' "#)?);
        assert_eq!((r#"\'"#, ""), parse_string(r#" '\'' "#)?);

        Ok(())
    }
}
