mod input;
mod parsing;
mod functions;
use std::io::{stdout, Write};
use parsing::{parse_input};
use std::env;
use anyhow::{Result};
use input::read_line;
use functions::{get_history,write_history};





fn main() -> Result<()>{
    let mut history = get_history();
    loop {
        // use the `>` character as the prompt
        let mut cwd = env::current_dir().unwrap().display().to_string().split('/').last().unwrap().to_owned();
        if cwd.is_empty(){
            cwd = "".to_string();
        }
        let preset = format!("{} > ", cwd);
        print!("{}",preset);
        stdout().flush().unwrap();

        let mut input = String::new();

        read_line(preset, &mut input,&history);
        if &input != &"".to_string(){
            write_history(&input,&mut history);
        }
        let input = input.trim();

        parse_input(input)?;

        

        
    }
}