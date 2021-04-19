use super::StopCriterion;

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
        return self.iterations_without_change >= self.max_iterations_without_change;
    }
}

impl IterationsWithoutChange {
    pub fn new(max_iterations_without_change: usize) -> Self {
        Self {
            iterations_without_change: 0,
            max_iterations_without_change,
        }
    }
}
