use sdl2::pixels::Color;

use crate::editor::COLORS;

pub enum Tok {
    Number,
    StringLit,
    Identifier,
    ArithOp,
    Keyword,
    Punctuation,
    Separator,
    Function,
    Whitespace,
    Type
}

pub fn tokenize(line: &str) -> Vec<(Tok, String)> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut toks: Vec<(Tok, String)> = Vec::new();
    let mut current: String = String::new();

    for ch in chars {
        match ch {
            '0'..='9' => current.push(ch),
            'a'..='z' | 'A'..='Z' | '_' => {
                if current.chars().all(|c| c.is_ascii_digit()) {
                    toks.push((classify(&current), current.clone()));
                    current.clear()
                }
                current.push(ch)
            },
            _ => {
                if !current.is_empty() {
                    toks.push((classify(&current), current.clone()));
                    current.clear()
                }
                toks.push((classify(&ch.to_string()), ch.to_string()))
            }
        }
    }
    if !current.is_empty() {
        toks.push((classify(&current), current.clone()));
        current.clear()
    }

    toks
}

pub fn classify(tok: &String) -> Tok {
    match &**tok {
        " " => Tok::Whitespace,
        "+" | "-" | "*" | "/" => Tok::ArithOp,
        "(" | ")" | "{" | "}" | "[" | "]" => Tok::Separator,
        "var" => Tok::Keyword,
        "println" => Tok::Function,
        "int" | "float" | "str" | "bool" => Tok::Type,
        ":" => Tok::Punctuation,
        _ => if tok.parse::<i64>().is_ok() | tok.parse::<f64>().is_ok() {
            Tok::Number
        } else if tok.starts_with('"') && tok.ends_with('"') {
            Tok::StringLit
        } else {
            Tok::Identifier
        }
    }
}

pub fn to_color(tok: Tok) -> Color {
    match tok {
        Tok::Number => COLORS.5,
        Tok::StringLit => COLORS.4,
        Tok::Identifier => COLORS.2,
        Tok::ArithOp => COLORS.10,
        Tok::Keyword => COLORS.3,
        Tok::Punctuation => COLORS.9,
        Tok::Separator => COLORS.11,
        Tok::Function => COLORS.8,
        Tok::Whitespace => COLORS.0,
        Tok::Type => COLORS.6
    }
}