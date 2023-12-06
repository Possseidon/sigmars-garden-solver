use std::time::{Duration, Instant};

use crate::board::{Board, Step};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct InitialBoard(Board);

pub(crate) enum SolveResult {
    Unsolvable,
    Timeout,
    Solution(Vec<Step>),
}

impl InitialBoard {
    pub(crate) fn new(board: Board) -> Option<Self> {
        board.is_valid_initial_state().then_some(Self(board))
    }

    pub(crate) fn solve(self, timeout: Duration) -> SolveResult {
        let mut valid_steps = Vec::<Vec<Step>>::new();
        let mut final_steps = Vec::<Step>::new();
        let mut board = self.0;
        let start = Instant::now();

        valid_steps.push(board.valid_steps());

        loop {
            let Some(step) = valid_steps.last_mut().unwrap().pop() else {
                if valid_steps.pop().is_none() {
                    break SolveResult::Unsolvable;
                }

                let step = final_steps.pop().unwrap();
                board.set(step.0[0], self.0.get(step.0[0]));
                board.set(step.0[1], self.0.get(step.0[1]));
                continue;
            };

            final_steps.push(step);
            board.set(step.0[0], None);
            board.set(step.0[1], None);

            if board.is_solved() {
                break SolveResult::Solution(final_steps);
            }

            valid_steps.push(board.valid_steps());

            if start.elapsed() > timeout {
                break SolveResult::Timeout;
            }
        }
    }
}
