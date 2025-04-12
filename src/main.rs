#[derive(Clone, Debug)]
enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,
    Let,
}

#[derive(Clone)]
struct Token {
    value: String,
    t_type: TokenType,
}

fn token(value: &str, t_type: TokenType) -> Token {
    Token {
        value: value.to_string(),
        t_type,
    }
}

fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src: Vec<char> = source_code.chars().collect();

    while !src.is_empty() {
        let c = src.remove(0);

        if c == '(' {
            tokens.push(token("(", TokenType::OpenParen));
        } else if c == ')' {
            tokens.push(token(")", TokenType::CloseParen));
        } else if c == '+' || c == '-' {
            tokens.push(token(&c.to_string(), TokenType::BinaryOperator));
        } else if c == '*' || c == '/' {
            tokens.push(token(&c.to_string(), TokenType::BinaryOperator));
        } else if c == '=' {
            tokens.push(token("=", TokenType::Equals));
        } else if c.is_digit(10) {
            let mut num = String::new();
            num.push(c); 
            while !src.is_empty() && src[0].is_digit(10) {
                num.push(src.remove(0)); 
            }
            tokens.push(Token {
                value: num,
                t_type: TokenType::Number,
            });
        } else if c.is_alphabetic() {
            let mut identifier = String::new();
            identifier.push(c); 
            while !src.is_empty() && src[0].is_alphabetic() {
                identifier.push(src.remove(0)); 
            }
            tokens.push(Token {
                value: identifier,
                t_type: TokenType::Identifier,
            });
        }
    }

    tokens
}

fn main() {
    let source_code = "let x = 42 + 3 * (y - 1)";
    let tokens = tokenize(source_code);
    for token in tokens {
        println!("Token: {}, Type: {:?}", token.value, token.t_type);
    }
}

