use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

// On startup, ensure "notes/" directory exists
fn setup_notes_dir() {
    let dir = "notes";
    if !Path::new(dir).exists() {
        fs::create_dir(dir).expect("Could not create notes directory.");
    }
}

// Helper: returns the note file path for a given title
fn note_path(title: &str) -> String {
    format!("notes/{}.md", title)
}

fn main() {
    setup_notes_dir();
    println!("PlainMark: Minimal Markdown Note Manager\nCommands: add, list, view, delete, search, quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        let line = input.trim();
        if line.is_empty() { continue; }

        // Basic split: cmd and args
        let mut words = line.splitn(2, ' ');
        let cmd = words.next().unwrap_or("");
        let args = words.next().unwrap_or("").trim();

        match cmd {
            "add" => {
                // Usage: add "<title>" <content>
                let mut quote_split = args.splitn(2, '"'); // Try to split at the quote
                quote_split.next(); // Skip first empty
                if let Some(title) = quote_split.next() {
                    let rest = quote_split.next().unwrap_or("").trim();
                    if title.is_empty() || rest.is_empty() {
                        println!("Usage: add \"<title>\" <content>");
                        continue;
                    }
                    let path = note_path(title);
                    let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
                        Ok(f) => f,
                        Err(_) => {
                            println!("Note with this title already exists.");
                            continue;
                        },
                    };
                    writeln!(file, "# {}\n{}", title, rest).ok();
                    println!("Note '{}' added.", title);
                } else {
                    println!("Usage: add \"<title>\" <content>");
                }
            },
            "list" => {
                // List all notes (.md files)
                if let Ok(entries) = fs::read_dir("notes") {
                    let mut found = false;
                    for entry in entries.flatten() {
                        if let Some(name) = entry.path().file_stem() {
                            println!("{}", name.to_string_lossy());
                            found = true;
                        }
                    }
                    if !found {
                        println!("<no notes>");
                    }
                }
            },
            "view" => {
                // Usage: view <title>
                if args.is_empty() { println!("Usage: view <title>"); continue; }
                let path = note_path(args);
                let file = File::open(&path);
                match file {
                    Ok(f) => {
                        let reader = BufReader::new(f);
                        for line in reader.lines().flatten() {
                            println!("{}", line);
                        }
                    },
                    Err(_) => println!("Note not found."),
                }
            },
            "delete" => {
                // Usage: delete <title>
                if args.is_empty() { println!("Usage: delete <title>"); continue; }
                let path = note_path(args);
                match fs::remove_file(&path) {
                    Ok(_) => println!("Deleted."),
                    Err(_) => println!("Note not found."),
                }
            },
            "search" => {
                // Usage: search <word>
                if args.is_empty() { println!("Usage: search <word>"); continue; }
                let word = args;
                // For each note, check filename AND content
                let mut found_any = false;
                if let Ok(entries) = fs::read_dir("notes") {
                    for entry in entries.flatten() {
                        let filename = entry.file_name();
                        let title = filename.to_string_lossy().replace(".md", "");
                        let path = entry.path();
                        let mut found = false;
                        // Search in title first
                        if title.contains(word) {
                            println!("{}", title);
                            found = true;
                            found_any = true;
                        }
                        // Search in content
                        if !found {
                            if let Ok(file) = File::open(&path) {
                                let reader = BufReader::new(file);
                                for line in reader.lines().flatten() {
                                    if line.contains(word) {
                                        println!("{}", title);
                                        found_any = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                if !found_any {
                    println!("No notes found.");
                }
            },
            "quit" | "exit" => {
                println!("Bye!");
                break;
            },
            _ => println!("Unknown command."),
        }
    }
}
