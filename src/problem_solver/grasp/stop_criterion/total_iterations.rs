use super::StopCriterion;

/// A stop criterion that stops after a fixed number of iterations
#[derive(Clone)]
pub struct TotalIterations {
    total_iterations: usize,
    max_iterations: usize,
}

impl StopCriterion for TotalIterations {
    fn stop(&mut self, _actual: usize, _new: usize) -> bool {
        self.total_iterations += 1;
        if self.total_iterations >= self.max_iterations {
            self.total_iterations = 0;
            return true;
        }
        return false;
    }
}

impl TotalIterations {
    /// Creates a new instance with the specified number of iterations
    pub fn new(max_iterations: usize) -> Self {
        Self {
            total_iterations: 0,
            max_iterations,
        }
    }
}
