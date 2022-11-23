use std::io::Write;

use console::{Term, Key};


pub fn read_line(preset: String, inp: &mut String){
    *inp = String::new();
    let mut term = Term::stdout();
    let mut cursor_index = 0;
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
            Key::ArrowUp => todo!(),
            Key::ArrowDown => todo!(),
            Key::Enter => {
                term.write_line("");
                return
            },
            Key::Backspace => {
                if ( 0 < cursor_index ) && (cursor_index <= inp.len()){

                    cursor_index-=1;
                    inp.remove(cursor_index);
                    term.clear_line();
                    term.write(format!("{}{}",preset,inp).as_bytes());
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
                    term.clear_line();
                    term.write(format!("{}{}",preset,inp).as_bytes());
                    term.move_cursor_left(inp.len()-cursor_index);
                }
            },
            Key::Char(c) => {
                inp.insert(cursor_index, c);
                cursor_index+=1;
                term.clear_line();
                term.write(format!("{}{}",preset,inp).as_bytes());
                term.move_cursor_left(inp.len()-cursor_index);
            },
            _ => (),
        }


    }
}