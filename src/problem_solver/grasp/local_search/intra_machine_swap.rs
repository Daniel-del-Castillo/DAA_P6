use super::*;

/// A local search that consists on doing swaps between tasks in the same machine
pub struct IntraMachineSwap {}

impl LocalSearch for IntraMachineSwap {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 1)
            .flat_map(|machine| {
                (0..solution.task_assignment_matrix[machine].len()).flat_map(move |task_index| {
                    (0..solution.task_assignment_matrix[machine].len())
                        .filter(move |&possible_task| possible_task != task_index)
                        .map(move |possible_task_index| {
                            IntraMachineSwap::get_solution(
                                instance,
                                solution,
                                machine,
                                task_index,
                                possible_task_index,
                            )
                        })
                })
            })
            .min_by_key(|solution| solution.get_total_completion_time())
    }
}

impl IntraMachineSwap {
    pub fn new() -> Self {
        IntraMachineSwap {}
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
