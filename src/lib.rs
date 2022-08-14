
pub struct Board {
    pieces: [char; 9], 
	player: char
}

impl Board {
	pub fn new() -> Board {
		let pieces = ['0','1','2','3','4','5','6','7','8'];
		let player = 'X';
		Board{pieces, player}
	}

    pub fn play(&mut self, square: &usize) -> bool {
        if *square >= 9 || self.pieces[*square] == 'X' || self.pieces[*square] == 'O' {
	    	println!("Invalid Input. Please enter number from 0 - 9.");
	    	return true
		}
		self.pieces[*square] = self.player;
		
		let winner: char = self.checkWinner();
        if winner != ' ' {
            println!("Game Over! Winner is Player {}", winner);
			board.display();
			return false
        }



		if self.player == 'X' {
		    self.player = 'O';
		} else {
		    self.player = 'X';
		}
		return true
	}
	
    pub fn checkWinner(&self) -> char {
		//check rows
		let mut i = 0;
		while i < 8 {
			if self.pieces[i] == self.pieces[i+1] && self.pieces[i] == self.pieces[i+2] {
				return self.player
			}
			i += 3;
		}
		
		// check cols
		for i in 0..3 {
			if self.pieces[i] == self.pieces[i+3] && self.pieces[i] == self.pieces[i+6] {
				return self.player
			}
		}
		
		// check diagnals

		if self.pieces[0] == self.pieces[4] && self.pieces[0] == self.pieces[8] || self.pieces[2] == self.pieces[4] && self.pieces[2] == self.pieces[6] {
			return self.player
		}

		return ' '
	}

	pub fn display(&mut self) {
		let mut i = 0;
		println!("Turn : {}", self.player);
		while i < 5 {
			println!(" {} | {} | {} ", self.pieces[i], self.pieces[i+1], self.pieces[i+2]);
			println!("-------------");
			i += 3;
		}
		println!(" {} | {} | {} ", self.pieces[i], self.pieces[i+1], self.pieces[i+2]);
	}
	pub fn isFull(&self) -> bool {
		for i in 0..self.pieces.len() {
			if self.pieces[i].is_numeric() {
				return false
			}
		}
		return true
	}
	


}
