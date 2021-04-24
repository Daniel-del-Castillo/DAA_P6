use super::ProblemInstance;
use std::cmp::Ordering;

mod fast_greedy_solver;
pub mod grasp;
mod greedy_solver;
mod problem_solution;
mod randomized_greedy_solver;
pub use fast_greedy_solver::FastGreedySolver;
pub use grasp::GRASP;
pub use greedy_solver::GreedySolver;
pub use problem_solution::ProblemSolution;
pub use randomized_greedy_solver::RandomizedGreedySolver;

/// A trait for an algorithm that is able to solve an instance of the problem
pub trait ProblemSolver {
    /// Solves an instance of the problem
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution;
}

#[derive(Eq)]
struct NewTask {
    machine: usize,
    task: usize,
    position: usize,
    tct_increment: usize,
}

impl Ord for NewTask {
    fn cmp(&self, other: &Self) -> Ordering {
        other.tct_increment.cmp(&self.tct_increment)
    }
}

impl PartialOrd for NewTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NewTask {
    fn eq(&self, other: &Self) -> bool {
        self.tct_increment == other.tct_increment
    }
}
