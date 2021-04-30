mod sudoku;

use std::time::Instant;
use std::io;
use sudoku::*;
use std::fs::File;

fn main() -> io::Result<()> {

    let mut file    = File::open( "test.txt" )?;
    let mut sud     = Sudoku::from_file( &mut file )?;
    
    let start = Instant::now();
    if sud.solve() {
        let elapsed = start.elapsed().as_millis();
        println!( "Solution found in {}ms.", elapsed );
        sud.print();
    }
    else {
        println!( "Couldn't solve the Sudoku!" );
    }

    Ok(())
}
