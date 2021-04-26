use super::*;
use std::mem;

/// A local search that consists on doing swaps between tasks in the different machines
pub struct InterMachineSwap {}

impl LocalSearch for InterMachineSwap {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let solution_ref = &solution;
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 1)
            .flat_map(|from_machine| {
                (0..solution.task_assignment_matrix[from_machine].len()).flat_map(
                    move |task_index| {
                        (0..solution_ref.task_assignment_matrix.len())
                            .filter(move |&to_machine| to_machine != from_machine)
                            .flat_map(move |to_machine| {
                                (0..solution_ref.task_assignment_matrix[to_machine].len()).map(
                                    move |possible_swap_index| {
                                        InterMachineSwap::get_solution(
                                            instance,
                                            solution_ref,
                                            from_machine,
                                            task_index,
                                            to_machine,
                                            possible_swap_index,
                                        )
                                    },
                                )
                            })
                    },
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap_or(solution)
    }
}

impl InterMachineSwap {
    pub fn new() -> Self {
        InterMachineSwap {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        from_machine: usize,
        task_index: usize,
        to_machine: usize,
        possible_swap_index: usize,
    ) -> ProblemSolution {
        let mut possible_solution = solution.clone();
        let mut to_machine_tasks = possible_solution.task_assignment_matrix[to_machine].clone();
        mem::swap(
            &mut possible_solution.task_assignment_matrix[from_machine][task_index],
            &mut to_machine_tasks[possible_swap_index],
        );
        possible_solution.task_assignment_matrix[to_machine] = to_machine_tasks;
        possible_solution.tcts_by_machine[from_machine] = instance.calculate_total_completion_time(
            &possible_solution.task_assignment_matrix[from_machine],
        );
        possible_solution.tcts_by_machine[to_machine] = instance
            .calculate_total_completion_time(&possible_solution.task_assignment_matrix[to_machine]);
        possible_solution
    }
}
