# PlainMark

**A minimalist Markdown note manager for your terminal.  
Manage your notes as plain `.md` files, locally, with a handful of simple commands.  
No magic, no fuss—just Rust, the file system, and you.**

---

## Features
- `add "<title>" <content>`: create a note (title = filename, content = .md body)
- `list`: show all notes
- `view <title>`: show note content
- `delete <title>`: remove a note
- `search <word>`: show titles/notes matching a word
- All notes kept in the `/notes` folder next to this project

## Usage

Build and run:

```sh
cargo run
```

### Examples
```
> add "meeting" Discussed Rust memory ownership.  
> add "todo" Finish the CLI project.
> list
meeting
todo
> view meeting
# meeting
Discussed Rust memory ownership.
> delete meeting
Deleted.
> search CLI
todo
> quit
```

**All notes are local `.md` files, instantly readable/editable with any editor.**

---

## This project is dedicated to all the “very discreet” ones in tech  
The ones who see everything, clone everything, but never say a word…  
_we see you_ 👀

---
