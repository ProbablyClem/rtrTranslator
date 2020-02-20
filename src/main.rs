#![allow(unused_must_use)]
#![allow(non_snake_case)]

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

                //println!("{}", &args[0]);
                if &args[0] == &"view" {
                    //if no arguments are specified
                    if input.len() == 4 {
                        let mut cpt = 0;
                        let path = env::current_dir().expect("coulnd't get the current dir");
                        let new_path = format!("{}/lang/*.txt", path.to_str().unwrap());
                        //println!("{}", new_path);
                        for entry in match glob(&new_path){
                            Ok(path) => path,
                            Err(e) => {
                                println!("couln't open dir {}, Error : {}", &new_path, e);
                                main();
                                Err(e).unwrap()
                            }
                        } {
                            println!("{}", entry.unwrap().display());
                            cpt = cpt + 1;
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
                            path.truncate(path.len() - 1);
                            path.push_str("**/*.txt");
                            //println!("{}", path);
                            for entry in glob(&path.to_string()).unwrap() {
                                println!("{}", entry.unwrap().display());
                                cpt = cpt + 1;
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
                        let origin =
                            CreateOrigin(env::current_dir().unwrap().to_str().unwrap().to_string());
                        exportFile(
                            env::current_dir().unwrap().to_str().unwrap().to_string(),
                            "origin".to_string(),
                            origin,
                        )
                        .unwrap();
                    }
                    //create origin file from specified dir
                    else {
                        if &args.len() == &3 {
                            if args[1] == "." {
                                if args[2] == "origin" {
                                    let origin = CreateOrigin(
                                        env::current_dir().unwrap().to_str().unwrap().to_string(),
                                    );
                                    exportFile(
                                        env::current_dir().unwrap().to_str().unwrap().to_string(),
                                        "origin".to_string(),
                                        origin,
                                    )
                                    .unwrap();
                                } else {
                                    let newFile = CreateNew(
                                        LoadOrigin(
                                            env::current_dir()
                                                .unwrap()
                                                .to_str()
                                                .unwrap()
                                                .to_string(),
                                        )
                                        .unwrap(),
                                    );
                                    exportFile(args[1].to_string(), args[2].to_string(), newFile)
                                        .unwrap();
                                }
                            } else {
                                let origin = LoadOrigin(args[1].to_string()).unwrap();
                                let newFile = CreateNew(origin);
                                exportFile(args[1].to_string(), args[2].to_string(), newFile)
                                    .unwrap();
                            }
                        } else {
                            let origin = CreateOrigin(args[1].to_string());
                            exportFile(args[1].to_string(), "origin".to_string(), origin).unwrap();
                        }
                    }
                } else if &args[0] == &"quit" {
                    std::process::exit(0);
                } else {
                    println!("Unknown {} command", args[0]);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    fn CreateOrigin(mut path: String) -> Vec<String> {
        //the origin file vect
        let mut origin: Vec<String> = Vec::new();
        //stores the content of every.rs files in the directory
        let mut fileBuf = String::new();
        path.push_str("/**/*.rs");
        println!("Creating from path {}", path);

        //read every .rs file and happend it in the fileBuf variable
        //for entry in glob(&path).unwrap_or_else(inputError) {
        // for entry in glob(&path).unwrap() {
        //     let f = File::open(&entry.unwrap()).unwrap();
        //     let mut f = BufReader::new(f);
        //     f.read_to_string(&mut fileBuf).expect("failed to read file");
        // }

        for entry in glob(&path).unwrap() {
            // let mut f = File::open(&entry.unwrap());
            //let mut f : std::io::Result<fs::File> = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
            
            let f = match File::open(&entry.unwrap()) {
                Ok(file) => Ok(file),
                Err(e) => {
                    println!("couldn't open file, Error {}", e);
                    main(); 
                    Err(e)
                },
            };

            let mut f = BufReader::new(f.unwrap());
            f.read_to_string(&mut fileBuf).expect("failed to read file");
        }

        let mut lineBuf = String::new();
        let mut buff: Vec<char> = vec![' '; 6];
        let mut inside: bool = false;
        let mut lines : usize = 0;

        for c in fileBuf.chars() {
            buff[0] = buff[1];
            buff[1] = buff[2];
            buff[2] = buff[3];
            buff[3] = buff[4];
            buff[4] = buff[5];
            buff[5] = c;

            if buff[0] == 'r'
                && buff[1] == 't'
                && buff[2] == 'r'
                && buff[3] == '('
                && buff[4] == '\"'
            {
                inside = true;
            } else if c == '\"' && buff[4] != '\\' {
                inside = false;
                lineBuf.push('\n');
                origin.push(lineBuf.clone());
                lines = lines +1;
                lineBuf.clear();
            }

            
            if inside == true && c != '\\' || (c == '\\' && buff[4] == '\\') {
                lineBuf.push(c);
            }
        }
        return origin;
    }

    fn CreateNew(origin: Vec<String>) -> Vec<String> {
        let mut newVec: Vec<String> = Vec::new();
        let mut lineBuf = String::new();
        for i in 0..origin.len() {
            println!("translation for :");
            println!("{}", origin[i]);
            io::stdin().read_line(&mut lineBuf);
            newVec.push(lineBuf.clone());
            lineBuf.clear();
        }
        return newVec;
    }

    fn LoadOrigin(mut path: String) -> Result<Vec<String>, io::Error> {
        println!("{}", &path);
        path.push_str("/lang/origin.txt");
        println!("{}", &path);
        let f = match File::open(path) {
            Ok(file) => { Ok(file) },
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

    fn exportFile(mut path: String, lang: String, mut vec: Vec<String>) -> io::Result<()> {

        //println!("{}", &path);
        path.push_str("/lang/");
        path.push_str(&lang);
        path.push_str(".txt");
        //println!("{}", &path);
        match fs::create_dir_all("./lang"){
            Ok(()) => Ok(()),
            Err(e) => {
                println!("Couldn't create dir ./lang. Error : {}", e);
                main();
                Err(e)
            }
        };

        // match File::create(&path){
        //     Ok(file) => Ok(file),
        //     Err(e) => {
        //         println!("Couldn't create file at path {} Error : {}", &path, e);
        //         main();
        //         Err(e)
        //     }
        // };

        let f = match File::create(&path){
            Ok(file) => Ok(file),
            Err(e) => {
                println!("Couldn't create file at path {} Error : {}", &path, e);
                main();
                Err(e)
            }
        };
        let mut f = LineWriter::new(f.unwrap());
        &vec.sort_unstable();
        &vec.dedup();
        for i in 0..vec.len() - 1 {
            if vec[i] != "\n" {
 //               println!("{}", vec[i]);
            match f.write_all(vec[i].as_bytes()){
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("couldn't write to file {}, error : {}", &path, e);
                    main();
                    Err(e)
                }
            };
            }         
        }
        let mut lastElement = vec[vec.len() - 1].clone();
        lastElement.pop();
        match f.write_all(lastElement.as_bytes()) {
            Ok(()) => Ok(()),
                Err(e) => {
                    println!("couldn't write to file {}, error : {}", &path, e);
                    main();
                    Err(e)
                }
        };
        println!("file created!");
        return Ok(());
    }
}
