use super::{ProblemInstance, ProblemSolution};

mod inter_machine_swap;
mod intra_machine_reinsertion;
mod intra_machine_swap;
mod no_search;
pub use inter_machine_swap::InterMachineSwap;
pub use intra_machine_reinsertion::IntraMachineReinsertion;
pub use intra_machine_swap::IntraMachineSwap;
pub use no_search::NoSearch;

pub trait LocalSearch {
    fn improve(&self, instance: &ProblemInstance, solution: ProblemSolution) -> ProblemSolution;
}
