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

pub trait LocalSearch {
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

    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution>;
}
