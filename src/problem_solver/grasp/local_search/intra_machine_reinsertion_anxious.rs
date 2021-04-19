use super::*;

pub struct IntraMachineReinsertionAnxious {}

impl LocalSearch for IntraMachineReinsertionAnxious {
    fn perform_search(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
    ) -> Option<ProblemSolution> {
        let solution_tct = solution.get_total_completion_time();
        for machine in 0..solution.task_assignment_matrix.len() {
            if solution.task_assignment_matrix[machine].len() < 2 {
                continue;
            }
            for task_index in 0..solution.task_assignment_matrix[machine].len() {
                for possible_task_index in 0..=solution.task_assignment_matrix[machine].len() {
                    if task_index == possible_task_index {
                        continue;
                    }
                    let new_solution = IntraMachineReinsertionAnxious::get_solution(
                        instance,
                        solution,
                        machine,
                        task_index,
                        possible_task_index,
                    );
                    if new_solution.get_total_completion_time() < solution_tct {
                        return Some(new_solution);
                    }
                }
            }
        }
        None
    }
}

impl IntraMachineReinsertionAnxious {
    pub fn new() -> Self {
        IntraMachineReinsertionAnxious {}
    }

    fn get_solution(
        instance: &ProblemInstance,
        solution: &ProblemSolution,
        machine: usize,
        task_index: usize,
        mut possible_task_index: usize,
    ) -> ProblemSolution {
        if possible_task_index > task_index {
            possible_task_index -= 1;
        }
        let mut possible_solution = solution.clone();
        let task = possible_solution.task_assignment_matrix[machine].remove(task_index);
        possible_solution.task_assignment_matrix[machine].insert(possible_task_index, task);
        possible_solution.tcts_by_machine[machine] = instance
            .calculate_total_completion_time(&possible_solution.task_assignment_matrix[machine]);
        possible_solution
    }
}
