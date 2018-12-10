mod labels;
mod token;
mod util;

pub use crate::lexer::token::*;

use crate::lexer::labels::*;
use crate::lexer::token::Token::*;

use crate::error::Error;
use std::str;
use toolshed::Arena;

macro_rules! expect_byte {
    ($lex:ident) => {{
        match $lex.read_byte() {
            0 => return $lex.token = UnexpectedEndOfProgram,
            _ => $lex.bump(),
        }
    }};
}

macro_rules! unwind_loop {
    ($iteration:expr) => ({
        $iteration
        $iteration
        $iteration
        $iteration
        $iteration

        loop {
            $iteration
            $iteration
            $iteration
            $iteration
            $iteration
        }
    })
}

/// Contextual check describing which Automatic Semicolon Insertion rules can be applied.
#[derive(Clone, Copy, PartialEq)]
pub enum Asi {
    /// Current token is a semicolon. Parser should consume it and finalize the statement.
    ExplicitSemicolon,

    /// Current token is not a semicolon, but previous token is either followed by a
    /// line termination, or allows semicolon insertion itself. Parser should finalize the
    /// statement without consuming the current token.
    ImplicitSemicolon,

    /// Current token is not a semicolon, and no semicolon insertion rules were triggered.
    /// Parser should continue parsing the statement or error.
    NoSemicolon,
}

type ByteHandler = Option<for<'arena> fn(&mut Lexer<'arena>)>;

/// Lookup table mapping any incoming byte to a handler function defined below.
static BYTE_HANDLERS: [ByteHandler; 256] = [
    //   0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F   //
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1
    ___, EXL, QOT, ERR, IDT, PRC, AMP, QOT, PNO, PNC, ATR, PLS, COM, MIN, PRD, SLH, // 2
    ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, COL, SEM, LSS, EQL, MOR, QST, // 3
    ERR, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
    IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BTO, IDT, BTC, CRT, IDT, // 5
    TPL, L_A, L_B, L_C, L_D, L_E, L_F, IDT, IDT, L_I, IDT, IDT, L_L, IDT, L_N, IDT, // 6
    L_P, IDT, L_R, L_S, L_T, L_U, L_V, L_W, IDT, L_Y, IDT, BEO, PIP, BEC, TLD, ERR, // 7
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 8
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 9
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // A
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // B
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // F
];

const ___: ByteHandler = None;

const ERR: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = UnexpectedToken;
});

const EOF: ByteHandler = Some(|lex| {
    lex.asi = Asi::ImplicitSemicolon;

    lex.token = EndOfProgram;
});

// ;
const SEM: ByteHandler = Some(|lex| {
    lex.bump();

    lex.asi = Asi::ExplicitSemicolon;

    lex.token = Semicolon;
});

// :
const COL: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = Colon;
});

// ,
const COM: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = Comma;
});

// (
const PNO: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = ParenOpen;
});

// )
const PNC: ByteHandler = Some(|lex| {
    lex.bump();

    lex.asi = Asi::ImplicitSemicolon;

    lex.token = ParenClose;
});

// [
const BTO: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = BracketOpen;
});

// ]
const BTC: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = BracketClose;
});

// {
const BEO: ByteHandler = Some(|lex| {
    lex.bump();

    lex.token = BraceOpen;
});

// }
const BEC: ByteHandler = Some(|lex| {
    lex.bump();

    lex.asi = Asi::ImplicitSemicolon;

    lex.token = BraceClose;
});

// =
const EQL: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'=' => match lex.next_byte() {
            b'=' => {
                lex.bump();
                StrictEquality
            },
            _ => Equality,
        },
        b'>' => {
            lex.bump();
            FatArrow
        },
        _ => Assign,
    };
});

// !
const EXL: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'=' => match lex.next_byte() {
            b'=' => {
                lex.bump();
                StrictInequality
            }
            _ => Inequality,
        },
        _ => Exclamation,
    };
});

// <
const LSS: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Lesser;
});

// >
const MOR: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Greater;
});

// ?
const QST: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = QuestionMark;
});

// ~
const TLD: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Tilde;
});

// ^
const CRT: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Caret;
});

// &
const AMP: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'&' => {
            lex.bump();
            LogicalAnd
        },
        _ => Ampersand,
    };
});

// |
const PIP: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'|' => {
            lex.bump();
            LogicalOr
        },
        _ => Pipe,
    };
});

