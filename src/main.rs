// THIS WILL BE A TEXT EDITOR ONE DAY PROBABLY
use std::io::{stdout, stdin};

use crossterm::{
    style::{Color, SetForegroundColor}, 
    ExecutableCommand
};

fn main() -> std::io::Result<()> {

  //  crossterm::terminal::enable_raw_mode().expect("Cannot enable raw mode");

    
    stdout()
        .execute(SetForegroundColor(Color::Blue)).expect("Could not set color");

    let mut input = String::new();

    loop { 
        input.clear();

        stdin().read_line(&mut input)?;
        let trimmed = input.trim();
        
        if trimmed == "q" {
            break;
        }
        println!("You typed: {}", input.trim());

    }

    Ok(())

}
