mod fancy_printer;
use fancy_printer::delay_printer;

fn main(){
    // let msg: &'static str = "[letter:50]{Hello World!} [wait:1000] [word:200]{This is a great program}";
    let mut msg: String = String::new();
    match std::io::stdin().read_line(&mut msg) {
        Ok(_n) => {
            if msg.contains("!test!") {
                println!("Using test");
                let dummy: &str = "[letter:50:2][w:100:1]{Hello World!}[wait:500][l:50]{This is a [wait:250][letter:5]GREAT [letter:50]program.} [letter:50:7][word:250:5]{And here, you can even(WordToHere) see how(letterToHere) the settings can be limited to a certain ammount of words.}";
                println!("{}", dummy);
                delay_printer::decode(dummy);
            }
            else if msg.contains("!pit!"){
                delay_printer::print_in_time(&msg, 2000);
            }
            else {
                // println!("Formatting...");
                delay_printer::decode(&msg);
            }
        },
        Err(exception) => {
            println!("Exception while reading input: {}", exception);
        },
    }
}