// +
const PLS: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'+' => {
            lex.bump();
            Increment
        },
        _ => Plus,
    };
});

// -
const MIN: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'-' => {
            lex.bump();
            Decrement
        },
        _ => Minus,
    };
});

// *
const ATR: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'*' => {
            lex.bump();
            Exponent
        },
        _ => Asterisk,
    };
});

// /
const SLH: ByteHandler = Some(|lex| {
    lex.token = match lex.next_byte() {
        b'/' => {
            // Keep consuming bytes until new line or end of source
            unwind_loop!({
                match lex.next_byte() {
                    0 | b'\n' => {
                        lex.bump();
                        return lex.token = Comment(CommentType::SingleLine);
                    },
                    _ => {}
                }
            });
        },
        b'*' => {
            lex.bump();
            // Keep consuming bytes until */ happens in a row
            unwind_loop!({
                match lex.read_byte() {
                    b'*' => match lex.next_byte() {
                        b'/' => {
                            lex.bump();
                            return lex.token = Comment(CommentType::MultiLine);
                        }
                        0 => return lex.token = UnexpectedEndOfProgram,
                        _ => {}
                    },
                    0 => return lex.token = UnexpectedEndOfProgram,
                    _ => lex.bump(),
                }
            });
        },
        _ => ForwardSlash,
    };
});

// %
const PRC: ByteHandler = Some(|lex| {
    lex.bump();
    lex.token = Remainder;
});

// Unicode character
const UNI: ByteHandler = Some(|lex| {
    let start = lex.index;

    // TODO: unicodes with different lengths
    let first = lex
        .slice_source(start, start + 4)
        .chars()
        .next()
        .expect("Has to have one");

    if !first.is_alphanumeric() {
        return lex.token = UnexpectedToken;
    }

    // `read_label` bumps one at the beginning,
    // so we subtract it here.
    lex.index += first.len_utf8() - 1;

    lex.read_label();

    lex.token = Identifier;
});

// 0
const ZER: ByteHandler = Some(|lex| {
    match lex.next_byte() {
        b'b' | b'B' => {
            lex.bump();

            return lex.read_binary();
        }

        b'o' | b'O' => {
            lex.bump();

            return lex.read_octal();
        }

        b'x' | b'X' => {
            lex.bump();

            return lex.read_hexadec();
        }

        _ => {}
    }

    unwind_loop!({
        match lex.read_byte() {
            b'0'..=b'9' => {
                lex.read_decimal();
            }
            b'.' => {
                lex.bump();

                return lex.read_float();
            }
            b'e' | b'E' => {
                lex.bump();

                return lex.read_scientific();
            }
            _ => {
                return lex.token = Literal(LiteralType::Number);
            },
        }
    });
});

// 1 to 9
const DIG: ByteHandler = Some(|lex| {
    unwind_loop!({
        match lex.read_byte() {
            b'0'..=b'9' => {
                lex.read_decimal();
            }
            b'.' => {
                lex.bump();

                return lex.read_float();
            }
            b'e' | b'E' => {
                lex.bump();

                return lex.read_scientific();
            }
            _ => {
                return lex.token = Literal(LiteralType::Number);
            },
        }
    });
});

// .
const PRD: ByteHandler = Some(|lex| {
    match lex.next_byte() {
        b'0'..=b'9' => {
            lex.bump();

            lex.read_float()
        },
        b'.' => {
            lex.token = match lex.next_byte() {
                b'.' => {
                    lex.bump();
                    return lex.token = RestSpread;
                },
                _ => UnexpectedToken,
            }
        },
        _ => lex.token = Dot,
    };
});

// " or '
const QOT: ByteHandler = Some(|lex| {
    let style = lex.read_byte();

    lex.bump();

    unwind_loop!({
        match lex.read_byte() {
            ch if ch == style => {
                lex.bump();
                return lex.token = Literal(LiteralType::String);
            },
            b'\\' => {
                lex.bump();
                expect_byte!(lex);
            },
            0 => {
                return lex.token = UnexpectedEndOfProgram;
            },
            _ => lex.bump(),
        }
    });
});

// `
const TPL: ByteHandler = Some(|lex| {
    lex.bump();
    lex.read_template_kind();
});

pub struct Lexer<'arena> {
    /// Current `Token` from the source.
    pub token: Token,

    /// Flags whether or not a new line was read before the token
    asi: Asi,

    /// Source to parse, must be a C-style buffer ending with 0 byte
    ptr: *const u8,

    /// Current index
    index: usize,

    /// Position of current token in source
    token_start: usize,

    accessor_start: usize,

    pub quasi: &'arena str,
}

