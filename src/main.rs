fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let row1 = [" ","x","0"];
    let row2 = ["x","0"," "];
    let row3 = ["0"," ","x"];


    draw_game_board(&row1, &row2, &row3);
}

// Prints out the game board
fn draw_game_board(row1: &[&str; 3],row2: &[&str; 3],row3: &[&str; 3]) {
    println!("{}|{}|{}", row1[0], row1[1] ,row1[2]);
    println!("_ _ _");
    println!("{}|{}|{}", row2[0], row2[1] ,row2[2]);
    println!("_ _ _");
    println!("{}|{}|{}", row3[0], row3[1] ,row3[2]);
}