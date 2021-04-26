use super::*;
use std::mem;

/// An anxious local search that consists on doing swaps between tasks in the different machines
pub struct InterMachineSwapAnxious {}

impl LocalSearch for InterMachineSwapAnxious {
    fn perform_search(
        &self,
        instance: &ProblemInstance,
        solution: ProblemSolution,
    ) -> ProblemSolution {
        let solution_tct = solution.get_total_completion_time();
        for from_machine in 0..solution.task_assignment_matrix.len() {
            if solution.task_assignment_matrix[from_machine].len() == 0 {
                continue;
            }
            for task_index in 0..solution.task_assignment_matrix[from_machine].len() {
                for to_machine in 0..solution.task_assignment_matrix.len() {
                    if from_machine == to_machine {
                        continue;
                    }
                    for possible_task_index in 0..solution.task_assignment_matrix[to_machine].len()
                    {
                        let new_solution = InterMachineSwapAnxious::get_solution(
                            instance,
                            &solution,
                            from_machine,
                            task_index,
                            to_machine,
                            possible_task_index,
                        );
                        if new_solution.get_total_completion_time() < solution_tct {
                            return new_solution;
                        }
                    }
                }
            }
        }
        solution
    }
}

impl InterMachineSwapAnxious {
    pub fn new() -> Self {
        InterMachineSwapAnxious {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        from_machine: usize,
        task_index: usize,
        to_machine: usize,
        possible_task_index: usize,
    ) -> ProblemSolution {
        let mut possible_solution = solution.clone();
        let mut to_machine_tasks = possible_solution.task_assignment_matrix[to_machine].clone();
        mem::swap(
            &mut possible_solution.task_assignment_matrix[from_machine][task_index],
            &mut to_machine_tasks[possible_task_index],
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
