#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    /* --- Identifiers & Literals --- */
    Identifier, // e.g., 's', 'y', 'U_f'...
    IntLiteral,
    BitsLiteral, // e.g., 0b10
    StringLiteral, //e.g., "0+"

    /* --- Keywords --- */
    Bits,
    Const,
    Qubits,
    Function,
    Oracle,
    Loads,
    Circuit,
    Register,
    Apply,
    // Measure, // measure should be an identifier, but it has some special behaviour

    /* --- Symbols --- */
    Equals,
    Semicolon,
    Colon,
    Comma,
    Underscore,
    Period,
    Arrow, // i.e., "->"
    LParen, RParen,
    LBrace, RBrace,
    LSqBracket, RSqBracket,
    
    /* --- Arithmetic Symbols --- */
    Caret,
    Ampersand,
    Pipe,
    Star,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    line: usize,
    col: usize
}

/// CONSTRUCTORS AND PROJECTORS
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, col: usize) -> Self {
        Self { token_type, lexeme, line, col }
    }

    pub fn line(&self) -> usize { self.line }

    pub fn col(&self) -> usize { self.col }
}