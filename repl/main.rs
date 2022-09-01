use rustyline::{Editor, error::ReadlineError};

use monkey_lib::lexer::Lexer;


fn main() {
    let mut rl = Editor::<()>::new().unwrap();
    
    loop {
        let line =  rl.readline(">> ");
        match line {
            Ok(line) => {
                let mut l = Lexer::new(line);
                l.next_token();
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}