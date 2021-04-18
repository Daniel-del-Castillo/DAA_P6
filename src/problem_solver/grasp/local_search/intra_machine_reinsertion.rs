use super::*;

pub struct IntraMachineReinsertion {}

impl LocalSearch for IntraMachineReinsertion {
    fn improve(
        &self,
        instance: &ProblemInstance,
        mut solution: ProblemSolution,
    ) -> ProblemSolution {
        loop {
            let another_solution =
                match IntraMachineReinsertion::perform_search(instance, &solution) {
                    None => return solution,
                    Some(another_solution) => another_solution,
                };
            if another_solution.get_total_completion_time() >= solution.get_total_completion_time()
            {
                return solution;
            }
            solution = another_solution;
        }
    }
}

impl IntraMachineReinsertion {
    pub fn new() -> Self {
        IntraMachineReinsertion {}
    }
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
        (0..solution.task_assignment_matrix[machine].len())
            .filter(|&possible_task| possible_task != task_index)
            .map(|possible_task| {
                let mut possible_solution = solution.clone();
                possible_solution.task_assignment_matrix[machine].swap(possible_task, task_index);
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
