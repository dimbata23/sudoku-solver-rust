mod sudoku;

use std::io;
use sudoku::*;
use std::fs::File;

fn main() -> io::Result<()> {

    let mut file    = File::open( "test.txt" )?;
    let     sud     = Sudoku::from_file( &mut file )?;
    
    sud.print();

    Ok(())
}
