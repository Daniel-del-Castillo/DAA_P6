use super::*;

pub struct NoSearch {}

impl LocalSearch for NoSearch {
    fn perform_search(
        _instance: &ProblemInstance,
        _solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        None
    }
}

impl NoSearch {
    pub fn new() -> Self {
        NoSearch {}
    }
}
