use super::{ProblemInstance, ProblemSolution, ProblemSolver, RandomizedGreedySolver};

pub mod local_search;
pub mod stop_criterion;
use local_search::LocalSearch;
use stop_criterion::StopCriterion;

/// A implementation of a GRASP algorithm. The stop
/// criterion and the local search to be used can be chosen and passed to the
/// constructor. For the constructive phase it will use the
/// [Randomized greedy solver algorithm](super::RandomizedGreedySolver), the k can
/// also be passed as an argument in the constructor
pub struct GRASP<L: LocalSearch, S: StopCriterion> {
    size_to_choose_from: usize,
    local_search: L,
    stop_criterion: S,
}

impl<L: LocalSearch, S: StopCriterion> ProblemSolver for GRASP<L, S> {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        let mut solver = RandomizedGreedySolver::new(self.size_to_choose_from);
        let mut solution = self.local_search.improve(instance, solver.solve(instance));
        let mut solution_tct = solution.get_total_completion_time();
        loop {
            let new_solution = self.local_search.improve(instance, solver.solve(instance));
            let new_solution_tct = new_solution.get_total_completion_time();
            if self.stop_criterion.stop(solution_tct, new_solution_tct) {
                if solution_tct <= new_solution_tct {
                    return solution;
                } else {
                    return new_solution;
                }
            }
            if new_solution_tct < solution_tct {
                solution = new_solution;
                solution_tct = new_solution_tct;
            }
        }
    }
}

impl<L: LocalSearch, S: StopCriterion> GRASP<L, S> {
    /// Creates a new GRASP with the specified arguments
    pub fn new(size_to_choose_from: usize, local_search: L, stop_criterion: S) -> Self {
        assert!(size_to_choose_from > 0);
        GRASP {
            size_to_choose_from,
            local_search,
            stop_criterion,
        }
    }
}
