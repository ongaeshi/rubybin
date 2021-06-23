use std::process::Command;

fn main()
{
    Command::new("ruby")
            .args(&["-e", include_str!("main.rb")])
            .status()
            .expect("failed to execute process");
}
