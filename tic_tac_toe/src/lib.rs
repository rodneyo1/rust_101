pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return String::from("player X won");
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return String::from("player O won");
    }
    String::from("tie")
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
    // Check anti-diagonal (top-right to bottom-left)
    (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|&row| row.iter().all(|&c| c == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..3).any(|col| table.iter().all(|row| row[col] == player))
}