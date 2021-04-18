use super::*;
use std::mem;

pub struct InterMachineSwap {}

impl LocalSearch for InterMachineSwap {
    fn improve(
        &self,
        instance: &ProblemInstance,
        mut solution: ProblemSolution,
    ) -> ProblemSolution {
        loop {
            let another_solution = match InterMachineSwap::perform_search(instance, &solution) {
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

impl InterMachineSwap {
    pub fn new() -> Self {
        InterMachineSwap {}
    }

    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        (0..solution.task_assignment_matrix.len())
            .filter(|&machine| solution.task_assignment_matrix[machine].len() > 1)
            .map(|machine| {
                InterMachineSwap::get_best_swap_from_machine(instance, solution, machine)
            })
            .min_by_key(|solution| solution.get_total_completion_time())
    }

    fn get_best_swap_from_machine(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix[machine].len())
            .map(|task_index| {
                InterMachineSwap::get_best_swap_from_machine_and_task_index(
                    instance, solution, machine, task_index,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }

    fn get_best_swap_from_machine_and_task_index(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix.len())
            .filter(|&to_machine| to_machine != machine)
            .map(|to_machine| {
                InterMachineSwap::get_best_swap_from_machine_and_task_index_to_machine(
                    instance, solution, machine, task_index, to_machine,
                )
            })
            .min_by_key(|solution| solution.get_total_completion_time())
            .unwrap()
    }

    fn get_best_swap_from_machine_and_task_index_to_machine(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        from_machine: usize,
        task_index: usize,
        to_machine: usize,
    ) -> ProblemSolution {
        (0..solution.task_assignment_matrix[to_machine].len())
            .map(|possible_task_swap| {
                let mut possible_solution = solution.clone();
                let mut to_machine_tasks =
                    possible_solution.task_assignment_matrix[to_machine].clone();
                mem::swap(
                    &mut possible_solution.task_assignment_matrix[from_machine][task_index],
                    &mut to_machine_tasks[possible_task_swap],
                );
                possible_solution.task_assignment_matrix[to_machine] = to_machine_tasks;
                possible_solution.tcts_by_machine[from_machine] = instance
                    .calculate_total_completion_time(
                        &possible_solution.task_assignment_matrix[from_machine],
                    );
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
