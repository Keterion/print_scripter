#![allow(unused)]
pub mod delay_printer {
    use regex::{Regex, CaptureMatches};
    use lazy_static::lazy_static;
    use std::{thread, time::{self, Duration}, str::SplitWhitespace, io::{stdout, Write}};

    struct Command{
        delay_timing: u16,
        words_affected: u16,
    }
    impl Command {
        fn set_timing(&mut self, new_num: u16){
            self.delay_timing = new_num;
            // println!("Delay set to {}", new_num);
        }
        fn words_affected(&mut self, affected: u16){
            // println!("Words affected: {}", affected);
            self.words_affected = affected;
        }
    }

    pub fn decode(message: &str) {
        lazy_static!{
            static ref RE: Regex = Regex::new
            (r"(?:\[(\w+):(\d+)(?::(\d+))?){0,}(?:[{\]\s]([\w\s!.'_:,\?\\)(]+)[}\]\s]){0,}")
            .unwrap();
        }
        static DELAY_SETTING_POS: usize = 1;
        static DELAY_TIMING_POS: usize = 2;
        static WORD_AFFECT_AMMOUNT_POS: usize = 3;
        static TEXT_POS: usize = 4;

        let mut word_settings:Command = Command {
            delay_timing: 0,
            words_affected: 0,
        };
        let mut letter_settings:Command = Command {
            delay_timing: 0,
            words_affected: 0,
        };

        let mut delay_setting: Option<&str>;
        let mut delay_timing: u16;
        let mut word_affect_ammount: Option<u16>;
        let mut text: Option<&str>;
        for entry in RE.captures_iter(message) {
            //println!("Entry is: {:?}", entry);
            delay_setting = 
            if let Some(str) = entry.get(DELAY_SETTING_POS) {
                // println!("{}", entry.get(DELAY_SETTING_POS).unwrap().as_str());
                Some(entry.get(DELAY_SETTING_POS).unwrap().as_str())
            }
            else {
                None
            };

            delay_timing = if let Some(str) = entry.get(DELAY_TIMING_POS) {
                // println!("{}", entry.get(DELAY_TIMING_POS).unwrap().as_str().parse::<u16>().unwrap());
                entry.get(DELAY_TIMING_POS).unwrap().as_str().parse::<u16>().unwrap()
            } else {
                0
            };

            word_affect_ammount = if let Some(str) = entry.get(WORD_AFFECT_AMMOUNT_POS) {
                // println!("{}", entry.get(WORD_AFFECT_AMMOUNT_POS).unwrap().as_str().parse::<u16>().unwrap());
                Some(entry.get(WORD_AFFECT_AMMOUNT_POS).unwrap().as_str().parse::<u16>().unwrap())
            } else {
                None
            };

            text = if let Some(str) = entry.get(TEXT_POS) {
                    // println!("{}", entry.get(TEXT_POS).unwrap().as_str());
                    Some(entry.get(TEXT_POS).unwrap().as_str())
                }
                else {
                    None
                };
            
            // println!("Delay Setting: {} \nDelay Timing: {}\nMessage: {}", delay_setting.unwrap_or("There was no setting"), delay_timing, text.unwrap_or("There was no text"));

            match delay_setting { // applies the found delay_timing to the setting that correlates, if none of the settings are found, it does nothing
                Some(str) => {
                    match delay_setting.unwrap() {
                        "letter" | "l" => {
                            letter_settings.set_timing(delay_timing);
                            letter_settings.words_affected(word_affect_ammount.unwrap_or(100));
                        },
                        "word" | "w" => {
                            word_settings.set_timing(delay_timing);
                            word_settings.words_affected(word_affect_ammount.unwrap_or(100));
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
                    if letter_settings.delay_timing != 0{
                        if word_settings.delay_timing == 0 {
                            display_text(false, true, &word_settings, &letter_settings, text.unwrap());
                            letter_settings.set_timing(0);
                        } else {
                            display_text(true, true, &word_settings, &letter_settings, text.unwrap());
                            letter_settings.set_timing(0);
                            word_settings.set_timing(0);
                        }
                    } else if word_settings.delay_timing != 0{
                        display_text(true, false, &word_settings, &letter_settings, text.unwrap());
                        word_settings.set_timing(0);
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
        //println!("{:?}", time_per_char.as_millis());
        for character in message.chars() {
            thread::sleep(time_per_char);
            print!("{}", character);
            _ = stdout().flush();
        }
    }

    fn display_text(word_separate: bool, letter_separate: bool, word_settings: &Command, letter_settings: &Command, message: &str) {

        let word_delay_duration: Duration = time::Duration::from_millis(word_settings.delay_timing as u64);
        let letter_delay_duration: Duration = time::Duration::from_millis(letter_settings.delay_timing as u64);
        let word_split_message: SplitWhitespace = message.split_whitespace();

        let word_counter: u8 = 0;
    
        if word_separate {
            if letter_separate {
                print_by_word(message, letter_settings, word_settings);
            } else {
                print_by_word(message, letter_settings, word_settings);
            }
        } else if letter_separate {
            for word in message.split_whitespace() {
                print_by_char(word, letter_delay_duration);
                print!(" ");
            }
        }
    }

    fn print_by_char(word: &str, char_delay: Duration){
        for char in word.chars() {
            print!("{}", char);
            _ = stdout().flush();
            thread::sleep(char_delay);
        }
    }
    fn print_by_word(text: &str, char_setting: &Command, word_setting: &Command){
        let mut char_delay: Duration = Duration::from_millis(char_setting.delay_timing as u64);
        let mut word_delay: Duration = Duration::from_millis(word_setting.delay_timing as u64);
        let mut words: usize = 0;
        let mut letters: usize = 0;
        if char_delay.is_zero() {
            for word in text.split_whitespace() {
                print!("{}", word);
                print!(" ");
                _ = stdout().flush();
                if(words >= word_setting.words_affected as usize) { // if the affected words are less or equals the ammount that is to be affected
                    word_delay = Duration::ZERO;
                } else {
                    words = words + 1;
                }
                thread::sleep(word_delay);
            }
        } else {
            for word in text.split_whitespace() {
                if(letters >= char_setting.words_affected as usize) {
                    char_delay = Duration::ZERO;
                } else {
                    letters = letters + 1;
                }
                print_by_char(word, char_delay);
                print!(" ");
                if(words >= word_setting.words_affected as usize) { // if the affected words are less or equals the ammount that is to be affected
                    word_delay = Duration::ZERO;
                } else {
                    words = words + 1;
                }
                thread::sleep(word_delay);
            }
        }
    }
}