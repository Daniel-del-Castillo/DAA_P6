use super::*;

/// A struct to allow a GRASP to execute without performing a local search
pub struct NoSearch {}

impl LocalSearch for NoSearch {
    fn perform_search(
        &self,
        _instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        solution
    }
}

impl NoSearch {
    /// Returns an instance
    pub fn new() -> Self {
        NoSearch {}
    }
}
