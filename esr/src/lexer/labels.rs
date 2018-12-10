use crate::lexer::token::{KeywordName, TypeName, LiteralType};
use crate::lexer::token::Token::*;
use crate::lexer::{util, ByteHandler};

macro_rules! match_label {
    ($lex:ident [$( $byte:expr )* => $token:expr]) => {
        if $(
            $lex.next_byte() == $byte &&
        )* {$lex.bump(); !util::legal_in_label($lex.read_byte())} {
            return $lex.token = $token;
        }
    };

    ($lex:ident { [=> $token:expr] $( $match:tt $cont:tt )+ }) => {
        match $lex.next_byte() {
            $(
                $match => match_label!($lex $cont),
            )*
            ch if !util::legal_in_label(ch) => return $lex.token = $token,
            _ => {}
        }
    };

    ($lex:ident { $match:tt $cont:tt }) => {
        if $lex.next_byte() == $match {
            match_label!($lex $cont)
        }
    };

    ($lex:ident { $( $match:tt $cont:tt )+ }) => {
        match $lex.next_byte() {
            $(
                $match => match_label!($lex $cont),
            )*
            _ => {}
        }
    }
}

// Non-keyword Identifier: starting with a letter, _ or $
pub const IDT: ByteHandler = Some(|lex| {
    lex.bump();
    match lex.token {
        Dot => lex.read_accessor(),
        _ => lex.read_label(),
    }
    return lex.token = Identifier;
});

// Identifier or keyword starting with a letter `a`
pub const L_A: ByteHandler = Some(|lex| {
    match_label!(lex {
        b's'[b'y' b'n' b'c' => Keyword(KeywordName::Async)]
        b'w'[b'a' b'i' b't' => Keyword(KeywordName::Await)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `b`
pub const L_B: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'r'[b'e' b'a' b'k' => Keyword(KeywordName::Break)]
        b'o'[b'o' b'l' b'e' b'a' b'n' => Type(TypeName::Boolean)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `c`
pub const L_C: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'o'{
            b'n'{
                b's'[b't'                => Keyword(KeywordName::Const)]
                b't'[b'i' b'n' b'u' b'e' => Keyword(KeywordName::Continue)]
            }
        }
        b'a'{
            b's'[b'e'       => Keyword(KeywordName::Case)]
            b't'[b'c' b'h'  => Keyword(KeywordName::Catch)]
        }
        b'l'[b'a' b's' b's' => Keyword(KeywordName::Class)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `d`
pub const L_D: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'o'[                             => Keyword(KeywordName::Do)]
        b'e'{
            b'l'[b'e' b't' b'e'           => Delete]
            b'f'[b'a' b'u' b'l' b't'      => Keyword(KeywordName::Default)]
            b'b'[b'u' b'g' b'g' b'e' b'r' => Keyword(KeywordName::Debugger)]
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `e`
pub const L_E: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'l'[b's' b'e'               => Keyword(KeywordName::Else)]
        b'x'{
            b'p'[b'o' b'r' b't'      => Keyword(KeywordName::Export)]
            b't'[b'e' b'n' b'd' b's' => Keyword(KeywordName::Extends)]
        }
        b'n'[b'u' b'm'               => Keyword(KeywordName::Enum)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `f`
pub const L_F: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'i'[b'n' b'a' b'l' b'l' b'y'      => Keyword(KeywordName::Finally)]
        b'o'[b'r'                          => Keyword(KeywordName::For)]
        b'u'[b'n' b'c' b't' b'i' b'o' b'n' => Keyword(KeywordName::Function)]
        b'a'[b'l' b's' b'e'                => Literal(LiteralType::False)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `i`
pub const L_I: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'n'{
            [                                       => In]
            b's'[b't' b'a' b'n' b'c' b'e' b'o' b'f' => Instanceof]
            b't'[b'e' b'r' b'f' b'a' b'c' b'e'      => Keyword(KeywordName::Interface)]
        }
        b'f'[                                       => Keyword(KeywordName::If)]
        b'm'{
            b'p'{
                b'o'[b'r' b't'                      => Keyword(KeywordName::Import)]
                b'l'[b'e' b'm' b'e' b'n' b't' b's'  => Keyword(KeywordName::Implements)]
            }
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `l`
pub const L_L: ByteHandler = Some(|lex| {
    match_label!(lex [b'e' b't' => Keyword(KeywordName::Let)]);

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `n`
pub const L_N: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'e'[b'w'                => New]
        b'u'{
            b'l'[ b'l'          => Literal(LiteralType::Null)]
            b'm'[b'b' b'e' b'r' => Type(TypeName::Number)]
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `p`
pub const L_P: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'a'[b'c' b'k' b'a' b'g' b'e'          => Keyword(KeywordName::Package)]
        b'u'[b'b' b'l' b'i' b'c'               => Keyword(KeywordName::Public)]
        b'r'{
            b'o'[b't' b'e' b'c' b't' b'e' b'd' => Keyword(KeywordName::Protected)]
            b'i'[b'v' b'a' b't' b'e'           => Keyword(KeywordName::Private)]
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `r`
pub const L_R: ByteHandler = Some(|lex| {
    match_label!(lex [b'e' b't' b'u' b'r' b'n' => Keyword(KeywordName::Return)]);

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `s`
pub const L_S: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'u'[b'p' b'e' b'r'      => Keyword(KeywordName::Super)]
        b'w'[b'i' b't' b'c' b'h' => Keyword(KeywordName::Switch)]
        b't'{
            b'a'[b't' b'i' b'c' => Keyword(KeywordName::Static)]
            b'r'[b'i' b'n' b'g' => Type(TypeName::String)]
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `t`
pub const L_T: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'y'[b'p' b'e' b'o' b'f' => Typeof]
        b'h'{
            b'i'[b's'            => Keyword(KeywordName::This)]
            b'r'[b'o' b'w'       => Keyword(KeywordName::Throw)]
        }
        b'r'{
            b'y'[                => Keyword(KeywordName::Try)]
            b'u'[b'e'            => Literal(LiteralType::True)]
        }
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `u`
pub const L_U: ByteHandler = Some(|lex| {
    match_label!(lex [b'n' b'd' b'e' b'f' b'i' b'n' b'e' b'd' => Literal(LiteralType::Undefined)]);

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `v`
pub const L_V: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'a'[b'r'      => Keyword(KeywordName::Var)]
        b'o'[b'i' b'd' => Void]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `w`
pub const L_W: ByteHandler = Some(|lex| {
    match_label!(lex {
        b'h'[b'i' b'l' b'e' => Keyword(KeywordName::While)]
        b'i'[b't' b'h'      => Keyword(KeywordName::With)]
    });

    lex.read_label();
    lex.token = Identifier;
});

// Identifier or keyword starting with a letter `y`
pub const L_Y: ByteHandler = Some(|lex| {
    match_label!(lex [b'i' b'e' b'l' b'd' => Keyword(KeywordName::Yield)]);

    lex.read_label();
    lex.token = Identifier;
});
