use std::io::Write;

use anyhow::Result;
use console::{Term, Key};


fn move_cursor_to_front(term: &Term,cursor_index:&mut usize, inp: &String){
    *cursor_index = inp.len();
    term.move_cursor_right(*cursor_index);
}

fn reprint_line(term: &mut Term,preset: &String, inp: &String){
    term.clear_line();
    term.write(format!("{}{}",preset,inp).as_bytes());
}

pub fn read_line(preset: String, inp: &mut String, history: &Vec<String>){
    *inp = String::new();
    let mut term = Term::stdout();
    let mut cursor_index = 0;
    let mut history_index = history.len();
    let mut current_input = String::new();
    todo!("fix cursor contorl isues when using history");
    loop {

        match term.read_key().unwrap(){
            Key::ArrowLeft => {
                if cursor_index > 0
                {
                    term.move_cursor_left(1);
                    cursor_index-=1;
                }
            },
            Key::ArrowRight => 
            {
                if cursor_index < inp.len()
                {
                term.move_cursor_right(1);
                cursor_index+=1;
                
            }
        },
            Key::ArrowUp => {
                if history_index == history.len(){current_input = inp.clone()}
                if history_index == 0 {continue;}
                history_index -= 1;
                *inp = history.get(history_index).unwrap().clone();
                reprint_line(&mut term, &preset, inp);

            },
            Key::ArrowDown => {
                history_index += 1;
                if history_index == history.len() {
                    *inp = current_input.clone();
                    reprint_line(&mut term, &preset, inp);
                    continue;
                    
                }
                if history_index > history.len(){
                    history_index = history.len();
                    continue;

                }
                *inp = history.get(history_index).unwrap().clone();
                reprint_line(&mut term, &preset, inp);
            },
            Key::Enter => {
                term.write_line("");
                return
            },
            Key::Backspace => {
                if ( 0 < cursor_index ) && (cursor_index <= inp.len()){

                    cursor_index-=1;
                    inp.remove(cursor_index);
                    reprint_line(&mut term, &preset, inp);
                    term.move_cursor_left(inp.len()-cursor_index);
                }
            },
            Key::Home => {
                term.move_cursor_left(cursor_index);
                cursor_index = 0;
            },
            Key::End => {
                cursor_index = inp.len();
                term.move_cursor_right(cursor_index);
            },
            Key::Del => {
                let ln = inp.len(); 
                if ln > 0{

                    cursor_index;
                    inp.remove(cursor_index);
                    reprint_line(&mut term, &preset, inp);
                    term.move_cursor_left(inp.len()-cursor_index);
                }
            },
            Key::Char(c) => {
                inp.insert(cursor_index, c);
                cursor_index+=1;
                reprint_line(&mut term, &preset, inp);
                term.move_cursor_left(inp.len()-cursor_index);
            },
            _ => (),
        }


    }
}