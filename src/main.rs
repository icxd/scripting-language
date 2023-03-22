use std::collections::HashMap;
use colored::*;
#[derive(Debug, Clone, PartialEq)] struct TokenLocation {
    start: usize,
    end: usize
}
#[derive(Debug, Clone, PartialEq)] enum Error {
    SyntaxError(String, TokenLocation),
    TypeError(String, TokenLocation),
    RuntimeError(String, TokenLocation),
}
impl Error {
    pub fn to_string(&self, filename: String, contents: String) -> String {
        format!("[{}:{}:{}] {}: {}",
                filename,
                self.get_line_number_from_index(contents.clone()),
                self.get_column_from_index(contents.clone()),
                self.name().red(),
                self.message()).red().to_string()
    }
    pub fn location(&self) -> TokenLocation {
        match self {
            Error::SyntaxError(_, location) => location.clone(),
            Error::TypeError(_, location) => location.clone(),
            Error::RuntimeError(_, location) => location.clone()
        }
    }
    pub fn message(&self) -> String {
        match self {
            Error::SyntaxError(message, _) => message.clone(),
            Error::TypeError(message, _) => message.clone(),
            Error::RuntimeError(message, _) => message.clone()
        }
    }
    pub fn name(&self) -> String {
        match self {
            Error::SyntaxError(_, _) => "SyntaxError".to_string(),
            Error::TypeError(_, _) => "TypeError".to_string(),
            Error::RuntimeError(_, _) => "RuntimeError".to_string()
        }
    }
    pub fn get_line_number_from_index(&self, contents: String) -> usize {
        let mut line_number: usize = 1;
        for (index, character) in contents.chars().enumerate() {
            if index == self.location().start {
                break;
            }
            if character == '\n' {
                line_number += 1;
            }
        }
        line_number
    }
    pub fn get_column_from_index(&self, contents: String) -> usize {
        let mut column: usize = 1;
        for (index, character) in contents.chars().enumerate() {
            if index == self.location().start {
                break;
            }
            if character == '\n' {
                column = 1;
            } else {
                column += 1;
            }
        }
        column
    }
}
#[derive(Debug, Clone, PartialEq)] enum TokenKind {
    // Literals
    Identifier,
    StringLit,
    CharLit,
    NumberLit,

    // Keywords
    Annotation,
    Struct,
    End,
    Enum,
    External,
    Inline,
    Func,
    Type,
    Var,
    Return,
    Import,
    As,
    SizeOf,
    New,
    True,
    False,
    Null,
    If,
    Else,
    While,
    For,
    In,
    Switch,
    Case,
    Break,
    Continue,
    Default,

    // Types
    Int,
    Usize,
    String,
    CString,
    Char,
    Bool,
    Void,

    // Type Qualifiers
    Volatile,
    Const,
    Restrict,

    // Operators and Punctuation
    Colon,
    Comma,
    Dot,
    At,
    Pipe,
    Ampersand,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,
    FatArrow,
    Range, // ..

    // Special
    Newline,
    Error,
    EndOfFile,
}
#[derive(Debug, Clone)] struct Token {
    kind: TokenKind,
    value: String,
    location: TokenLocation,
}
impl Token {
    pub fn location(&mut self) -> TokenLocation {
        self.location.clone()
    }
}
#[derive(Debug, Clone)] struct Lexer {
    contents: String,
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<Error>,
}
impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            tokens: vec![],
            current: 0,
            errors: vec![]
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        while self.current < self.contents.len() {
            match self.current() {
                '\t' | ' ' | '\r' => self.advance(),
                '\n' => {
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::Newline, value: "\n".to_string(), location: TokenLocation { start: self.current, end: self.current } });
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut value: String = String::new();
                    let start: usize = self.current;
                    while self.current < self.contents.len() && self.current().is_alphanumeric() || self.current() == '_' {
                        value.push_str(self.current().to_string().as_str());
                        self.advance();
                    }
                    let kind: TokenKind = match value.as_str() {
                        "annotation" => TokenKind::Annotation,
                        "struct" => TokenKind::Struct,
                        "enum" => TokenKind::Enum,
                        "end" => TokenKind::End,
                        "external" => TokenKind::External,
                        "inline" => TokenKind::Inline,
                        "func" => TokenKind::Func,
                        "type" => TokenKind::Type,
                        "var" => TokenKind::Var,
                        "return" => TokenKind::Return,
                        "import" => TokenKind::Import,
                        "as" => TokenKind::As,
                        "sizeof" => TokenKind::SizeOf,
                        "new" => TokenKind::New,
                        "true" => TokenKind::True,
                        "false" => TokenKind::False,
                        "null" => TokenKind::Null,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "while" => TokenKind::While,
                        "for" => TokenKind::For,
                        "in" => TokenKind::In,
                        "switch" => TokenKind::Switch,
                        "case" => TokenKind::Case,
                        "break" => TokenKind::Break,
                        "continue" => TokenKind::Continue,
                        "default" => TokenKind::Default,
                        "int" => TokenKind::Int,
                        "usize" => TokenKind::Usize,
                        "string" => TokenKind::String,
                        "cstring" => TokenKind::CString,
                        "char" => TokenKind::Char,
                        "bool" => TokenKind::Bool,
                        "void" => TokenKind::Void,
                        "volatile" => TokenKind::Volatile,
                        "const" => TokenKind::Const,
                        "restrict" => TokenKind::Restrict,
                        _ => TokenKind::Identifier
                    };
                    self.tokens.push(Token { kind, value, location: TokenLocation { start, end: self.current } });
                }
                '"' => {
                    let mut value: String = String::new();
                    let start: usize = self.current;
                    self.advance();
                    while self.current < self.contents.len() && self.current() != '"' {
                        let val: char = self.current();
                        match val {
                            '\\' => {
                                self.advance();
                                match self.current() {
                                    'n' => value.push_str("\\n"),
                                    't' => value.push_str("\\t"),
                                    'r' => value.push_str("\\r"),
                                    '0' => value.push_str("\\0"),
                                    '\'' => value.push_str("'"),
                                    '"' => value.push_str("\\\""),
                                    '\\' => value.push_str("\\\\"),
                                    _ => {
                                        self.errors.push(Error::SyntaxError("Invalid escape sequence".to_string(), TokenLocation { start: self.current, end: self.current }));
                                        self.advance();
                                    }
                                }
                            }
                            _ => value.push_str(val.to_string().as_str())
                        }
                        self.advance();
                    }
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::StringLit, value, location: TokenLocation { start, end: self.current } })
                }
                '\'' => {
                    let mut value: String = String::new();
                    let start: usize = self.current;
                    self.advance();
                    while self.current < self.contents.len() && self.current() != '\'' {
                        let val: char = self.current();
                        match val {
                            '\\' => {
                                self.advance();
                                match self.current() {
                                    'n' => value.push_str("\\n"),
                                    't' => value.push_str("\\t"),
                                    'r' => value.push_str("\\r"),
                                    '0' => value.push_str("\\0"),
                                    '\'' => value.push_str("'"),
                                    '"' => value.push_str("\\\""),
                                    '\\' => value.push_str("\\\\"),
                                    _ => {
                                        self.errors.push(Error::SyntaxError("Invalid escape sequence".to_string(), TokenLocation { start: self.current, end: self.current }));
                                        self.advance();
                                    }
                                }
                            }
                            _ => value.push_str(val.to_string().as_str())
                        }
                        self.advance();
                    }
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::CharLit, value, location: TokenLocation { start, end: self.current } })
                }
                '0'..='9' => {
                    let mut value: String = String::new();
                    let start: usize = self.current;
                    while self.current < self.contents.len() && self.current().is_numeric() {
                        value.push_str(self.current().to_string().as_str());
                        self.advance();
                    }
                    self.tokens.push(Token { kind: TokenKind::NumberLit, value, location: TokenLocation { start, end: self.current } });
                }
                ':' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::Colon, value: ":".to_string(), location: TokenLocation { start, end: self.current } });
                }
                ',' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::Comma, value: ",".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '.' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current() == '.' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::Range, value: "..".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Dot, value: ".".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '@' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::At, value: "@".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '|' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::Pipe, value: "|".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '&' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::Ampersand, value: "&".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '(' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::OpenParen, value: "(".to_string(), location: TokenLocation { start, end: self.current } });
                }
                ')' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::CloseParen, value: ")".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '[' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::OpenBracket, value: "[".to_string(), location: TokenLocation { start, end: self.current } });
                }
                ']' => {
                    let start: usize = self.current;
                    self.advance();
                    self.tokens.push(Token { kind: TokenKind::CloseBracket, value: "]".to_string(), location: TokenLocation { start, end: self.current } });
                }
                '=' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '>' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::FatArrow, value: "=>".to_string(), location: TokenLocation { start, end: self.current } });
                    } else if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::EqualEqual, value: "==".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Equal, value: "=".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '!' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::BangEqual, value: "!=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Bang, value: "!".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '<' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::LessEqual, value: "<=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Less, value: "<".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '>' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::GreaterEqual, value: ">=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Greater, value: ">".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '+' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::PlusEqual, value: "+=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Plus, value: "+".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '-' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::MinusEqual, value: "-=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Minus, value: "-".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '*' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::StarEqual, value: "*=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Star, value: "*".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '/' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current() == '/' {
                        self.advance();
                        while self.current < self.contents.len() && self.current() != '\n' {
                            self.advance();
                        }
                        self.advance();
                    } else if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::SlashEqual, value: "/=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Slash, value: "/".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                '%' => {
                    let start: usize = self.current;
                    self.advance();
                    if self.current < self.contents.len() && self.current() == '=' {
                        self.advance();
                        self.tokens.push(Token { kind: TokenKind::PercentEqual, value: "%=".to_string(), location: TokenLocation { start, end: self.current } });
                    } else {
                        self.tokens.push(Token { kind: TokenKind::Percent, value: "%".to_string(), location: TokenLocation { start, end: self.current } });
                    }
                }
                _ => {
                    self.errors.push(Error::SyntaxError(format!("Unexpected character: {}", self.clone().current()), TokenLocation { start: self.current, end: self.current + 1 }));
                    self.advance();
                }
            }
        }
        self.tokens.clone()
    }
    fn current(&mut self) -> char {
        if self.current >= self.contents.len() {
            return '\0';
        }
        self.contents.chars().nth(self.current).unwrap()
    }
    fn advance(&mut self) {
        self.current += 1
    }
}
#[derive(Debug, Clone)] enum Statement {
    Generic(Box<Statement>, Vec<(String, Option<Type>)>, TokenLocation),
    Annotated(Box<Statement>, Vec<Annotation>, TokenLocation),
    Annotation(String, Vec<(String, Type)>, TokenLocation),
    Struct(String, Vec<(String, Type)>, TokenLocation),
    Enum(String, Type, Vec<(String, Expression, TokenLocation)>, TokenLocation),
    TypeAlias(String, Vec<Type>, TokenLocation),
    Function(String, Vec<(String, Type)>, Type, Vec<Statement>, TokenLocation),
    StructFunction(String, String, Vec<(String, Type)>, Type, Vec<Statement>, TokenLocation),
    Variable(String, Type, Expression, TokenLocation),
    Constant(String, Type, Expression, TokenLocation),
    Return(Expression, TokenLocation),
    While(Expression, Vec<Statement>, TokenLocation),
    If(Expression, Vec<Statement>, Vec<Statement>, TokenLocation),
    External(Box<Statement>, TokenLocation),
    Inline(Box<Statement>, TokenLocation),
    Import(String, TokenLocation),
    Expression(Expression, TokenLocation),
}
impl Statement {
    pub fn location(&self) -> TokenLocation {
        match self {
            Statement::Generic(_, _, location) => location.clone(),
            Statement::Annotated(_, _, location) => location.clone(),
            Statement::Annotation(_, _, location) => location.clone(),
            Statement::Struct(_, _, location) => location.clone(),
            Statement::Enum(_, _, _, location) => location.clone(),
            Statement::TypeAlias(_, _, location) => location.clone(),
            Statement::Function(_, _, _, _, location) => location.clone(),
            Statement::StructFunction(_, _, _, _, _, location) => location.clone(),
            Statement::Variable(_, _, _, location) => location.clone(),
            Statement::Constant(_, _, _, location) => location.clone(),
            Statement::Return(_, location) => location.clone(),
            Statement::While(_, _, location) => location.clone(),
            Statement::If(_, _, _, location) => location.clone(),
            Statement::External(_, location) => location.clone(),
            Statement::Inline(_, location) => location.clone(),
            Statement::Import(_, location) => location.clone(),
            Statement::Expression(_, location) => location.clone(),
        }
    }
}
#[derive(Debug, Clone)] struct Annotation {
    name: String,
    arguments: Vec<Expression>,
    location: TokenLocation,
}
#[derive(Debug, Clone, PartialEq)] enum Expression {
    Number(i64, TokenLocation),
    String(String, TokenLocation),
    Char(String, TokenLocation),
    Boolean(bool, TokenLocation),
    Identifier(String, TokenLocation),
    Null,
    Call(String, Vec<Expression>, TokenLocation),
    GenericCall(String, Vec<Type>, Vec<Expression>, TokenLocation),
    Member(Box<Expression>, Box<Expression>, TokenLocation),
    NamedArgument(String, Box<Expression>, TokenLocation),
    Cast(Box<Expression>, Type, TokenLocation),
    SizeOf(Type, TokenLocation),
    Index(Box<Expression>, Box<Expression>, TokenLocation),
    Array(Vec<Expression>, TokenLocation),
    New(String, Vec<Expression>, TokenLocation),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>, TokenLocation),
    Assignment(Box<Expression>, Box<Expression>, TokenLocation),
    Binary(TokenKind, Box<Expression>, Box<Expression>, TokenLocation),
    Unary(TokenKind, Box<Expression>, TokenLocation),
    Grouping(Box<Expression>, TokenLocation),
    AddressOf(Box<Expression>, TokenLocation),
    Dereference(Box<Expression>, TokenLocation),
    Range(Box<Expression>, Box<Expression>, TokenLocation),

    Error(Error),
    Empty,
}
impl Expression {
    pub fn location(&self) -> TokenLocation {
        match self {
            Expression::Number(_, location) => location.clone(),
            Expression::String(_, location) => location.clone(),
            Expression::Char(_, location) => location.clone(),
            Expression::Boolean(_, location) => location.clone(),
            Expression::Identifier(_, location) => location.clone(),
            Expression::Null => TokenLocation { start: 0, end: 0 },
            Expression::Call(_, _, location) => location.clone(),
            Expression::GenericCall(_, _, _, location) => location.clone(),
            Expression::Member(_, _, location) => location.clone(),
            Expression::NamedArgument(_, _, location) => location.clone(),
            Expression::Cast(_, _, location) => location.clone(),
            Expression::SizeOf(_, location) => location.clone(),
            Expression::Index(_, _, location) => location.clone(),
            Expression::Array(_, location) => location.clone(),
            Expression::New(_, _, location) => location.clone(),
            Expression::Ternary(_, _, _, location) => location.clone(),
            Expression::Assignment(_, _, location) => location.clone(),
            Expression::Grouping(_, location) => location.clone(),
            Expression::Binary(_, _, _, location) => location.clone(),
            Expression::Unary(_, _, location) => location.clone(),
            Expression::AddressOf(_, location) => location.clone(),
            Expression::Dereference(_, location) => location.clone(),
            Expression::Range(_, _, location) => location.clone(),

            Expression::Error(error) => match error {
                Error::SyntaxError(_, location) => location.clone(),
                Error::TypeError(_, location) => location.clone(),
                Error::RuntimeError(_, location) => location.clone(),
            },
            Expression::Empty => TokenLocation { start: 0, end: 0 },
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Type {
    Int(TokenLocation),
    Usize(TokenLocation),
    String(TokenLocation),
    CString(TokenLocation),
    Char(TokenLocation),
    Bool(TokenLocation),
    Void(TokenLocation),

    Struct(String, TokenLocation),
    Enum(String, TokenLocation),
    Function(Vec<Type>, Box<Type>, TokenLocation),

    Pointer(Box<Type>, TokenLocation),
    Array(Box<Type>, Box<Expression>, TokenLocation),
    DynamicArray(Box<Type>, TokenLocation),
    
    Volatile(Box<Type>, TokenLocation),
    Const(Box<Type>, TokenLocation),
    Restrict(Box<Type>, TokenLocation),

    GenericType(String, TokenLocation),

    Unknown(String, TokenLocation),
    Error(Error, TokenLocation),
}
impl Type {
    pub fn location(&self) -> TokenLocation {
        match self {
            Type::Int(location) => location.clone(),
            Type::Usize(location) => location.clone(),
            Type::String(location) => location.clone(),
            Type::CString(location) => location.clone(),
            Type::Char(location) => location.clone(),
            Type::Bool(location) => location.clone(),
            Type::Void(location) => location.clone(),
            Type::Struct(_, location) => location.clone(),
            Type::Enum(_, location) => location.clone(),
            Type::Function(_, _, location) => location.clone(),
            Type::Pointer(_, location) => location.clone(),
            Type::Array(_, _, location) => location.clone(),
            Type::DynamicArray(_, location) => location.clone(),
            Type::Volatile(_, location) => location.clone(),
            Type::Const(_, location) => location.clone(),
            Type::Restrict(_, location) => location.clone(),
            Type::GenericType(_, location) => location.clone(),
            Type::Unknown(_, location) => location.clone(),
            Type::Error(_, location) => location.clone(),
        }
    }
}
#[derive(Debug, Clone)] struct Parser {
    tokens: Vec<Token>,
    statements: Vec<Statement>,
    current: usize,
    errors: Vec<Error>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            statements: vec![],
            current: 0,
            errors: vec![],
        }
    }
    pub fn parse(&mut self) -> Vec<Statement> {
        while self.current < self.tokens.len() {
            if self.current().kind == TokenKind::Newline {
                self.advance();
                continue;
            }
            let statement: Statement = self.parse_statement();
            self.statements.push(statement);
        }
        self.statements.clone()
    }
    fn parse_statement(&mut self) -> Statement {
        match self.current().kind.clone() {
            TokenKind::Annotation => self.parse_annotation(),
            TokenKind::At => self.parse_annotated(),
            TokenKind::External => self.parse_external(),
            TokenKind::Inline => self.parse_inline(),
            TokenKind::Struct => self.parse_struct(),
            TokenKind::Enum => self.parse_enum(),
            TokenKind::Type => self.parse_type_alias(),
            TokenKind::Func => self.parse_function(),
            TokenKind::Var => self.parse_variable(),
            TokenKind::Const => self.parse_constant(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Import => self.parse_import(),
            TokenKind::While => self.parse_while(),
            TokenKind::If => self.parse_if(),
            _ => Statement::Expression(self.parse_expression(), self.current().location().clone()),
        }
    }
    fn parse_annotation(&mut self) -> Statement {
        self.expect(TokenKind::Annotation);
        let name_location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        if self.current().kind == TokenKind::End {
            self.advance();
            return Statement::Annotation(name, vec![], self.current().location().clone());
        } else {
            self.expect(TokenKind::Newline);
        }
        let mut fields: Vec<(String, Type)> = vec![];
        while self.current().kind != TokenKind::End {
            let name: String = self.expect(TokenKind::Identifier).value;
            self.expect(TokenKind::Colon);
            let type_: Type = self.parse_type();
            fields.push((name, type_));
            self.expect(TokenKind::Newline);
        }
        self.expect(TokenKind::End);
        Statement::Annotation(name, fields, name_location)
    }
    fn parse_annotated(&mut self) -> Statement {
        let mut annotations: Vec<Annotation> = vec![];
        while self.current().kind == TokenKind::At {
            self.expect(TokenKind::At);
            let name_location: TokenLocation = self.current().location().clone();
            let name: String = self.expect(TokenKind::Identifier).value;
            let mut arguments: Vec<Expression> = vec![];
            if self.current().kind == TokenKind::OpenParen {
                self.expect(TokenKind::OpenParen);
                while self.current().kind != TokenKind::CloseParen {
                    arguments.push(self.parse_expression());
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseParen);
            }
            self.expect(TokenKind::Newline);
            annotations.push(Annotation { name, arguments, location: name_location });
        }
        let statement: Statement = self.parse_statement();
        Statement::Annotated(Box::new(statement.clone()), annotations, statement.location().clone())
    }
    fn parse_external(&mut self) -> Statement {
        let location: TokenLocation = self.current().location().clone();
        self.expect(TokenKind::External);
        let statement: Statement = self.parse_statement();
        Statement::External(Box::new(statement), location)
    }
    fn parse_inline(&mut self) -> Statement {
        let location: TokenLocation = self.current().location().clone();
        self.expect(TokenKind::Inline);
        let statement: Statement = self.parse_statement();
        Statement::Inline(Box::new(statement), location)
    }
    fn parse_struct(&mut self) -> Statement {
        self.expect(TokenKind::Struct);
        let location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        if self.current().kind == TokenKind::End {
            self.advance();
            return Statement::Struct(name, vec![], location);
        } else {
            self.expect(TokenKind::Newline);
        }
        let mut fields: Vec<(String, Type)> = vec![];
        while self.current().kind != TokenKind::End {
            if self.current().kind == TokenKind::Newline {
                self.advance();
                continue;
            }
            let field_name: String = self.expect(TokenKind::Identifier).value;
            self.expect(TokenKind::Colon);
            let field_type: Type = self.parse_type();
            self.expect(TokenKind::Newline);
            fields.push((field_name, field_type));
        }
        self.expect(TokenKind::End);
        Statement::Struct(name, fields, location)
    }
    fn parse_enum(&mut self) -> Statement {
        self.expect(TokenKind::Enum);
        let location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        self.expect(TokenKind::Colon);
        let enum_type: Type = self.parse_type();
        self.expect(TokenKind::Newline);
        let mut variants: Vec<(String, Expression, TokenLocation)> = vec![];
        while self.current().kind != TokenKind::End {
            if self.current().kind == TokenKind::Newline {
                self.advance();
                continue;
            }
            let value_location: TokenLocation = self.current().location().clone();
            let variant_name: String = self.expect(TokenKind::Identifier).value;
            self.expect(TokenKind::Equal);
            let variant_value: Expression = self.parse_expression();
            self.expect(TokenKind::Newline);
            variants.push((variant_name, variant_value, value_location));
        }
        self.expect(TokenKind::End);
        Statement::Enum(name, enum_type, variants, location)
    }
    fn parse_type_alias(&mut self) -> Statement {
        self.expect(TokenKind::Type);
        let location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        self.expect(TokenKind::Equal);
        let mut types: Vec<Type> = vec![];
        while self.current().kind != TokenKind::Newline {
            types.push(self.parse_type());
            if self.current().kind == TokenKind::Pipe {
                self.expect(TokenKind::Pipe);
            }
        }
        Statement::TypeAlias(name, types, location)
    }
    fn parse_function(&mut self) -> Statement {
        self.expect(TokenKind::Func);
        let location: TokenLocation = self.current().location().clone();
        let mut name: String = self.expect(TokenKind::Identifier).value;
        let mut struct_name: String = "".to_string();
        if self.current().kind == TokenKind::Dot {
            struct_name = name;
            self.expect(TokenKind::Dot);
            name = self.expect(TokenKind::Identifier).value;
        }
        let mut type_parameters: Vec<(String, Option<Type>)> = vec![];
        if self.current().kind == TokenKind::OpenBracket {
            self.expect(TokenKind::OpenBracket);
            while self.current().kind != TokenKind::CloseBracket {
                let type_parameter_name: String = self.expect(TokenKind::Identifier).value;
                let mut type_parameter_type: Option<Type> = None;
                if self.current().kind == TokenKind::Colon {
                    self.expect(TokenKind::Colon);
                    type_parameter_type = Some(self.parse_type());
                }
                type_parameters.push((type_parameter_name, type_parameter_type));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseBracket);
        }
        self.expect(TokenKind::OpenParen);
        let mut args: Vec<(String, Type)> = vec![];
        while self.current().kind != TokenKind::CloseParen {
            let arg_name: String = self.expect(TokenKind::Identifier).value;
            self.expect(TokenKind::Colon);
            let arg_type: Type = self.parse_type();
            args.push((arg_name, arg_type));
            if self.current().kind == TokenKind::Comma {
                self.expect(TokenKind::Comma);
            }
        }
        self.expect(TokenKind::CloseParen);
        let mut return_type: Type = Type::Void(self.current().location().clone());
        if self.current().kind == TokenKind::Colon {
            self.expect(TokenKind::Colon);
            return_type = self.parse_type();
        }
        let mut body: Vec<Statement> = vec![];
        if self.current().kind == TokenKind::FatArrow {
            self.expect(TokenKind::FatArrow);
            let expression: Expression = self.parse_expression();
            body.push(Statement::Return(expression.clone(), expression.location()));
            self.expect(TokenKind::Newline);
        } else {
            while self.current().kind != TokenKind::End {
                if self.current().kind == TokenKind::Newline {
                    self.expect(TokenKind::Newline);
                    continue;
                }
                let statement: Statement = self.parse_statement();
                body.push(statement);
            }
            self.expect(TokenKind::End);
        }
        let mut statement: Statement = Statement::Function(name.clone(), args.clone(), return_type.clone(), body.clone(), location.clone());
        if struct_name != "" {
            statement = Statement::StructFunction(struct_name, name, args, return_type, body, location.clone());
        }
        if type_parameters.len() > 0 {
            statement = Statement::Generic(Box::new(statement), type_parameters, location);
        }
        statement
    }
    fn parse_variable(&mut self) -> Statement {
        self.expect(TokenKind::Var);
        let location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        let mut t: Type = Type::Unknown("".to_string(), self.current().location().clone());
        if self.current().kind == TokenKind::Colon {
            self.expect(TokenKind::Colon);
            t = self.parse_type();
        }
        let mut value: Expression = Expression::Empty;
        if self.current().kind == TokenKind::Equal {
            self.expect(TokenKind::Equal);
            value = self.parse_expression();
        }
        self.expect(TokenKind::Newline);
        Statement::Variable(name, t, value, location)
    }
    fn parse_constant(&mut self) -> Statement {
        self.expect(TokenKind::Const);
        let location: TokenLocation = self.current().location().clone();
        let name: String = self.expect(TokenKind::Identifier).value;
        self.expect(TokenKind::Colon);
        let t: Type = self.parse_type();
        self.expect(TokenKind::Equal);
        let value: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        Statement::Constant(name, t, value, location)
    }
    fn parse_return(&mut self) -> Statement {
        self.expect(TokenKind::Return);
        let value: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        Statement::Return(value.clone(), value.location().clone())
    }
    fn parse_import(&mut self) -> Statement {
        let location: TokenLocation = self.current().location().clone();
        self.expect(TokenKind::Import);
        let path: String = self.expect(TokenKind::StringLit).value;
        self.expect(TokenKind::Newline);
        Statement::Import(path, location)
    }
    fn parse_while(&mut self) -> Statement {
        let location: TokenLocation = self.current().location().clone();
        self.expect(TokenKind::While);
        let condition: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        let mut body: Vec<Statement> = vec![];
        while self.current().kind != TokenKind::End {
            if self.current().kind == TokenKind::Newline {
                self.expect(TokenKind::Newline);
                continue;
            }
            let statement: Statement = self.parse_statement();
            body.push(statement);
        }
        self.expect(TokenKind::End);
        Statement::While(condition, body, location)
    }
    fn parse_if(&mut self) -> Statement {
        let location: TokenLocation = self.current().location().clone();
        self.expect(TokenKind::If);
        let condition: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        let mut body: Vec<Statement> = vec![];
        while self.current().kind != TokenKind::End && self.current().kind != TokenKind::Else {
            if self.current().kind == TokenKind::Newline {
                self.expect(TokenKind::Newline);
                continue;
            }
            let statement: Statement = self.parse_statement();
            body.push(statement);
        }
        let mut else_body: Vec<Statement> = vec![];
        if self.current().kind == TokenKind::Else {
            self.expect(TokenKind::Else);
            if self.current().kind == TokenKind::If {
                let statement: Statement = self.parse_if();
                else_body.push(statement.clone());
                return Statement::If(condition, body, else_body, statement.location().clone());
            }
            self.expect(TokenKind::Newline);
            while self.current().kind != TokenKind::End {
                if self.current().kind == TokenKind::Newline {
                    self.expect(TokenKind::Newline);
                    continue;
                }
                let statement: Statement = self.parse_statement();
                else_body.push(statement);
            }
        }
        self.expect(TokenKind::End);
        Statement::If(condition, body, else_body, location)
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_ternary()
    }
    fn parse_ternary(&mut self) -> Expression {
        let mut expression: Expression = self.parse_assignment();
        if self.current().kind == TokenKind::If {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::If);
            let condition: Expression = self.parse_expression();
            self.expect(TokenKind::Else);
            let else_expression: Expression = self.parse_expression();
            expression = Expression::Ternary(Box::new(condition), Box::new(expression), Box::new(else_expression), location);
        }
        expression
    }
    fn parse_assignment(&mut self) -> Expression {
        let mut expression: Expression = self.parse_comparison();
        if self.current().kind == TokenKind::Equal {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::Equal);
            let right: Expression = self.parse_expression();
            expression = Expression::Assignment(Box::new(expression), Box::new(right), location);
        }
        expression
    }
    fn parse_comparison(&mut self) -> Expression {
        let mut expression: Expression = self.parse_additive();
        while self.current().kind == TokenKind::EqualEqual
            || self.current().kind == TokenKind::BangEqual
            || self.current().kind == TokenKind::Less
            || self.current().kind == TokenKind::LessEqual
            || self.current().kind == TokenKind::Greater
            || self.current().kind == TokenKind::GreaterEqual
        {
            let location: TokenLocation = self.current().location().clone();
            let op: TokenKind = self.current().kind;
            self.expect(op.clone());
            let right: Expression = self.parse_additive();
            expression = Expression::Binary(op, Box::new(expression), Box::new(right), location);
        }
        expression
    }
    fn parse_additive(&mut self) -> Expression {
        let mut expression: Expression = self.parse_multiplicative();
        while self.current().kind == TokenKind::Plus || self.current().kind == TokenKind::Minus {
            let location: TokenLocation = self.current().location().clone();
            let op: TokenKind = self.current().kind;
            self.expect(op.clone());
            let right: Expression = self.parse_multiplicative();
            expression = Expression::Binary(op, Box::new(expression), Box::new(right), location);
        }
        expression
    }
    fn parse_multiplicative(&mut self) -> Expression {
        let mut expression: Expression = self.parse_grouping();
        while self.current().kind == TokenKind::Star || self.current().kind == TokenKind::Slash || self.current().kind == TokenKind::Percent {
            let location: TokenLocation = self.current().location().clone();
            let op: TokenKind = self.current().kind;
            self.expect(op.clone());
            let right: Expression = self.parse_unary();
            expression = Expression::Binary(op, Box::new(expression), Box::new(right), location);
        }
        expression
    }
    fn parse_grouping(&mut self) -> Expression {
        let location: TokenLocation = self.current().location().clone();
        if self.current().kind == TokenKind::OpenParen {
            self.expect(TokenKind::OpenParen);
            let expression: Expression = self.parse_expression();
            self.expect(TokenKind::CloseParen);
            Expression::Grouping(Box::new(expression), location)
        } else {
            self.parse_unary()
        }
    }
    fn parse_unary(&mut self) -> Expression {
        let location: TokenLocation = self.current().location().clone();
        if self.current().kind == TokenKind::Minus {
            self.expect(TokenKind::Minus);
            let expression: Expression = self.parse_unary();
            Expression::Unary(TokenKind::Minus, Box::new(expression), location)
        } else if self.current().kind == TokenKind::Bang {
            self.expect(TokenKind::Bang);
            let expression: Expression = self.parse_unary();
            Expression::Unary(TokenKind::Bang, Box::new(expression), location)
        } else if self.current().kind == TokenKind::Ampersand {
            self.expect(TokenKind::Ampersand);
            let expression: Expression = self.parse_unary();
            Expression::AddressOf(Box::new(expression), location)
        } else if self.current().kind == TokenKind::Star {
            self.expect(TokenKind::Star);
            let expression: Expression = self.parse_unary();
            Expression::Dereference(Box::new(expression), location)
        } else {
            self.parse_index()
        }
    }
    fn parse_index(&mut self) -> Expression {
        let mut expression: Expression = self.parse_member();
        while self.current().kind == TokenKind::OpenBracket {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::OpenBracket);
            let index: Expression = self.parse_expression();
            self.expect(TokenKind::CloseBracket);
            expression = Expression::Index(Box::new(expression), Box::new(index), location);
        }
        expression
    }
    fn parse_member(&mut self) -> Expression {
        let mut expression: Expression = self.parse_cast();
        while self.current().kind == TokenKind::Dot {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::Dot);
            expression = Expression::Member(Box::new(expression), Box::new(self.parse_expression()), location);
        }
        expression
    }
    fn parse_cast(&mut self) -> Expression {
        let mut expression: Expression = self.parse_range();
        while self.current().kind == TokenKind::As {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::As);
            let t: Type = self.parse_type();
            expression = Expression::Cast(Box::new(expression), t, location);
        }
        expression
    }
    fn parse_range(&mut self) -> Expression {
        let mut expression: Expression = self.parse_call();
        while self.current().kind == TokenKind::Range {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::Range);
            expression = Expression::Range(Box::new(expression), Box::new(self.parse_expression()), location);
        }
        expression
    }
    fn parse_call(&mut self) -> Expression {
        let mut expression: Expression = self.parse_primary();
        while self.current().kind == TokenKind::OpenParen || self.current().kind == TokenKind::OpenBracket {
            if self.current().kind == TokenKind::OpenBracket
                    && (self.tokens[self.current + 2].kind == TokenKind::CloseBracket || self.tokens[self.current + 2].kind == TokenKind::Comma)
            {
                // Parse generic function call
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::OpenBracket);
                let mut types: Vec<Type> = vec![];
                while self.current().kind != TokenKind::CloseBracket {
                    let t: Type = self.parse_type();
                    types.push(t);
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseBracket);
                self.expect(TokenKind::OpenParen);
                let mut args: Vec<Expression> = vec![];
                while self.current().kind != TokenKind::CloseParen {
                    let arg: Expression = self.parse_expression();
                    args.push(arg);
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseParen);
                let name = match expression {
                    Expression::Identifier(name, _) => name,
                    _ => {
                        self.errors.push(Error::SyntaxError(format!("Expected identifier, found {:?}", expression), self.clone().current().location()));
                        "".to_string()
                    }
                };
                expression = Expression::GenericCall(name, types, args, location);
            }
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::OpenParen);
            let mut args: Vec<Expression> = vec![];
            while self.current().kind != TokenKind::CloseParen {
                if self.current().kind == TokenKind::Identifier && self.tokens[self.current + 1].kind == TokenKind::Colon {
                    let name_location: TokenLocation = self.current().location().clone();
                    let name: String = self.expect(TokenKind::Identifier).value;
                    self.expect(TokenKind::Colon);
                    let value: Expression = self.parse_expression();
                    args.push(Expression::NamedArgument(name, Box::new(value), name_location));
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                    continue;
                }
                let arg: Expression = self.parse_expression();
                args.push(arg);
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseParen);
            let name = match expression {
                Expression::Identifier(name, _) => name,
                _ => {
                    self.errors.push(Error::SyntaxError(format!("Expected identifier, found {:?}", expression), self.clone().current().location()));
                    "".to_string()
                }
            };
            expression = Expression::Call(name, args, location);
        }
        expression
    }
    fn parse_primary(&mut self) -> Expression {
        match self.current().kind {
            TokenKind::NumberLit => {
                let location: TokenLocation = self.current().location().clone();
                let value: i64 = self.expect(TokenKind::NumberLit).value.parse::<i64>().unwrap();
                Expression::Number(value, location)
            }
            TokenKind::StringLit => {
                let location: TokenLocation = self.current().location().clone();
                let value: String = self.expect(TokenKind::StringLit).value;
                Expression::String(value, location)
            }
            TokenKind::CharLit => {
                let location: TokenLocation = self.current().location().clone();
                let value: String = self.expect(TokenKind::CharLit).value;
                Expression::Char(value, location)
            }
            TokenKind::True => {
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::True);
                Expression::Boolean(true, location)
            }
            TokenKind::False => {
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::False);
                Expression::Boolean(false, location)
            }
            TokenKind::Identifier => {
                let location: TokenLocation = self.current().location().clone();
                let name: String = self.expect(TokenKind::Identifier).value;
                Expression::Identifier(name, location)
            }
            TokenKind::SizeOf => {
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::SizeOf);
                let t: Type = self.parse_type();
                Expression::SizeOf(t, location)
            }
            TokenKind::OpenBracket => {
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::OpenBracket);
                let mut values: Vec<Expression> = vec![];
                while self.current().kind != TokenKind::CloseBracket {
                    let expression: Expression = self.parse_expression();
                    values.push(expression);
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseBracket);
                Expression::Array(values, location)
            }
            TokenKind::New => {
                let location: TokenLocation = self.current().location().clone();
                self.expect(TokenKind::New);
                let identifier: String = self.expect(TokenKind::Identifier).value;
                self.expect(TokenKind::OpenParen);
                let mut args: Vec<Expression> = vec![];
                while self.current().kind != TokenKind::CloseParen {
                    let arg: Expression = self.parse_expression();
                    args.push(arg);
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseParen);
                Expression::New(identifier, args, location)
            }
            TokenKind::Null => {
                self.expect(TokenKind::Null);
                Expression::Null
            }
            _ => Expression::Error(Error::SyntaxError(format!("expected Expression, but got {:?}", self.current().kind), self.clone().current().location()))
        }
    }

    fn parse_type(&mut self) -> Type {
        let location: TokenLocation = self.current().location().clone();
        let t: Type = match self.current().kind {
            TokenKind::Int => {
                self.expect(TokenKind::Int);
                Type::Int(location)
            }
            TokenKind::Usize => {
                self.expect(TokenKind::Usize);
                Type::Usize(location)
            }
            TokenKind::String => {
                self.expect(TokenKind::String);
                Type::String(location)
            }
            TokenKind::CString => {
                self.expect(TokenKind::CString);
                Type::CString(location)
            }
            TokenKind::Char => {
                self.expect(TokenKind::Char);
                Type::Char(location)
            }
            TokenKind::Bool => {
                self.expect(TokenKind::Bool);
                Type::Bool(location)
            }
            TokenKind::Void => {
                self.expect(TokenKind::Void);
                Type::Void(location)
            }
            TokenKind::Identifier => {
                let name: String = self.expect(TokenKind::Identifier).value;
                Type::Unknown(name, location)
            }
            TokenKind::Func => {
                self.expect(TokenKind::Func);
                self.expect(TokenKind::OpenParen);
                let mut args: Vec<Type> = vec![];
                while self.current().kind != TokenKind::CloseParen {
                    let arg: Type = self.parse_type();
                    args.push(arg);
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseParen);
                let location: TokenLocation = self.current().location().clone();
                let mut return_type: Type = Type::Void(location.clone());
                if self.current().kind == TokenKind::Colon {
                    self.expect(TokenKind::Colon);
                    return_type = self.parse_type();
                }
                Type::Function(args, Box::new(return_type), location)
            }
            TokenKind::Volatile => {
                self.expect(TokenKind::Volatile);
                let t: Type = self.parse_type();
                Type::Volatile(Box::new(t), location)
            }
            TokenKind::Const => {
                self.expect(TokenKind::Const);
                let t: Type = self.parse_type();
                Type::Const(Box::new(t), location)
            }
            TokenKind::Restrict => {
                self.expect(TokenKind::Restrict);
                let t: Type = self.parse_type();
                Type::Restrict(Box::new(t), location)
            }
            _ => Type::Error(Error::SyntaxError(format!("expected Type, but got {:?}", self.current().kind), self.clone().current().location()), location)
        };
        if self.current().kind == TokenKind::Star {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::Star);
            Type::Pointer(Box::new(t), location)
        } else if self.current().kind == TokenKind::OpenBracket {
            let location: TokenLocation = self.current().location().clone();
            self.expect(TokenKind::OpenBracket);
            if self.current().kind == TokenKind::CloseBracket {
                self.expect(TokenKind::CloseBracket);
                return Type::DynamicArray(Box::new(t), location);
            }
            let size: Expression = self.parse_expression();
            self.expect(TokenKind::CloseBracket);
            Type::Array(Box::new(t), Box::new(size), location)
        } else {
            t
        }
    }

    fn advance(&mut self) { self.current += 1; }
    fn current(&mut self) -> Token {
        if self.current >= self.tokens.len() {
            return Token {
                kind: TokenKind::EndOfFile,
                value: String::new(),
                location: TokenLocation {
                    start: self.tokens.last().unwrap().location.end,
                    end: self.tokens.last().unwrap().location.end,
                }
            }
        }
        self.tokens.get(self.current).unwrap().clone()
    }
    fn expect(&mut self, kind: TokenKind) -> Token {
        if self.current().kind == TokenKind::EndOfFile {
            return Token {
                kind: TokenKind::Error,
                value: "unexpected end of file".to_string(),
                location: TokenLocation {
                    start: self.tokens.last().unwrap().location.end,
                    end: self.tokens.last().unwrap().location.end,
                }
            };
        }
        if self.current().kind == kind {
            let curr: Token = self.current();
            self.advance();
            return curr;
        }
        Token {
            kind: TokenKind::Error,
            value: format!("expected {:?}, but got {:?}", kind, self.current().kind),
            location: self.current().location,
        }
    }
}
#[derive(Debug, Clone)] struct Codegen {
    statements: Vec<Statement>,
    structs: Vec<String>,
    struct_fields: HashMap<String, Vec<(String, Type)>>,
    struct_functions: HashMap<String, Vec<String>>,
    enums: Vec<String>,
    type_aliases: Vec<String>,
    variable_types: HashMap<String, Type>,
    parameter_types: HashMap<String, Type>,
    annotations: HashMap<String, Vec<(String, Type)>>,
    errors: Vec<Error>,
    generic_types: HashMap<String, Vec<String>>,
    generic_type_names: Vec<String>,
    to_undef: Vec<String>,
}
impl Codegen {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            structs: vec![],
            struct_fields: HashMap::new(),
            struct_functions: HashMap::new(),
            enums: vec![],
            type_aliases: vec![],
            variable_types: HashMap::new(),
            parameter_types: HashMap::new(),
            annotations: HashMap::new(),
            errors: vec![],
            generic_types: HashMap::new(),
            generic_type_names: vec![],
            to_undef: vec![],
        }
    }
    pub fn codegen(&mut self) -> String {
        let mut code: String = String::new();
        for statement in self.clone().statements.iter() {
            let statement_code: String = self.codegen_statement(statement);
            code.push_str(&statement_code);
            for (i, undef) in self.clone().to_undef.iter().enumerate() {
                self.to_undef.remove(i);
                code.push_str(&format!("#undef {}\n", undef));
            }
        }
        code
    }
    fn codegen_statement(&mut self, statement: &Statement) -> String {
        match statement {
            Statement::Generic(statement, type_parameters, _) => self.codegen_generic(statement, type_parameters.clone()),
            Statement::Annotation(name, fields, _) => self.codegen_annotation_statement(name, fields),
            Statement::Annotated(statement, annotations, _) => self.codegen_annotated(statement, annotations),
            Statement::External(statement, _) => self.codegen_external(statement),
            Statement::Inline(statement, _) => self.codegen_inline(statement),
            Statement::Struct(name, fields, _) => self.codegen_struct(name, fields),
            Statement::Enum(name, enum_type, variants, _) => self.codegen_enum(name, enum_type, variants),
            Statement::TypeAlias(name, t, _) => self.codegen_type_alias(name, t),
            Statement::Function(name, args, return_type, body, _) => self.codegen_function(name, args, return_type, body),
            Statement::StructFunction(struct_name, name, args, return_type, body, _) => self.codegen_struct_function(struct_name, name, args, return_type, body),
            Statement::Variable(name, t, value, _) => self.codegen_variable(name, t, value),
            Statement::Constant(name, t, value, _) => self.codegen_constant(name, t, value),
            Statement::Return(value, _) => self.codegen_return(value),
            Statement::Import(path, _) => self.codegen_import(path),
            Statement::While(condition, body, _) => self.codegen_while(condition, body),
            Statement::If(condition, body, else_body, _) => self.codegen_if(condition, body, else_body),
            Statement::Expression(expression, _) => {
                let expression_code: String = self.codegen_expression(expression);
                format!("{};\n", expression_code)
            }
        }
    }
    fn codegen_generic(&mut self, statement: &Statement, type_parameters: Vec<(String, Option<Type>)>) -> String {
        let mut code: String = String::new();
        let mut generic_types: Vec<String> = vec![];
        for (name, t) in type_parameters.iter() {
            generic_types.push(name.clone());
            self.generic_type_names.push(name.clone());
            code.push_str(&format!("#define {}", name));
            if t.is_some() {
                let t: Type = t.clone().unwrap();
                code.push_str(&format!(" {}", self.codegen_type(&t)));
            }
            code.push_str("\n");
        }
        match statement {
            Statement::Function(name, _, _, _, _) => {
                self.generic_types.insert(name.clone(), generic_types);
            }
            _ => {}
        }
        code.push_str(&self.codegen_statement(statement));
        for (name, _) in type_parameters.iter() {
            code.push_str(&format!("#undef {}\n", name));
        }
        code
    }
    fn codegen_annotation_statement(&mut self, name: &String, fields: &Vec<(String, Type)>) -> String {
        self.annotations.insert(name.clone(), fields.clone());
        let mut code: String = String::new();
        code.push_str(format!("#define {}(", name).as_str());
        for (i, (field_name, _)) in fields.iter().enumerate() {
            code.push_str(format!("{}", field_name).as_str());
            if i != fields.len() - 1 {
                code.push_str(", ");
            }
        }
        code.push_str(format!(") __attribute__((annotate(\"{}\")))\n", name).as_str());
        code
    }
    fn codegen_annotated(&mut self, statement: &Statement, annotations: &Vec<Annotation>) -> String {
        let mut code: String = String::new();
        for annotation in annotations.iter() {
            code.push_str(&self.codegen_annotation(&annotation.name, &annotation.arguments, &annotation.location));
        }

        match statement {
            Statement::Struct(name, fields, _) => {
                code.push_str(&self.codegen_struct(name, fields));
                code.pop();
                code.pop();
                for annotation in annotations.iter() {
                    code.push_str(format!(" {}(", annotation.name).as_str());
                    for (i, argument) in annotation.arguments.iter().enumerate() {
                        code.push_str(&self.codegen_expression(argument));
                        if i != annotation.arguments.len() - 1 {
                            code.push_str(", ");
                        }
                    }
                    code.push_str(")");
                }
                code.push_str(";\n");
            }
            _ => self.errors.push(Error::TypeError("cannot annotate this statement".to_string(), statement.location())),
        }
        code
    }
    fn codegen_annotation(&mut self, name: &String, _fields: &Vec<Expression>, location: &TokenLocation) -> String {
        if !self.annotations.contains_key(name) {
            self.errors.push(Error::TypeError(format!("unknown annotation {}", name), location.clone()));
        }
        "".to_string()
    }
    fn codegen_external(&mut self, statement: &Statement) -> String {
        let mut code: String = String::new();
        code.push_str("extern ");
        code.push_str(&self.codegen_statement(statement));
        code
    }
    fn codegen_inline(&mut self, statement: &Statement) -> String {
        let mut code: String = String::new();
        code.push_str("inline ");
        code.push_str(&self.codegen_statement(statement));
        code
    }
    fn codegen_struct(&mut self, name: &String, fields: &Vec<(String, Type)>) -> String {
        self.structs.push(name.clone());
        self.struct_fields.insert(name.clone(), fields.clone());
        self.struct_functions.insert(name.clone(), vec![]);
        let mut constructor: String = String::new();
        // let mut has_constructor: bool = false;
        let mut forward_declarations: String = String::new();
        let mut code: String = String::new();
        if fields.clone().len() == 0 {
            return format!("struct {};\n", name);
        }
        code.push_str(&format!("struct {} {{\n", name));
        let new_fields: Vec<(String, Type)> = fields.clone();
        for (field_name, field_type) in fields.iter() {
            if let Type::Function(args, return_type, _) = field_type {
                code.push_str(&format!("{} (*{})(", self.codegen_type(return_type), field_name));
                for arg_type in args.iter() {
                    code.push_str(&format!("{}, ", self.codegen_type(arg_type)));
                }
                if args.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str(");\n");
                if field_name == "constructor" {
                    // has_constructor = true;
                    constructor.push_str(&format!("static {} __{}_constructor(", self.codegen_type(return_type), name));
                    for (i, arg_type) in args.iter().enumerate() {
                        constructor.push_str(&format!("{} __{}, ", self.codegen_type(arg_type), i));
                    }
                    if args.len() > 0 {
                        constructor.pop();
                        constructor.pop();
                    }
                    constructor.push_str(") {\n");
                    constructor.push_str(&format!("{} self = ({})(malloc(sizeof({})));\n", self.codegen_type(return_type), self.codegen_type(return_type), self.codegen_type(return_type)));
                    for (i, _) in args.iter().enumerate() {
                        let struct_field: (String, Type) = fields.get(i).unwrap().clone();
                        constructor.push_str(&format!("self->{} = __{};\n", struct_field.0, i));
                    }
                    for (field_name, field_type) in new_fields.iter() {
                        if let Type::Function(_, _, _) = field_type {
                            constructor.push_str(&format!("self->{} = __{}_{};\n", field_name, name, field_name));
                        }
                    }
                    constructor.push_str(&format!("return self;\n"));
                    constructor.push_str("}\n");
                } else {
                    forward_declarations.push_str(&format!("{} __{}_{}(", self.codegen_type(return_type), name, field_name));
                    for arg_type in args.iter() {
                        forward_declarations.push_str(&format!("{}, ", self.codegen_type(arg_type)));
                    }
                    if args.len() > 0 {
                        forward_declarations.pop();
                        forward_declarations.pop();
                    }
                    forward_declarations.push_str(");\n");
                }
                continue;
            }
            code.push_str(&format!("{} {};\n", self.codegen_type(field_type), field_name));
        }
        code.push_str("};\n");
        // if has_constructor {
        //     code.push_str(&forward_declarations);
        // }
        // code.push_str(&constructor);
        code
    }
    fn codegen_enum(&mut self, name: &String, enum_type: &Type, variants: &Vec<(String, Expression, TokenLocation)>) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("enum {} {{\n", name));
        for (variant_name, _, _) in variants.iter() {
            code.push_str(&format!("{},\n", variant_name));
        }
        code.push_str("};\n");
        if let Type::Function(args, return_type, _) = enum_type {
            code.push_str(format!("static {} (*const __{}_values[])(", self.codegen_type(return_type), name).as_str());
            for arg_type in args.iter() {
                code.push_str(&format!("{}, ", self.codegen_type(arg_type)));
            }
            if args.len() > 0 {
                code.pop();
                code.pop();
            }
            code.push_str(") = {\n");
        } else {
            code.push_str(format!("static {} const __{}_values[] = {{\n", self.codegen_type(enum_type), name).as_str());
        }
        for (variant_name, variant_value, _) in variants.iter() {
            code.push_str(&format!("[{}] = {},\n", variant_name, self.codegen_expression(variant_value)));
        }
        code.push_str("};\n");
        self.enums.push(name.clone());
        code
    }
    fn codegen_type_alias(&mut self, name: &String, types: &Vec<Type>) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("typedef "));
        if types.len() == 1 {
            code.push_str(&self.codegen_type(&types[0]));
        } else {
            code.push_str(&format!("union {{\n"));
            for (i, t) in types.iter().enumerate() {
                code.push_str(&format!("{} __{};\n", self.codegen_type(t), i));
            }
            code.push_str(&format!("}}"));
        }
        code.push_str(&format!(" {};\n", name));
        self.type_aliases.push(name.clone());
        code
    }
    fn codegen_function(&mut self, name: &String, args: &Vec<(String, Type)>, return_type: &Type, body: &Vec<Statement>) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("{} {}(", self.codegen_type(return_type), name));
        for (arg_name, arg_type) in args.iter() {
            self.parameter_types.insert(arg_name.clone(), arg_type.clone());
            if let Type::Function(func_args, return_type, _) = arg_type {
                // return_type (*name)(args)
                code.push_str(&format!("{} (*{})(", self.codegen_type(return_type), arg_name));
                for func_arg_type in func_args.iter() {
                    code.push_str(&format!("{}, ", self.codegen_type(func_arg_type)));
                }
                if func_args.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str("), ");
            } else {
                code.push_str(&format!("{} {}, ", self.codegen_type(arg_type), arg_name));
            }
        }
        if args.len() > 0 {
            code.pop();
            code.pop();
        }
        code.push_str(") {\n");
        for statement in body.iter() {
            code.push_str(&self.codegen_statement(statement));
        }
        code.push_str("}\n");
        for (arg_name, _) in args.iter() {
            self.parameter_types.remove(arg_name);
        }
        code
    }
    fn codegen_struct_function(&mut self, struct_name: &String, name: &String, args: &Vec<(String, Type)>, return_type: &Type, body: &Vec<Statement>) -> String {
        self.struct_functions.get(&struct_name.clone()).unwrap().clone().push(name.clone());
        let mut code: String = String::new();
        code.push_str(&format!("{} __{}_{}(", self.codegen_type(return_type), struct_name, name));
        for (arg_name, arg_type) in args.iter() {
            self.parameter_types.insert(arg_name.clone(), arg_type.clone());
            code.push_str(&format!("{} {}, ", self.codegen_type(arg_type), arg_name));
        }
        if args.len() > 0 {
            code.pop();
            code.pop();
        }
        code.push_str(") {\n");
        for statement in body.iter() {
            code.push_str(&self.codegen_statement(statement));
        }
        code.push_str("}\n");
        for (arg_name, _) in args.iter() {
            self.parameter_types.remove(arg_name);
        }
        code
    }
    fn codegen_variable(&mut self, name: &String, t: &Type, value: &Expression) -> String {
        self.variable_types.insert(name.clone(), t.clone());
        let mut code: String = String::new();
        if let Type::Array(type_, size, _) = t {
            code.push_str(&format!("{} {}[{}]", self.codegen_type(type_), name, self.codegen_expression(size)));
        } else if let Type::Function(args, return_type, _) = t {
            code.push_str(&format!("{} (*{})(", self.codegen_type(return_type), name));
            for arg_type in args.iter() {
                code.push_str(&format!("{}, ", self.codegen_type(arg_type)));
            }
            if args.len() > 0 {
                code.pop();
                code.pop();
            }
            code.push_str(")");
        } else {
            code.push_str(&format!("{} {}", self.codegen_type(t), name));
        }
        if let Expression::Empty = value {
            code.push_str(";\n");
        } else {
            code.push_str(&format!(" = {};\n", self.codegen_expression(value)));
        }
        code
    }
    fn codegen_constant(&mut self, name: &String, t: &Type, value: &Expression) -> String {
        self.variable_types.insert(name.clone(), t.clone());
        let mut code: String = String::new();
        code.push_str(&format!("const {} {} = {};\n", self.codegen_type(t), name, self.codegen_expression(value)));
        code
    }
    fn codegen_return(&mut self, value: &Expression) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("return {};\n", self.codegen_expression(value)));
        code
    }
    fn codegen_import(&mut self, path: &String) -> String {
        let mut code: String = String::new();
        if path.starts_with("std/") {
            code.push_str(&format!("#include <{}>\n", path.trim_start_matches("std/")));
        } else {
            code.push_str(&format!("#include \"{}\"\n", path));
        }
        code
    }
    fn codegen_if(&mut self, condition: &Expression, body: &Vec<Statement>, else_body: &Vec<Statement>) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("if ({}) {{\n", self.codegen_expression(condition)));
        for statement in body.iter() {
            code.push_str(&self.codegen_statement(statement));
        }
        code.push_str("}\n");
        if else_body.len() > 0 {
            code.push_str("else {\n");
            for statement in else_body.iter() {
                code.push_str(&self.codegen_statement(statement));
            }
            code.push_str("}\n");
        }
        code
    }
    fn codegen_type(&mut self, t: &Type) -> String {
        match t {
            Type::Int(_) => "int".to_string(),
            Type::Usize(_) => "size_t".to_string(),
            Type::String(_) => "const char*".to_string(),
            Type::CString(_) => "char*".to_string(),
            Type::Char(_) => "char".to_string(),
            Type::Bool(_) => "bool".to_string(),
            Type::Void(_) => "void".to_string(),
            Type::Struct(name, _) => format!("struct {}", name),
            Type::Enum(name, _) => format!("enum {}", name),
            Type::Function(_, _, _) => {
                self.errors.push(Error::TypeError("Function type is not allowed here".to_string(), t.location().clone()));
                "".to_string()
            }
            Type::Pointer(t, _) => format!("{}*", self.codegen_type(t)),
            Type::Array(t, _, _) => format!("{}", self.codegen_type(t)), // The size is generated in the declarations because C is stupid
            Type::DynamicArray(t, _) => format!("{}*", self.codegen_type(t)),
            Type::Restrict(t, _) => format!("{} restrict", self.codegen_type(t)),
            Type::Const(t, _) => format!("const {}", self.codegen_type(t)),
            Type::Volatile(t, _) => format!("volatile {}", self.codegen_type(t)),
            Type::GenericType(name, _) => name.clone(),
            Type::Unknown(name, location) => {
                // This type is only for checking if it's a struct, enum, or type alias
                if self.structs.contains(name) {
                    format!("struct {}", name)
                } else if self.enums.contains(name) {
                    format!("enum {}", name)
                } else if self.type_aliases.contains(name) {
                    name.clone()
                } else if self.generic_type_names.contains(name) {
                    name.clone()
                } else {
                    self.errors.push(Error::TypeError(format!("Unknown type {}", name), location.clone()));
                    "ERROR".to_string()
                }
            }
            Type::Error(error, _) => {
                self.errors.push(error.clone());
                "ERROR".to_string()
            }
        }
    }
    fn codegen_while(&mut self, condition: &Expression, body: &Vec<Statement>) -> String {
        let mut code: String = String::new();
        code.push_str(&format!("while ({}) {{\n", self.codegen_expression(condition)));
        for statement in body.iter() {
            code.push_str(&self.codegen_statement(statement));
        }
        code.push_str("}\n");
        code
    }
    fn codegen_expression(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::Number(value, _) => value.to_string(),
            Expression::String(value, _) => format!("\"{}\"", value ),
            Expression::Char(value, _) => format!("'{}'", value),
            Expression::Boolean(value, _) => value.to_string(),
            Expression::Identifier(name, _) => name.clone(),
            Expression::Null => "NULL".to_string(),
            Expression::Call(name, args, _) => {
                let mut code: String = String::new();
                if self.structs.contains(name) {
                    code.push_str(format!("&(struct {}){{", name).as_str());
                    for (_, arg) in args.iter().enumerate() {
                        code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                    }
                    if args.len() > 0 {
                        code.pop();
                        code.pop();
                    }
                    code.push_str("}");
                    return code;
                }
                code.push_str(&format!("{}(", name));
                for arg in args.iter() {
                    code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                }
                if args.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str(")");
                code
            }
            Expression::GenericCall(name, types, args, _) => {
                let mut code: String = String::new();
                for (i, t) in self.generic_types.get(name).unwrap().iter().enumerate() {
                    code.push_str(&format!("#define {} {}\n", t, self.clone().codegen_type(&types[i])));
                }
                code.push_str(&format!("{}(", name));
                for arg in args.iter() {
                    code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                }
                if args.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str(")");
                for t in self.generic_types.get(name).unwrap().iter() {
                    self.to_undef.push(t.clone());
                }
                code
            }

            Expression::Member(expression, member, _) => {
                match &**expression {
                    Expression::Identifier(name, _) => {
                        if self.variable_types.contains_key(name) {
                            if let Expression::Call(callee, args, _) = &**member {
                                let mut code: String = String::new();
                                code.push_str(&format!("{}->{}({}, ", name, callee, name));
                                for arg in args.iter() {
                                    code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                                }
                                code.pop();
                                code.pop();
                                code.push_str(")");
                                code
                            } else {
                                let t = self.variable_types.get(name).unwrap();
                                match t {
                                    Type::Pointer(_, _) => {
                                        format!("{}->{}", self.codegen_expression(expression), self.codegen_expression(member))
                                    }
                                    _ => {
                                        self.errors.push(Error::RuntimeError("Invalid member access".to_string(), expression.location().clone()));
                                        "".to_string()
                                    }
                                }
                            }
                        } else if self.parameter_types.contains_key(name) {
                            let t = self.parameter_types.get(name).unwrap();
                            match t {
                                Type::Pointer(_, _) => {
                                    format!("{}->{}", self.codegen_expression(expression), self.codegen_expression(member))
                                }
                                _ => {
                                    self.errors.push(Error::RuntimeError("Invalid member access".to_string(), expression.location().clone()));
                                    "".to_string()
                                }
                            }
                        } else if self.structs.contains(name) {
                            if let Expression::Identifier(member_id, _) = &**member {
                                let curr_struct_fields: &Vec<(String, Type)> = self.struct_fields.get(name).unwrap();
                                for (field_name, field_type) in curr_struct_fields.iter() {
                                    if field_name != member_id {
                                        continue;
                                    }
                                    if let Type::Function(_, _, _) = field_type.clone() {
                                        return format!("__{}_{}", name, member_id);
                                    } else {
                                        return format!("{}.{}", name, member_id);
                                    }
                                }
                                self.errors.push(Error::RuntimeError(format!("Unknown field {} in struct {}", member_id, name), expression.location().clone()));
                                "".to_string()
                            } else {
                                format!("{}.{}", name, self.codegen_expression(member))
                            }
                        } else if self.enums.contains(&name) {
                            match &**member {
                                Expression::Call(callee, args, _) => {
                                    let mut code: String = String::new();
                                    code.push_str(&format!("__{}_values[{}](", name, callee));
                                    for arg in args.iter() {
                                        code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                                    }
                                    if args.len() > 0 {
                                        code.pop();
                                        code.pop();
                                    }
                                    code.push_str(")");
                                    code
                                }
                                Expression::Identifier(member, _) => {
                                    format!("__{}_values[{}]", name, member)
                                }
                                _ => {
                                    self.errors.push(Error::RuntimeError("Invalid enum member access".to_string(), expression.location().clone()));
                                    "".to_string()
                                }
                            }
                        } else {
                            format!("{}.{}", name, self.codegen_expression(member))
                        }
                    }
                    _ => format!("{}.{}", self.codegen_expression(expression), self.codegen_expression(member)),
                }
            }
            Expression::Grouping(expression, _) => {
                format!("({})", self.codegen_expression(expression))
            }
            Expression::NamedArgument(name, expression, _) => {
                format!(".{} = {}", name, self.codegen_expression(expression))
            }
            Expression::Cast(expression, t, _) => {
                format!("({}){}", self.codegen_type(t), self.codegen_expression(expression))
            }
            Expression::SizeOf(t, _) => {
                format!("sizeof({})", self.codegen_type(t))
            }
            Expression::Index(expression, index, _) => {
                format!("{}[{}]", self.codegen_expression(expression), self.codegen_expression(index))
            }
            Expression::Array(elements, _) => {
                let mut code: String = String::new();
                code.push_str("{");
                for element in elements.iter() {
                    code.push_str(&format!("{}, ", self.codegen_expression(element)));
                }
                if elements.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str("}");
                code
            }
            Expression::New(identifier, args, _) => {
                let mut code: String = String::new();
                code.push_str(&format!("__{}_constructor(", identifier));
                for arg in args.iter() {
                    code.push_str(&format!("{}, ", self.codegen_expression(arg)));
                }
                if args.len() > 0 {
                    code.pop();
                    code.pop();
                }
                code.push_str(")");
                code
            }
            Expression::Unary(op, expression, _) => {
                let op: String = match op {
                    TokenKind::Minus => "-".to_string(),
                    TokenKind::Bang => "!".to_string(),
                    _ => {
                        self.errors.push(Error::RuntimeError("Invalid unary operator".to_string(), expression.location().clone()));
                        return String::new();
                    }
                };
                format!("{}{}", op, self.codegen_expression(expression))
            }
            Expression::Binary(op, left, right, _) => {
                let op: String = match op {
                    TokenKind::Plus => "+".to_string(),
                    TokenKind::Minus => "-".to_string(),
                    TokenKind::Star => "*".to_string(),
                    TokenKind::Slash => "/".to_string(),
                    TokenKind::Percent => "%".to_string(),
                    TokenKind::EqualEqual => "==".to_string(),
                    TokenKind::BangEqual => "!=".to_string(),
                    TokenKind::Less => "<".to_string(),
                    TokenKind::LessEqual => "<=".to_string(),
                    TokenKind::Greater => ">".to_string(),
                    TokenKind::GreaterEqual => ">=".to_string(),
                    _ => {
                        self.errors.push(Error::RuntimeError("Invalid binary operator".to_string(), left.location().clone()));
                        return String::new();
                    }
                };
                format!("{} {} {}", self.codegen_expression(left), op, self.codegen_expression(right))
            }
            Expression::Ternary(condition, left, right, _) => {
                format!("{} ? {} : {}", self.codegen_expression(condition), self.codegen_expression(left), self.codegen_expression(right))
            }
            Expression::Assignment(left, right, _) => {
                format!("{} = {}", self.codegen_expression(left), self.codegen_expression(right))
            }
            Expression::AddressOf(expression, _) => {
                format!("&{}", self.codegen_expression(expression))
            }
            Expression::Dereference(expression, _) => {
                format!("*{}", self.codegen_expression(expression))
            }
            Expression::Range(from, to, _) => {
                format!("{}..{}", self.codegen_expression(from), self.codegen_expression(to))
            }
            Expression::Empty => String::new(),
            Expression::Error(err) => {
                self.errors.push(err.clone());
                String::new()
            }
        }
    }
}
// Ide Support Plans
//   Convert the AST into Json for the frontend to use
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Ide {
    filename: String,
    contents: String,
    json: String,
    statements: Vec<Statement>,
}
#[allow(dead_code)]
impl Ide {
    fn new(filename: String, contents: String) -> Self {
        Self {
            filename,
            contents,
            json: String::new(),
            statements: Vec::new(),
        }
    }
    pub fn jsonify(&mut self) -> String {
        self.json.push_str("[");
        for statement in self.statements.iter() {
            self.json.push_str(&format!("{}, ", self.clone().jsonify_statement(statement.clone())));
        }
        if self.statements.len() > 0 {
            self.json.pop();
            self.json.pop();
        }
        self.json.push_str("]");
        self.json.clone()
    }
    fn jsonify_statement(&mut self, statement: Statement) -> String {
        match statement {
            Statement::Annotated(statement, annotations, location) => {
                let mut json: String = String::new();
                json.push_str(&format!("{{\"type\": \"Annotated\", \"annotations\": ["));
                for annotation in annotations.iter() {
                    json.push_str(&format!("{}, ", self.jsonify_annotation(annotation.clone())));
                }
                if annotations.len() > 0 {
                    json.pop();
                    json.pop();
                }
                json.push_str(&format!("], \"statement\": {}, \"location\": {}}}", self.jsonify_statement(*statement), self.jsonify_location(location)));
                json
            }
            Statement::Annotation(name, parameters, location) => {
                let mut json: String = String::new();
                json.push_str(&format!("{{\"type\": \"Annotation\", \"name\": \"{}\", \"parameters\": [", name));
                for parameter in parameters.iter() {
                    json.push_str(&format!("\"{}\": {}, ", parameter.0, self.jsonify_type(parameter.1.clone())));
                }
                if parameters.len() > 0 {
                    json.pop();
                    json.pop();
                }
                json.push_str(&format!("], \"location\": {}}}", self.jsonify_location(location)));
                json
            }
            Statement::Enum(name, value_type, values, location) => {
                let mut json: String = String::new();
                json.push_str(&format!("{{\"type\": \"Enum\", \"name\": \"{}\", \"value_type\": {}, \"values\": [", name, self.jsonify_type(value_type.clone())));
                for value in values.iter() {
                    json.push_str(&format!("\"{}\": {}, ", value.0, self.jsonify_expression(value.1.clone())));
                }
                if values.len() > 0 {
                    json.pop();
                    json.pop();
                }
                json.push_str(&format!("], \"location\": {}}}", self.jsonify_location(location)));
                json
            }
            Statement::Expression(expression, location) => {
                format!("{{\"type\": \"Expression\", \"expression\": {}, \"location\": {}}}", self.jsonify_expression(expression), self.jsonify_location(location))
            }
            Statement::External(statement, location) => {
                format!("{{\"type\": \"External\", \"statement\": {}, \"location\": {}}}", self.jsonify_statement(*statement), self.jsonify_location(location))
            }
            Statement::Function(name, parameters, return_type, body, location) => {
                let mut json: String = String::new();
                json.push_str(&format!("{{\"type\": \"Function\", \"name\": \"{}\", \"parameters\": [", name));
                for parameter in parameters.iter() {
                    json.push_str(&format!("\"{}\": {}, ", parameter.0, self.jsonify_type(parameter.1.clone())));
                }
                if parameters.len() > 0 {
                    json.pop();
                    json.pop();
                }
                json.push_str(&format!("], \"return_type\": {}, \"body\": [", self.jsonify_type(return_type.clone())));
                for statement in body.iter() {
                    json.push_str(&format!("{}, ", self.jsonify_statement(statement.clone())));
                }
                if body.len() > 0 {
                    json.pop();
                    json.pop();
                }
                json.push_str(&format!("], \"location\": {}}}", self.jsonify_location(location)));
                json
            }
            _ => String::new(),
        }
    }
    fn jsonify_expression(&mut self, expression: Expression) -> String {
        match expression {
            _ => String::new(),
        }
    }
    fn jsonify_type(&mut self, type_: Type) -> String {
        match type_ {
            _ => String::new(),
        }
    }
    fn jsonify_annotation(&mut self, annotation: Annotation) -> String {
        let mut json: String = String::new();
        json.push_str(&format!("{{\"name\": \"{}\", \"args\": [", annotation.name));
        for arg in annotation.arguments.iter() {
            json.push_str(&format!("{}, ", self.jsonify_expression(arg.clone())));
        }
        if annotation.arguments.len() > 0 {
            json.pop();
            json.pop();
        }
        json.push_str(format!("], \"location\": {}}}", self.jsonify_location(annotation.location)).as_str());
        json
    }
    fn jsonify_location(&mut self, location: TokenLocation) -> String {
        format!("{{\"start\": {}, \"end\": {}}}", location.start, location.end)
    }
}
fn main() {
    let mut args = std::env::args().skip(1);
    let filename: String = args.next().unwrap();
    let contents: String = std::fs::read_to_string(filename.clone()).unwrap();

    let mut lexer: Lexer = Lexer::new(contents.clone());
    let tokens: Vec<Token> = lexer.lex();
    if lexer.errors.len() > 0 {
        for error in lexer.errors.iter() {
            println!("{}", error.to_string(filename.clone(), contents.clone()));
        }
        return;
    }

    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Statement> = parser.parse();
    if parser.errors.len() > 0 {
        for error in parser.errors.iter() {
            println!("{}", error.to_string(filename.clone(), contents.clone()));
        }
        return;
    }   

    let mut codegen: Codegen = Codegen::new(statements);
    let code: String = codegen.codegen();
    if codegen.errors.len() > 0 {
        for error in codegen.errors.iter() {
            println!("{}", error.to_string(filename.clone(), contents.clone()));
        }
        return;
    }

    let output_filename: String = filename.clone().replace(".sl", ".c");
    std::fs::write(output_filename, code).unwrap();
}