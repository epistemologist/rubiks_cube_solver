mod cube;
use std::ops::Deref;

use cube::*;

fn main() {
    let mut _moves = [U, R, F, B, L, D];
    for (mv, string) in _moves.iter().zip("URFBLD".chars()) {
        print!("pub const {}: Move = ", format!("{}3", string));
        print!(
            "{:?}",
            CubeSlow::new(true)
                .apply_move(mv)
                .apply_move(mv)
                .apply_move(mv)
                ._to_move()
        );
        print!(";");
        println!();
    }
}
