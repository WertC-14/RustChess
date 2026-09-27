use std::io;

type Board = [[Option<ChessPiece>; 8]; 8];

#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
}

#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct ChessPiece {
    piece: Piece,
    color: Color,
}

impl Color {
    fn next(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

fn main() {
    let mut cheese_table = start_up();
    let mut turn = Color::White;

    loop {
        #[allow(clippy::needless_bool)]
        let flipped = if turn == Color::White { true } else { false };
        write_terminal(&cheese_table, &[], flipped);

        match turn {
            Color::White => println!("Sıra: Beyaz"),
            Color::Black => println!("Sıra: Siyah"),
        }

        let from = match read_square("Hangi taş? (örnek: B2)") {
            Some(square) => square,
            None => continue,
        };

        let piece = match cheese_table[from.1][from.0] {
            Some(p) => p,
            None => {
                println!("Orada taş yok");
                continue;
            }
        };

        if piece.color != turn {
            println!("Bu senin taşın değil");
            continue;
        }

        let moves = possible_moves(cheese_table, piece, from);
        write_terminal(&cheese_table, &moves, flipped);

        let to = match read_square("Nereye?") {
            Some(square) => square,
            None => continue,
        };

        if !moves.contains(&to) {
            println!("Oraya gidemez");
            continue;
        }

        move_piece(&mut cheese_table, from, to);

        turn = turn.next();
    }
}

fn read_square(message: &str) -> Option<(usize, usize)> {
    println!("{}", message);

    let mut input = String::new();
    if io::stdin().read_line(&mut input).unwrap() == 0 {
        std::process::exit(0);
    }

    let input = input.trim().to_uppercase();
    let chars: Vec<char> = input.chars().collect();

    if chars.len() != 2 {
        println!("Geçersiz kare");
        return None;
    }

    let letter = chars[0];
    let number = chars[1];

    if !('A'..='H').contains(&letter) || !('1'..='8').contains(&number) {
        println!("Geçersiz kare");
        return None;
    }

    let col = letter as usize - 'A' as usize;
    let row = number as usize - '1' as usize;

    Some((col, row))
}

fn move_piece(table: &mut Board, from: (usize, usize), to: (usize, usize)) {
    if let Some(from_piece) = table[from.1][from.0] {
        if let Some(to_piece) = table[to.1][to.0] {
            if from_piece.color != to_piece.color {
                table[to.1][to.0] = table[from.1][from.0];
                table[from.1][from.0] = None;
            }
        } else {
            table[to.1][to.0] = table[from.1][from.0]; // Taşı hedefe koy
            table[from.1][from.0] = None; // Eski yerini boşalt
        }
    }
}

fn possible_moves(
    table: Board,
    piece: ChessPiece,
    position: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let (x, y) = position;

    match piece.piece {
        Piece::Pawn => {
            let dir: i32 = match piece.color {
                Color::White => 1,
                Color::Black => -1,
            };
            let new_y = (y as i32 + dir) as usize;

            if (0..8).contains(&new_y) {
                if table[new_y][x].is_none() {
                    moves.push((x, new_y));
                }
                #[allow(clippy::collapsible_if)]
                for dx in [-1, 1] {
                    let new_x = (x as i32 + dx) as usize;
                    if (0..8).contains(&new_x) {
                        if let Some(other) = table[new_y][new_x] {
                            if other.color != piece.color {
                                moves.push((new_x, new_y));
                            }
                        }
                    }
                }
            }
        }
        Piece::King => {
            for dir in -1..=1 {
                let new_y = (y as i32 + dir) as usize;

                if (0..8).contains(&new_y) {
                    #[allow(clippy::collapsible_if)]
                    for dx in -1..=1 {
                        let new_x = (x as i32 + dx) as usize;
                        if (0..8).contains(&new_x) {
                            if table[new_y][new_x].is_none() {
                                moves.push((new_x, new_y));
                            }
                            if let Some(other) = table[new_y][new_x] {
                                if other.color != piece.color {
                                    moves.push((new_x, new_y));
                                }
                            }
                        }
                    }
                }
            }
        }
        Piece::Rook => {
            // yukarı
            for step in 1..=5 {
                let new_y = (y as i32 - step) as usize;
                if !(0..8).contains(&new_y) {
                    break;
                }

                match table[new_y][x] {
                    None => moves.push((x, new_y)),
                    Some(other) => {
                        if other.color != piece.color {
                            moves.push((x, new_y));
                        }
                        break;
                    }
                }
            }

            // aşağı
            for step in 1..=5 {
                let new_y = (y as i32 + step) as usize;
                if !(0..8).contains(&new_y) {
                    break;
                }

                match table[new_y][x] {
                    None => moves.push((x, new_y)),
                    Some(other) => {
                        if other.color != piece.color {
                            moves.push((x, new_y));
                        }
                        break;
                    }
                }
            }

            // sol
            for step in 1..=5 {
                let new_x = (x as i32 - step) as usize;
                if !(0..8).contains(&new_x) {
                    break;
                }

                match table[y][new_x] {
                    None => moves.push((new_x, y)),
                    Some(other) => {
                        if other.color != piece.color {
                            moves.push((new_x, y));
                        }
                        break;
                    }
                }
            }

            // sağ
            for step in 1..=5 {
                let new_x = (x as i32 + step) as usize;
                if !(0..8).contains(&new_x) {
                    break;
                }

                match table[y][new_x] {
                    None => moves.push((new_x, y)),
                    Some(other) => {
                        if other.color != piece.color {
                            moves.push((new_x, y));
                        }
                        break;
                    }
                }
            }
        }
        Piece::Bishop => {
            {
                // sağ - üst
                for step in 1..=5 {
                    let new_y = y as i32 - step;
                    let new_x = x as i32 + step;
                    if !(0..8).contains(&new_y) || !(0..8).contains(&new_x) {
                        break;
                    }
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;

                    match table[new_y][new_x] {
                        None => moves.push((new_x, new_y)),
                        Some(other) => {
                            if other.color != piece.color {
                                moves.push((new_x, new_y));
                            }
                            break;
                        }
                    }
                }

                // sol - üst
                for step in 1..=5 {
                    let new_y = y as i32 - step;
                    let new_x = x as i32 - step;
                    if !(0..8).contains(&new_y) || !(0..8).contains(&new_x) {
                        break;
                    }
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;

                    match table[new_y][new_x] {
                        None => moves.push((new_x, new_y)),
                        Some(other) => {
                            if other.color != piece.color {
                                moves.push((new_x, new_y));
                            }
                            break;
                        }
                    }
                }

                // sağ - alt
                for step in 1..=5 {
                    let new_y = y as i32 + step;
                    let new_x = x as i32 + step;
                    if !(0..8).contains(&new_y) || !(0..8).contains(&new_x) {
                        break;
                    }
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;

                    match table[new_y][new_x] {
                        None => moves.push((new_x, new_y)),
                        Some(other) => {
                            if other.color != piece.color {
                                moves.push((new_x, new_y));
                            }
                            break;
                        }
                    }
                }

                // sol - alt
                for step in 1..=5 {
                    let new_y = y as i32 + step;
                    let new_x = x as i32 - step;
                    if !(0..8).contains(&new_y) || !(0..8).contains(&new_x) {
                        break;
                    }
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;

                    match table[new_y][new_x] {
                        None => moves.push((new_x, new_y)),
                        Some(other) => {
                            if other.color != piece.color {
                                moves.push((new_x, new_y));
                            }
                            break;
                        }
                    }
                }
            }
        }
        Piece::Queen => {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    for step in 1..=5 {
                        let new_y = y as i32 + dy * step;
                        let new_x = x as i32 + dx * step;

                        if !(0..8).contains(&new_y) || !(0..8).contains(&new_x) {
                            break;
                        }
                        let new_y = new_y as usize;
                        let new_x = new_x as usize;

                        match table[new_y][new_x] {
                            None => moves.push((new_x, new_y)),
                            Some(other) => {
                                if other.color != piece.color {
                                    moves.push((new_x, new_y));
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
        Piece::Knight => {
            let knight_moves = [
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ];

            for (dx, dy) in &knight_moves {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;

                if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
                    continue;
                }

                match table[new_y][new_x] {
                    None => moves.push((new_x, new_y)),
                    Some(other) => {
                        if other.color != piece.color {
                            moves.push((new_x, new_y));
                        }
                    }
                }
            }
        }
    }

    moves
}

fn start_up() -> Board {
    let cheese_table: Board = [
        [
            // [0][0]
            Some(ChessPiece {
                piece: Piece::Rook,
                color: Color::White,
            }),
            // [0][1]
            Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::White,
            }),
            // [0][2]
            Some(ChessPiece {
                piece: Piece::Knight,
                color: Color::White,
            }),
            // [0][3]
            Some(ChessPiece {
                piece: Piece::Queen,
                color: Color::White,
            }),
            // [0][4]
            Some(ChessPiece {
                piece: Piece::King,
                color: Color::White,
            }),
            // [0][5]
            Some(ChessPiece {
                piece: Piece::Knight,
                color: Color::White,
            }),
            // [0][6]
            Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::White,
            }),
            // [0][7]
            Some(ChessPiece {
                piece: Piece::Rook,
                color: Color::White,
            }),
        ],
        // [1][0] to [1][7]
        [Some(ChessPiece {
            piece: Piece::Pawn,
            color: Color::White,
        }); 8],
        // [2][0] to [2][7]
        [None; 8],
        // [3][0] to [3][7]
        [None; 8],
        // [4][0] to [4][7]
        [None; 8],
        // [5][0] to [5][7]
        [None; 8],
        // [6][0] to [6][7]
        [Some(ChessPiece {
            piece: Piece::Pawn,
            color: Color::Black,
        }); 8],
        [
            // [7][0]
            Some(ChessPiece {
                piece: Piece::Rook,
                color: Color::Black,
            }),
            // [7][1]
            Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::Black,
            }),
            // [7][2]
            Some(ChessPiece {
                piece: Piece::Knight,
                color: Color::Black,
            }),
            // [7][3]
            Some(ChessPiece {
                piece: Piece::Queen,
                color: Color::Black,
            }),
            // [7][4]
            Some(ChessPiece {
                piece: Piece::King,
                color: Color::Black,
            }),
            // [7][5]
            Some(ChessPiece {
                piece: Piece::Knight,
                color: Color::Black,
            }),
            // [7][6]
            Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::Black,
            }),
            // [7][7]
            Some(ChessPiece {
                piece: Piece::Rook,
                color: Color::Black,
            }),
        ],
    ];
    cheese_table
}

