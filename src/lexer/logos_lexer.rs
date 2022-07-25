use crate::lexer::lexer_error::LexerError;
use crate::lexer::token::*;
use logos::Lexer;
use logos::Logos;
use logos::Span;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum LogosToken {
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurlyBrace,
    #[token("}")]
    RightCurlyBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("=>")]
    FatArrow,
    #[token("\\")]
    LambdaStart,
    #[token("and")]
    And,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("fn")]
    Fn,
    #[token("for")]
    For,
    #[token("if")]
    If,
    #[token("null")]
    Null,
    #[token("or")]
    Or,
    #[token("return")]
    Return,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,
    #[token("true")]
    True,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("while")]
    While,

    // TODO: escape chars in strings
    #[regex("\"[^\"\\\\]*(\\\\.[^\"\\\\]*)*\"")]
    StringLiteral,
    #[regex("(0x|0b)?[0-9]+")]
    IntegerLiteral,
    #[regex(r"[0-9]+\.[0-9]+")]
    FloatLiteral,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"(\n|\r\n)")]
    NewLine,

    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    #[regex("//[^(\n|\r)]*(\n|\r)", logos::skip)]
    // TODO: newlines in multiline comments
    #[regex("/\\*[^\\*]*\\*/", logos::skip)]
    Error,
}

impl From<LogosToken> for TokenType {
    fn from(item: LogosToken) -> TokenType {
        match item {
            LogosToken::LeftParen => TokenType::LeftParen,
            LogosToken::RightParen => TokenType::RightParen,
            LogosToken::LeftCurlyBrace => TokenType::LeftCurlyBrace,
            LogosToken::RightCurlyBrace => TokenType::RightCurlyBrace,
            LogosToken::LeftBracket => TokenType::LeftBracket,
            LogosToken::RightBracket => TokenType::RightBracket,
            LogosToken::Comma => TokenType::Comma,
            LogosToken::Dot => TokenType::Dot,
            LogosToken::Minus => TokenType::Minus,
            LogosToken::Plus => TokenType::Plus,
            LogosToken::Semicolon => TokenType::Semicolon,
            LogosToken::Slash => TokenType::Slash,
            LogosToken::Star => TokenType::Star,
            LogosToken::Bang => TokenType::Bang,
            LogosToken::BangEqual => TokenType::BangEqual,
            LogosToken::Equal => TokenType::Equal,
            LogosToken::EqualEqual => TokenType::EqualEqual,
            LogosToken::Greater => TokenType::Greater,
            LogosToken::GreaterEqual => TokenType::GreaterEqual,
            LogosToken::Less => TokenType::Less,
            LogosToken::LessEqual => TokenType::LessEqual,
            LogosToken::FatArrow => TokenType::FatArrow,
            LogosToken::LambdaStart => TokenType::LambdaStart,
            LogosToken::And => TokenType::And,
            LogosToken::Or => TokenType::Or,
            LogosToken::Else => TokenType::Else,
            LogosToken::Fn => TokenType::Fn,
            LogosToken::For => TokenType::For,
            LogosToken::If => TokenType::If,
            LogosToken::Null => TokenType::Null,
            LogosToken::Return => TokenType::Return,
            LogosToken::Continue => TokenType::Continue,
            LogosToken::Break => TokenType::Break,
            LogosToken::True => TokenType::True,
            LogosToken::False => TokenType::False,
            LogosToken::Let => TokenType::Let,
            LogosToken::Mut => TokenType::Mut,
            LogosToken::While => TokenType::While,

            LogosToken::StringLiteral => TokenType::StringLiteral,
            LogosToken::IntegerLiteral => TokenType::IntegerLiteral,
            LogosToken::FloatLiteral => TokenType::FloatLiteral,
            LogosToken::Identifier => TokenType::Identifier,
            _ => todo!(),
        }
    }
}

pub struct LogosLexer {
    line: usize,
    line_span_start: usize,
    filename: String,
}

impl LogosLexer {
    pub fn new(filename: &str) -> LogosLexer {
        LogosLexer {
            line: 0,
            line_span_start: 0,
            filename: String::from(filename),
        }
    }

    pub fn make(&self, token_type: TokenType, value: &str, span: Span) -> Token {
        let literal_value = match token_type {
            TokenType::StringLiteral => {
                // Cut off the leading and trailing quotes
                let end = value.len() - 1;
                Some(String::from(&value[1..end]))
            }
            TokenType::IntegerLiteral => Some(String::from(value)),
            TokenType::FloatLiteral => Some(String::from(value)),
            _ => None,
        };

        Token {
            token_type,
            literal_value,
            id: self.make_id(value, span),
        }
    }

    pub fn make_id(&self, value: &str, span: Span) -> SpanIdentifier {
        let start = span.start - self.line_span_start;
        let end = start + (span.end - span.start);

        SpanIdentifier {
            filename: self.filename.clone(),
            line: self.line,
            span: (start, end),
            value: String::from(value),
        }
    }

    pub fn new_line(&mut self, span: Span) {
        self.line += 1;
        self.line_span_start = span.end;
    }
}

pub fn lex(filename: &str, content: &str) -> Result<Vec<Token>, LexerError> {
    let mut lex = LogosToken::lexer(content);

    let mut tokens = vec![];
    let mut ll = LogosLexer::new(filename);

    while let Some(logos_token) = lex.next() {
        let slice = lex.slice();
        let span = lex.span();
        match logos_token {
            LogosToken::NewLine => ll.new_line(span),

            LogosToken::Error => {
                return Err(LexerError {
                    id: ll.make_id(slice, span),
                    message: String::from("Unknown input"),
                });
            }
            _ => {
                let t = TokenType::from(logos_token);
                tokens.push(ll.make(t, slice, span));
            }
        };
    }

    Ok(tokens)
}
