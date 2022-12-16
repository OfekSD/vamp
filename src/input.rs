use std::io::Write;

use anyhow::{Result, Ok};
use console::{Term, Key};


#[derive(PartialEq, Eq)]
enum State {
    Input,
    ReverseSearch,
    Send,
} 
    
const CONTROL_R: char = '';


fn move_cursor_to_start(term: &Term,cursor_index:&mut usize) -> Result<()>{
    term.move_cursor_left(*cursor_index)?;
    *cursor_index = 0;
    Ok(())
}

fn move_cursor_to_end(term: &Term,cursor_index:&mut usize, inp: &String) -> Result<()>{
    move_cursor_to_start(term, cursor_index)?;
    *cursor_index = inp.len(); 
    term.move_cursor_right(*cursor_index)?;
    
    Ok(())
}

fn reprint_line(term: &mut Term,preset: &String, inp: &String)
-> Result<()>
{
    term.clear_line()?;
    term.write(format!("{}{}",preset,inp).as_bytes())?;
    Ok(())
}

fn input_mode(term: &mut Term, 
    inp: &mut String, current_input: &mut String,
    reverse_search: &mut String,
    preset: &String, history: &Vec<String>,
    cursor_index: &mut usize, history_index: &mut usize,
    STATE: &mut State
) -> Result<()>{
    match term.read_key().unwrap(){
        Key::ArrowLeft => {

            if *cursor_index > 0
            {
                term.move_cursor_left(1)?;
                *cursor_index-=1;
            }
        },
        Key::ArrowRight => 
        {
            if *cursor_index < inp.len()
            {
            term.move_cursor_right(1)?;
            *cursor_index+=1;
            
        }
    },
        Key::ArrowUp => {
            if *history_index == history.len(){*current_input = inp.clone()}
            if *history_index == 0 {return Ok(());}
            *history_index -= 1;
            *inp = history.get(*history_index).unwrap().clone();
            move_cursor_to_end(&term, cursor_index, inp)?;
            reprint_line(term, &preset, inp)?;

        },
        Key::ArrowDown => {
            *history_index += 1;
            if *history_index == history.len() {
                *inp = current_input.clone();
                move_cursor_to_end(&term, cursor_index, inp)?;
                reprint_line(term, &preset, inp)?;
                
                
            } else if *history_index > history.len(){
                *history_index = history.len();
                return Ok(());
            } else {
                *inp = history.get(*history_index).unwrap().clone();
                move_cursor_to_end(term, cursor_index, inp)?;
                reprint_line(term, &preset, inp)?;
            }
                
        },
        Key::Enter => {
            term.write_line("")?;
            *STATE = State::Send;
            return Ok(())
        },
        Key::Backspace => {
                if ( 0 < *cursor_index ) && (*cursor_index <= inp.len()){

                    *cursor_index-=1;
                    inp.remove(*cursor_index);
                    reprint_line(term, &preset, inp)?;
                    term.move_cursor_left(inp.len()-*cursor_index)?;
                }
        },
        Key::Home => {
            term.move_cursor_left(*cursor_index)?;
            *cursor_index = 0;
        },
        Key::End => {
            move_cursor_to_end(&term, cursor_index, inp)?;
        },
        Key::Del => {
            let ln = inp.len(); 
            if ln == *cursor_index{Ok(());}
            if ln > 0{
                inp.remove(*cursor_index);
                reprint_line(term, &preset, inp)?;
                term.move_cursor_left(inp.len()-*cursor_index)?;
            }
        },
        Key::Char(c) => {
            if c.eq(&CONTROL_R){
                    *STATE=State::ReverseSearch;
                    term.hide_cursor()?;
                    reprint_line(term, &"(reverse search) ".to_string(), &reverse_search)?;
                } else {
                inp.insert(*cursor_index, c);
                *cursor_index+=1;
                reprint_line(term, &preset, inp)?;
                term.move_cursor_left(inp.len()-*cursor_index)?;
            }

            },
        _ => (),
    };
    Ok(())
}



pub fn read_line(preset: String, inp: &mut String, history: &Vec<String>)
-> Result<()>
{
    let mut STATE = State::Input;
    *inp = String::new();
    let mut term = Term::stdout();
    let mut cursor_index = 0;
    let mut history_index = history.len();
    let mut current_input = String::new();
    let mut reverse_search = String::new();
    loop {
        match STATE {
            State::Input => input_mode(&mut term, inp, &mut current_input, &mut reverse_search, &preset, history, &mut cursor_index, &mut history_index, &mut STATE)?,
            State::ReverseSearch => {todo!()},
            State::Send => {return Ok(());}
        }
        


    }
}