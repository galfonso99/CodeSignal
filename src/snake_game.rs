use std::collections::VecDeque;
#[derive(Debug)]
struct Snake {
    direction: char,
    positions: VecDeque<(usize, usize)>,
}

impl Snake {
    fn new(direction: char, positions: VecDeque<(usize, usize)>) -> Self {
        Self { direction, positions }
    }
    fn move_forward(&mut self, board: &mut [Vec<char>]) {
        let next_position = self.next_position();
        let (i, j) = next_position;
        board[i][j] = self.direction;
        let (i, j) = self.positions[0];
        board[i][j] = '*';
        self.positions.push_front(next_position);
        let (i, j) = self.positions.pop_back().unwrap();
        board[i][j] = '.';
    }

    fn turn_right(&mut self, board: &mut [Vec<char>]) {
        let current_dir = self.direction;
        match current_dir {
            '^' => self.direction = '>',
            '<' => self.direction = '^',
            '>' => self.direction = 'v',
            'v' => self.direction = '<',
            _ => (),
        }
        let (i, j) = self.positions[0];
        board[i][j] = self.direction;
    }

    fn turn_left(&mut self, board: &mut [Vec<char>]) {
        let current_dir = self.direction;
        match current_dir {
            '^' => self.direction = '<',
            '<' => self.direction = 'v',
            '>' => self.direction = '^',
            'v' => self.direction = '>',
            _ => (),
        }
        let (i, j) = self.positions[0];
        board[i][j] = self.direction;
    }

    fn can_move(&mut self, board: &mut [Vec<char>]) -> bool {
        let (i, j) = self.next_position();
        let inside = within_board(board, i as i32, j as i32);
        let eats_tail = self.positions.contains(&(i, j));
        inside && !eats_tail
    }

    fn next_position(&mut self) -> (usize, usize) {
        let delta: (i32, i32) = match self.direction {
            '^' => (-1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => (0, 0),
        };
        let head = self.positions[0];

        let new_head = (head.0 as i32 + delta.0, head.1 as i32 + delta.1);
        (new_head.0 as usize, new_head.1 as usize)
    }
}

fn solution(mut board: Vec<Vec<char>>, commands: String) -> Vec<Vec<char>> {
    // Get initial board configuration
    let (initial_direction, initial_positions) = handle_initial_configuration(&board);
    let mut snake = Snake::new(initial_direction, initial_positions);
    walk_the_snake(&mut board, &commands, snake);
    board
}

fn handle_initial_configuration(board: &[Vec<char>]) -> (char, VecDeque<(usize, usize)>) {
    let dir_pos = get_initial_direction_pos(&board);
    let initial_direction = board[dir_pos.0][dir_pos.1];
    let mut initial_positions: VecDeque<(usize, usize)> = VecDeque::from([dir_pos]);
    set_snake_positions(board, &mut initial_positions, dir_pos);
    (initial_direction, initial_positions)
}

fn walk_the_snake(board: &mut [Vec<char>], commands: &str, mut snake: Snake) {
    for c in commands.chars() {
        match c {
            'F' => {
                if !snake.can_move(board) {
                    kill_snake(board, snake);
                    break;
                }
                snake.move_forward(board);
            }
            'R' => snake.turn_right(board),
            'L' => snake.turn_left(board),
            _ => (),
        }
    }
}

fn kill_snake(board: &mut [Vec<char>], snake: Snake) {
    for pos in snake.positions {
        board[pos.0][pos.1] = 'X';
    }
}

fn get_initial_direction_pos(board: &[Vec<char>]) -> (usize, usize) {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let dir = board[i][j];
            if dir == '>' || dir == '<' || dir == '^' || dir == 'v' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn set_snake_positions(board: &[Vec<char>], positions: &mut VecDeque<(usize, usize)>, dir_pos: (usize, usize)) {
    let mut prev = (dir_pos.0, dir_pos.1);
    for _ in 0.. {
        if possible_tail(board, positions, prev, (-1, 0)) && board[prev.0 - 1][prev.1] == '*' {
            positions.push_back((prev.0 - 1, prev.1));
            prev = (prev.0 - 1, prev.1);
        } else if possible_tail(board, positions, prev, (0, -1)) && board[prev.0][prev.1 - 1] == '*' {
            positions.push_back((prev.0, prev.1 - 1));
            prev = (prev.0, prev.1 - 1);
        } else if possible_tail(board, positions, prev, (0, 1)) && board[prev.0][prev.1 + 1] == '*' {
            positions.push_back((prev.0, prev.1 + 1));
            prev = (prev.0, prev.1 + 1);
        } else if possible_tail(board, positions, prev, (1, 0)) && board[prev.0 + 1][prev.1] == '*' {
            positions.push_back((prev.0 + 1, prev.1));
            prev = (prev.0 + 1, prev.1);
        } else {
            break;
        }
    }
}

fn possible_tail(board: &[Vec<char>], positions: &VecDeque<(usize, usize)>, prev: (usize, usize), dir: (i32, i32)) -> bool {
    let square = (prev.0 as i32 + dir.0, prev.1 as i32 + dir.1);
    let inside_board = within_board(board, square.0, square.1);
    let already_tail = positions.contains(&(square.0 as usize, square.1 as usize));
    inside_board && !already_tail
}

fn within_board(board: &[Vec<char>], y: i32, x: i32) -> bool {
    x >= 0 && x < board[0].len() as i32 && y >= 0 && y < board.len() as i32
}