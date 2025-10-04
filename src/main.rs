mod editor;
mod lexer;
use editor::CodeEditor;

use std::io::Write;
use std::process;
use std::{io, fs};

fn main() {
    println!("Edit: ");
    io::stdout().flush().expect("Unable to flush stdout");

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Unable to read line");
    file_path = file_path.trim().replace("\\", "/");
    
    let mut editor = match CodeEditor::new("fonts/Cascadia.ttf") {
        Ok(editor) => editor,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };

    let mut src: String = fs::read_to_string(file_path.clone()).expect("Unable to read from path");

    let edit = editor.edit(&mut src, &file_path);
    match edit {
        Ok(()) => {
            fs::write(file_path, src).expect("Unable to save file");
            println!("Script edited and saved!")
        },
        Err(e) => {
            println!("Unable to edit script due to error:\n{}", e);
            process::exit(1)
        }
    }
    
}