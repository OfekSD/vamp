use std::{path::Path, fs::File, io::{BufReader, BufRead, Write}};
use dirs::{home_dir};
use regex::{Regex};

use crate::parsing::parse_input;


pub fn get_home_dir(str: &mut String) {
    *str = home_dir().unwrap().to_str().unwrap().to_string();
}

pub fn get_history() -> Vec<String> {
    
    // getting the path to the history file
    let mut home_dir = String::new();
    get_home_dir(&mut home_dir);
    let path = &format!("{}/.vampstory",home_dir);
    let history_path = Path::new(path);

    // putting the history file in the vector
    let mut history: Vec<String> = Vec::new();
    match File::open(history_path){
        Ok(f) => {
            let reader = BufReader::new(&f);
            history = reader.lines().collect::<Result<_,_>>().unwrap();
        },
        Err(_) => {File::create(history_path).unwrap();},
    };

    history
}

pub fn write_history(command: &String, history: &mut Vec<String>){
    if history.len() == 1000{
        history.remove(0);
    }
    history.push(command.clone());

    // getting the history file
    let mut home_dir = String::new();
    get_home_dir(&mut home_dir); 
    let path = &format!("{}/.vampstory",home_dir);
    let history_path = Path::new(path);
    
    // writing the history to the file
    if let Ok(mut f) = File::create(history_path){
        for ln in history {
            writeln!(f,"{}",ln).unwrap();
            
        }
    }
}

pub fn run_script(file: &str) {
    let script = File::open(file).unwrap();
    let reader = BufReader::new(&script);
    for line in reader.lines(){
        let line = line.unwrap();
        parse_input(line.as_str());
    }
}


pub fn find_and_replace(input: &str, regex: &str, f: fn(&str) -> String) -> String {
   
    let mut command = String::new();
    let mut last_end = 0;
    let re = Regex::new(regex).unwrap();
    let results = re.captures_iter(input);
    
    for result in results{
        let mut result = result.iter().filter(|group|{
            group.is_some()
        });
        let out = result.next().unwrap().unwrap();
        let iner = result.next().unwrap().unwrap();
        let start = out.start();
        let end = out.end();
        let between = &input[iner.start()..iner.end()];
        
        let new_value = f(between);
        
        command.push_str(&input[last_end..start]);
        command.push_str(&new_value);
        last_end = end
        

    }

    command.push_str(&input[last_end..]);
    command
    
    
}