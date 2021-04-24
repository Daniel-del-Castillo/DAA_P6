use super::*;

/// An anxious local search that consists on reinserting a task in a different machine
pub struct InterMachineReinsertionAnxious {}

impl LocalSearch for InterMachineReinsertionAnxious {
    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
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
                    for possible_insertion_index in
                        0..=solution.task_assignment_matrix[to_machine].len()
                    {
                        let new_solution = InterMachineReinsertionAnxious::get_solution(
                            instance,
                            solution,
                            from_machine,
                            task_index,
                            to_machine,
                            possible_insertion_index,
                        );
                        if new_solution.get_total_completion_time() < solution_tct {
                            return Some(new_solution);
                        }
                    }
                }
            }
        }
        None
    }
}

impl InterMachineReinsertionAnxious {
    pub fn new() -> Self {
        InterMachineReinsertionAnxious {}
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
