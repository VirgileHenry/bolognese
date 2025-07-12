const END_OF_FILE: u8 = 0x03;
const TABULATION: u8 = 0x09;
const NEW_LINE: u8 = 0x0A;
const CARRIAGE_RETURN: u8 = 0x0D;
const SPACE: u8 = 0x20;
const HASHTAG: u8 = 0x23;
const LIT_MAJ_START: u8 = 0x41;
const LIT_MAJ_END: u8 = 0x5B;
const LIT_MIN_START: u8 = 0x61;
const LIT_MIN_END: u8 = 0x7B;
const BRACKET_OPEN: u8 = 0x5B;
const BRACKET_CLOSED: u8 = 0x5D;
const COMMA: u8 = 0x2C;

/// Single token that can be returned by the parser.
#[derive(Debug)]
pub enum ParserToken {
    /// Literal token, a single sequence of a-zA-Z characters.
    Litteral(String),
    /// Comma token, a single ','.
    Comma,
    /// A section token, which is a literal in square brackets, i.e. "[abc]"
    Section(String),
}

impl ParserToken {
    pub fn kind(&self) -> ParserTokenKind {
        match self {
            ParserToken::Litteral(_) => ParserTokenKind::Litteral,
            ParserToken::Comma => ParserTokenKind::Comma,
            ParserToken::Section(_) => ParserTokenKind::Section,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserTokenKind {
    Litteral,
    Comma,
    Section,
}

pub struct Parser {
    data: Vec<u8>,
    current: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;

        Ok(Self {
            data,
            current: 0,
            line: 1,
            column: 1,
        })
    }
}

impl Parser {
    pub fn expect_section(&mut self, expected: &'static str) -> Result<(), std::io::Error> {
        match self.next() {
            Some(Ok(ParserToken::Section(found))) => {
                if found == expected {
                    Ok(())
                } else {
                    Err(std::io::Error::other(format!(
                        "At line {}, col {}, Expected section '{expected}', found '{found}'.",
                        self.line, self.column
                    )))
                }
            }
            Some(other) => Err(std::io::Error::other(format!(
                "At line {}, col {}, Expected section (for '{expected}'), found token {other:?}",
                self.line, self.column
            ))),
            None => Err(std::io::Error::other(format!(
                "At line {}, col {}, Excpected section (for '{expected}'), found end of file.",
                self.line, self.column
            ))),
        }
    }

    pub fn iter_seq<const N: usize>(
        &mut self,
        seq: [Option<ParserTokenKind>; N],
    ) -> impl Iterator<Item = Result<[Option<ParserToken>; N], std::io::Error>> {
        ParserIterSeq { parser: self, seq }
    }

    fn tick(&mut self) {
        if self.current < self.data.len() {
            match self.data[self.current] {
                NEW_LINE => {
                    self.line += 1;
                    self.column = 1;
                }
                CARRIAGE_RETURN => self.column = 1,
                _ => self.column += 1,
            }
        }
        self.current += 1;
    }

    fn consume_whitespaces_comments(&mut self) -> std::io::Result<()> {
        while self.current < self.data.len()
            && (is_space(self.data[self.current]) || is_comment_start(self.data[self.current]))
        {
            if is_comment_start(self.data[self.current]) {
                while self.current < self.data.len() && !is_newline(self.data[self.current]) {
                    self.tick();
                }
            } else {
                self.tick();
            }
        }

        Ok(())
    }

    fn read_litteral(&mut self) -> std::io::Result<String> {
        let mut result = String::new();

        while self.current < self.data.len() && is_litteral(self.data[self.current]) {
            result.push(char::from(self.data[self.current]));
            self.tick();
        }

        Ok(result)
    }
}

impl Iterator for Parser {
    type Item = Result<ParserToken, std::io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Err(e) = self.consume_whitespaces_comments() {
            return Some(Err(e.into()));
        }
        if self.current >= self.data.len() {
            return None;
        }
        match self.data[self.current] {
            COMMA => {
                self.tick();
                Some(Ok(ParserToken::Comma))
            }
            BRACKET_OPEN => {
                self.tick();
                let litteral = match self.read_litteral() {
                    Ok(lit) => lit,
                    Err(e) => return Some(Err(e.into())),
                };
                if self.current >= self.data.len() || self.data[self.current] != BRACKET_CLOSED {
                    Some(Err(std::io::Error::other(format!(
                        "At line {}, col {}, expected {}, found {}.",
                        self.line,
                        self.column,
                        char::from(BRACKET_CLOSED),
                        if self.current < self.data.len() {
                            char::from(self.data[self.current])
                        } else {
                            char::from(END_OF_FILE)
                        },
                    ))))
                } else {
                    self.tick();
                    Some(Ok(ParserToken::Section(litteral)))
                }
            }
            LIT_MAJ_START..LIT_MAJ_END | LIT_MIN_START..LIT_MIN_END => {
                let litteral = match self.read_litteral() {
                    Ok(lit) => lit,
                    Err(e) => return Some(Err(e.into())),
                };
                Some(Ok(ParserToken::Litteral(litteral)))
            }
            other => Some(Err(std::io::Error::other(format!(
                "At line {}, col {}, unexpected token '{}'",
                self.line,
                self.column,
                char::from(other),
            )))),
        }
    }
}

pub struct ParserIterSeq<'a, const N: usize> {
    parser: &'a mut Parser,
    seq: [Option<ParserTokenKind>; N],
}

