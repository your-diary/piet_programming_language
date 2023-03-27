use std::error::Error;

use piet_programming_language::image::Image;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = format!(
        "{}/Downloads/power2_big.png",
        std::env::var("HOME").unwrap()
    );
    let img = Image::new(&filename)?;
    println!("{}", img);

    Ok(())
}
