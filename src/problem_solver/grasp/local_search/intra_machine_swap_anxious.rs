use super::*;

/// An anxious local search that consists on doing swaps between tasks in the same machine
pub struct IntraMachineSwapAnxious {}

impl LocalSearch for IntraMachineSwapAnxious {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let solution_tct = solution.get_total_completion_time();
        for machine in 0..solution.task_assignment_matrix.len() {
            if solution.task_assignment_matrix[machine].len() < 2 {
                continue;
            }
            for task_index in 0..solution.task_assignment_matrix[machine].len() {
                for possible_task_index in 0..solution.task_assignment_matrix[machine].len() {
                    if task_index == possible_task_index {
                        continue;
                    }
                    let new_solution = IntraMachineSwapAnxious::get_solution(
                        instance,
                        &solution,
                        machine,
                        task_index,
                        possible_task_index,
                    );
                    if new_solution.get_total_completion_time() < solution_tct {
                        return new_solution;
                    }
                }
            }
        }
        solution
    }
}

impl IntraMachineSwapAnxious {
    pub fn new() -> Self {
        IntraMachineSwapAnxious {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
        possible_task_index: usize,
    ) -> ProblemSolution {
        let mut possible_solution = solution.clone();
        possible_solution.task_assignment_matrix[machine].swap(possible_task_index, task_index);
        possible_solution.tcts_by_machine[machine] = instance
            .calculate_total_completion_time(&possible_solution.task_assignment_matrix[machine]);
        possible_solution
    }
}
