struct Pieces{
    pawns:  u64,
    king: u64,
    queens: u64,
    knights: u64,
    rooks: u64,
    bishops: u64
}

pub struct Board{
    white: Pieces,
    black: Pieces
}

impl Board{
    pub fn new() -> Board {
        Board {
            white: Pieces {
                pawns: 0x000000000000FF00,
                king: 0x0000000000000010,
                queens: 0x0000000000000008,
                knights: 0x0000000000000042,
                rooks: 0x0000000000000081,
                bishops: 0x0000000000000024,
            },
            black: Pieces {
                pawns: 0x00FF000000000000,
                king: 0x1000000000000000,
                queens: 0x0800000000000000,
                knights: 0x4200000000000000,
                rooks: 0x8100000000000000,
                bishops: 0x2400000000000000,
            },
        }
    }

    fn empty(&self) -> u64{
        return !(self.white.bishops | self.white.king | self.white.knights | self.white.pawns | self.white.queens | self.white.rooks |
                self.black.bishops | self.black.king | self.black.knights | self.black.pawns | self.black.queens | self.black.rooks)
    }

    fn black(&self) -> u64{
        return self.black.pawns | self.black.knights | self.black.bishops | 
                self.black.rooks | self.black.queens | self.black.king;
    }

    fn white_pawn_pushes(pawn: u64, empty: u64) -> u64{
        let rank4 = 0x00000000FF000000;

        let single = pawn << 8 & empty;
        let double = single << 8 & empty & rank4;

        single | double
    } 

    fn white_pawn_attack(pawn: u64, black: u64) -> u64{
        //0x7F... and 0xF7 are excluding A and H columns
        let west = (pawn << 9) & 0xF7F7F7F7F7F7F7F7; 
        let east = (pawn << 7) & 0x7F7F7F7F7F7F7F7F;

        (west | east) & black
    } 

    pub fn white_pawn_legal(&self, pawn: u64, target: u64) -> bool {
        if pawn & self.white.pawns == 0 {
            return false;
        }

        target & Board::white_pawn_pushes(pawn, self.empty()) > 0 || 
        target & Board::white_pawn_attack(pawn, self.black()) > 0
    }

    fn piece_at(&self, square: usize) -> char {
        let mask = 1u64 << square; // Create a mask for the current square

        if self.white.pawns & mask != 0 {
            return 'P';
        } else if self.white.knights & mask != 0 {
            return 'N';
        } else if self.white.bishops & mask != 0 {
            return 'B';
        } else if self.white.rooks & mask != 0 {
            return 'R';
        } else if self.white.queens & mask != 0 {
            return 'Q';
        } else if self.white.king & mask != 0 {
            return 'K';
        }

        if self.black.pawns & mask != 0 {
            return 'p';
        } else if self.black.knights & mask != 0 {
            return 'n';
        } else if self.black.bishops & mask != 0 {
            return 'b';
        } else if self.black.rooks & mask != 0 {
            return 'r';
        } else if self.black.queens & mask != 0 {
            return 'q';
        } else if self.black.king & mask != 0 {
            return 'k';
        }

        // If no piece is present on the square, return empty space ('.')
        '.'
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() { // Start from rank 8 down to rank 1
            for file in 0..8 { // Loop through files a to h
                let square = rank * 8 + file;
                let piece = self.piece_at(square);
                print!("{} ", piece);
            }
            println!(); // Newline after each rank
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_pawn_single_push() {
        // Create a new board where the pawn is on the 2nd rank (e2)
        let mut board = Board::new();
        let pawn = 1 << 12; // Pawn at e2 (bit 12)
        board.white.pawns |= pawn; // Place the white pawn on e2

        // The target square is e3 (bit 20)
        let target_e3 = 1 << 20;

        assert!(board.white_pawn_legal(pawn, target_e3), "Pawn should be able to move to e3");
    }

    #[test]
    fn test_white_pawn_double_push() {
        // Create a new board where the pawn is on the 2nd rank (e2)
        let mut board = Board::new();
        let pawn = 1 << 12; // Pawn at e2 (bit 12)
        board.white.pawns |= pawn; // Place the white pawn on e2

        // The target square is e4 (bit 28)
        let target_e4 = 1 << 28;

        assert!(board.white_pawn_legal(pawn, target_e4), "Pawn should be able to double push to e4");
    }

    #[test]
    fn test_white_pawn_attack() {
        // Create a new board where the pawn is on the 5th rank (e5)
        let mut board = Board::new();
        let pawn = 1 << 28; // Pawn at e5 (bit 28)
        let black_piece = 1 << 35; // Black piece at f6 (bit 35)
        board.white.pawns |= pawn; // Place the white pawn on e5
        board.black.pawns |= black_piece; // Place a black piece on f6

        // The target square is f6 (bit 35)
        let target_f6 = 1 << 35;

        assert!(board.white_pawn_legal(pawn, target_f6), "Pawn should be able to capture on f6");
    }

    #[test]
    fn test_white_pawn_blocked_single_push() {
        // Create a new board where the pawn is on the 2nd rank (e2)
        let mut board = Board::new();
        let pawn = 1 << 12; // Pawn at e2 (bit 12)
        let blocking_piece = 1 << 20; // Another piece is blocking at e3 (bit 20)
        board.white.pawns |= pawn; // Place the white pawn on e2
        board.white.pawns |= blocking_piece; // Place a white piece on e3

        // The target square is e3 (bit 20)
        let target_e3 = 1 << 20;

        assert!(!board.white_pawn_legal(pawn, target_e3), "Pawn should NOT be able to move to e3 because it's blocked");
    }

    #[test]
    fn test_white_pawn_blocked_double_push() {
        // Create a new board where the pawn is on the 2nd rank (e2)
        let mut board = Board::new();
        let pawn = 1 << 12; // Pawn at e2 (bit 12)
        let blocking_piece = 1 << 20; // Another piece is blocking at e3 (bit 20)
        board.white.pawns |= pawn; // Place the white pawn on e2
        board.white.pawns |= blocking_piece; // Place a white piece on e3

        // The target square is e4 (bit 28)
        let target_e4 = 1 << 28;

        assert!(!board.white_pawn_legal(pawn, target_e4), "Pawn should NOT be able to double push to e4 because it's blocked");
    }

    #[test]
    fn test_white_pawn_invalid_attack() {
        // Create a new board where the pawn is on the 5th rank (e5)
        let mut board = Board::new();
        let pawn = 1 << 28; // Pawn at e5 (bit 28)
        let black_piece = 1 << 36; // Black piece at g6 (bit 36)
        board.white.pawns |= pawn; // Place the white pawn on e5
        board.black.pawns |= black_piece; // Place a black piece on g6

        // The target square is f6 (bit 35), but no black piece is there
        let target_f6 = 1 << 35;

        assert!(!board.white_pawn_legal(pawn, target_f6), "Pawn should NOT be able to capture on f6 because no black piece is there");
    }

    #[test] 
    fn test_white_pawn_illegal_move() {
        // Create a new board where the pawn is on the 5th rank (e5)
        let mut board = Board::new();
        let pawn = 1 << 28; // Pawn at e5 (bit 28)
        board.white.pawns |= pawn; // Place the white pawn on e5

        // The target square is f5 (bit 29), which is an illegal move as it's not a valid forward move or capture
        let target_f5 = 1 << 29;

        assert!(!board.white_pawn_legal(pawn, target_f5), "Pawn should NOT be able to move to f5");
    }

}
