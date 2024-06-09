use std::env;
use std::io::prelude::*;

// put all the file stuff to a function

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = "output.txt";

    if args.len() == 1 {
        println!("No arguments provided");
    } else if args.len() == 2 && args[1] == "--help" {
        println!("Usage: {} [the note that you want to take] [-t = todo -lt = long time todo]", args[0]);
    } else if args.len() == 2 && args[1] == "--version" {
        println!("Version 1.0.0");
    } else if args[1] == "-t" {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("todo.txt")
            .unwrap();

        for arg in &args[2..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else if args[1] == "-lt" {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("longtime.txt")
            .unwrap();

        for arg in &args[2..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else if args[1] == "-n" {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("note.txt")
            .unwrap();

        for arg in &args[2..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else if args[1] == "-d" {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("done.txt")
            .unwrap();

        for arg in &args[2..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else if args[1] == "-r" {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("reminder.txt")
            .unwrap();

        for arg in &args[2..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    } else if args[1] == "-s" {
        let _file = std::fs::OpenOptions::new()
            .write(true);
    } else {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename)
            .unwrap();

        for arg in &args[1..] {
            file.write_all(arg.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }
}
