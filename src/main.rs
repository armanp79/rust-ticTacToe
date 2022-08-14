use ticTacToe::Board;
use std::io;

fn play_again() -> bool {
	println!("Do you want to play again? [y/n]");
	let mut flag = String::new();
	io::stdin().read_line(&mut flag).expect("Failed to get input");
	match flag.as_str().trim() {
		"y" => true,
		"n" => false,
		_ => {
			println!("invalid input {}", flag);
			return play_again()
		}
	}
}


fn main() {
    println!("Let's Play Tic Tac Toe");
    let mut board = Board::new();

	loop {
		let mut input = String::new();
		board.display();
		println!("Enter Square: ");
		io::stdin().read_line(&mut input).expect("Failed to get input");
		input.pop();
		println!("\n");
		let square = input.parse::<usize>().unwrap();
		let flag = board.play(&square);
		if !flag {
			if !play_again() {
				break;
			} else {
				board = Board::new();
			}
		}
		if board.isFull() {
			println!("TIE");
		
			if !play_again() { 	
            	break;
            } else {
            	board = Board::new();
            }
	
		}
	}

}
