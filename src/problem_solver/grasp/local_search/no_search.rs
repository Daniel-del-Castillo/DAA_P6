use super::*;

pub struct NoSearch {}

impl LocalSearch for NoSearch {
    fn improve(&self, _instance: &ProblemInstance, solution: ProblemSolution) -> ProblemSolution {
        solution
    }
}

impl NoSearch {
    pub fn new() -> Self {
        NoSearch {}
    }
}
