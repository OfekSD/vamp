use std::collections::HashMap;
use std::fmt::Alignment;
use std::{env};
use anyhow::Result;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio,};
use std::io::{Read, Write, ErrorKind};
use std::str::SplitWhitespace;
use std::sync::Mutex;

use lazy_static::lazy_static;
use regex::{Regex, Matches, CaptureMatches};


lazy_static! {
    static ref ALIASES: Mutex<HashMap<String,String>> = {
    let mut m = Mutex::new(HashMap::new());
    m
    };
}

fn get_home_dir(str: &mut String) {
    *str = env::home_dir().unwrap().to_str().unwrap().to_string();
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
    let command; 
    match parts.next() {
        Some(inp) => command = inp,
        None => return Ok(String::new())
    }
    let mut args: SplitWhitespace = parts;
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
            let args = args.collect::<String>();
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
