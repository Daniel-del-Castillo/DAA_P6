use super::*;

pub struct IntraMachineReinsertion {}

impl LocalSearch for IntraMachineReinsertion {
    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 1)
            .map(|machine| {
                (0..solution.task_assignment_matrix[machine].len())
                    .map(move |task_index| {
                        (0..=solution.task_assignment_matrix[machine].len())
                            .filter(move |&possible_index| possible_index != task_index)
                            .map(move |possible_task_index| {
                                IntraMachineReinsertion::get_solution(
                                    instance,
                                    solution,
                                    machine,
                                    task_index,
                                    possible_task_index,
                                )
                            })
                    })
                    .flatten()
            })
            .flatten()
            .min_by_key(|solution| solution.get_total_completion_time())
    }
}

impl IntraMachineReinsertion {
    pub fn new() -> Self {
        IntraMachineReinsertion {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
        possible_task_index: usize,
    ) -> ProblemSolution {
        let mut possible_solution = solution.clone();
        let task = possible_solution.task_assignment_matrix[machine].remove(task_index);
        possible_solution.task_assignment_matrix[machine].insert(possible_task_index, task);
        possible_solution.tcts_by_machine[machine] = instance
            .calculate_total_completion_time(&possible_solution.task_assignment_matrix[machine]);
        possible_solution
    }
}
