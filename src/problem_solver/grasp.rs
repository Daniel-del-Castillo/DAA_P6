use super::{ProblemInstance, ProblemSolution, ProblemSolver, RandomizedGreedySolver};

pub struct GRASP {
    size_to_choose_from: usize,
    repetitions: usize,
}

impl ProblemSolver for GRASP {
    fn solve(self, instance: &ProblemInstance) -> ProblemSolution {
        (0..self.repetitions)
            .map(|_| {
                let solver = RandomizedGreedySolver::new(self.size_to_choose_from);
                solver.solve(instance)
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }
}

impl GRASP {
    pub fn new(size_to_choose_from: usize, repetitions: usize) -> Self {
        assert!(size_to_choose_from > 0 && repetitions > 0);
        GRASP {
            size_to_choose_from,
            repetitions,
        }
    }
}
