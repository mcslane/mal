use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct ReadLine<'a> {
    history_file: &'a str,
    editor: Editor<()>,
}

impl ReadLine<'_> {
    pub fn new(history_file: &str) -> ReadLine {
        let mut rl = ReadLine { history_file: history_file, editor: Editor::<()>::new() };
        println!("Loading repl history from {}", rl.history_file);
        if rl.editor.load_history(rl.history_file).is_err() {
            println!("No previous history.");
        }
        rl
    }

    pub fn read_line(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        let line = self.editor.readline(prompt);
        match line {
            Ok(line) => {
                self.editor.add_history_entry(line.clone());
                Ok(line)
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn close(&mut self) {
        match self.editor.save_history(self.history_file) {
            Ok(()) => (),
            Err(err) => println!("{}", err),
        }
    }
}
