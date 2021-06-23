use std::process::Command;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    
    Command::new("ruby")
        .args(&["-e", include_str!("main.rb"), &args[1]])
        .status()
        .expect("failed to execute process");
}
