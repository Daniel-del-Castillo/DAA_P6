//! This project was made by Daniel del Castillo de la Rosa for the Algorithm Design
//! and Analysis class (DAA in spanish) at the La Laguna University.
//!
//! This code can be used to solve a parallel machine scheduling problem with dependent
//! setup times. For that it uses different algorithms and Metaheuristics like VNS or GRASP

mod problem_instance;
pub use problem_instance::{ProblemInstance, ProblemInstanceError};
pub mod problem_solver;
