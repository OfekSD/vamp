use std::{io::Write};

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
    state: &mut State
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
            *state = State::Send;
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
            if ln == *cursor_index{return Ok(());}
            if ln > 0{
                inp.remove(*cursor_index);
                reprint_line(term, &preset, inp)?;
                term.move_cursor_left(inp.len()-*cursor_index)?;
            }
        },
        Key::Char(c) => {
            if c.eq(&CONTROL_R){
                    *state=State::ReverseSearch;
                    move_cursor_to_start(&term, cursor_index)?;
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

fn search_history(reverse_search: &String, history: &Vec<String>) -> Vec<usize>{
    let mut results = Vec::new();
    history.iter().enumerate().for_each(|(ind, command)|{
        if command.contains(reverse_search){
            results.push(ind)
        }
    });

    results
}

fn reverse_search_mode(
    term: &mut Term, 
    inp: &mut String, current_input: &mut String,
    reverse_search: &mut String,
    reverse_search_results: &mut Vec<usize>,
    preset: &String, history: &Vec<String>,
    cursor_index: &mut usize,history_index: &mut usize, reverse_search_index: &mut usize,
    state: &mut State
) -> Result<()>
{
    match term.read_key().unwrap() {
        Key::Backspace => {
            reverse_search.pop();
            *reverse_search_results = search_history(&reverse_search, history);
            if !reverse_search_results.is_empty(){
                *reverse_search_index = reverse_search_results.len() - 1;
            }
            
        },
        Key::Char(c) => {
            if c.eq(&CONTROL_R){
                if reverse_search.is_empty(){
                    *state = State::Input;
                    reprint_line(term, &preset, inp)?;
                    return Ok(());
                }
                *reverse_search_index -= 1;
            } else {   
                (*reverse_search).push(c);
                
                *reverse_search_results = search_history(&reverse_search, history);
                if !reverse_search_results.is_empty(){
                    *reverse_search_index = reverse_search_results.len() - 1;
                }
            }
        },
        _ => {
            *state = State::Input;
            *history_index =  if reverse_search_results.is_empty(){
                *history_index
            } else {
                *reverse_search_results.get(*reverse_search_index).unwrap()
            };
            if current_input.is_empty(){
                *current_input = inp.clone();
            } 
            *inp = history.get(*history_index).unwrap().to_string();
            move_cursor_to_end(&term, cursor_index, inp)?;
            reprint_line(term, &preset, inp)?;
            return Ok(());
        },
    };

    let result;
    if reverse_search_results.is_empty(){
        result = reverse_search.clone();
    } else {
        let command = history.get(*reverse_search_results.get(*reverse_search_index).unwrap()).unwrap().to_string();
        result = command.replace(&*reverse_search, &format!("{}",console::Style::new().cyan().apply_to(&*reverse_search)));
    }
    reprint_line(term, &"(reverse search) ".to_string(), &result)?;
    Ok(())

}



pub fn read_line(preset: String, inp: &mut String, history: &Vec<String>)
-> Result<()>
{
    let mut state = State::Input;
    *inp = String::new();
    let mut term = Term::stdout();
    let mut cursor_index = 0;
    let mut history_index = history.len();
    let mut reverse_search_index = 0;
    let mut current_input = String::new();
    let mut reverse_search = String::new();
    let mut reverse_search_results = Vec::new();
    loop {
        match state {
            State::Input => input_mode(&mut term, inp, &mut current_input, &mut reverse_search, &preset, history, &mut cursor_index, &mut history_index, &mut state)?,
            State::ReverseSearch => reverse_search_mode(&mut term, inp, &mut current_input,
                &mut reverse_search, &mut reverse_search_results, &preset, history, 
                &mut cursor_index, &mut history_index, &mut reverse_search_index, &mut state)?,
            State::Send => {return Ok(());}
        }
        


    }
}