use super::ProblemInstance;

mod fast_greedy_solver;
mod greedy_solver;
mod problem_solution;
pub use fast_greedy_solver::FastGreedySolver;
pub use greedy_solver::GreedySolver;
pub use problem_solution::ProblemSolution;

pub trait ProblemSolver {
    fn solve(self, instance: &ProblemInstance) -> ProblemSolution;
}
