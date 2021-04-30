use std::{collections::HashSet, io::BufRead};
use std::fs::File;
use std::io::{ self, BufReader };

const   SIZE            : usize     = 9;
const   SMALL_SIZE      : usize     = 3;
const   DEFAULT_CHAR    : char      = '0';

#[derive(Clone)]
struct SudokuCell {
    fvalue      : char,
    fguesses    : HashSet< char >,
}


pub struct Sudoku {
    fgrid       : [ [ SudokuCell; SIZE ]; SIZE ],
}


impl Sudoku {

    pub fn new() -> Sudoku {
        let mut res         = Sudoku { fgrid: Default::default() };
        let mut def_guesses = HashSet::new();

        for guess in 1..=SIZE {
            if let Some( ch ) = std::char::from_digit( guess as u32, (SIZE + 1) as u32 ) {
                def_guesses.insert( ch );
            }
            else {
                return res
            }
        }

        res.fgrid.iter_mut()
            .for_each( |row| row.iter_mut()
                .for_each( |cell| cell.fguesses = def_guesses.clone() ) );

        res
    }

    pub fn from_file( file: &mut File ) -> Result< Sudoku, io::Error > {
        let mut res     = Sudoku::new();
        let     fbuf    = BufReader::new( file );

        // Chained approach
        // fbuf.lines().take(SIZE).enumerate()
        //     .for_each(|(lindex, line)| line.unwrap().split(char::is_whitespace).take(SIZE).enumerate()
        //         .for_each(|(cindex, str)| res.fgrid[lindex][cindex].fvalue = str.chars().nth(0).unwrap()));

        for ( lindex, line ) in fbuf.lines().take( SIZE ).enumerate() {
            for ( cindex, str ) in line.unwrap().split( char::is_whitespace ).take( SIZE ).enumerate() {
                res.fgrid[ lindex ][ cindex ].fvalue = str.chars().nth( 0 ).unwrap()
            }
        }
        
        Ok( res )
    }

    pub fn clone( &self ) -> Sudoku {
        let mut res = Sudoku { fgrid: Default::default() };
        
        for i in 0..SIZE {
            res.fgrid[ i ].clone_from_slice( &self.fgrid[ i ] ); 
        }

        res
    }

    pub fn print( &self ) {
        for row in self.fgrid.iter() {
            for cell in row {
                print!( "{} ", cell.fvalue );
            }
            println!();
        }
    }

    pub fn solve( &mut self ) -> bool {
        if self.remove_guesses() && self.is_filled() || self.complex_solve() {
            true
        }
        else {
            false
        }
    }

    fn remove_guesses( &mut self ) -> bool {
        let mut counter = 0usize;

        for y in 0..SIZE {
            for x in 0..SIZE {
                let guess = self.fgrid[ y ][ x ].fvalue.clone();
                if guess != DEFAULT_CHAR {
                    self.fgrid[ y ][ x ].fguesses.clear();
                    let mut block_x;
                    let mut block_y;
                    
                    for k in 0..SIZE {
                        block_y     = ( y / SMALL_SIZE ) * SMALL_SIZE + k / SMALL_SIZE;
                        block_x     = ( x / SMALL_SIZE ) * SMALL_SIZE + k % SMALL_SIZE;

                        counter     += self.fgrid[ y ][ k ].fguesses.remove( &guess )               as usize;
                        counter     += self.fgrid[ k ][ x ].fguesses.remove( &guess )               as usize;
                        counter     += self.fgrid[ block_y ][ block_x ].fguesses.remove( &guess )   as usize;
                    }
                }
            }
        }

        if counter > 0 {
            self.set_single_guesses()
        }
        else {
            true
        }
    }

    fn set_single_guesses( &mut self ) -> bool {
        let mut counter = 0usize;

        for y in 0..SIZE {
            for x in 0..SIZE {
                if !self.fgrid[ y ][ x ].is_solvable() {
                    return false;
                }

                if self.fgrid[ y ][ x ].fguesses.len() == 1 {
                    let guess = self.fgrid[ y ][ x ].fguesses.iter().next().unwrap().clone();
                    if self.can_be_placed_at( &guess, y, x ) {
                        self.fgrid[ y ][ x ].fvalue = guess.clone();
                        self.fgrid[ y ][ x ].fguesses.remove( &guess );
                        counter += 1;
                    }
                    else {
                        return false;
                    }
                }
            }
        }

        if counter > 0 {
            self.remove_guesses()
        }
        else {
            true
        }
    }

    fn complex_solve( &mut self ) -> bool {
        if self.is_filled() {
            true    // Check if we ever land here
        }
        else {
            let old_sud = self.clone();

            for y in 0..SIZE {
                for x in 0..SIZE {
                    if self.fgrid[ y ][ x ].fvalue == DEFAULT_CHAR {
                        for guess in old_sud.fgrid[ y ][ x ].fguesses.iter() {
                            self.fgrid[ y ][ x ].fvalue = guess.clone();
                            if self.solve() {
                                return true;
                            }
                            else {
                                *self = old_sud.clone();
                            }
                        }

                        if self.fgrid[ y ][ x ].fvalue == DEFAULT_CHAR {
                            return false;   // Check if we ever land here
                        }
                    }
                }
            }
            
            self.is_filled()
        }
    }

    pub fn is_filled( &self ) -> bool {
        self.fgrid.iter()
            .all( |row| row.iter()
                .all( |cell| cell.fvalue != DEFAULT_CHAR ) )
    }

    fn can_be_placed_at( &self, guess: &char, y: usize, x: usize ) -> bool {
        if self.fgrid[ y ][ x ].fvalue == DEFAULT_CHAR {
            let block_y = ( y / SMALL_SIZE ) * SMALL_SIZE;
            let block_x = ( x / SMALL_SIZE ) * SMALL_SIZE;

            for k in 0..SIZE {
                let xk = self.fgrid[ y ][ k ].fvalue;
                let ky = self.fgrid[ k ][ x ].fvalue;
                let bk = self.fgrid[ block_y + k / SMALL_SIZE ][ block_x + k / SMALL_SIZE ].fvalue;
                if xk == *guess || ky == *guess || bk == *guess {
                    return false;
                }
            }

            true
        }
        else {
            false
        }
    }

}


impl SudokuCell {
    pub fn is_solvable( &self ) -> bool {
        self.fvalue != DEFAULT_CHAR || !self.fguesses.is_empty()
    }
}


impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}


impl Default for SudokuCell {
    fn default() -> Self {
        SudokuCell { fvalue: DEFAULT_CHAR, fguesses: Default::default() }
    }
}

