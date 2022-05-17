use std::io::Write;

use schwift::{grammar, state::State};

fn get_line<'a>(file: &'a str, err: &grammar::ParseError) -> &'a str {
    let mut count = 0;
    let mut last_newline = 0usize;

    for i in 0..file.len() {
        if file.is_char_boundary(i) {
            let symbol = &file[i..=i];

            if symbol == "\n" {
                count += 1;

                if count == err.location.line {
                    return &file[last_newline..i];
                }

                last_newline = i + 1;
            }
        }
    }

    panic!(
        "Got grammar error with invalid line number {}",
        err.location.line
    );
}

fn place_carat(err: &grammar::ParseError) -> String {
    let mut s = String::new();

    for _ in 0..err.location.column - 1 {
        s.push(' ');
    }

    s.push('^');

    s
}

fn main() {
    let mut state = State::new();

    let stdin = std::io::stdin();

    loop {
        print!(">>>");
        let _ = std::io::stdout().flush();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();

        let statements = match grammar::file(&buffer) {
            Ok(statements) => statements,
            Err(e) => {
                println!(
                    "SYNTAX ERROR: {}\n{}\n{}",
                    e.location.line,
                    get_line(&buffer, &e),
                    place_carat(&e)
                );
                continue;
            }
        };

        match state.run(&statements) {
            Ok(_) => {}
            Err(e) => {
                println!("i will implement error messages later i promise (i won't)");
            }
        }
    }
}
