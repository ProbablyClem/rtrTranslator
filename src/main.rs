extern crate glob;

use std::env::args;
use glob::glob;
use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::LineWriter;
use std::fs::{self};
use std::path::Path;
extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                //splits the input into pieces backward
                 input = input.trim_matches('\n').to_string();
                let mut args: Vec<&str> = input.rsplit(' ').collect();
                //we reverse the vec so that it's in same order as the input (so args[0] would be the keyword)
                args.reverse();
                //lowercase the first argument (supposed to bey the key word)
                args[0].to_lowercase();

                //println!("{}", &args[0]);
                if &args[0] == &"view" {
                    //if no arguments are specified
                    if input.len() == 4 {
                                    let mut cpt = 0;
                                    let path = env::current_dir().unwrap();
                                    let new_path = format!("{}/*.txt", path.to_str().unwrap());
                                    println!("{}", new_path);
                                    for entry in glob(&new_path).unwrap() {
                                        println!("{}", entry.unwrap().display());
                                        cpt = cpt +1;
                                    }
                                    if cpt < 1 {
                                        println!("no txt files in this directory");
                                    }
                    }
                    //with arguments
                    else {
                        if input.len() > 5 {
                            let mut cpt = 0;
                            let mut path = input[5..].to_string();
                            path.truncate(path.len() -1);
                            path.push_str("**/*.txt");
                            println!("{}", path);
                            for entry in glob(&path.to_string()).unwrap() {
                                println!("{}", entry.unwrap().display());
                                cpt = cpt +1;
                            }
                            if cpt < 1 {
                                println!("no txt files in this directory");
                            }
                        }
                    }
                }
                //create origin file from current dir
                else if &args[0] == &"create" {
                    if input.len() == 6 {
                        CreateOrigin(env::current_dir().unwrap().to_str().unwrap().to_string(), "origin".to_string());
                    }
                    //create origin file from specified dir
                    else {
                        if &args.len() == &3 {
                            if args[1] == "." {
                                CreateOrigin(env::current_dir().unwrap().to_str().unwrap().to_string(), args[2].to_string());
                            }
                            else {CreateOrigin(args[1].to_string(), args[2].to_string());}
                            
                        }
                        else {                        
                            CreateOrigin(args[1].to_string(), "origin".to_string());
                        }
                    }
                } 
                else if &args[0] == &"quit" {
                    std::process::exit(0);
                } 
                else {
                    println!("Unknown {} command", args[0]);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    fn CreateOrigin(mut path : String, lang : String){
        //the origin file vect
        let mut origin : Vec<String> = Vec::new();
        //stores the content of every.rs files in the directory
        let mut fileBuf = String::new();
        path.push_str("/**/*.rs");
        println!("Creating from path {}", path);


        //read every .rs file and happend it in the fileBuf variable
        for entry in glob(&path).unwrap() {
            let f = File::open(&entry.unwrap()).unwrap();
            let mut f = BufReader::new(f);
            f.read_to_string(&mut fileBuf).expect("failed to read file");
        }

        let mut cpt = 0;
        let mut c = fileBuf.chars();
        let mut lineBuf = String::new();
        let mut buff : Vec<char> = vec![' '; 6];
        let mut inside : bool = false;

        for c in fileBuf.chars() {            
            buff[0] = buff[1];
            buff[1] = buff[2];
            buff[2] = buff[3];
            buff[3] = buff[4];
            buff[4] = buff[5];
            buff[5] = c;
            // for i in &buff {
            //     println!("{}" ,i);
            // }
            // println!("------------");
            
            
            if buff[0] == 'r' && buff[1] == 't' && buff[2] == 'r' && buff[3] == '(' && buff[4] == '\"'{
                inside = true;
               // println!("{}", inside);
            }
            else if c == '\"'{
                inside = false;
                //println!("{}", &lineBuf);
                origin.push(lineBuf.clone());
                lineBuf.clear();
            }

            if inside == true{
                lineBuf.push(c);
            }   
        }

        println!("{}", origin.len());
        for i in &origin {
            println!("{}", i);
        }
            //export the file
            println!("{}", &path);
            path.truncate(path.len() - 7);
            path.push_str(&lang);
            path.push_str(".txt");
            println!("{}", &path);
            File::create(&path).expect("couldn't create file ");
            let f = File::create(path).expect("Couldn't create file");
            let mut f = LineWriter::new(f);
            &origin.sort_unstable();
            &origin.dedup();
            for i in 1..origin.len() -1 {
                origin[i].push_str("\n");
                f.write_all(origin[i].as_bytes()).expect("Couldn't write");
            }
            f.write_all(origin[origin.len() -1].as_bytes()).expect("Couldn't write");
    }

    

}


