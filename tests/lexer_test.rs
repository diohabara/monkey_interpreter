extern crate monkey_interpreter;

#[cfg(test)]
mod tests {
    use monkey_interpreter::{
        lexer::lexer::new,
        token::token::{Token, TokenType},
    };

    #[test]
    fn test_next_token_() {
        let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ";

        let tests = [
            // let five = 5;
            make_token(TokenType::Ident, "let"),
            make_token(TokenType::Ident, "five"),
            make_token(TokenType::Assign, "="),
            make_token(TokenType::Int, "5"),
            make_token(TokenType::Semicolon, ";"),
            // let ten = 10;
            make_token(TokenType::Let, "let"),
            make_token(TokenType::Ident, "ten"),
            make_token(TokenType::Assign, "="),
            make_token(TokenType::Int, "10"),
            make_token(TokenType::Semicolon, ";"),
            // let add = fn(x, y) {
            make_token(TokenType::Let, "let"),
            make_token(TokenType::Ident, "add"),
            make_token(TokenType::Assign, "="),
            make_token(TokenType::Function, "fn"),
            make_token(TokenType::LParen, "("),
            make_token(TokenType::Ident, "x"),
            make_token(TokenType::Comma, ","),
            make_token(TokenType::Ident, "y"),
            make_token(TokenType::RParen, ")"),
            make_token(TokenType::LBrace, "{"),
            // x + y
            make_token(TokenType::Ident, "x"),
            make_token(TokenType::Plus, "+"),
            make_token(TokenType::Ident, "y"),
            // };
            make_token(TokenType::RBrace, "}"),
            make_token(TokenType::Semicolon, ";"),
            // let result = add(five, ten);
            make_token(TokenType::Let, "let"),
            make_token(TokenType::Ident, "result"),
            make_token(TokenType::Assign, "="),
            make_token(TokenType::Ident, "add"),
            make_token(TokenType::LParen, "("),
            make_token(TokenType::Ident, "five"),
            make_token(TokenType::Comma, ","),
            make_token(TokenType::Ident, "ten"),
            make_token(TokenType::RParen, ")"),
            make_token(TokenType::Semicolon, ";"),
            make_token(TokenType::Eof, "")
        ];

        let l = new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.clone().next_token();

            if tok.typ != tt.typ {
                eprintln!(
                    "tests[{}] - token type wrong. expected={:?}, got={:?}",
                    i, tt.typ, tok.typ,
                )
            }

            if tok.literal != tt.literal {
                eprintln!(
                    "tests[{}] - literal wrong. expected={:?}, got={:?}",
                    i, tt.typ, tok.literal
                )
            }
        }
    }
    pub fn make_token(_type: TokenType, _literal: &str) -> Token {
        Token {
            typ: _type,
            literal: String::from(_literal),
        }
    }
}
