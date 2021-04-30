mod sudoku;

use std::io;
use sudoku::*;
use std::fs::File;

fn main() -> io::Result<()> {

    let mut file    = File::open( "test.txt" )?;
    let mut sud     = Sudoku::from_file( &mut file )?;
    
    sud.solve();
    sud.print();

    Ok(())
}
