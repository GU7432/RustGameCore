use std::collections::VecDeque;

pub struct TicTacToe {
    board: Vec<i32>,
    turn: bool,
    p1: VecDeque<u32>,
    p2: VecDeque<u32>,
    limit: usize,
}

impl TicTacToe {
    pub fn new(lim: usize) -> TicTacToe {
        TicTacToe {
            board: vec![0; 9],
            turn: false,
            p1: VecDeque::new(),
            p2: VecDeque::new(),
            limit: lim,
        }
    }

    pub fn player1(&mut self, id: u32) -> bool {
        // turn == false -> can move
        if self.turn == true {
            return false;
        }

        if self.board[id as usize] == 0 {
            self.board[id as usize] = 1;
            self.p1.push_back(id);
        } else {
            return false;
        }
        self.cleanup_expired();
        self.turn = !self.turn;
        true
    }

    pub fn player2(&mut self, id: u32) -> bool {
        // turn == true -> can move
        if self.turn == false {
            return false;
        }

        if self.board[id as usize] == 0 {
            self.board[id as usize] = 2;
            self.p2.push_back(id);
        } else {
            return false;
        }
        self.cleanup_expired();
        self.turn = !self.turn;
        true
    }

    pub fn show(&self) {
        for i in 0..=3 {
            for j in 0..=3 {
                print!("{} ", self.board[i * 3 + j]);
            }
            println!();
        }
    }

    fn cleanup_expired(&mut self) {
        while self.p1.len() > self.limit {
            let x = self.p1.pop_front().unwrap();
            self.board[x as usize] = 0; 
        }

        while self.p2.len() > self.limit {
            let x = self.p2.pop_front().unwrap();
            self.board[x as usize] = 0;
        }
    }
}
