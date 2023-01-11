#![allow(unused)]
pub mod delay_printer {
    use regex::{Regex, CaptureMatches};
    use lazy_static::lazy_static;
    use std::{thread, time::{self, Duration}, str::SplitWhitespace, io::{stdout, Write}};
    pub fn decode(message: &str) {
        lazy_static!{
            static ref RE: Regex = Regex::new
            (r"(?:\[(\w+):(\d+)){0,}(?:[{\]]([\w\s\._!:;,'\?\\]+)[}\]\s]?){0,}")
            .unwrap();
        }
        let mut letter_delay_ammount: u16 = 0;
        let mut word_delay_ammount: u16 = 0;
        for entry in RE.captures_iter(message) {
            //println!("{:?}", entry);
            let delay_setting: Option<&str> = 
            if let Some(str) = entry.get(1) {
                Some(entry.get(1).unwrap().as_str())
            }
            else {
                None
            };

            let delay_timing: u16 = if let Some(str) = entry.get(2) {
                entry.get(2).unwrap().as_str().parse::<u16>().unwrap_or(0)
            } else {
                0
            };

            let text: Option<&str> = 
                if let Some(str) = entry.get(3) {
                    Some(entry.get(3).unwrap().as_str())
                }
                else {
                    None
                };
            
            // println!("Delay Setting: {} \nDelay Timing: {}\nMessage: {}", delay_setting.unwrap_or("There was no setting"), delay_timing, text.unwrap_or("There was no text"));

            match delay_setting { // applies the found delay_timing to the setting that correlates, if none of the settings are found, it does nothing
                Some(str) => {
                    match delay_setting.unwrap() {
                        "letter" | "l" => {
                            letter_delay_ammount = delay_timing;
                        },
                        "word" | "w" => {
                            word_delay_ammount = delay_timing;
                        },
                        "wait" => {
                            thread::sleep(Duration::from_millis(delay_timing as u64));
                        }
                        _ => {},
                    }
                }
                None => {},
            }
            match text { // checks if there has been text in the capture clump
                Some(str) => {
                    if letter_delay_ammount != 0{
                        if word_delay_ammount == 0 {
                            display_text(false, true, 0, letter_delay_ammount, text.unwrap());
                            letter_delay_ammount = 0;
                        } else {
                            display_text(true, true, word_delay_ammount, letter_delay_ammount, text.unwrap());
                            letter_delay_ammount = 0;
                            word_delay_ammount = 0;
                        }
                    } else if word_delay_ammount != 0{
                        display_text(true, false, word_delay_ammount, 0, text.unwrap());
                        word_delay_ammount = 0;
                    } else {
                        print!("{}", text.unwrap());
                    }
                },
                None => {},
            }
        }
    }

    pub fn print_in_time(message: &str, time_in_ms: usize){
        let char_ammount: usize = message.chars().count();
        let time_per_char: Duration = Duration::from_millis((time_in_ms / char_ammount) as u64);
        println!("{:?}", time_per_char.as_millis());
        for character in message.chars() {
            thread::sleep(time_per_char);
            print!("{}", character);
            _ = stdout().flush();
        }
    }

    fn display_text(word_separate: bool, letter_separate: bool, word_delay: u16, letter_delay: u16, message: &str) {

        let word_delay_duration: Duration = time::Duration::from_millis(word_delay as u64);
        let letter_delay_duration: Duration = time::Duration::from_millis(letter_delay as u64);
        let word_split_message: SplitWhitespace = message.split_whitespace();
    
        if word_separate {
            if letter_separate {
    
                for word in word_split_message {
                    for char in word.chars() {
                        print!("{}", char);
                        _ = stdout().flush();
                        thread::sleep(letter_delay_duration);
                    }
                    print!(" ");
                    thread::sleep(word_delay_duration);
                }
    
            }
            else {
    
                for word in word_split_message {
                    print!("{}", word);
                    print!(" ");
                    _ = stdout().flush();
                    thread::sleep(word_delay_duration);
                }
                
            }
        }
        else if letter_separate {
            for word in word_split_message {
                for char in word.chars() {
                    print!("{}", char);
                    _ = stdout().flush();
                    thread::sleep(letter_delay_duration);
                }
                print!(" ");
                _ = stdout().flush();
                thread::sleep(letter_delay_duration);
            }
        }
        else {
            print!("{}", message);
        }
    }
}