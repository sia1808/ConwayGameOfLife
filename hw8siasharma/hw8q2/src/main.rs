// Sia Sharma
// Collaborators: Om Italiya, Nikita Salkar

#[derive(Debug)]
//Creating a struct to hold the game board
struct Board{
    real: Vec<Vec<i64>>, // shows the real game board
    n: usize, // Size of the board (number of rows/columns)
}

// Making the game board
impl Board {
    // Initializing the game board
    fn init(&self) -> Vec<Vec<i64>> {
        let mut board = self.real.clone(); // Cloning the current state of the game board
        let difference: Vec<isize> = vec! [-1, 0, 1]; // Vector representing the difference between indices for neighboring cells

        // Looping through each cell of the board
        for x in 0..self.n {
            for y in 0..self.n {
                let mut count = 0; // Counter to keep track of live neighbors for the current cell
                // Looping through neighboring cells
                for x2 in &difference {
                    for y2 in &difference {
                        // Finding index of the neighbors with wrap around
                        let neighborx = (x as isize + self.n as isize - x2) % self.n as isize; 
                        let neighbory = (y as isize + self.n as isize - y2) % self.n as isize; 
                        if *x2 == 0 && *y2 ==0 { 
                            continue; 
                        }
                        // Adding to the count if the neighboring cell is alive
                        count += self.real[neighborx as usize][neighbory as usize];
                        // checking cell status in the next generation
                        checker(count, &mut board, &self.real, x,y);
                    }
                }
            }
        }
        return board
    }
}

// checking state of a cell in the next generation
fn checker(count: i64, board: &mut Vec<Vec<i64>>, oldboard: &Vec<Vec<i64>>, x: usize, y: usize) {
    if count == 2 {
        board[x][y] = oldboard[x][y]; // alive if it has 2 live neighbors
    } else if count == 3 { 
        board[x][y] = 1; // alive if it has exactly 3 live neighbors
    } else {
        board[x][y] = 0; // dead otherwise
    }
}

// creating the initial game board 
fn createboard(vecs: Vec<Vec<i64>>, n2: usize) -> Board {
    let board = Board{
        real:vecs,
        n: n2, // Setting the size of the board
    };
    return board;
}



// Making first game board vectors
fn gamevec1() -> Vec<Vec<i64>> {
    let mut first = vec![vec![0; 16]; 16]; // Creating a 16x16 board filled with zeros
    let ones_place= [(0,1), (1,2), (2,0), (2,1), (2,2)]; // Coordinates of live cells
    for (x, y) in ones_place.iter() {
        first[*x][*y] = 1;
    }
    return first
}

// Making second game board vectors
fn gamevec2() -> Vec<Vec<i64>> {
    let mut second = vec![vec![0; 16];16]; // Creating a 16x16 board filled with zeros
    let ones_place = [(1,0), (1,2), (2,1), (2,2), (3,1)]; // Coordinates of live cells
    for (x,y) in ones_place.iter() {
        second[*x][*y] = 1;
    }
    return second
}

fn main() {

    let start = gamevec1();
    let mut oldboard = createboard(start, 16);
    for g in 0..16 {
        // Printing the initial game board
        println!("{:?}", oldboard.real[g]);
    }
    println!("---------------------------");
    // making and printing each generation
    for gen in 0..10 {
        oldboard.real = oldboard.init();
        for n in 0..16 {
            println!("{:?}", oldboard.real[n]);
        }
        println!("-----------------------");
    }
}

// Tester function to see if the first game board matches what the next generation should look like
#[test]
fn tester() {
    let board = createboard(gamevec1(), 16);
    assert_eq!(board.init(), gamevec2());
}