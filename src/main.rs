mod input;
mod parsing;
mod functions;
mod builtins;
use std::{io::{stdout, Write, BufReader, BufRead}, fs::{File, self}};
use parsing::{parse_input};
use std::env;
use anyhow::{Result};
use input::read_line;
use functions::{get_history,write_history, run_script, get_home_dir};





fn main() -> Result<()>{

    let args: Vec<String> = env::args().collect();
    if let Some(file) = args.get(1){
        run_script(file);
        return Ok(());
    };
    let mut home_dir = String::new();
    get_home_dir(&mut home_dir);
    match File::open(format!("{home_dir}/.vamp_blood")){
        Ok(f) => {
            let reader = BufReader::new(&f);
            reader.lines().for_each(|line|{
                parse_input(line.unwrap().as_str());
            });
        },
        Err(_) => {fs::OpenOptions::new().create(true).write(true).open(format!("{home_dir}/.vamp_blood")).unwrap();},
    };
    let mut history = get_history();
    loop {
        // getting current directory
        let mut cwd = env::current_dir().unwrap().display().to_string().split('/').last().unwrap().to_owned();
        if cwd.is_empty(){
            cwd = "".to_string();
        }
        // printing begining preset
        let preset = format!("{} > ", cwd);
        print!("{}",preset);
        stdout().flush().unwrap();

        // getting input
        let mut input = String::new();
        read_line(preset, &mut input,&history)?;
        
        if &input != &"".to_string(){
            write_history(&input,&mut history);
        }

        let input = input.trim();
        
        // parsing input
        parse_input(input);

        

        
    }
}

