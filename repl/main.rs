use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut rl = Editor::<()>::new().unwrap();

    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(line) => {
                let mut l = lib::lexer::Lexer::new(line);
                let mut p = lib::parser::Parser::new(l);
                let ast = match p.parse_program() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        continue
                    }
                };
                println!("{:?}", ast);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
