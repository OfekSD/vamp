use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{Read,Write};
use std::env;
use regex::Regex;
use std::process::{Command, Stdio, Child, exit,};
use std::sync::Mutex;
use crate::builtins::{cd, alias, create_variable};
use crate::functions::{find_and_replace, get_args, get_home_dir};
use lazy_static::{lazy_static};

#[derive(PartialEq, Eq)]
enum WriteMode {
    New,
    Append,
    None,
}




lazy_static! {
    pub static ref ALIASES: Mutex<HashMap<String,String>> = {
        let m = Mutex::new(HashMap::new());
        m
    };
    pub static ref VARIABLES: Mutex<HashMap<String,String>> = {
        let m = Mutex::new(HashMap::new());
        m
    };
}



fn get_alias(alias: &str)-> Option<String>{
    match ALIASES.lock().unwrap().get(alias){
        Some(command) => Some(command.clone()),
        None => None,
    }
}

pub fn parse_input(input: &str) {
    if input.starts_with('#'){
        return ;
    }
    // spliting the commands 
    let mut commands = input.trim().split(" | ").peekable();
    let mut out = None;
    let mut write_mode = WriteMode::None; 
    while let Some(mut command) = commands.next(){
        let mut file = "";
        if command.contains('>'){
            if command.contains(">>"){
                (command, file) = command.split_once(">>").unwrap();
                write_mode = WriteMode::Append;
            } else {
                (command, file) = command.split_once('>').unwrap();
                write_mode = WriteMode::New;
            }
        }
        let stdout = if commands.peek().is_some() || write_mode != WriteMode::None {
            Stdio::piped()
        } else {
            Stdio::inherit()
        };
        out = parse_command(command, out,stdout);
        if write_mode != WriteMode::None{
            let mut output = String::new();
            if let Some(out_child) = out{
                out_child.stdout.unwrap().read_to_string(&mut output).unwrap();
                out = None
            }
            match write_mode{
                WriteMode::New => {
                    match File::create(file.trim()){
                        Ok(mut f) => writeln!(f,"{}",output).unwrap(),
                        Err(e) => eprintln!("{e}"),
                    };
                }
                WriteMode::Append => {
                    match fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(file.trim())
                    {
                        Ok(mut f) => writeln!(f,"{}",output).unwrap(),
                        Err(e) => eprintln!("{e}"),
                    }

                },
                _ => (),
                
                

            }
        }
    }
    if let Some(mut out) = out{
        out.wait().unwrap();
    }

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
    let mut home_dir = String::new();
    get_home_dir(&mut home_dir);

    let binding = input.replace('~', &home_dir);
    let input = binding.as_str();

    let inlines = find_and_replace(input,r"(?:`([^`]+)`|\$\((.+)\))",|command|{
        let mut result = String::new();
        if let Some(output) = parse_command(command,None,Stdio::piped()){
            let mut out = String::new(); 
            output.stdout.unwrap().read_to_string(&mut out).unwrap_or_default();
            for line in out.lines(){
                result.push_str(&format!(" {}", line));
            }
        }
        result
    });
    let input = inlines.as_str();    
    
    // Regex For Variables "([^"]+)"|'([^']+)'|([\S]+)
    

    
    let vars = find_and_replace(input, r"\$(\w+)" , |var_name|{
        env::var(&var_name).
        unwrap_or_else(|_|{
            VARIABLES.lock().unwrap()
            .get(var_name)
            .unwrap_or(&String::new()).to_owned()
        })
    });

    let input = vars.as_str();
    
    let create_vars = Regex::new(r#"^(\w+)=(.+)"#).unwrap();
    if let Some(var) = create_vars.captures_iter(input).next(){
        let name = var.get(1).unwrap().as_str();
        let mut value = var.get(2).unwrap().as_str();
        if value.starts_with('\'') && value.ends_with('\''){
            value = value.strip_prefix("'").unwrap().strip_suffix("'").unwrap();
        } else if value.starts_with('"') && value.ends_with('"') {
            value = value.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        }
        create_variable(name,value);   
        return None;
    }

    
   
    let  ( command, args )  = if let Some(c) = decompose_command(input){
        c
    } else{
        return  None;
    };
    let args = get_args(args);
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



fn run_command(command: &str, args: Vec<String>, stdin: Option<Child>, stdout: Stdio) -> Option<Child> {
    if command == "" {
        return None;
    }
    let stdin = stdin.map_or(Stdio::inherit(), |out| Stdio::from(out.stdout.unwrap()));
    let output =  Command::new(command)
    .args(args)
    .stdin(stdin)
    .stdout(stdout)
    .spawn();
    match output{
        Ok(output) => {
            Some(output)
        },
        Err(e) => {
            eprintln!("{command}: {}",e);
            None
        },
    }

}   

