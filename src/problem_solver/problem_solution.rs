pub struct ProblemSolution {
    pub(super) tcts_by_machine: Vec<usize>,
    pub(super) task_assignment_matrix: Vec<Vec<usize>>,
}

impl ProblemSolution {
    pub fn get_total_completion_time(&self) -> usize {
        self.tcts_by_machine.iter().sum()
    }

    pub fn get_tcts_by_machine(&self) -> &Vec<usize> {
        &self.tcts_by_machine
    }

    pub fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>> {
        &self.task_assignment_matrix
    }
}
