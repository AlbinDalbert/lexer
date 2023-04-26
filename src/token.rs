pub struct Token {
    kind: TokenKind,
    len: u32,
}

pub enum TokenKind {
    Atomic { kind: Atomic },
    Compound { kind: Compound },
}

pub enum Compound {
    Cluster,
    ClusterEnd,

    Enum,
    EnumEnd,

    Struct,
    StructEnd,

    Function,
    FunctionEnd,
}

pub enum Atomic {
    LineComment, // #
    WhiteSpace,
    Literal { kind: LiteralKind },
    // seperation characters
    SemiCol, // ;
    Comma,   // ,
    Dot,     // .

    OpenParen,  // (
    CloseParen, // )

    OpenBrace,  // {
    CloseBrace, // }

    OpenBracket,  // [
    CloseBracket, // ]

    And, // &
    Or,  // |

    // oporation characters
    Equal,     // =
    Bang,      // !
    LowThen,   // <
    GreatThen, // >
    Subtract,  // -
    Add,       // +
    Star,      // *
    Slash,     // /

    Unknown, // Unknown token kind, error

    Eof, // end of file
}

pub enum LiteralKind {
    Int,
    Float,
    Char,
}
