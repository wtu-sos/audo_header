use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false) 
}

fn is_skip(entry: &DirEntry) -> bool {
    if entry.path().is_dir() {
        return true;
    }

    return false;
}

#[derive(Debug)]
struct SingleCommit {
    author: String,
    date: String,
    content: String,
}

impl SingleCommit {
    pub fn new() -> Self {
        Self {
            author: String::new(),
            date: String::new(),
            content: String::new(),
        }
    }

    pub fn is_filled_complete(&self) -> bool {
        !self.author.is_empty() && !self.date.is_empty()
    }

    pub fn author(&mut self, author: &str) -> bool {
        let mut author = String::from(author);
        if author.starts_with("Author") {
            let sp = author.find(":").unwrap_or(0);
            let author_str = author.split_off(sp+1);
            println!("author: {}", author_str);
            self.author += author_str.trim_start();

            return true;
        }

        false
    }

    pub fn date(&mut self, date: &str) -> bool {
        let mut date = String::from(date);
        if date.starts_with("Date") {
            let sp = date.find(":").unwrap_or(0);
            let date_str = date.split_off(sp+1);
            println!("date: {}", date_str);
            self.date += date_str.trim_start();

            return true;
        }

        false
    }

    pub fn content(&mut self, content: &str) {
        self.content += content.trim_start();
    }
}

fn print_git_log(entry: &DirEntry) {
    println!("{}", entry.path().display());
    let mut cmd = Command::new("git");
    cmd.current_dir("/home/sos/Document/hp-io/");
    cmd.arg("log").arg(entry.path());
    println!("cmd: {:?}", cmd);
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            let output = String::from_utf8_lossy(&output.stdout);
            let log_deal = output.lines();
            let mut ci = SingleCommit::new();
            for content in log_deal {
                if content.starts_with("commit") {
                    if ci.is_filled_complete() {
                        println!("output: {:?}", ci);
                        // todo: 
                    }

                    ci = SingleCommit::new();
                    continue;
                }
                if ci.author(&content) || ci.date(&content) {
                    continue;
                } else {
                    ci.content(&content);
                }   
            }
            if ci.is_filled_complete() {
                println!("output: {:?}", ci);
                // todo: 
            }
        } else {
            println!("error: {:?}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

fn main() {
    let mut it = WalkDir::new("/home/sos/Document/hp-io/").into_iter();

    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };
        if is_hidden(&entry) {
            if entry.file_type().is_dir() {
                it.skip_current_dir();
            }
            continue;
        }

        if is_skip(&entry) {
            continue;
        }

        print_git_log(&entry);
    }
}
