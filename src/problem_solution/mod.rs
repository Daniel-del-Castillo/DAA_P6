use super::ProblemInstance;

mod fast_greedy_solution;
mod greedy_solution;
pub use fast_greedy_solution::FastGreedySolution;
pub use greedy_solution::GreedySolution;

pub trait ProblemSolution {
    fn solve(instance: &ProblemInstance) -> Self;

    fn get_total_completion_time(&self) -> usize;

    fn get_tcts_by_machine(&self) -> &Vec<usize>;

    fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>>;
}
