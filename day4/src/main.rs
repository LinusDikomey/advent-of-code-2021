fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines().peekable();
    let numbers = lines.next().unwrap().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut boards = Vec::new();
    while let Some("") = lines.next() {
        if lines.peek().is_none() { break; }
        let parse_line = |line: &str| {
            let mut nums = line.split(' ').filter(|s| !s.is_empty());
            let mut line = [(0, false); 5];
            for i in 0..5 {
                line[i] = (nums.next().unwrap().parse().unwrap(), false);
            }
            line
        };
        let mut board = [[(0, false); 5]; 5];
        for i in 0..5 {
            board[i] = parse_line(lines.next().unwrap());
        }
        boards.push(board);
    }

    println!("Part 1: {}", part_1(&numbers, boards.clone()));
    println!("Part 2: {}", part_2(&numbers, boards));
}

type Board = [[(i32, bool); 5]; 5];

fn part_1(numbers: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for num in numbers {
        for board in &mut boards {
            mark_num(*num, board);
            if won(board) {
                return score(board, *num);
            }
        }
    }
    panic!("No winning board found")
}

fn part_2(numbers: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for num in numbers {
        for board in &mut boards {
            mark_num(*num, board);
        }
        if boards.len() == 1 && won(&boards[0]) {
            return score(&boards[0], *num);
        }
        boards.retain(|b| !won(b));

    }
    panic!("No last losing board found")
}


fn won(board: &Board) -> bool {
    // horizontal
    for row in board {
        if row.iter().filter(|(_, m)| !m).count() == 0 {
            return true;
        }
    }
    // vertical
    for col in 0..5 {
        let mut win = true;
        for row in 0..5 {
            if !board[row][col].1 {
                win = false;
                break;
            }
        }
        if win { return true; }
    }
    // diagonal top left -> bottom right
    let mut diag_win = true;
    for diag in 0..5 {
        if !board[diag][diag].1 {
            diag_win = false;
            break;
        }
    }
    if diag_win { return true; }
    // diagonal top right -> bottom left
    let mut diag_win = true;
    for diag in 0..5 {
        if !board[4 - diag][diag].1 {
            diag_win = false;
            break;
        }
    }
    if diag_win { return true; }
    false
}

fn mark_num(num: i32, board: &mut Board) {
    for row in board.iter_mut() {
        for (board_num, marked) in row {
            if *board_num == num {
                *marked = true;
            }
        }
    }
}

fn score(board: &Board, last_num: i32) -> i32 {
    last_num * board.iter()
        .map(|row| row.iter().filter_map(|(n, m)| if *m { None } else {Some(n)} ).sum::<i32>())
        .sum::<i32>()
}