fn write_terminal(cheese_table: &Board, moves: &[(usize, usize)], flipped: bool) {
    let mut sayac = 1;
    let mut x = 1;
    let mut y = 1;
    let chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    for g in 0..17 {
        if sayac % 2 == 0 {
            let row = if flipped { 8 - y } else { y - 1 };
            print!(" {} ", row + 1);
            for _ in 0..8 {
                let col = x - 1;

                print!("┃");

                if moves.contains(&(col, row)) {
                    if cheese_table[row][col].is_some() {
                        print!("\x1b[41m");
                    } else {
                        print!("\x1b[42m");
                    }
                }

                match cheese_table[row][col] {
                    Some(chess_piece) => match (chess_piece.piece, chess_piece.color) {
                        (Piece::Pawn, Color::White) => print!(" ♟ "),
                        (Piece::Pawn, Color::Black) => print!(" ♙ "),
                        (Piece::King, Color::White) => print!(" ♚ "),
                        (Piece::King, Color::Black) => print!(" ♔ "),
                        (Piece::Rook, Color::White) => print!(" ♜ "),
                        (Piece::Rook, Color::Black) => print!(" ♖ "),
                        (Piece::Bishop, Color::White) => print!(" ♝ "),
                        (Piece::Bishop, Color::Black) => print!(" ♗ "),
                        (Piece::Queen, Color::White) => print!(" ♛ "),
                        (Piece::Queen, Color::Black) => print!(" ♕ "),
                        (Piece::Knight, Color::White) => print!(" ♞ "),
                        (Piece::Knight, Color::Black) => print!(" ♘ "),
                    },
                    None => print!("   "),
                }
                print!("\x1b[0m");
                x += 1;

                if x == 9 {
                    y += 1;
                    x = 1;
                }
            }
            println!("┃");
            sayac += 1;
        } else {
            if g == 0 {
                print!("   ");
                for i in 0..8 {
                    print!("  {} ", chars[i]);
                }
                println!();
                print!("   ");
                for _ in 0..8 {
                    print!("┃───");
                }
                println!("┃");
                sayac += 1;
                continue;
            }

            print!("   ");
            for _ in 0..8 {
                print!("┃───");
            }
            println!("┃");
            sayac += 1;
        }
    }
}
