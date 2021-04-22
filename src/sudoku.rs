use std::{collections::HashSet, convert::TryInto, io::BufRead};
use std::fs::File;
use std::io::Read;
use std::io::{self, BufReader};

const   SIZE        : usize     = 9;
const   SMALL_SIZE  : usize     = 3;

pub struct Sudoku {
    fgrid       : [ [ char; SIZE ]; SIZE ],
    fguesses    : HashSet< char >,
}


impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku { fgrid: [ [ ' '; SIZE ]; SIZE ], fguesses: HashSet::new() }
    }

    pub fn from_file( file: &mut File ) -> Result< Sudoku, io::Error > {
        let mut res     = Sudoku::new();
        let     fbuf    = BufReader::new( file );

        // Chained approach
        // fbuf.lines().take(SIZE).enumerate()
        //     .for_each(|(lindex, line)| line.unwrap().split(char::is_whitespace).take(SIZE).enumerate()
        //         .for_each(|(cindex, str)| res.fgrid[lindex][cindex] = str.chars().nth(0).unwrap()));

        for ( lindex, line ) in fbuf.lines().take( SIZE ).enumerate() {
            for ( cindex, str ) in line.unwrap().split( char::is_whitespace ).take( SIZE ).enumerate() {
                res.fgrid[ lindex ][ cindex ] = str.chars().nth( 0 ).unwrap()
            }
        }
        
        Ok( res )
    }

    pub fn print( &self ) {
        for row in self.fgrid.iter() {
            for ch in row {
                print!( "{} ", ch );
            }
            println!();
        }
    }

}
