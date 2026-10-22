use notify::{Event, RecursiveMode, Result, Watcher};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

// scan directory data structure
#[derive(Debug, Clone)]
pub enum Kind {
    File,
    Symlink,
    Other,
    Dir(BTreeMap<0sString, Node>),
}

fn scan_directory() {
    // returns the directory structure as a Merkel tree
    todo!();
}

fn handle_event(event: Event) {
    todo!();
    // created file
    // 1. run the scan and resolve current state against metadata db cache
    // 2. create WAL entry

    // delete file
    // 1. run the scan and resolve current state against metadata db cache
    // 2. create WAL entry

    // modified file
    // 1. run the scan and resolve current state against meatadata db cache
    // 2. create WAL log (currently treat as overwritten file)
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("/test_dir/"), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        // watcher is live and events are coming in
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn write_files_test() {
        let mut file =
            File::create("test_dir/test_file1.txt").expect("Faild to create or open file");
        file.write_all(b"Example text").expect("Failed to write");
    }
}
