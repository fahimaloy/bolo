use std::env;
use std::io::{self, Write};

/// Bolo – a tiny Rust version of the `echo` command
///
/// Usage:
///   bolo [OPTIONS] [ARG...]
///
/// Options:
///   -n            do not output the trailing newline
///   -e            enable interpretation of backslash escapes
///   -E            disable interpretation of backslash escapes (default)
///   --help        display this help and exit
///   --version     output version information and exit
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().skip(1).collect(); // skip the program name

    if args.is_empty() {
        // No arguments → just print a newline (POSIX echo behaviour)
        println!();
        return;
    }

    // Flags
    let mut no_newline = false;
    let mut interpret_escapes = false; // -e enables, -E disables (default false)

    // Separate options from the words we have to print
    let mut words = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => no_newline = true,
            "-e" => interpret_escapes = true,
            "-E" => interpret_escapes = false,
            "--help" => {
                print_help();
                return;
            }
            "--version" => {
                print_version();
                return;
            }
            arg if arg.starts_with('-') => {
                eprintln!("bolo: invalid option -- '{}'", arg);
                eprintln!("Try 'bolo --help' for more information.");
                std::process::exit(1);
            }
            _ => {
                // everything else is a word to print
                words.push(&args[i]);
            }
        }
        i += 1;
    }

    // Build the final string
    let mut output = String::new();
    for (idx, word) in words.iter().enumerate() {
        if idx > 0 {
            output.push(' ');
        }

        if interpret_escapes {
            output.push_str(&expand_escapes(word));
        } else {
            output.push_str(word);
        }
    }

    // Print it
    if no_newline {
        print!("{}", output);
        // Flush stdout because we omitted the newline
        let _ = io::stdout().flush();
    } else {
        println!("{}", output);
    }
}

// ---------------------------------------------------------------------
// Helper: print usage information
fn print_help() {
    println!(
        r#"Usage: bolo [OPTIONS] [STRING...]
Echo the STRING(s) to standard output.

  -n             do not output the trailing newline
  -e             enable interpretation of backslash escapes
  -E             disable interpretation of backslash escapes (default)
  --help         display this help and exit
  --version      output version information and exit

Escape sequences (when -e is used):
  \\   backslash
  \a   alert (bell)
  \b   backspace
  \c   produce no further output
  \e   escape
  \f   form feed
  \n   new line
  \r   carriage return
  \t   horizontal tab
  \v   vertical tab
  \0nnn  byte with octal value nnn (1 to 3 digits)
  \xHH   byte with hexadecimal value HH (1 to 2 digits)"#
    );
}

// ---------------------------------------------------------------------
// Helper: print version
fn print_version() {
    println!("bolo 1.0.0");
}

// ---------------------------------------------------------------------
// Helper: expand backslash escapes (-e)
fn expand_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        // We saw a backslash
        match chars.peek() {
            Some(&next) => {
                chars.next(); // consume the peeked char
                match next {
                    '\\' => result.push('\\'),
                    'a' => result.push('\x07'), // bell
                    'b' => result.push('\x08'), // backspace
                    'c' => {
                        // \c stops all further output
                        return result;
                    }
                    'e' => result.push('\x1B'), // escape
                    'f' => result.push('\x0C'), // form feed
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    'v' => result.push('\x0B'), // vertical tab
                    '0' | '1'..='7' => {
                        // Octal escape: up to 3 digits
                        let mut oct = String::new();
                        oct.push(next);
                        while let Some(&d) = chars.peek() {
                            if d.is_digit(8) && oct.len() < 3 {
                                oct.push(d);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if let Ok(val) = u8::from_str_radix(&oct, 8) {
                            result.push(val as char);
                        } else {
                            result.push_str(&format!("\\{}", oct));
                        }
                    }
                    'x' => {
                        // Hex escape: \xHH
                        chars.next(); // consume the 'x'
                        let mut hex = String::new();
                        while let Some(&d) = chars.peek() {
                            if d.is_ascii_hexdigit() && hex.len() < 2 {
                                hex.push(d);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if hex.is_empty() {
                            result.push_str("\\x");
                        } else if let Ok(val) = u8::from_str_radix(&hex, 16) {
                            result.push(val as char);
                        } else {
                            result.push_str(&format!("\\x{}", hex));
                        }
                    }
                    _ => {
                        // Unknown escape – print the backslash and the char
                        result.push('\\');
                        result.push(next);
                    }
                }
            }
            None => result.push('\\'), // trailing backslash
        }
    }
    result
}

// ---------------------------------------------------------------------
// Tests (run with `cargo test`)
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Output};

    // Helper to run `bolo` as if from the shell
    fn run_bolo(args: &[&str]) -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_bolo"))
            .args(args)
            .output()
            .expect("failed to execute bolo");
        String::from_utf8_lossy(&output.stdout).trim_end().to_string()
    }

    #[test]
    fn basic() {
        assert_eq!(run_bolo(&["hello", "world"]), "hello world");
    }

    #[test]
    fn no_newline() {
        let out = Command::new(env!("CARGO_BIN_EXE_bolo"))
            .args(&["-n", "hi"])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stdout), "hi");
        assert!(!out.stdout.ends_with(b"\n"));
    }

    #[test]
    fn escape_sequences() {
        assert_eq!(run_bolo(&["-e", "Hello\\nWorld"]), "Hello\nWorld");
        assert_eq!(run_bolo(&["-e", "Bell\\a"]), "Bell\u{7}");
        assert_eq!(run_bolo(&["-e", "Stop\\cmore"]), "Stop");
    }
}
