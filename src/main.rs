use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use clap::{arg, command};

const TAB: &str = "    ";  // Tab = 4 spaces

fn main() {
    let matches = command!()
        .arg(arg!([path] "Path to file to prettify"))
        .get_matches();
    let path = match matches
        .get_one::<String>("path")
    {
        Some(string) => PathBuf::from(string),
        None => {
            println!("Please specify a path");
            return
        },
    };
    let file_contents: String = match get_file_contents(&path) {
        Ok(string) => string,
        Err(e) => panic!("{}", e)
    };
    let prettified: String = prettify(file_contents);
    match save_file(&path, prettified) {
        Ok(()) => println!("{} successfully prettified", path.display()),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/// Insert line breaks and tabs to a string
///     - each opening paren starts a new line below it at +1 tab spacing.
///     - each closing paren starts a new line below it at -1 tab spacing.
///     - each comma starts a new line below it at +/- 0 tab spacing.
fn prettify(s: String) -> String {
    let mut output: String = String::new();
    let mut tab_level: usize = 0;
    for character in s.chars() {
        match character {
            '(' => open(&mut tab_level, &mut output, '('),
            '[' => open(&mut tab_level, &mut output, '['),
            '{' => open(&mut tab_level, &mut output, '{'),
            ')' => close(&mut tab_level, &mut output, ')'),
            ']' => close(&mut tab_level, &mut output, ']'),
            '}' => close(&mut tab_level, &mut output, '}'),
            ',' => comma(&mut tab_level, &mut output),
            c => output.push(c)
        }
    }
    output
}

/// Add open paren to output
fn open(tab_level: &mut usize, output: &mut String, opener: char) {
    output.push(opener);
    output.push('\n');
    *tab_level += 1;
    for _ in 0..*tab_level {
        output.push_str(TAB);
    }
}

/// Add close paren to output
fn close(tab_level: &mut usize, output: &mut String, closer: char) {
    output.push('\n');
    *tab_level -= 1;
    for _ in 0..*tab_level {
        output.push_str(TAB);
    }
    output.push(closer);
}

/// Add comma to output
fn comma(tab_level: &usize, output: &mut String) {
    output.push(',');
    output.push('\n');
    for _ in 0..*tab_level {
        output.push_str(TAB);
    }
}

fn get_file_contents(path: &Path) -> std::io::Result<String> {
    let file: File = File::open(path)?;
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn save_file(path: &Path, contents: String) -> std::io::Result<()> {
    let tmp = path.file_stem().expect("path should have extension");
    let mut file: File = File::create(tmp)?;
    match file.write(contents.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            fs::remove_file(tmp)?;
            println!("{}", e);
            return Err(e)
        }
    };
    match fs::rename(tmp, path) {
        Ok(_) => {},
        Err(e) => {
            fs::remove_file(tmp)?;
            println!("{}", e);
            return Err(e)
        }
    };
    Ok(())
}
