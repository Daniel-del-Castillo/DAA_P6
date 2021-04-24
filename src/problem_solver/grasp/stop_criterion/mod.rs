mod iterations_without_change;
mod total_iterations;
pub use iterations_without_change::IterationsWithoutChange;
pub use total_iterations::TotalIterations;

/// A trait that specifies how a stop criterion should behave.
pub trait StopCriterion: Clone {
    /// The call to stop receives two costs, the actual and the new and must
    /// return wheter it is time to stop or not. It also must reset
    /// itself when it reaches its criterion so it can be used several
    /// times in a row
    fn stop(&mut self, actual: usize, new: usize) -> bool;
}
