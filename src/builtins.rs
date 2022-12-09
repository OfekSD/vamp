use std::{str::SplitWhitespace, path::Path, env};

use regex::{Regex, CaptureMatches};

use crate::{functions::get_home_dir, parsing::ALIASES};

pub fn cd(args: SplitWhitespace){
    let mut home_dir = String::new();
    get_home_dir(&mut home_dir);
    let new_dir = args.peekable().peek().map_or(home_dir.as_str(), |x| *x);
    let root = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root){
    eprintln!("{}",e);
    };
}

pub fn alias(args: SplitWhitespace){
    let re = Regex::new(r#"(\w+)="([^"]+)""#).unwrap();
    let args = args.collect::<Vec<&str>>().join(" ");
    if re.is_match(&args){
        let matches = re.captures_iter(&args);
        create_alias(matches);
    } else {
        print_aliases();
    }
}


fn print_aliases(){
    for (key,val) in ALIASES.lock().unwrap().iter(){
        println!("alias {}='{}'",key,val);   
    }
}

fn create_alias(mut matches: CaptureMatches) {
    
    while let Some(alias) = matches.next() {
        let key = alias.get(1).unwrap().as_str();
        let val = alias.get(2).unwrap().as_str();

        ALIASES.lock().unwrap().insert(key.to_string(), val.to_string());
    }
}