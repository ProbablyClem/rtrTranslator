#![allow(unconditional_recursion)]

extern crate glob;

use glob::glob;
use std::env;
use std::fs::File;
use std::fs::{self};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
extern crate walkdir;

fn main() {
    println!("Waiting for instruction:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //splits the input into pieces backward
                input = input.trim_matches('\n').to_string();
                let mut args: Vec<&str> = input.rsplit(' ').collect();
                //we reverse the vec so that it's in same order as the input (so args[0] would be the keyword)
                args.reverse();
                //lowercase the first argument (supposed to bey the key word)
                args[0].to_lowercase();

                if args[0] == "view" {
                    //if no arguments are specified
                    if input.len() == 4 {
                        let mut cpt = 0;
                        let path = env::current_dir().expect("coulnd't get the current dir");
                        let new_path = format!("{}/lang/*.txt", path.to_str().unwrap());
                        for entry in match glob(&new_path) {
                            Ok(path) => path,
                            Err(e) => {
                                println!("couln't open dir {}, Error : {}", &new_path, e);
                                main();
                                Err(e).unwrap()
                            }
                        } {
                            println!("{}", entry.unwrap().display());
                            cpt += 1;
                        }
                        if cpt < 1 {
                            println!("no txt files in this directory");
                        }
                    }
                    //with arguments
                    else if input.len() > 5 {
                        let mut cpt = 0;
                        let mut path = input[5..].to_string();
                        path.truncate(path.len());
                        path.push_str("/**/*.txt");
                        for entry in match glob(&path.to_string()) {
                            Ok(path) => path,
                            Err(e) => {
                                println!("couldn't access the {} directory. Error : {} ", path, e);
                                main();
                                Err(e).unwrap()
                            }
                        } {
                            println!("{}", entry.unwrap().display());
                            cpt += 1;
                        }
                        if cpt < 1 {
                            println!("no txt files in this directory");
                        }
                    }
                }
                //create origin file from current dir
                else if args[0] == "create" {
                    if input.len() == 6 {
                        let origin = create_origin(
                            env::current_dir().unwrap().to_str().unwrap().to_string(),
                        );

                        export_file(
                            env::current_dir().unwrap().to_str().unwrap().to_string(),
                            "origin".to_string(),
                            origin,
                        )
                        .unwrap();
                    }
                    //create origin file from specified dir
                    else if args.len() == 3 {
                        if args[1] == "." {
                            if args[2] == "origin" {
                                let origin = create_origin(
                                    env::current_dir().unwrap().to_str().unwrap().to_string(),
                                );
                                export_file(
                                    env::current_dir().unwrap().to_str().unwrap().to_string(),
                                    "origin".to_string(),
                                    origin,
                                )
                                .unwrap();
                            } else {
                                let new_file = create_new(
                                    load_origin(
                                        env::current_dir().unwrap().to_str().unwrap().to_string(),
                                    )
                                    .unwrap(),
                                );
                                export_file(args[1].to_string(), args[2].to_string(), new_file)
                                    .unwrap();
                            }
                        } else {
                            let origin = load_origin(args[1].to_string()).unwrap();
                            let new_file = create_new(origin);
                            export_file(args[1].to_string(), args[2].to_string(), new_file)
                                .unwrap();
                        }
                    } else {
                        let origin = create_origin(args[1].to_string());
                        export_file(args[1].to_string(), "origin".to_string(), origin).unwrap();
                    }
                } else if args[0] == "quit" {
                    std::process::exit(0);
                } else {
                    println!("Unknown {} command", args[0]);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    fn create_origin(mut path: String) -> Vec<String> {
        //the origin file vect
        let mut origin: Vec<String> = Vec::new();
        //stores the content of every.rs files in the directory
        let mut file_buf = String::new();

        path.push_str("/**/*.rs");
        println!("Creating from path {}", path);

        //read every .rs file and appends it in the file_buf variable

        for entry in match glob(&path) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Couldn't access files. Error {}", e);
                main();
                Err(e).unwrap()
            }
        } {
            let f = match File::open(&entry.unwrap()) {
                Ok(file) => file,
                Err(e) => {
                    println!("couldn't open file, Error {}", e);
                    main();
                    Err(e).unwrap()
                }
            };

            let mut f = BufReader::new(f);
            f.read_to_string(&mut file_buf)
                .expect("failed to read file");
        }

        let mut line_buf = String::new();
        let mut buff: Vec<char> = vec![' '; 7];
        let mut inside: bool = false;
        let mut lines: usize = 0;

        for c in file_buf.chars() {
            buff[0] = buff[1];
            buff[1] = buff[2];
            buff[2] = buff[3];
            buff[3] = buff[4];
            buff[4] = buff[5];
            buff[5] = buff[6];
            buff[6] = c;

            if  (buff[1] == 'r'
                && buff[2] == 't'
                && buff[3] == 'r'
                && buff[4] == '('
                && buff[5] == '\"')
                ||
                (buff[0] == 'r'
                && buff[1] == 't'
                && buff[2] == 'r'
                && buff[3] == '!'
                && buff[4] == '('
                && buff[5] == '\"')
            {
                inside = true;
            } else if c == '\"' && buff[5] != '\\' {
                inside = false;
                line_buf.push('\n');
                origin.push(line_buf.clone());
                lines += 1;
                line_buf.clear();
            }

            if inside && (c != '\\' || buff[5] == '\\') {
                line_buf.push(c);
            }
        }

        //check if there is at least one line in the origin vector
        if lines == 0 {
            println!("nothing to create from in {}", path);
            main();
        }

        origin.sort_unstable();
        origin.dedup();
        return origin;
    }

    fn create_new(origin: Vec<String>) -> Vec<String> {
        let mut new_vec: Vec<String> = Vec::new();
        let mut line_buf = String::new();
        for line in &origin {
            println!("translation for :");
            println!("{}", line);
            io::stdin().read_line(&mut line_buf).unwrap();
            new_vec.push(line_buf.clone());
            line_buf.clear();
        }
        return new_vec;
    }

    fn load_origin(mut path: String) -> Result<Vec<String>, io::Error> {
        path.push_str("/lang/origin.txt");
        let f = match File::open(path) {
            Ok(file) => Ok(file),
            Err(e) => {
                println!("Couldn't load origin at specified path. Error : {}", e);
                main();
                Err(e)
            }
        };
        let f = BufReader::new(f.unwrap());
        let mut v = Vec::new();

        for line in f.lines() {
            v.push(line?);
        }
        return Ok(v);
    }

    fn export_file(mut path: String, lang: String, vec: Vec<String>) -> io::Result<()> {
        match fs::create_dir_all(format!("{}/lang", &path)) {
            Ok(_) => (),
            Err(e) => {
                println!("Couldn't create dir {}/lang. Error : {}", &path, e);
                main();
            }
        };

        path.push_str("/lang/");
        path.push_str(&lang);
        path.push_str(".txt");

        let f = match File::create(&path) {
            Ok(file) => Ok(file),
            Err(e) => {
                println!("Couldn't create file at path {} Error : {}", &path, e);
                main();
                Err(e)
            }
        };
        let mut f = LineWriter::new(f.unwrap());

        for i in vec.iter().take(vec.len() - 1) {
            if i != "\n" {
                match f.write_all(i.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("couldn't write to file {}, error : {}", &path, e);
                        main();
                    }
                };
            }
        }
        let mut last_element = vec[vec.len() - 1].clone();
        last_element.pop();
        match f.write_all(last_element.as_bytes()) {
            Ok(()) => (),
            Err(e) => {
                println!("couldn't write to file {}, error : {}", &path, e);
                main();
            }
        };
        println!("file created!");
        return Ok(());
    }
}
