use super::token::Token;
use std::collections::HashMap;

pub struct Lexer {
    bencode: Vec<char>,
    tokens: Vec<Token>,
    current_index: usize,
}

impl Lexer {
    pub fn new(bencode: String) -> Lexer {
        Lexer {
            bencode: bencode.chars().collect(),
            tokens: Vec::new(),
            current_index: 0,
        }
    }

    pub fn lex(&mut self) {
        while self.current_index < self.bencode.len() {
            if self.bencode[self.current_index].eq(&'i') {
                let t = self.parse_list();
                self.add_token(t);
            } else if self.bencode[self.current_index].is_digit(10) {
                //means it is a string
                let t = self.parse_string();
                self.add_token(t);
            } else if self.bencode[self.current_index].eq(&'l') {
                let t = self.parse_list();
                self.add_token(t);
            } else if self.bencode[self.current_index].eq(&'d') {
                let t = self.parse_dictionary();
                self.add_token(t);
            }
        }
    }

    fn parse_dictionary(&mut self) -> Token {
        self.advance();
        let mut d: HashMap<String, Token> = HashMap::new();

        while self.bencode[self.current_index] != 'e' {
            let parsed_key = match self.parse_string() {
                Token::String(s) => s,
                _ => panic!("expecting a String")
            };
            let value = self.parse_value();
            d.insert(parsed_key, value);
        }


        self.expecting_e();

        Token::Dictionary(d)
    }

    fn parse_value(&mut self) -> Token {
        if self.bencode[self.current_index] == 'i' {
            return self.parse_int();
        } else if self.bencode[self.current_index].is_digit(10) {
            return self.parse_string();
        } else if self.bencode[self.current_index] == 'l' {
            return self.parse_list();
        } else if self.bencode[self.current_index] == 'd' {
            return self.parse_dictionary();
        } else if self.bencode[self.current_index] == 'e' {
            self.advance();
            return Token::Eps;
        }

        panic!();
    }

    fn parse_list(&mut self) -> Token {
        self.advance();
        let mut l = Vec::new();

        while self.bencode[self.current_index] != 'e' {
            let value = self.parse_value();
//            let item = match self.parse_string() {
//                Token::String(s) => s,
//                _ => panic!()
//            };
            l.push(value);
        }

        self.expecting_e();

        Token::List(l)
    }

    fn parse_string(&mut self) -> Token {
        let parsed_integer = self.iter_until_not_integer_and_return_it();
        self.expecting_colon();

        let s = self.bencode[self.current_index..self.current_index + parsed_integer].iter().collect::<String>();
        self.current_index += parsed_integer;

        Token::String(s)
    }

    fn parse_int(&mut self) -> Token {
        self.advance();
        let parsed_integer = self.iter_until_not_integer_and_return_it();

        self.expecting_e();

        Token::Integer(parsed_integer as i32)
    }


    fn iter_until_not_integer_and_return_it(&mut self) -> usize {
        let start_index = self.current_index;
        self.advance();

        while self.bencode[self.current_index].is_digit(10) {
            self.advance();
        }

//        let parsed_integer = self.bencode[start_index..self.current_index].iter().collect::<String>().parse::<usize>().unwrap();
        let parsed_integer = match self.bencode[start_index..self.current_index].iter().collect::<String>().parse::<usize>() {
            Ok(parsed_integer) => parsed_integer,
            Err(_) => {
                println!("=============================================");
                println!("{}", self.bencode.iter().collect::<String>());
                println!("{}", self.bencode[self.current_index]);
                println!("{}", self.bencode[self.current_index + 1]);
                println!("{}", self.bencode[self.current_index + 2]);
                panic!();
            }
        };

        parsed_integer
    }

    fn expecting_colon(&mut self) {
        if self.bencode[self.current_index] != ':' {
            panic!("expecting colon")
        }

        self.advance();
    }

    fn expecting_e(&mut self) {
        if self.bencode[self.current_index] != 'e' {
            panic!("expecting colon")
        }

        self.advance();
    }


    fn add_token(&mut self, t: Token) {
        println!("{:?}", t);
        self.tokens.push(t);
    }

    fn advance(&mut self) {
        self.current_index += 1;
    }
}