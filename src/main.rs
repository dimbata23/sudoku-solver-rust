mod sudoku;

use std::{fs, fs::File, time::Instant, io};
use sudoku::Sudoku;

fn main() -> io::Result<()> {
    solve_single( "sudoku_samples/s16.txt", true, true )?;
    println!( "---------------------------------------------------------------------" );
    test_all_from( "sudoku_samples", 1, true )?;
    println!( "---------------------------------------------------------------------" );
    test_all_from( "sudoku_samples", 10, false )
}


fn solve_single( path: &str, show_solution: bool, show_exec_time: bool ) -> io::Result<()> {
    let mut file    = File::open( path )?;
    let mut sud     = Sudoku::from_file( &mut file )?;
    
    let sud_start = Instant::now();
    if sud.solve() {
        if show_solution {
            sud.print();
        }
        if show_exec_time {
            let elapsed = sud_start.elapsed().as_millis();
            println!( "{} solved in {}ms.", path, elapsed );
        }
    }
    else {
        println!( "Couldn't solve the Sudoku!" );
    }

    Ok(())
}


fn test_all_from( dir: &str, times: u128, show_individial: bool ) -> io::Result<()> {
    let     sud_count   = fs::read_dir( dir )?.count();
    let mut average     = 0u128;

    for i in 1..=times {
        println!( "Solving {} sudokus... ({} out of {} times)", sud_count, i, times );
        let     start       = Instant::now();
        let     dir         = fs::read_dir( "sudoku_samples" )?;
        for entry in dir {
            let     path    = entry?.path().display().to_string();            
            solve_single( &path, false, show_individial )?;
        }

        let elapsed = start.elapsed().as_millis();
        average += elapsed;
        println!( "Solved all {} sudokus in {} ms.", sud_count, elapsed );
    }

    average /= times;
    println!( "Solved {} sudokus in {}ms. on average.", sud_count, average );

    Ok(())
}
