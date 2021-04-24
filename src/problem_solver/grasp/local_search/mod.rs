use super::{ProblemInstance, ProblemSolution};

mod inter_machine_reinsertion;
mod inter_machine_reinsertion_anxious;
mod inter_machine_swap;
mod inter_machine_swap_anxious;
mod intra_machine_reinsertion;
mod intra_machine_reinsertion_anxious;
mod intra_machine_swap;
mod intra_machine_swap_anxious;
mod no_search;
pub use inter_machine_reinsertion::InterMachineReinsertion;
pub use inter_machine_reinsertion_anxious::InterMachineReinsertionAnxious;
pub use inter_machine_swap::InterMachineSwap;
pub use inter_machine_swap_anxious::InterMachineSwapAnxious;
pub use intra_machine_reinsertion::IntraMachineReinsertion;
pub use intra_machine_reinsertion_anxious::IntraMachineReinsertionAnxious;
pub use intra_machine_swap::IntraMachineSwap;
pub use intra_machine_swap_anxious::IntraMachineSwapAnxious;
pub use no_search::NoSearch;

/// A trait that specifies how a local search should behave. A local search
/// should search for better solutions inside an specific environment and
/// only stop searching when it can't find a better one. They are usually divided
/// in two types:
/// * Greedy: The explore all the solutions in the environment and move to the best
/// one to keep exploring
/// * Anxious: They move to a new solution as soon as they have found a better one
pub trait LocalSearch {
    /// Performs a local search that stops when there isn't a better solution
    fn improve(
        &self,
        instance: &ProblemInstance,
        mut solution: ProblemSolution,
    ) -> ProblemSolution {
        loop {
            let another_solution = match Self::perform_search(instance, &solution) {
                None => return solution,
                Some(another_solution) => another_solution,
            };
            if another_solution.get_total_completion_time() >= solution.get_total_completion_time()
            {
                return solution;
            }
            solution = another_solution;
        }
    }

    /// Performs a local search **only** in the environment of the actual solution.
    /// It can return a solution that **might** be a better one than the actual
    /// or it can return None
    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution>;
}
