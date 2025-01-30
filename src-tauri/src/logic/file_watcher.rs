use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use std::path::Path;

fn main() {
    // Setup the watcher and get the event receiver
    let rx = setup_watcher("/path/to/directory").expect("Failed to setup watcher");

    // Create a thread for the main logic
    let main_logic_handle = thread::spawn(move || {
        if let Err(e) = main_logic() {
            eprintln!("Application error: {:?}", e);
        }
    });

    // Handle the events in the main thread
    handle_events(rx);

    // Wait for the main logic thread to finish
    main_logic_handle.join().unwrap();
}

fn main_logic() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate some long-running logic
    loop {
        println!("Running main logic...");
        thread::sleep(Duration::from_secs(5));
    }
}

fn setup_watcher(path: &str) -> Result<Receiver<DebouncedEvent>, Box<dyn std::error::Error>> {
    // Create a channel to receive the events
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events
    let mut watcher = watcher(tx, Duration::from_millis(10))?;

    // Specify the directory to watch, with recursive mode to watch subdirectories
    let path_to_watch = Path::new(path);
    watcher.watch(path_to_watch, RecursiveMode::Recursive)?;

    println!("Watching for changes in {:?}", path_to_watch);

    // Return the receiver to be used in the main function
    Ok(rx)
}

fn handle_events(rx: Receiver<DebouncedEvent>) {
    // Handle the events
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) => println!("File created: {:?}", path),
                DebouncedEvent::Write(path) => println!("File modified: {:?}", path),
                DebouncedEvent::Remove(path) => println!("File deleted: {:?}", path),
                DebouncedEvent::Rename(old_path, new_path) => println!("File renamed from {:?} to {:?}", old_path, new_path),
                DebouncedEvent::Rescan => println!("Rescan requested"),
                _ => println!("Other event"),
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}
