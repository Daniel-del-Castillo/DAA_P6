use super::StopCriterion;

/// A stop criterion that stops whenever a certain number of iterations
/// haven't improved the actual value
#[derive(Clone)]
pub struct IterationsWithoutChange {
    iterations_without_change: usize,
    max_iterations_without_change: usize,
}

impl StopCriterion for IterationsWithoutChange {
    fn stop(&mut self, actual: usize, new: usize) -> bool {
        if new < actual {
            self.iterations_without_change = 0;
        } else {
            self.iterations_without_change += 1;
        }
        if self.iterations_without_change >= self.max_iterations_without_change {
            self.iterations_without_change = 0;
            return true;
        }
        return false;
    }
}

impl IterationsWithoutChange {
    /// Creates a new instance specifying the maximum number of iterations
    /// without changes that are allowed
    pub fn new(max_iterations_without_change: usize) -> Self {
        Self {
            iterations_without_change: 0,
            max_iterations_without_change,
        }
    }
}
