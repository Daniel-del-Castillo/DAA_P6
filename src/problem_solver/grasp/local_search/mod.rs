use super::{ProblemInstance, ProblemSolution};

mod intra_machine_reinsertion;
mod no_search;
pub use intra_machine_reinsertion::IntraMachineReinsertion;
pub use no_search::NoSearch;

pub trait LocalSearch {
    fn improve(&self, instance: &ProblemInstance, solution: ProblemSolution) -> ProblemSolution;
}
