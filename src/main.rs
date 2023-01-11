mod printer;

fn main(){
    // let msg: &'static str = "[letter:50]{Hello World!} [wait:1000] [word:200]{This is a great program}";
    let mut msg: String = String::new();
    match std::io::stdin().read_line(&mut msg) {
        Ok(_n) => {
            if msg.contains("!test!") {
                println!("Using test");
                let dummy: &str =   "[letter:50][w:200]{Hello World!}[wait:500][l:50]{This is a [wait:250][letter:5]GREAT [letter:50]program.}";
                println!("{}", dummy);
                printer::decode(dummy);
            }
            else if msg.contains("!pit!"){
                printer::print_in_time(&msg, 2000);
            }
            else {
                // println!("Formatting...");
                printer::decode(&msg);
            }
        },
        Err(exception) => {
            println!("Exception while reading input: {}", exception);
        },
    }
}
