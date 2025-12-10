// Лексический анализатор для Krait

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Func, End, Return, If, Else, While, For, Try, Catch, Raise, Route,
    Json, Auto, ErrorCode, Public, Private, Import, From,
    Int, Float, Double, Txt, Bool,
    Get, Post, Put, Delete,
    Plus, Minus, Star, Slash, Percent, Equal, EqualEqual, NotEqual,
    Less, Greater, LessEqual, GreaterEqual, And, Or, Not,
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Dot, Colon, Arrow,
    Identifier(String), Number(String), String(String), Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }
    
    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.input.len() {
            Some(self.input[self.pos + offset])
        } else {
            None
        }
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
            while let Some(ch) = self.current_char() {
                self.advance();
                if ch == '\n' {
                    break;
                }
            }
        }
    }
    
    fn read_string(&mut self, quote: char) -> String {
        let mut result = String::new();
        self.advance();
        
        while let Some(ch) = self.current_char() {
            if ch == quote {
                self.advance();
                break;
            }
            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        '\'' => result.push('\''),
                        _ => result.push(escaped),
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        result
    }
    
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }
    
    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_numeric() || ch == '.' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }
    
    fn keyword_or_identifier(&self, word: &str) -> TokenType {
        match word {
            "func" => TokenType::Func,
            "end" => TokenType::End,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "raise" => TokenType::Raise,
            "route" => TokenType::Route,
            "json" => TokenType::Json,
            "auto" => TokenType::Auto,
            "error_code" => TokenType::ErrorCode,
            "public" => TokenType::Public,
            "private" => TokenType::Private,
            "import" => TokenType::Import,
            "from" => TokenType::From,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "double" => TokenType::Double,
            "txt" => TokenType::Txt,
            "bool" => TokenType::Bool,
            "get" => TokenType::Get,
            "post" => TokenType::Post,
            "put" => TokenType::Put,
            "delete" => TokenType::Delete,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            _ => TokenType::Identifier(word.to_string()),
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }
            
            if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
                self.skip_comment();
                continue;
            }
            
            let line = self.line;
            let column = self.column;
            let ch = self.current_char().unwrap();
            
            let token_type = match ch {
                '(' => { self.advance(); TokenType::LeftParen }
                ')' => { self.advance(); TokenType::RightParen }
                '{' => { self.advance(); TokenType::LeftBrace }
                '}' => { self.advance(); TokenType::RightBrace }
                '[' => { self.advance(); TokenType::LeftBracket }
                ']' => { self.advance(); TokenType::RightBracket }
                ',' => { self.advance(); TokenType::Comma }
                '.' => { self.advance(); TokenType::Dot }
                ':' => { self.advance(); TokenType::Colon }
                '+' => { self.advance(); TokenType::Plus }
                '-' => {
                    self.advance();
                    if self.current_char() == Some('>') {
                        self.advance();
                        TokenType::Arrow
                    } else {
                        TokenType::Minus
                    }
                }
                '*' => { self.advance(); TokenType::Star }
                '/' => { self.advance(); TokenType::Slash }
                '%' => { self.advance(); TokenType::Percent }
                '=' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        TokenType::NotEqual
                    } else {
                        TokenType::Not
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '"' => TokenType::String(self.read_string('"')),
                '\'' => TokenType::String(self.read_string('\'')),
                _ if ch.is_alphabetic() || ch == '_' => {
                    let word = self.read_identifier();
                    self.keyword_or_identifier(&word)
                }
                _ if ch.is_numeric() => TokenType::Number(self.read_number()),
                _ => { self.advance(); TokenType::Identifier(ch.to_string()) }
            };
            
            tokens.push(Token { token_type, line, column });
        }
        
        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
        });
        
        tokens
    }
}
