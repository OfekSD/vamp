use std::collections::HashMap;
use std::{env};
use anyhow::Result;
use std::path::Path;
use std::process::{Command, Stdio,};
use std::io::{Read, Write};
use std::str::SplitWhitespace;
use std::sync::Mutex;
use crate::functions::get_home_dir;
use lazy_static::lazy_static;
use regex::{Regex, CaptureMatches};


lazy_static! {
    static ref ALIASES: Mutex<HashMap<String,String>> = {
    let mut m = Mutex::new(HashMap::new());
    m
    };
}


fn print_aliases(){
    for (key,val) in ALIASES.lock().unwrap().iter(){
        println!("alias {}='{}'",key,val);   
    }
}

fn create_alias(mut matches: CaptureMatches) -> Result<()>{
    // regex = (\w+)=\"([^"]+)"
    
    while let Some(alias) = matches.next() {
        let key = alias.get(1).unwrap().as_str();
        let val = alias.get(2).unwrap().as_str();

        ALIASES.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    Ok(())
}

fn get_command(alias: &str)-> Option<String>{
    match ALIASES.lock().unwrap().get(alias){
        Some(command) => Some(command.clone()),
        None => None,
    }
}

pub fn parse_input(input: &str) -> Result<()>{
    // spliting the commands 
    let mut commands = input.trim().split(" | ").peekable();

    let mut output = String::new();
      
    while let Some(command) = commands.next(){
        output = parse_command(command,output)?;
    }  
    output = output.trim().to_string(); 
    if !output.is_empty(){
        println!("{}",output);
    }


    Ok(())
}

fn parse_command(input: &str,stdin: String) -> Result<String>{

    let mut parts = input.trim().split_whitespace();
    let mut command; 
    match parts.next() {
        Some(c) => command = c,
        None => return Ok(String::new())
    }
    let mut full_command = String::new();
    match get_command(&command){
        Some(alias) => {
            full_command = format!("{} {}",alias, parts.clone().collect::<String>());
        },
        None => {},
    };
    command = if full_command.len() > 0{
        parts = full_command.trim().split_whitespace().to_owned();
        match parts.next(){
            Some(c) => c,
            None => return Ok(String::new()),
        }
    } else { command };

    let mut args = parts;
    match command {
        "cd" => {
        let mut home_dir = String::new();
        get_home_dir(&mut home_dir);
        let new_dir = args.peekable().peek().map_or(home_dir.as_str(), |x| *x);
        let root = Path::new(&new_dir);
        env::set_current_dir(&root)?;
        Ok(String::new())
    }
        "alias" => {
            let re = Regex::new(r#"(\w+)="([^"]+)""#).unwrap();
            let args = args.collect::<Vec<&str>>().join(" ");
            if re.is_match(&args){
                let matches = re.captures_iter(&args);
                create_alias(matches);
            } else {
                print_aliases();
            }
            Ok(String::new())
        }
        _ => Ok(run_command(command, args,stdin)?)
    }
}



fn run_command(command: &str, args: SplitWhitespace, stdin: String) -> Result<String> {
    let mut output;
     
    match Command::new(command)
    .args(args)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn(){
        Ok(out) => output = out,
        Err(_) => {println!("{}: command not found",command);
    return  Ok(String::new());},
    }
    

    output.stdin.as_mut().unwrap().write(format!("{}\n",stdin).as_bytes()).unwrap();
    output.wait().unwrap();
    let mut out = String::new();
    output.stdout.as_mut().unwrap().read_to_string(&mut out).unwrap();
    Ok(out)

}
