mod board;

use board::Board;

fn main() {
    let b: Board = Board::new();
    b.print_board();
    let Hpawn: u64 = 0x0000000000000100;
    let Hpawn_advance = Hpawn << 9;
    if b.white_pawn_legal(Hpawn, Hpawn_advance) {
        println!("legal move");
    }
    else {
        println!("illegal move");
    }
}
