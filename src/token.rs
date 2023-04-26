pub enum Token {
    TopLevel(TopLevel),
    Word(String),
    Number(i64),
    Punctuation(char),
}

pub enum TopLevel {
    Cluster,
    Enum,
    Struct,
    Function,
}

pub enum TokenKind {}