impl<'a, const N: usize> Iterator for ParserIterSeq<'a, N> {
    type Item = Result<[Option<ParserToken>; N], std::io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = std::array::from_fn(|_| None);
        let save_current = self.parser.current;

        for i in 0..N {
            match (self.seq[i], self.parser.next()) {
                (None, Some(Ok(_))) => { /* required to skip the token */ }
                (Some(expected), Some(Ok(token))) => {
                    if token.kind() == expected {
                        result[i] = Some(token)
                    } else {
                        self.parser.current = save_current;
                        return None;
                    }
                }
                (_, Some(Err(e))) => {
                    return Some(Err(e));
                }
                (_, None) => {
                    self.parser.current = save_current;
                    return None;
                }
            }
        }

        Some(Ok(result))
    }
}

fn is_newline(byte: u8) -> bool {
    match byte {
        NEW_LINE => true,
        CARRIAGE_RETURN => true,
        _ => false,
    }
}

fn is_space(byte: u8) -> bool {
    match byte {
        TABULATION => true,
        NEW_LINE => true,
        CARRIAGE_RETURN => true,
        SPACE => true,
        _ => false,
    }
}

fn is_comment_start(byte: u8) -> bool {
    match byte {
        HASHTAG => true,
        _ => false,
    }
}

fn is_litteral(byte: u8) -> bool {
    match byte {
        LIT_MAJ_START..LIT_MAJ_END => true,
        LIT_MIN_START..LIT_MIN_END => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_newline() {
        assert_eq!(is_newline('\r' as u8), true);
        assert_eq!(is_newline('\n' as u8), true);
        assert_eq!(is_newline(' ' as u8), false);
        assert_eq!(is_newline('a' as u8), false);
    }

    #[test]
    fn test_is_space() {
        assert_eq!(is_space(' ' as u8), true);
        assert_eq!(is_space('\n' as u8), true);
        assert_eq!(is_space('\r' as u8), true);
        assert_eq!(is_space('\n' as u8), true);
        assert_eq!(is_space('a' as u8), false);
        assert_eq!(is_space(',' as u8), false);
    }

    #[test]
    fn test_comment_start() {
        assert_eq!(is_comment_start('#' as u8), true);
        assert_eq!(is_comment_start('~' as u8), false);
    }

    #[test]
    fn test_litteral() {
        assert_eq!(is_litteral('A' as u8), true);
        assert_eq!(is_litteral('Z' as u8), true);
        assert_eq!(is_litteral('@' as u8), false);
        assert_eq!(is_litteral('[' as u8), false);
        assert_eq!(is_litteral('a' as u8), true);
        assert_eq!(is_litteral('z' as u8), true);
        assert_eq!(is_litteral('`' as u8), false);
        assert_eq!(is_litteral('{' as u8), false);
    }
}
