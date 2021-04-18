use super::StopCriterion;

pub struct TotalIterations {
    total_iterations: usize,
    max_iterations: usize,
}

impl StopCriterion for TotalIterations {
    fn stop(&mut self, _actual: usize, _new: usize) -> bool {
        self.total_iterations += 1;
        return self.total_iterations >= self.max_iterations;
    }
}

impl TotalIterations {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            total_iterations: 0,
            max_iterations,
        }
    }
}
