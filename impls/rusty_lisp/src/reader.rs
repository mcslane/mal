use regex::Regex;
use crate::types::{MalType, MalList, MalNumber, MalSymbol};

struct Reader<'a> {
    position: usize,
    tokens: Vec<&'a str>,
}

impl Reader<'_> {
    /// Returns the current token and increments the position
    fn next(&mut self) -> &str {
        let result = self.tokens[self.position];
        self.position += 1;
        result
    }

    /// Returns the current token without changing the position
    fn peek(&self) -> &str {
        self.tokens[self.position]
    }
}

pub fn read_str(input: &str) -> MalType {
    let tokens = tokenize(input);
    let mut reader = Reader { position: 0, tokens: tokens };
    read_form(&mut reader)
}

fn tokenize(input: &str) -> Vec<&str> {
    let mut buffer = &input[..];
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let mut result = Vec::new();
    while buffer.len() > 0 {
        let cap = match re.captures(buffer) {
            Some(cap) => cap,
            None => {
                println!("No more valid tokens found!");
                break;
            }
        };
        let whole_match = cap.get(0).unwrap();
        if whole_match.start() != 0 {
            // this is an error, handle it at some point
            println!("matching group must start at zero!");
        }
        let token = cap.get(1).unwrap().as_str();
        if token.len() != 0 {
            result.push(token);
        }
        buffer = &buffer[whole_match.end()..];
    }
    result
}

fn read_form(mut reader: &mut Reader) -> MalType {
    let token = reader.peek();
    if token == "(" {
        read_list(reader)
    } else {
        read_atom(&mut reader)
    }
}

fn read_list(mut reader: &mut Reader) -> MalType {
    // go past the opening paren
    reader.next();
    let mut result = Vec::new();
    loop {
        if reader.position == reader.tokens.len() {
            // EOF before closing paren, which is an error
            println!("Error: EOF before list is closed");
            break;
        }
        
        if reader.peek() == ")" {
            // skip past the end so that the next token starts in the correct place
            reader.next();
            break;
        }

        let mal_obj = read_form(&mut reader);
        result.push(mal_obj);
    }
    Box::new(MalList { list: result })
}

fn read_atom(reader: &mut Reader) -> MalType {
    let token = reader.next();
    let parsed_number = token.parse::<i64>();
    if parsed_number.is_ok() {
        Box::new(MalNumber { value: parsed_number.unwrap() })
    } else {
        Box::new(MalSymbol { name: String::from(token) })
    }
}
