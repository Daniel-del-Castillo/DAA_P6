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
                IntraMachineReinsertion::get_best_reinsertion_by_machine(
                    instance, solution, machine,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
    }
}

impl IntraMachineReinsertion {
    pub fn new() -> Self {
        IntraMachineReinsertion {}
    }

    fn get_best_reinsertion_by_machine(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix[machine].len())
            .map(|task_index| {
                IntraMachineReinsertion::get_best_reinsertion_by_machine_and_task_index(
                    instance, solution, machine, task_index,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }

    fn get_best_reinsertion_by_machine_and_task_index(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
    ) -> ProblemSolution {
        let mut solution = solution.clone();
        let task = solution.task_assignment_matrix[machine].remove(task_index);
        (0..=solution.task_assignment_matrix[machine].len())
            .filter(|&possible_index| possible_index != task_index)
            .map(|possible_index| {
                let mut possible_solution = solution.clone();
                possible_solution.task_assignment_matrix[machine].insert(possible_index, task);
                possible_solution.tcts_by_machine[machine] = instance
                    .calculate_total_completion_time(
                        &possible_solution.task_assignment_matrix[machine],
                    );
                possible_solution
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }
}
