/// A struct that represents a solution to the problem. It contains a vector
/// with the total completion time of each machine and a matrix with the tasks
/// assigned to each machine
#[derive(Clone)]
pub struct ProblemSolution {
    pub(super) tcts_by_machine: Vec<usize>,
    pub(super) task_assignment_matrix: Vec<Vec<usize>>,
}

impl ProblemSolution {
    /// Allows getting the total completion time of the solution
    pub fn get_total_completion_time(&self) -> usize {
        self.tcts_by_machine.iter().sum()
    }

    /// Allows getting the total completion time of each machine
    pub fn get_tcts_by_machine(&self) -> &Vec<usize> {
        &self.tcts_by_machine
    }

    /// Allows getting the task assignment matrix
    pub fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>> {
        &self.task_assignment_matrix
    }
}
