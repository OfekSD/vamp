use std::collections::HashMap;
use anyhow::{Result, Ok};
use std::process::{Command, Stdio, ChildStdout, Child, exit,};
use std::str::SplitWhitespace;
use std::sync::Mutex;
use crate::builtins::{cd, alias};
use lazy_static::{lazy_static};
use std::result::Result::Ok as Oks;

lazy_static! {
    pub static ref ALIASES: Mutex<HashMap<String,String>> = {
        let m = Mutex::new(HashMap::new());
        m
    };
    static ref VARIABLES: Mutex<HashMap<String,String>> = {
        let m = Mutex::new(HashMap::new());
        m
    };
}


enum Output {
    Str(String),
    Out(ChildStdout)
}

fn get_alias(alias: &str)-> Option<String>{
    match ALIASES.lock().unwrap().get(alias){
        Some(command) => Some(command.clone()),
        None => None,
    }
}

pub fn parse_input(input: &str) -> Result<()>{
    // spliting the commands 
    let mut commands = input.trim().split(" | ").peekable();
    let mut out = None;
    while let Some(command) = commands.next(){
        let stdout = if commands.peek().is_some(){
            Stdio::piped()
        } else {
            Stdio::inherit()
        };
        out = parse_command(command, out,stdout);
    }
    if let Some(mut out) = out{
        out.wait().unwrap();
    }
    Ok(())
}

fn decompose_command<'a>(input: &'a str) -> Option<(String,String)> {

    let mut command = String::new();
    let mut parts = input.trim().split_whitespace();
    if let Some(c)  = parts.next() {
        command = c.to_string();
    };
    if let Some(alias) = get_alias(&command){
        let command = format!("{} {}",alias, parts.collect::<Vec<&str>>().join(" "));
        
        let mut parts = command.trim().split_whitespace();
        let command;
        if let Some(c) = parts.next(){
            command = c.to_string();
        } else{
            command = String::new();
        };
        return Some((command ,parts.collect::<Vec<&str>>().join(" ").to_owned()));
    };
    
    
    Some((command.to_owned() ,parts.collect::<Vec<&str>>().join(" ").to_owned()))
}

fn parse_command(input: &str,stdin: Option<Child>,stdout: Stdio) -> Option<Child>{

    // let find_vars = Regex::new(r"\$(\w+)|\${(w+)}").unwrap();
    // let vars = find_vars.capture_locations();

    
   
    let  ( command, args )  = if let Some(c) = decompose_command(input){
        c
    } else{
        return  None;
    };
    let args = args.trim().split_whitespace();
    let command = command.as_str();
    match command {
        "cd" => {
        cd(args);
        None
        },
        "exit" => {
            exit(0)
        }
        "alias" => {
            alias(args);
            None
        }
        _ => run_command(command, args,stdin,stdout)
    }
}

fn run_command(command: &str, args: SplitWhitespace, stdin: Option<Child>, stdout: Stdio) -> Option<Child> {
    let stdin = stdin.map_or(Stdio::inherit(), |out| Stdio::from(out.stdout.unwrap()));
    let output =  Command::new(command)
    .args(args)
    .stdin(stdin)
    .stdout(stdout)
    .spawn();
    match output{
        Oks(output) => {
            // output.stdin.as_mut().unwrap().write(format!("{}\n",stdin).as_bytes()).unwrap();
            // let mut out = String::new();
            // output.stdout.as_mut().unwrap().read_to_string(&mut out).unwrap();
            // Ok(out)
            Some(output)
        },
        Err(e) => {
            eprintln!("{}",e);
            None
        },
    }

}   
