use std::cmp::PartialEq;
use std::collections::HashSet;
use rand::distr::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Copy,Clone,Debug, PartialEq, Deserialize, Serialize)]
pub enum CaseState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Copy,Clone,Debug, PartialEq, Deserialize, Serialize)]
pub enum Case {
    Number(u8, CaseState),
    Mine(CaseState),
}

impl Case {

    pub fn is_revealed(&self) -> bool {
        match self {
            Case::Number(_, state) | Case::Mine(state) => *state == CaseState::Revealed,
        }
    }

    pub fn is_flagged(&self) -> bool {
        match self {
            Case::Number(_, state) | Case::Mine(state) => *state == CaseState::Flagged,
        }
    }

    pub fn is_mine(&self) -> bool {
        match self {
            Case::Mine(_) => true,
            _ => false,
        }
    }

    pub fn get_mines_around(&self) -> u8 {
        match self {
            Case::Number(number, _) => { *number}
            Case::Mine(_) => {0}
        }
    }

    pub fn reveal(&mut self) {
        match self {
            Case::Number(n, state) => {
                *state = CaseState::Revealed;
            },
            Case::Mine(state) => {
                *state = CaseState::Revealed;
            },
        }
    }


    pub fn  flag(&mut self, status: bool) {
        match self {
            Case::Number(n, state) => {
                *state = CaseState::Flagged;
            },
            Case::Mine(state) => {
                *state = CaseState::Flagged;
            },
        }
    }
}

#[derive(Copy,Clone,Debug, PartialEq)]
pub struct Board {
     cases: [[Case; 10]; 10],
}

impl Board {
    pub fn new(mines_count: usize) -> Board {
        let mut cases = [[Case::Number(0, CaseState::Hidden); 10]; 10];
        let mut mines_cases = HashSet::<(u8, u8)>::new();
        let step = Uniform::new(0, 10).unwrap();
        let mut rng = rand::rng();

        while mines_cases.len() < mines_count {
            let x = step.sample(&mut rng);
            let y = step.sample(&mut rng);
            mines_cases.insert((x, y));
        }


        for (x, y) in mines_cases {
            cases[x as usize][y as usize] = Case::Mine(CaseState::Hidden);
            for i in -1..=1 {
                for j in -1..=1 {
                    let nx = x as i8 + i;
                    let ny = y as i8 + j;
                    if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                        cases[nx as usize][ny as usize] = match cases[nx as usize][ny as usize] {
                            Case::Number(n, state) => Case::Number(n + 1, state),
                            other => {other}
                        };
                    }
                }
            }
        }
        Board { cases: cases }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Case> {
        self.cases.iter().flatten()
    }

    pub fn reveal(&mut self, position: usize) -> Vec<(usize, Case)> {
        let mut acc = vec![];
        let y = position % 10;
        let x = position / 10;
        println!("Revealing position {} case at ({}, {})", position, x, y);
        self.reveal_case(x, y, &mut acc);
        println!("acc returned {:?}", acc);
        acc
    }

    fn reveal_case(&mut self,x: usize,y: usize, acc: &mut Vec<(usize, Case)>) {
        match self.cases[x][y] {
            Case::Number(n, CaseState::Hidden) => {
                self.cases[x][y] = Case::Number(n, CaseState::Revealed);
                acc.push((x * 10 + y, self.cases[x][y]));
                if n == 0 {
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let nx = x as i8 + i;
                            let ny = y as i8 + j;
                            if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                                self.reveal_case(nx as usize, ny as usize, acc);
                            }
                        }
                    }
                }
            },
            Case::Mine(CaseState::Hidden) => {
                self.cases[x][y] = Case::Mine(CaseState::Revealed);
                acc.push((x * 10 + y, self.cases[x][y]));
            },
            _ => {}
        };
    }
}