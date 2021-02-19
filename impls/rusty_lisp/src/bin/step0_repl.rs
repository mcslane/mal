use std::io::{self, Write};

fn read() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn eval(input: String) -> String {
    //pass through
    input
}

fn print(output: String) {
    // as of right now the output has a newline, so we print and flush rather than println
    print!("{}", output);
    io::stdout().flush().unwrap();
}

fn rep() -> bool {
    let input = match read() {
        Ok(input) => input,
        Err(e) => {
            println!("Error: {}", e);
            return true
        }
    };
    // if input has length of 0, it is EOF
    if input.len() == 0 {
        return false
    }
    let output = eval(input);
    print(output);
    return true
}

fn main() {
    let mut keep_going = true;
    while keep_going {
        print!("user> ");
        io::stdout().flush().unwrap();
        keep_going = rep();
    }
    // add a last new line so the users shell comes back right
    println!("");
}
