# PlainMark 📝

**A minimalist Markdown note manager for your terminal.  
Manage your notes as plain `.md` files — locally and transparently, with a handful of simple commands.  
No magic — just Rust, the filesystem, and you.**

---

## 🧩 What This Is

PlainMark is a tiny CLI tool built to explore:

- simple file-backed storage  
- Markdown as a natural data model  
- minimal, explicit commands  
- how small systems behave when nothing is hidden  

Each note = one `.md` file inside a local `notes/` directory.  
Readable. Hackable. Permanent.

---

## 🚀 Features

- `add "<title>" <content>` — create a Markdown note  
- `list` — list all notes  
- `view <title>` — display a note  
- `delete <title>` — remove a note  
- `search <word>` — search in titles and content  
- all notes stored in a local `notes/` folder  
- folder created automatically on first run

---

## ⚙️ Installation & Running

Clone the repository:

    git clone https://github.com/whispem/plainmark
    cd plainmark

Run the CLI:

    cargo run

Example session:

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

All notes are local `.md` files, instantly readable/editable with any editor.

---

## 📌 Current Status

Part of a broader Rust learning journey (2025).  
Current phase: practicing filesystem operations and small-tool clarity.

Implemented so far:

- Markdown-based persistence  
- automatic `notes/` directory creation  
- add / list / view / delete  
- text search  
- interactive CLI  
- basic error handling  

Each feature is intentionally small so the behavior stays fully understandable.

---

## 📈 Learning Roadmap

Planned improvements:

- safer parsing & error handling  
- richer search (regex, maybe fuzzy)  
- minimal indexing layer  
- optional tags  
- timestamps  
- improved CLI UX  
- cleaner module structure  

Incremental by design — clarity first.

---

## 🦀 Why Rust?

This project helps practice:

- ownership & borrowing with file-backed data  
- clean error handling  
- building tiny, explicit subsystems  
- designing simple Rust CLIs  
- understanding persistence and filesystem I/O  

Perfect playground for learning filesystem-based storage.

---

## 📚 Resources

- Rust documentation  
- filesystem I/O guides  
- CLI design patterns  
- articles on small, clear system design  

Updated as I learn.

---

## 🗒️ Notes

A beginner-friendly exploratory project — intentionally small.  
The code evolves as I deepen my understanding of Rust and system design.  
Feedback welcome. 🙏

---

## 🎩 Dedication

Dedicated to all the “very discreet” ones in tech.  
Those who watch everything, clone everything…  
…and never say a word.

We see you. 👀

---

Built while exploring Rust and file-backed storage — 2025 🦀

---

If you spot anything that could be written in a more idiomatic or elegant Rust style, I’m always curious to understand why.