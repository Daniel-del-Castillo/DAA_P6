mod iterations_without_change;
mod total_iterations;
pub use iterations_without_change::IterationsWithoutChange;
pub use total_iterations::TotalIterations;

pub trait StopCriterion {
    fn stop(&mut self, actual: usize, new: usize) -> bool;
}
