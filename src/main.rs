use std::io::{stdin};

fn main() -> std::io::Result<()>
{

    #[cfg(target_os = "windows")]
    let new_line_char_count = 2;

    #[cfg(not(target_os = "windows"))]
    let new_line_char_count = 1;

    println!("Input String, get length:");

    loop {
     
        let mut buffer = String::new();

        let stdin = stdin();

        stdin.read_line(&mut buffer)?;
        
        let count = buffer.chars().count() - new_line_char_count;

        if count == 0
        {

            break;

        }
        
        println!("{}", count.to_string().as_str());
        
    }

    Ok(())

}