impl<'arena> Lexer<'arena> {
    /// Create a new `Lexer` from source using an existing arena.
    #[inline]
    pub fn new(arena: &'arena Arena, source: &str) -> Self {
        unsafe { Lexer::from_ptr(arena.alloc_str_with_nul(source)) }
    }

    /// Create a new `Lexer` from a raw pointer to byte string.
    ///
    /// **The source must be null terminated!**
    /// Passing a pointer that is not null terminated is undefined behavior!
    ///
    /// **The source must be valid UTF8!**
    /// Passing a pointer to data that is not valid UTF8 will lead
    /// to bugs or undefined behavior.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const u8) -> Self {
        let mut lexer = Lexer {
            token: UnexpectedToken,
            asi: Asi::NoSemicolon,
            ptr,
            index: 0,
            token_start: 0,
            accessor_start: 0,
            quasi: "",
        };

        lexer.consume();

        lexer
    }

    /// Advances the lexer, produces a new `Token` and stores it on `self.token`.
    #[inline]
    pub fn consume(&mut self) {
        self.asi = Asi::NoSemicolon;

        let mut ch;

        unwind_loop!({
            ch = self.read_byte();

            if let Some(handler) = self.handler_from_byte(ch) {
                self.token_start = self.index;
                return handler(self);
            }

            self.bump();

            if ch == b'\n' {
                self.asi = Asi::ImplicitSemicolon;
            }
        })
    }

    /// Create an `&str` slice from source spanning current token.
    #[inline]
    pub fn token_as_str(&self) -> &'arena str {
        let start = self.token_start;
        self.slice_from(start)
    }

    /// Specialized version of `token_as_str` that crates an `&str`
    /// slice for the identifier following an accessor (`.`).
    #[inline]
    pub fn accessor_as_str(&self) -> &'arena str {
        let start = self.accessor_start;
        self.slice_from(start)
    }

    #[inline]
    fn handler_from_byte(&mut self, byte: u8) -> ByteHandler {
        unsafe { *(&BYTE_HANDLERS as *const ByteHandler).offset(byte as isize) }
    }

    /// Get the start and end positions of the current token.
    #[inline]
    pub fn loc(&self) -> (u32, u32) {
        (self.start(), self.end())
    }

    /// Get the start position of the current token.
    #[inline]
    pub fn start(&self) -> u32 {
        self.token_start as u32
    }

    /// Get the end position of the current token.
    #[inline]
    pub fn end(&self) -> u32 {
        self.index as u32
    }

    /// Get the start position of the current token, then advance the lexer.
    #[inline]
    pub fn start_then_consume(&mut self) -> u32 {
        let start = self.start();
        self.consume();
        start
    }

    /// Get the end position of the current token, then advance the lexer.
    #[inline]
    pub fn end_then_consume(&mut self) -> u32 {
        let end = self.end();
        self.consume();
        end
    }

    /// On top of being called when the opening backtick (`) of a template
    /// literal occurs, this method needs to be used by the parser while
    /// parsing a complex template string expression.
    ///
    /// **Note:** Parser needs to expect a BraceClose token before calling
    /// this method to ensure that the tokenizer state is not corrupted.
    #[inline]
    pub fn read_template_kind(&mut self) {
        let start = self.index;

        loop {
            match self.read_byte() {
                b'`' => {
                    let end = self.index;

                    self.bump();
                    self.quasi = self.slice_source(start, end);
                    self.token = TemplateClosed;

                    return;
                }
                b'$' => {
                    let end = self.index;

                    self.bump();

                    match self.read_byte() {
                        b'{' => self.bump(),
                        _ => continue,
                    }

                    self.quasi = self.slice_source(start, end);
                    self.token = TemplateOpen;
                    return;
                }
                b'\\' => {
                    self.bump();

                    match self.read_byte() {
                        0 => {
                            self.token = UnexpectedEndOfProgram;
                            return;
                        }
                        _ => self.bump(),
                    }
                }
                _ => self.bump(),
            }
        }
    }

    /// Get a definition of which ASI rules can be applied.
    #[inline]
    pub fn asi(&self) -> Asi {
        self.asi
    }

    pub fn invalid_token(&mut self) -> Error {
        let start = self.token_start;
        let end = self.index;
        let token = self.token;

        if token != EndOfProgram {
            self.consume();
        }

        Error {
            token,
            start,
            end,
            raw: self.slice_source(start, end).to_owned().into_boxed_str(),
        }
    }

    /// Read a byte from the source. Note that this does not increment
    /// the index. In few cases (all of them related to number parsing)
    /// we want to peek at the byte before doing anything. This will,
    /// very very rarely, lead to a situation where the same byte is read
    /// twice, but since this operation is using a raw pointer, the cost
    /// is virtually irrelevant.
    #[inline]
    fn read_byte(&self) -> u8 {
        unsafe { *self.ptr.add(self.index) }
    }

    /// Manually increment the index. Calling `read_byte` and then `bump`
    /// is equivalent to consuming a byte on an iterator.
    #[inline]
    fn bump(&mut self) {
        self.index += 1;
    }

    #[inline]
    fn next_byte(&mut self) -> u8 {
        self.bump();
        self.read_byte()
    }

    /// This is a specialized method that expects the next token to be an identifier,
    /// even if it would otherwise be a keyword.
    ///
    /// This is useful when parsing member expressions such as `foo.function`, where
    /// `function` is actually allowed as a regular identifier, not a keyword.
    ///
    /// The perf gain here comes mainly from avoiding having to first match the `&str`
    /// to a keyword token, and then match that token back to a `&str`.
    #[inline]
    pub fn read_accessor(&mut self) {
        // Look up table that marks which ASCII characters are allowed to start an ident
        const AL: bool = true; // alphabet
        const DO: bool = true; // dollar sign $
        const US: bool = true; // underscore
        const BS: bool = true; // backslash
        const __: bool = false;

        static TABLE: [bool; 128] = [
            // 0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
            __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 0
            __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 1
            __, __, __, __, DO, __, __, __, __, __, __, __, __, __, __, __, // 2
            __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
            __, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, // 4
            AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, __, BS, __, __, US, // 5
            __, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, // 6
            AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, AL, __, __, __, __, __, // 7
        ];

        let mut ch;

        unwind_loop!({
            ch = self.read_byte();

            if ch > 0x20 {
                self.accessor_start = self.index;

                if ch > 127 {
                    unimplemented!();
                // return unicode(self)
                } else if TABLE[ch as usize] {
                    self.read_label();
                } else {
                    return self.token = UnexpectedToken;
                }
            }

            self.bump();
        })
    }

    #[inline]
    fn read_label(&mut self) {
        while util::legal_in_label(self.read_byte()) {
            self.bump();
        }
    }

    #[inline]
    fn slice_from(&self, start: usize) -> &'arena str {
        let end = self.index;
        self.slice_source(start, end)
    }

    #[inline]
    fn slice_source(&self, start: usize, end: usize) -> &'arena str {
        use std::slice::from_raw_parts;
        use std::str::from_utf8_unchecked;

        unsafe { from_utf8_unchecked(from_raw_parts(self.ptr.add(start), end - start)) }
    }

    #[inline]
    fn read_binary(&mut self) {
        match self.read_byte() {
            b'0'..=b'1' => {
                self.bump();
            },
            _ => {
                self.token = UnexpectedToken;
                return;
            },
        }

        loop {
            match self.read_byte() {
                b'0'..=b'1' => {
                    self.bump();
                },
                b'_' => {
                    self.bump();
                    return self.read_binary();
                },
                _ => {
                    break;
                },
            }
        }

        self.token = Literal(LiteralType::Number);
    }

    #[inline]
    fn read_octal(&mut self) {
        match self.read_byte() {
            b'0'..=b'7' => {
                self.bump();
            },
            _ => {
                self.token = UnexpectedToken;
                return;
            },
        }
        loop {
            match self.read_byte() {
                b'0'..=b'7' => {
                    self.bump();
                },
                b'_' => {
                    self.bump();
                    return self.read_octal();
                },
                _ => {
                    break;
                },
            }
        }

        self.token = Literal(LiteralType::Number);
    }

    #[inline]
    fn read_decimal(&mut self) {
        match self.read_byte() {
            b'0'..=b'9' => {
                self.bump();
            },
            _ => {
                self.token = UnexpectedToken;
                return;
            },
        }
        loop {
            match self.read_byte() {
                b'0'..=b'9' => {
                    self.bump();
                },
                b'_' => {
                    self.bump();
                    return self.read_decimal();
                },
                _ => {
                    break;
                },
            }
        }

        self.token = Literal(LiteralType::Number);
    }

    #[inline]
    fn read_hexadec(&mut self) {
        match self.read_byte() {
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                self.bump();
            },
            _ => {
                self.token = UnexpectedToken;
                return;
            },
        }
        loop {
            match self.read_byte() {
                b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                    self.bump();
                },
                b'_' => {
                    self.bump();
                    return self.read_binary();
                },
                _ => {
                    break;
                },
            }
        }

        self.token = Literal(LiteralType::Number);
    }

    #[inline]
    fn read_float(&mut self) {
        match self.read_byte() {
            b'0'..=b'9' => {
                self.bump();
            },
            _ => {
                return;
            },
        }
        loop {
            match self.read_byte() {
                b'0'..=b'9' => {
                    self.bump();
                },
                b'_' => {
                    self.bump();
                    return self.read_decimal();
                },
                _ => {
                    break;
                },
            }
        }

        self.token = Literal(LiteralType::Number);
    }

    #[inline]
    fn read_scientific(&mut self) {
        match self.read_byte() {
            b'-' | b'+' => self.bump(),
            _ => {}
        }

        self.read_decimal()
    }

    #[inline]
    pub fn read_regular_expression(&mut self) -> &'arena str {
        let start = self.index - 1;
        let mut in_class = false;
        loop {
            match self.read_byte() {
                b'[' => {
                    self.bump();
                    in_class = true;
                }
                b']' => {
                    self.bump();
                    in_class = false;
                }
                b'/' => {
                    self.bump();
                    if !in_class {
                        break;
                    }
                }
                b'\\' => match self.next_byte() {
                    0 => {
                        self.token = UnexpectedEndOfProgram;
                        return "";
                    }
                    _ => self.bump(),
                },
                b'\n' => {
                    self.bump();
                    self.token = UnexpectedToken;
                    return "";
                }
                _ => self.bump(),
            }
        }

        loop {
            match self.read_byte() {
                b'g' | b'i' | b'm' | b'u' | b'y' => {
                    self.bump();
                }
                _ => {
                    break;
                }
            }
        }

        self.token = Literal(LiteralType::RegEx);
        self.slice_from(start)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_lex<T>(source: &str, tokens: T)
    where
        T: AsRef<[(Token, &'static str)]>,
    {
        let arena = Arena::new();
        let mut lex = Lexer::new(&arena, source);

        for &(ref token, slice) in tokens.as_ref() {
            assert_eq!(lex.token, *token);
            assert_eq!(lex.token_as_str(), slice);
            lex.consume();
        }

        assert_eq!(lex.token, EndOfProgram);
    }

    #[test]
    fn empty_lexer() {
        assert_lex("   ", []);
    }

    #[test]
    fn line_comment() {
        assert_lex(" // foo", [(Comment(CommentType::SingleLine), "// foo")]);
    }

    #[test]
    fn block_comment() {
        assert_lex(" /* foo */ bar", [(Comment(CommentType::MultiLine), "/* foo */"), (Identifier, "bar")]);
        assert_lex(" /** foo **/ bar", [(Comment(CommentType::MultiLine), "/** foo **/"), (Identifier, "bar")]);
        assert_lex(" /*abc foo **/ bar", [(Comment(CommentType::MultiLine), "/*abc foo **/"), (Identifier, "bar")]);
    }

    #[test]
    fn method_call() {
        assert_lex(
            "foo.bar();",
            [
                (Identifier, "foo"),
                (Dot, "."),
                (Identifier, "bar"),
                (ParenOpen, "("),
                (ParenClose, ")"),
                (Semicolon, ";"),
            ],
        );
    }

    #[test]
    fn method_call_with_keyword() {
        assert_lex(
            "foo.function();",
            [
                (Identifier, "foo"),
                (Dot, "."),
                (Identifier, "function"),
                (ParenOpen, "("),
                (ParenClose, ")"),
                (Semicolon, ";"),
            ],
        );
    }

    #[test]
    fn simple_math() {
        assert_lex(
            "let foo = 2 + 2;",
            [
                (Keyword(KeywordName::Let), "let"),
                (Identifier, "foo"),
                (Assign, "="),
                (Literal(LiteralType::Number), "2"),
                (Plus, "+"),
                (Literal(LiteralType::Number), "2"),
                (Semicolon, ";"),
            ],
        );
    }

    #[test]
    fn variable_declaration() {
        assert_lex(
            "var x, y, z = 42;",
            [
                (Keyword(KeywordName::Var), "var"),
                (Identifier, "x"),
                (Comma, ","),
                (Identifier, "y"),
                (Comma, ","),
                (Identifier, "z"),
                (Assign, "="),
                (Literal(LiteralType::Number), "42"),
                (Semicolon, ";"),
            ],
        );
    }

    #[test]
    fn function_statement() {
        assert_lex(
            "function foo(bar) { return bar }",
            [
                (Keyword(KeywordName::Function), "function"),
                (Identifier, "foo"),
                (ParenOpen, "("),
                (Identifier, "bar"),
                (ParenClose, ")"),
                (BraceOpen, "{"),
                (Keyword(KeywordName::Return), "return"),
                (Identifier, "bar"),
                (BraceClose, "}"),
            ],
        );
    }

    #[test]
    fn unexpected_token() {
        assert_lex("..", [(UnexpectedToken, "..")]);
    }

    #[test]
    fn unexpected_end() {
        assert_lex("'foo", [(UnexpectedEndOfProgram, "'foo")]);
    }

    #[test]
    fn keywords() {
        assert_lex(
            "
                break case class const debugger default delete do else
                export extends false finally for function if implements
                import in instanceof interface let new null package
                protected public return static super switch this throw
                true try undefined typeof var void while with yield
            ",
            &[
                (Keyword(KeywordName::Break), "break"),
                (Keyword(KeywordName::Case), "case"),
                (Keyword(KeywordName::Class), "class"),
                (Keyword(KeywordName::Const), "const"),
                (Keyword(KeywordName::Debugger), "debugger"),
                (Keyword(KeywordName::Default), "default"),
                (Delete, "delete"),
                (Keyword(KeywordName::Do), "do"),
                (Keyword(KeywordName::Else), "else"),
                (Keyword(KeywordName::Export), "export"),
                (Keyword(KeywordName::Extends), "extends"),
                (Literal(LiteralType::False), "false"),
                (Keyword(KeywordName::Finally), "finally"),
                (Keyword(KeywordName::For), "for"),
                (Keyword(KeywordName::Function), "function"),
                (Keyword(KeywordName::If), "if"),
                (Keyword(KeywordName::Implements), "implements"),
                (Keyword(KeywordName::Import), "import"),
                (In, "in"),
                (Instanceof, "instanceof"),
                (Keyword(KeywordName::Interface), "interface"),
                (Keyword(KeywordName::Let), "let"),
                (New, "new"),
                (Literal(LiteralType::Null), "null"),
                (Keyword(KeywordName::Package), "package"),
                (Keyword(KeywordName::Protected), "protected"),
                (Keyword(KeywordName::Public), "public"),
                (Keyword(KeywordName::Return), "return"),
                (Keyword(KeywordName::Static), "static"),
                (Keyword(KeywordName::Super), "super"),
                (Keyword(KeywordName::Switch), "switch"),
                (Keyword(KeywordName::This), "this"),
                (Keyword(KeywordName::Throw), "throw"),
                (Literal(LiteralType::True), "true"),
                (Keyword(KeywordName::Try), "try"),
                (Literal(LiteralType::Undefined), "undefined"),
                (Typeof, "typeof"),
                (Keyword(KeywordName::Var), "var"),
                (Void, "void"),
                (Keyword(KeywordName::While), "while"),
                (Keyword(KeywordName::With), "with"),
                (Keyword(KeywordName::Yield), "yield"),
            ][..],
        );
    }

    #[test]
    fn operators() {
        assert_lex(
            "
                => new ++ -- ! ~ typeof void delete * / % ** + - << >>
                >>> < <= > >= instanceof in === !== == != & ^ | && ||
                ? = += -= **= *= /= %= <<= >>= >>>= &= ^= |= ...
            ",
            &[
                (FatArrow, "=>"),
                (New, "new"),
                (Increment, "++"),
                (Decrement, "--"),
                (Exclamation, "!"),
                (Tilde, "~"),
                (Typeof, "typeof"),
                (Void, "void"),
                (Delete, "delete"),
                (Asterisk, "*"),
                (ForwardSlash, "/"),
                (Remainder, "%"),
                (Exponent, "**"),
                (Plus, "+"),
                (Minus, "-"),
                // <<
                (Lesser, "<"),
                (Lesser, "<"),
                // >>
                (Greater, ">"),
                (Greater, ">"),
                // >>>
                (Greater, ">"),
                (Greater, ">"),
                (Greater, ">"),
                (Lesser, "<"),
                // <=
                (Lesser, "<"),
                (Assign, "="),
                (Greater, ">"),
                //  >=
                (Greater, ">"),
                (Assign, "="),
                (Instanceof, "instanceof"),
                (In, "in"),
                (StrictEquality, "==="),
                (StrictInequality, "!=="),
                (Equality, "=="),
                (Inequality, "!="),
                (Ampersand, "&"),
                (Caret, "^"),
                (Pipe, "|"),
                (LogicalAnd, "&&"),
                (LogicalOr, "||"),
                (QuestionMark, "?"),
                (Assign, "="),
                // +=
                (Plus, "+"),
                (Assign, "="),
                // -=
                (Minus, "-="),
                (Assign, "="),
                // **=
                (Exponent, "**"),
                (Assign, "="),
                // *=
                (Asterisk, "*"),
                (Assign, "="),
                // /=
                (ForwardSlash, "/"),
                (Assign, "="),
                // %=
                (Remainder, "%"),
                (Assign, "="),
                // <<=
                (Lesser, "<"),
                (Lesser, "<"),
                (Assign, "="),
                // >>=
                (Greater, ">"),
                (Greater, ">"),
                (Assign, "="),
                // >>>=
                (Greater, ">"),
                (Greater, ">"),
                (Greater, ">"),
                (Assign, "="),
                // &=
                (Ampersand, "&"),
                (Assign, "="),
                // ^=
                (Caret, "^"),
                (Assign, "="),
                // |=
                (Pipe, "|"),
                (Assign, "="),
                (RestSpread, "..."),
            ][..],
        );
    }

    #[test]
    fn type_assertions() {
        assert_lex(
            "const foo: number = 2 + 2;",
            &[
                (Keyword(KeywordName::Const), "const"),
                (Identifier, "foo"),
                (Colon, ":"),
                (Type(TypeName::Number), "number"),
                (Assign, "="),
                (Literal(LiteralType::Number), "2"),
                (Plus, "+"),
                (Literal(LiteralType::Number), "2"),
                (Semicolon, ";"),
            ][..],
        );
    }

    #[test]
    fn typed_function() {
        assert_lex(
            "function isFoo(bar: string): boolean { return bar }",
            &[
                (Keyword(KeywordName::Function), "function"),
                (Identifier, "isFoo"),
                (ParenOpen, "("),
                (Identifier, "bar"),
                (Colon, ":"),
                (Type(TypeName::String), "string"),
                (ParenClose, ")"),
                (Colon, ":"),
                (Type(TypeName::Boolean), "boolean"),
                (BraceOpen, "{"),
                (Return, "return"),
                (Identifier, "bar"),
                (BraceClose, "}"),
            ][..],
        );
    }

    #[test]
    fn async_function() {
        assert_lex(
            "async function isFoo(bar) { await bar(); return; }",
            &[
                (Keyword(KeywordName::Async), "async"),
                (Keyword(KeywordName::Function), "function"),
                (Identifier, "isFoo"),
                (ParenOpen, "("),
                (Identifier, "bar"),
                (ParenClose, ")"),
                (BraceOpen, "{"),
                (Keyword(KeywordName::Await), "await"),
                (Identifier, "bar"),
                (ParenOpen, "("),
                (ParenClose, ")"),
                (Semicolon, ";"),
                (Keyword(KeywordName::Return), "return"),
                (Semicolon, ";"),
                (BraceClose, "}"),
            ][..],
        );
    }

    #[test]
    fn type_argument() {
        assert_lex(
            "function isFoo<T>(bar: T): T { return bar; }",
            &[
                (Keyword(KeywordName::Function), "function"),
                (Identifier, "isFoo"),
                (Lesser, "<"),
                (Identifier, "T"),
                (Greater, ">"),
                (ParenOpen, "("),
                (Identifier, "bar"),
                (Colon, ":"),
                (Identifier, "T"),
                (ParenClose, ")"),
                (Colon, ":"),
                (Identifier, "T"),
                (BraceOpen, "{"),
                (Keyword(KeywordName::Return), "return"),
                (Identifier, "bar"),
                (Semicolon, ";"),
                (BraceClose, "}"),
            ][..],
        );
    }
}
