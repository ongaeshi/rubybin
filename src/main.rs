use std::process::Command;
use std::env;

fn main()
{
    let mut args = vec![
        "-e".to_string(),
        include_str!("main.rb").to_string()
        ];

    let mut i = 0;
    for argument in env::args() {
        if i > 0 {
            args.push(argument);
        }
        i += 1;
    }
    
    Command::new("ruby")
        .args(&args)
        .status()
        .expect("failed to execute process");
}
