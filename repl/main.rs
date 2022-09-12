use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut rl = Editor::<()>::new().unwrap();

    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(line) => {
                let l = lib::lexer::Lexer::new(line);
                let mut p = lib::parser::Parser::new(l);
                match p.parse_program() {
                    Ok(p) => {
                        let mut eval = lib::eval::Evaluator::new();
                        let evaluated = eval.evaluate(&p);
                        match evaluated {
                            Ok(eval) => println!("{}", eval),
                            Err(e) => {
                                eprintln!("Error: {:?}", e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        continue;
                    }
                };
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
