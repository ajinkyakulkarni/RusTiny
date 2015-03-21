//! All tokens that RusTiny understands

use std::fmt;
use driver;
use front::ast::{BinOp, UnOp, Ident};

// --- List of tokens -----------------------------------------------------------

#[derive(Copy, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    BinOp(BinOp),
    UnOp(UnOp),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    RArrow,
    Eq,

    Keyword(Keyword),
    Ident(Ident),
    Type(Ident),
    Int(u32),
    Char(char),

    EOF,
    PLACEHOLDER
}

impl Token {
    pub fn ty(&self) -> TokenType {
        match *self {
            Token::BinOp(op)    => TokenType::BinOp(op),
            Token::UnOp(..)     => TokenType::UnOp,
            Token::LParen       => TokenType::LParen,
            Token::Eq           => TokenType::Eq,

            Token::Keyword(kw)  => {
                match kw {
                    Keyword::True   => TokenType::Literal,
                    Keyword::False  => TokenType::Literal,

                    _               => TokenType::Other
                }
            },
            Token::Ident(..)    => TokenType::Ident,
            Token::Int(..)      => TokenType::Literal,
            Token::Char(..)     => TokenType::Literal,

            _ => TokenType::Other
        }
    }
}

/// Token type. Used for the Prett Parser
#[derive(Copy, Eq, PartialEq, Hash, Debug)]
pub enum TokenType {
    Literal,
    Ident,
    LParen,
    Eq,
    UnOp,
    BinOp(BinOp),

    Other
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        match *self {
            BinOp(ref op)       => write!(f, "{}", op),
            UnOp(ref op)        => write!(f, "{}", op),

            LParen              => write!(f, "("),
            RParen              => write!(f, ")"),
            LBrace              => write!(f, "{{"),
            RBrace              => write!(f, "}}"),
            Comma               => write!(f, ","),
            Colon               => write!(f, ":"),
            Semicolon           => write!(f, ";"),
            RArrow              => write!(f, "->"),
            Eq                  => write!(f, "="),

            Int(i)              => write!(f, "{}", i),
            Char(c)             => write!(f, "{}", c),

            Keyword(ref kw)     => write!(f, "{}", kw),
            Ident(id)           => write!(f, "{}", id),
            Type(ty)            => write!(f, "{}", ty),
            Token::EOF          => write!(f, "EOF"),
            Token::PLACEHOLDER  => write!(f, "PLACEHOLDER")
        }
    }
}


// --- List of keywords ---------------------------------------------------------

macro_rules! keywords(
    ($($kw:ident => $name:expr),*) => {
        #[derive(Copy, Debug, Eq, PartialEq, Hash)]
        pub enum Keyword {
            $($kw),*
        }

        impl fmt::Display for Keyword {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use self::Keyword::*;

                match *self {
                    $(
                        $kw => write!(f, $name)
                    ),*
                }
            }
        }

        /// Load all keywords into the interner
        pub fn intern_keywords() {
            $( driver::session().interner.intern($name); )*
        }

        /// Get the keyword a string represents, if possible
        pub fn lookup_keyword(s: &str) -> Option<Keyword> {
            use self::Keyword::*;

            match s {
                $(
                    $name => Some($kw),
                )*
                _ => None
            }
        }
    };
);

keywords! {
    Break   => "break",
    Const   => "const",
    Else    => "else",
    False   => "false",
    Fn      => "fn",
    If      => "if",
    Impl    => "impl",
    Let     => "let",
    Return  => "return",
    Static  => "static",
    True    => "true",
    While   => "while"
}