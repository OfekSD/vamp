use std::{env, path::Path, fs::{File, read}, io::{Read, BufReader, BufRead, Write}};



pub fn get_home_dir(str: &mut String) {
    *str = env::home_dir().unwrap().to_str().unwrap().to_string();
}


pub fn get_history() -> Vec<String> {
    

    let mut home_dir = String::new();
    get_home_dir(&mut home_dir); 
    let path = &format!("{}/.vampstory",home_dir);
    let history_path = Path::new(path);
    let mut history: Vec<String> = Vec::new();
    match File::open(history_path){
        Ok(f) => {
            let reader = BufReader::new(&f);
            history = reader.lines().collect::<Result<_,_>>().unwrap();
        },
        Err(_) => {File::create(history_path);},
    };

    history
}

pub fn write_history(command: &String, history: &mut Vec<String>){
    history.push(command.clone());

    let mut home_dir = String::new();
    get_home_dir(&mut home_dir); 
    let path = &format!("{}/.vampstory",home_dir);
    let history_path = Path::new(path);
    match File::create(history_path){
        Ok(mut f) => for ln in history {
            writeln!(f,"{}",ln).unwrap();
            
        },
        Err(_) => todo!(),
    }
}