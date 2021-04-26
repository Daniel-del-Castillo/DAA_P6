use super::*;

/// A local search that consists on reinserting a task in a different machine
pub struct InterMachineReinsertion {}

impl LocalSearch for InterMachineReinsertion {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let solution_ref = &solution;
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 0)
            .flat_map(|from_machine| {
                (0..solution.task_assignment_matrix[from_machine].len()).flat_map(
                    move |task_index| {
                        (0..solution_ref.task_assignment_matrix.len()).flat_map(move |to_machine| {
                            (0..=solution_ref.task_assignment_matrix[to_machine].len())
                                .filter(move |&to_machine| to_machine != from_machine)
                                .map(move |possible_insertion_index| {
                                    InterMachineReinsertion::get_solution(
                                        instance,
                                        solution_ref,
                                        from_machine,
                                        task_index,
                                        to_machine,
                                        possible_insertion_index,
                                    )
                                })
                        })
                    },
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap_or(solution)
    }
}

impl InterMachineReinsertion {
    pub fn new() -> Self {
        InterMachineReinsertion {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        from_machine: usize,
        task_index: usize,
        to_machine: usize,
        possible_insertion_index: usize,
    ) -> ProblemSolution {
        let mut possible_solution = solution.clone();
        possible_solution.task_assignment_matrix[to_machine]
            .insert(possible_insertion_index, task_index);
        possible_solution.tcts_by_machine[from_machine] = instance.calculate_total_completion_time(
            &possible_solution.task_assignment_matrix[from_machine],
        );
        possible_solution.tcts_by_machine[to_machine] = instance
            .calculate_total_completion_time(&possible_solution.task_assignment_matrix[to_machine]);
        possible_solution
    }
}
