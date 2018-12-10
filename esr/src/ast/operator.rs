use crate::lexer::Token;
use crate::lexer::Token::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorKind {
    FatArrow,         //   …  => …
    New,              //     new …
    Increment,        //      ++ … | … ++
    Decrement,        //      -- … | … --
    LogicalNot,       //       ! …
    BitwiseNot,       //       ~ …
    Typeof,           //  typeof …
    Void,             //    void …
    Delete,           //  delete …
    Multiplication,   //   …  *  …
    Division,         //   …  /  …
    Remainder,        //   …  %  …
    Exponent,         //   …  ** …
    Addition,         //   …  +  … | + …
    Subtraction,      //   …  -  … | - …
    BitShiftLeft,     //   …  << …
    BitShiftRight,    //   …  >> …
    UBitShiftRight,   //   … >>> …
    Lesser,           //   …  <  …
    LesserEquals,     //   …  <= …
    Greater,          //   …  >  …
    GreaterEquals,    //   …  >= …
    Instanceof,       //   … instanceof …
    In,               //   …  in …
    StrictEquality,   //   … === …
    StrictInequality, //   … !== …
    Equality,         //   …  == …
    Inequality,       //   …  != …
    BitwiseAnd,       //   …  &  …
    BitwiseXor,       //   …  ^  …
    BitwiseOr,        //   …  |  …
    LogicalAnd,       //   …  && …
    LogicalOr,        //   …  || …
    Conditional,      //   …  ?  …  :  …
    Assign,           //   …  =  …
    AddAssign,        //   …  += …
    SubtractAssign,   //   …  -= …
    ExponentAssign,   //   … **= …
    MultiplyAssign,   //   …  *= …
    DivideAssign,     //   …  /= …
    RemainderAssign,  //   …  %= …
    BSLAssign,        //   … <<= …
    BSRAssign,        //   … >>= …
    UBSRAssign,       //   … >>>= …
    BitAndAssign,     //   …  &= …
    BitXorAssign,     //   …  ^= …
    BitOrAssign,      //   …  |= …
    Spread,           //     ... …
}


#[derive(Debug, PartialEq, Clone)]
pub enum OperatorCategory {
    Minus,
    Plus,
    Safe,
    Word,
}

use self::OperatorKind::*;

impl OperatorKind {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            FatArrow         => "=>",
            New              => "new",
            Increment        => "++",
            Decrement        => "--",
            LogicalNot       => "!",
            BitwiseNot       => "~",
            Typeof           => "typeof",
            Void             => "void",
            Delete           => "delete",
            Multiplication   => "*",
            Division         => "/",
            Remainder        => "%",
            Exponent         => "**",
            Addition         => "+",
            Subtraction      => "-",
            BitShiftLeft     => "<<",
            BitShiftRight    => ">>",
            UBitShiftRight   => ">>>",
            Lesser           => "<",
            LesserEquals     => "<=",
            Greater          => ">",
            GreaterEquals    => ">=",
            Instanceof       => "instanceof",
            In               => "in",
            StrictEquality   => "===",
            StrictInequality => "!==",
            Equality         => "==",
            Inequality       => "!=",
            BitwiseAnd       => "&",
            BitwiseXor       => "^",
            BitwiseOr        => "|",
            LogicalAnd       => "&&",
            LogicalOr        => "||",
            Conditional      => "?",
            Assign           => "=",
            AddAssign        => "+=",
            SubtractAssign   => "-=",
            ExponentAssign   => "**=",
            MultiplyAssign   => "*=",
            DivideAssign     => "/=",
            RemainderAssign  => "%=",
            BSLAssign        => "<<=",
            BSRAssign        => ">>=",
            UBSRAssign       => ">>>=",
            BitAndAssign     => "&=",
            BitXorAssign     => "^=",
            BitOrAssign      => "|=",
            Spread           => "...",
        }
    }

    /// According to the Operator Precedence Table
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#Table
    #[inline]
    pub fn binding_power(&self) -> u8 {
        match self {
            FatArrow         => 18,

            New              => 17,

            Increment        |
            Decrement        => 16,

            LogicalNot       |
            BitwiseNot       |
            Typeof           |
            Void             |
            Delete           => 15,

            Multiplication   |
            Division         |
            Remainder        |
            Exponent         => 14,

            Addition         |
            Subtraction      => 13,

            BitShiftLeft     |
            BitShiftRight    |
            UBitShiftRight   => 12,

            Lesser           |
            LesserEquals     |
            Greater          |
            GreaterEquals    |
            Instanceof       |
            In               => 11,

            StrictEquality   |
            StrictInequality |
            Equality         |
            Inequality       => 10,

            BitwiseAnd       => 9,
            BitwiseXor       => 8,
            BitwiseOr        => 7,
            LogicalAnd       => 6,
            LogicalOr        => 5,
            Conditional      => 4,

            Assign           |
            AddAssign        |
            SubtractAssign   |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => 3,

            Spread           => 1,
        }
    }

    #[inline]
    pub fn prefix(&self) -> bool {
        match self {
            LogicalNot       |
            BitwiseNot       |
            Typeof           |
            Void             |
            Delete           |
            New              |
            Spread           |
            Increment        |
            Decrement        |
            Addition         |
            Subtraction      => true,

            _                => false
        }
    }

    #[inline]
    pub fn infix(&self) -> bool {
        match self {
            FatArrow         |
            Multiplication   |
            Division         |
            Remainder        |
            Exponent         |
            StrictEquality   |
            StrictInequality |
            Equality         |
            Inequality       |
            Lesser           |
            LesserEquals     |
            Greater          |
            GreaterEquals    |
            Instanceof       |
            In               |
            BitShiftLeft     |
            BitShiftRight    |
            UBitShiftRight   |
            BitwiseAnd       |
            BitwiseXor       |
            BitwiseOr        |
            LogicalAnd       |
            LogicalOr        |
            Conditional      |
            Addition         |
            Subtraction      |
            Assign           |
            AddAssign        |
            SubtractAssign   |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => true,

            _                => false
        }
    }

    #[inline]
    pub fn assignment(&self) -> bool {
        match self {
            Assign           |
            AddAssign        |
            SubtractAssign   |
            ExponentAssign   |
            MultiplyAssign   |
            DivideAssign     |
            RemainderAssign  |
            BSLAssign        |
            BSRAssign        |
            UBSRAssign       |
            BitAndAssign     |
            BitXorAssign     |
            BitOrAssign      => true,

            _                => false
        }
    }

    #[inline]
    pub fn category(&self) -> OperatorCategory {
        match self {
            In            |
            New           |
            Typeof        |
            Void          |
            Delete        |
            Instanceof    => OperatorCategory::Word,
            Addition      |
            Increment     => OperatorCategory::Plus,
            Subtraction   |
            Decrement     => OperatorCategory::Minus,

            _             => OperatorCategory::Safe,
        }
    }
}
