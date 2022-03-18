use std::{error::Error, env};

fn main() -> Result<(), Box<dyn Error>> {
    let size = env::args().count();
    if size >= 2 {
        Ok(())
    } else {
        Err("error")?
    }
}
