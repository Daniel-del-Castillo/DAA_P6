use super::*;

pub struct InterMachineReinsertion {}

impl LocalSearch for InterMachineReinsertion {
    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 0)
            .map(|machine| {
                InterMachineReinsertion::get_best_reinsertion_from_machine(
                    instance, solution, machine,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
    }
}

impl InterMachineReinsertion {
    pub fn new() -> Self {
        InterMachineReinsertion {}
    }

    fn get_best_reinsertion_from_machine(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix[machine].len())
            .map(|task_index| {
                InterMachineReinsertion::get_best_reinsertion_from_machine_and_task_index(
                    instance, solution, machine, task_index,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }

    fn get_best_reinsertion_from_machine_and_task_index(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix.len())
            .filter(|&to_machine| to_machine != machine)
            .map(|to_machine| {
                InterMachineReinsertion::get_best_reinsertion_from_machine_and_task_index_to_machine(
                    instance, solution, machine, task_index, to_machine,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }

    fn get_best_reinsertion_from_machine_and_task_index_to_machine(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        from_machine: usize,
        task_index: usize,
        to_machine: usize,
    ) -> ProblemSolution {
        let mut solution = solution.clone();
        let task = solution.task_assignment_matrix[from_machine].remove(task_index);
        solution.tcts_by_machine[from_machine] = instance
            .calculate_total_completion_time(&solution.task_assignment_matrix[from_machine]);
        (0..=solution.task_assignment_matrix[to_machine].len())
            .map(|possible_pos| {
                let mut possible_solution = solution.clone();
                possible_solution.task_assignment_matrix[to_machine].insert(possible_pos, task);
                possible_solution.tcts_by_machine[to_machine] = instance
                    .calculate_total_completion_time(
                        &possible_solution.task_assignment_matrix[to_machine],
                    );
                possible_solution
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }
}
