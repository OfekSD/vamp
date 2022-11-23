mod input;
mod parsing;
use std::{io::{stdout, Write, BufReader, BufRead}, env, fs::File};
use parsing::{parse_input};
use anyhow::{Result};
use input::read_line;






fn main() -> Result<()>{
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        let mut cwd = env::current_dir().unwrap().display().to_string().split('/').last().unwrap().to_owned();
        if cwd.is_empty(){
            cwd = "".to_string();
        }
        let preset = format!("{} > ", cwd);
        print!("{}",preset);
        stdout().flush().unwrap();

        let mut input = String::new();

        // stdin().read_line(&mut input);
        read_line(preset, &mut input);
        let input = input.trim();

        parse_input(input)?
        

        
    }
}