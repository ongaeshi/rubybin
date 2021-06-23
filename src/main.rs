use std::process::Command;

fn main()
{
    Command::new("ruby")
            .args(&["-e", "p 1, 2, 3"])
            .status()
            .expect("failed to execute process");
}
