use std::collections::VecDeque;

pub struct TicTacToe {
    board: Vec<Option<i32>>,
    turn: bool,
    p1: VecDeque<u32>,
    p2: VecDeque<u32>,
    limit: usize,
}

impl TicTacToe {
    pub fn new(lim: usize) -> TicTacToe {
        TicTacToe {
            board: vec![Option::None; 9],
            turn: false,
            p1: VecDeque::new(),
            p2: VecDeque::new(),
            limit: lim,
        }
    }

    pub fn player1(&mut self, id: u32) -> Result<bool, String> {
        // turn == false -> can move
        if self.turn == true {
            return Err("Not Your Turn!".to_string());
        }

        if self.board[id as usize] == None {
            self.board[id as usize] = Some(1);
            self.p1.push_back(id);
        } else {
            return Err("Invalid Move".to_string());
        }
        self.cleanup_expired();
        let iswin = self.is_win(1);
        self.turn = !self.turn;
        Ok(iswin)
    }

    pub fn player2(&mut self, id: u32) -> Result<bool, String> {
        // turn == true -> can move
        if self.turn == false {
            return Err("Not Your Turn!".to_string());
        }

        if self.board[id as usize] == None {
            self.board[id as usize] = Some(2);
            self.p2.push_back(id);
        } else {
            return Err("Invalid Move".to_string());
        }
        self.cleanup_expired();
        let iswin = self.is_win(2);
        self.turn = !self.turn;
        Ok(iswin)
    }

    pub fn show(&self) {
        for i in 0..3 {
            for j in 0..3 {
                if let Some(x) = self.board[i * 3 + j] {
                    print!("{} ", if x == 1 {"O"} else {"X"});
                } else {
                    print!(". ");    
                }
            }
            println!();
        }
    }

    fn is_win(&self, player: u32) -> bool {
        let winstatu: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], 
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6]
        ];
        for [a, b, c] in winstatu {
            if self.board[a] == self.board[b] && self.board[b] == self.board[c] && self.board[a] == Some(player as i32) {
                return true;
            }
        }
        false
    }
    fn cleanup_expired(&mut self) {
        while self.p1.len() > self.limit {
            let x = self.p1.pop_front().unwrap();
            self.board[x as usize] = None; 
        }

        while self.p2.len() > self.limit {
            let x = self.p2.pop_front().unwrap();
            self.board[x as usize] = None;
        }
    }
}
