#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiteralType {
    True,
    False,
    Null,
    Undefined,
    String,
    Number,
    RegEx,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeName {
    Number,
    String,
    Boolean,
    Any,
    Never,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordName {
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    Finally,
    For,
    Function,
    Get,
    If,
    Implements,
    Import,
    Interface,
    Let,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Set,
    Static,
    Super,
    Switch,
    This,
    Throw,
    Try,
    Var,
    While,
    With,
    Yield,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommentType {
    SingleLine,
    MultiLine,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    EndOfProgram,
    Identifier,
    Comment(CommentType),
    Literal(LiteralType),
    Type(TypeName),
    Keyword(KeywordName),
    Semicolon,        //      ;
    Colon,            //      :
    Comma,            //      ,
    QuestionMark,     //      ?
    ParenOpen,        //      (
    ParenClose,       //      )
    BracketOpen,      //      [
    BracketClose,     //      ]
    BraceOpen,        //      {
    BraceClose,       //      }
    Lesser,           //      <
    Greater,          //      >
    FatArrow,         //      =>
    New,              //      new
    Increment,        //      ++
    Decrement,        //      --
    Exclamation,      //      !
    Tilde,            //      ~
    Typeof,           //      typeof
    Void,             //      void
    Delete,           //      delete
    Asterisk,         //      *
    ForwardSlash,     //      /
    Remainder,        //      %
    Exponent,         //      **
    Plus,             //      +
    Minus,            //      -
    Instanceof,       //      instanceof
    In,               //      in
    Of,               //      of
    As,               //      as
    StrictEquality,   //      ===
    StrictInequality, //      !==
    Equality,         //      ==
    Inequality,       //      !=
    Ampersand,        //      &
    Caret,            //      ^
    Pipe,             //      |
    LogicalAnd,       //      &&
    LogicalOr,        //      ||
    Assign,           //      =
    Dot,              //      .
    RestSpread,       //      ...
    At,               //      @
    Hash,             //      #
    TemplateOpen,     //      ` … ` or ` … ${
    TemplateClosed,   //       … `
    UnexpectedToken,
    UnexpectedEndOfProgram,
}

impl Token {
    #[inline]
    pub fn is_word(&self) -> bool {
        use self::Token::*;

        match self {
            Identifier | New | Typeof | Void | Delete | Instanceof | In | Of | As => true,
            Keyword(kw) => true,
            Type(t) => true,
            _ => false,
        }
    }
}
