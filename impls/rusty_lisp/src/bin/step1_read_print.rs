use rusty_lisp::readline::ReadLine;
use rusty_lisp::types::MalType;
use rusty_lisp::reader::read_str;
use rusty_lisp::printer::pr_str;
use rustyline::error::ReadlineError;

fn read(rl: &mut ReadLine) -> Result<MalType, ReadlineError> {
    match rl.read_line("user> ") {
        Ok(line) => Ok(read_str(&line[..])),
        Err(e) => Err(e)
    }
}

fn eval(input: MalType) -> MalType {
    //pass through
    input
}

fn print(output: MalType) {
    println!("{}", pr_str(output));
}

fn rep(mut rl: &mut ReadLine) -> bool {
    let input = match read(&mut rl) {
        Ok(input) => input,
        Err(ReadlineError::Eof) => {
            return false
        }
        Err(e) => {
            println!("Error: {}", e);
            return true
        }
    };
    let output = eval(input);
    print(output);
    return true
}

fn main() {
    let mut keep_going = true;
    let mut rl = ReadLine::new("repl_history.txt");
    while keep_going {
        keep_going = rep(&mut rl);
    }
    rl.close();
    // add a last new line so the users shell comes back right
    println!("");
}
