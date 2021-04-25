use super::{
    grasp::{
        local_search::{InterMachineReinsertion, LocalSearch},
        stop_criterion::{StopCriterion, TotalIterations},
        GRASP,
    },
    ProblemInstance, ProblemSolution, ProblemSolver,
};

/// A VNS implementation. The different environments used and the stop criterion
/// can be specified in the constructor. It creates new solutions until the stop
/// criterion is fullfilled. For each solution it creates a solution using
/// (GRASP)[super::GRASP] and then modifies the solution with a number of random inter
/// machine reinsertions, performs local searches until it reaches a local minimum
/// for every environment and if that solution is better than the actual one, it updates
/// the solution. THe number of random reinsertions increases each time the solution found
/// was worse or equal than the actual one, until a max value that can be specified in the
/// costructor
pub struct VNS<S: StopCriterion> {
    max_k: usize,
    stop_criterion: S,
    searches: Vec<Box<dyn LocalSearch>>,
}

impl<S: StopCriterion> ProblemSolver for VNS<S> {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        let mut solution = self.search(instance);
        let mut solution_tct = solution.get_total_completion_time();
        loop {
            let new_solution = self.search(instance);
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

impl<S: StopCriterion> VNS<S> {
    /// Creates a new instance with the specified max number of random reinsertions,
    /// stop criterion and local searches. The max number of random reinsertions must be
    /// than 0. The order of the searches in the vector will be preserved and can affect
    /// the performance of the algorithm
    pub fn new(max_k: usize, stop_criterion: S, searches: Vec<Box<dyn LocalSearch>>) -> Self {
        assert!(max_k > 0);
        VNS {
            max_k,
            stop_criterion,
            searches,
        }
    }

    fn search(&self, instance: &ProblemInstance) -> ProblemSolution {
        let mut grasp = GRASP::new(1, InterMachineReinsertion::new(), TotalIterations::new(1));
        let mut solution = grasp.solve(instance);
        let mut k = 1;
        while k <= self.max_k {
            let mut new_solution = VNS::<S>::shake(instance, solution.clone(), k);
            new_solution = self.vnd(instance, new_solution);
            if solution.get_total_completion_time() < new_solution.get_total_completion_time() {
                k += 1;
            } else {
                solution = new_solution;
                k = 1;
            }
        }
        solution
    }

    fn shake(
        instance: &ProblemInstance,
        mut solution: ProblemSolution,
        number_of_shakes: usize,
    ) -> ProblemSolution {
        for _ in 0..number_of_shakes {
            let from_machine = rand::random::<usize>() % solution.task_assignment_matrix.len();
            let from_pos =
                rand::random::<usize>() % solution.task_assignment_matrix[from_machine].len();
            let to_machine = loop {
                let number = rand::random::<usize>() % solution.task_assignment_matrix.len();
                if number != from_machine {
                    break number;
                }
            };
            let to_pos =
                rand::random::<usize>() % (solution.task_assignment_matrix[to_machine].len() + 1);
            let task = solution.task_assignment_matrix[from_machine].remove(from_pos);
            solution.task_assignment_matrix[to_machine].insert(to_pos, task);
            solution.tcts_by_machine[from_machine] = instance
                .calculate_total_completion_time(&solution.task_assignment_matrix[from_machine]);
            solution.tcts_by_machine[to_machine] = instance
                .calculate_total_completion_time(&solution.task_assignment_matrix[to_machine]);
        }
        solution
    }

    fn vnd(&self, instance: &ProblemInstance, mut solution: ProblemSolution) -> ProblemSolution {
        let mut search_index = 0;
        while search_index < self.searches.len() {
            let new_solution = self.searches[search_index].improve(instance, solution.clone());
            if solution.get_total_completion_time() < new_solution.get_total_completion_time() {
                search_index += 1;
            } else {
                solution = new_solution;
                search_index = 0;
            }
        }
        solution
    }
}
