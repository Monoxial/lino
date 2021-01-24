// use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;
// use copypasta::ClipboardContext;
// use copypasta::ClipboardProvider;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

use super::*;
use super::keybindings::keys;

impl Lino {
    pub(crate) fn initiate_input_event_loop(&mut self, syntect_config: &mut SyntectConfig) {
        loop {
            if self.rendering.is_rendering { continue; }
            self.render(syntect_config);

            // let previous_cursor = self.cursor.clone();
            
            // `read()` blocks until an `Event` is available
            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(&Error::err4());
            }

            match event.unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    self.handle_key_event(&key_event);
                },
                crossterm::event::Event::Mouse(_) => (),
                crossterm::event::Event::Resize(_, _) => {
                    self.update_terminal_size();
                },
            }
            
            if self.should_exit { break; }
        }
    }

    pub(crate) fn handle_key_event(&mut self, event: &crossterm::event::KeyEvent) {
        let mut key_binding = format!("");

        self.highlighting.start_row = self.cursor.row;

        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::NONE {
                    self.input_char_buf = Some(c);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'w' || c == 'W') {
                    key_binding = format!("{}+{}", keys::CTRL, 'w');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}", keys::CTRL, 's');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}", keys::ALT, 's');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                | crossterm::event::KeyModifiers::SHIFT && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, 's');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'a' || c == 'A') {
                    key_binding = format!("{}+{}", keys::CTRL, 'a');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'c' || c == 'C') {
                    key_binding = format!("{}+{}", keys::CTRL, 'c');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'x' || c == 'X') {
                    key_binding = format!("{}+{}", keys::CTRL, 'x');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'v' || c == 'V') {
                    key_binding = format!("{}+{}", keys::CTRL, 'v');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'z' || c == 'Z') {
                    key_binding = format!("{}+{}", keys::CTRL, 'z');
                }
                
                else if event.modifiers == (crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT) && (c == 'z' || c == 'Z') {
                    key_binding = format!("{}+{}", keys::CTRL, 'y');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'y' || c == 'Y') {
                    key_binding = format!("{}+{}", keys::CTRL, 'y');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == ']' {
                    key_binding = format!("{}+{}", keys::ALT, ']');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == '[' {
                    key_binding = format!("{}+{}", keys::ALT, '[');
                }
                
            },
            crossterm::event::KeyCode::Tab => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::TAB);
                }
            },
            crossterm::event::KeyCode::Enter => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::ENTER);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::ENTER);
                }
            },
            crossterm::event::KeyCode::Backspace => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::BACKSPACE);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::BACKSPACE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::BACKSPACE);
                }
            },
            crossterm::event::KeyCode::Delete => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::DELETE);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::DELETE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::DELETE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::DELETE);
                }
            },
            crossterm::event::KeyCode::Home => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::HOME);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::HOME);
                }
            },
            crossterm::event::KeyCode::End => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::END);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::END);
                }
            },
            crossterm::event::KeyCode::PageUp => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::PAGE_UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::PAGE_UP);
                }
            },
            crossterm::event::KeyCode::PageDown => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::PAGE_DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::PAGE_DOWN);
                }
            },
            crossterm::event::KeyCode::Left => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::LEFT);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::LEFT);
                }
            },
            crossterm::event::KeyCode::Right => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::RIGHT);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::RIGHT);
                }
            },
            crossterm::event::KeyCode::Up => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::UP);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::UP);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::ALT, keys::SHIFT, keys::UP);
                }
            },
            crossterm::event::KeyCode::Down => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::DOWN);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::DOWN);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::ALT, keys::SHIFT, keys::DOWN);
                }
            },
            crossterm::event::KeyCode::Esc => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::ESC);
                }
            },
            _ => ()
        }

        let operation_to_perform = self.keybindings.get(&key_binding);
        
        if !operation_to_perform.is_none() {
            operation_to_perform.unwrap()(self);
        } else if !self.input_char_buf.is_none() {
            self.command_enter_character();
        }

        self.set_file_unsaved_if_applicable();

        self.highlighting.end_row = self.cursor.row;
    }

    pub(crate) fn handle_unsaved_changes_frame_input(&mut self) {
        self.render_unsaved_changes_frame();
        
        loop {
            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(&Error::err5());
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if c == 'y' || c == 'Y' {
                                self.file.should_save_as = true;
                                break;
                            }
                            if c == 'n' || c == 'N' {
                                self.file.should_save_as = false;
                                break;
                            }
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.file.should_save_as = false;
                            self.should_exit = false;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        };
    }
    
    pub(crate) fn handle_save_as_frame_input(&mut self) {
        let file_path_backup = self.file.path.clone();
        self.file.cursor_col_offset = self.file.path.len();

        loop {
            self.render_save_as_frame();

            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(&Error::err6());
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            self.file.path.insert(self.file.cursor_col_offset, c);
                            self.file.cursor_col_offset += 1;
                        },
                        crossterm::event::KeyCode::Backspace => {
                            if self.file.path.len() > 0 && self.file.cursor_col_offset > 0 {
                                self.file.cursor_col_offset -= 1;
                                self.file.path.remove(self.file.cursor_col_offset);
                            }
                        },
                        crossterm::event::KeyCode::Delete => {
                            if self.file.path.len() > 0 && self.file.cursor_col_offset < self.file.path.len() {
                                self.file.path.remove(self.file.cursor_col_offset);
                            }
                        },
                        crossterm::event::KeyCode::Left => {
                            if self.file.cursor_col_offset > 0 {
                                self.file.cursor_col_offset -= 1;
                            }
                        },
                        crossterm::event::KeyCode::Right => {
                            if self.file.cursor_col_offset < self.file.path.len() {
                                self.file.cursor_col_offset += 1;
                            }
                        },
                        crossterm::event::KeyCode::Home => {
                            self.file.cursor_col_offset = 0;
                        },
                        crossterm::event::KeyCode::End => {
                            self.file.cursor_col_offset = self.file.path.len();
                        },
                        crossterm::event::KeyCode::Enter => {
                            if self.file.path != "" {
                                self.save_to_file();
                            }
                            if self.file.is_saved && self.file.save_error == "" {
                                break;
                            }
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.file.path = file_path_backup;
                            self.file.cursor_col_offset = self.file.path.len();
                            self.should_exit = false;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            };
        };
    }

}
