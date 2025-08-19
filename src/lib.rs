#[derive(Default)]
pub struct TennisGame {
    p1points: u8,
    p2points: u8,
    _player1_name: String,
    _player2_name: String,
}

impl TennisGame {
    pub fn new() -> Self {
        Self {
            p1points: 0,
            p2points: 0,
            _player1_name: "player1".to_string(),
            _player2_name: "player2".to_string(),
        }
    }
    pub fn clear(&mut self) {
        self.p1points = 0;
        self.p2points = 0;
    }
    pub fn won_point(&mut self, player_name: &str) {
        if player_name == self._player1_name {
            self.p1_score();
        } else {
            self.p2_score();
        }
    }
    fn p1_score(&mut self) {
        self.p1points += 1;
    }
    fn p2_score(&mut self) {
        self.p2points += 1;
    }
    pub fn get_score(&self) -> String {
        if (self.p1points >= 4) && (self.p1points as i16 - self.p2points as i16) >= 2 {
            "Win for player1".to_string()
        } else if (self.p2points >= 4) && (self.p2points as i16 - self.p1points as i16) >= 2 {
            "Win for player2".to_string()
        } else if (self.p1points > self.p2points) && (self.p2points >= 3) {
            "Advantage player1".to_string()
        } else if (self.p2points > self.p1points) && (self.p1points >= 3) {
            "Advantage player2".to_string()
        } else if (self.p1points == self.p2points) && (self.p1points > 2) {
            "Deuce".to_string()
        } else if (self.p1points == self.p2points) && (self.p1points < 3) {
            let s = match self.p1points {
                0 => "Love",
                1 => "Fifteen",
                2 => "Thirty",
                _ => "",
            };
            format!("{}-All", s)
        } else if (self.p1points > 0) && (self.p2points == 0) {
            let s = match self.p1points {
                1 => "Fifteen",
                2 => "Thirty",
                3 => "Forty",
                _ => "",
            };
            format!("{}-Love", s)
        } else if (self.p2points > 0) && (self.p1points == 0) {
            let s = match self.p2points {
                1 => "Fifteen",
                2 => "Thirty",
                3 => "Forty",
                _ => "",
            };
            format!("Love-{}", s)
        } else if (self.p1points > self.p2points) && (self.p1points < 4) {
            let p1res = match self.p1points {
                2 => "Thirty",
                3 => "Forty",
                _ => "",
            };
            let p2res = match self.p2points {
                1 => "Fifteen",
                2 => "Thirty",
                _ => "",
            };
            format!("{}-{}", p1res, p2res)
        } else if (self.p2points > self.p1points) && (self.p2points < 4) {
            let p1res = match self.p1points {
                1 => "Fifteen",
                2 => "Thirty",
                _ => "",
            };
            let p2res = match self.p2points {
                2 => "Thirty",
                3 => "Forty",
                _ => "",
            };
            format!("{}-{}", p1res, p2res)
        } else {
            "".to_string()
        }
    }
}
