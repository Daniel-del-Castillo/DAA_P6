//! This module defines the class [ProblemInstance](ProblemInstance) which represents
//! an instance of this problem.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod problem_instance_error;
pub use problem_instance_error::{
    ProblemInstanceError,
    ProblemInstanceError::{IOError, SyntaxError},
};

const SEPARATOR: &'static str = "\t";

/// An instance of the problem. It is composed of a matrix of setup times, a vector with the times of
/// each task and the number of machines that will be used. Being N the number of tasks, the vector
/// must have a length of N and the matrix must be N+1xN+1
pub struct ProblemInstance {
    setup_times: Vec<Vec<usize>>,
    task_times: Vec<usize>,
    number_of_machines: usize,
}

impl ProblemInstance {
    /// This function allows to read a Problem instance from a file. The file must have an
    /// specific notation:<br/>
    /// The file with the problem instance. It should have the following format
    /// (You should substitute the {} with the correct values and use a tab as
    /// separator):<br/><br/>
    /// n:  {number of tasks}<br/>
    /// m:  {number of machines}<br/>
    /// {whatever but without have tabs}  {list of task times separated by tabs}<br/>
    /// {a line, you can put here whatever you want}<br/>
    /// {list of setup times to go from inactive to each task}<br/>
    /// {list of setup times to go from task 1 to each task}<br/>
    /// {list of setup times to go from task 2 to each task}<br/>
    /// Continues...<br/>
    /// * The first column and row of the matrix represent the inactive state
    /// * The matrix must be MxM, being M equal to th number of tasks + 1
    /// * The task times list must have an element for each task";
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ProblemInstanceError> {
        let mut file_reader = BufReader::new(File::open(path)?);
        let mut line = String::new();
        file_reader.read_line(&mut line)?;
        let number_of_tasks =
            ProblemInstance::parse_usize_with_prefix(&line, "n:").ok_or_else(|| SyntaxError(1))?;
        line.clear();
        file_reader.read_line(&mut line)?;
        let number_of_machines =
            ProblemInstance::parse_usize_with_prefix(&line, "m:").ok_or_else(|| SyntaxError(2))?;
        line.clear();
        file_reader.read_line(&mut line)?;
        let task_times = ProblemInstance::parse_usize_list(
            &line[line.find(SEPARATOR).ok_or_else(|| SyntaxError(3))? + 1..],
            SEPARATOR,
        )
        .ok_or_else(|| SyntaxError(3))?;
        line.clear();
        file_reader.read_line(&mut line)?; // separator line
        let mut setup_times = Vec::new();
        for i in 0..=number_of_tasks {
            line.clear();
            file_reader.read_line(&mut line)?;
            setup_times.push(match ProblemInstance::parse_usize_list(&line, SEPARATOR) {
                Some(times) if times.len() == number_of_tasks + 1 => times,
                _ => return Err(SyntaxError(i + 5)),
            });
        }
        Ok(ProblemInstance {
            setup_times,
            task_times,
            number_of_machines,
        })
    }

    fn parse_usize_with_prefix(str: &str, prefix: &str) -> Option<usize> {
        str.strip_prefix(prefix)?.trim().parse().ok()
    }

    fn parse_usize_list(str: &str, delimiter: &str) -> Option<Vec<usize>> {
        str.trim_end()
            .split(delimiter)
            .map(|str| str.trim().parse().ok())
            .collect()
    }

    /// Allows to calculate the total completion time (TCT) of certain order of tasks
    /// according to the times in the problem instance. The elements in the vector
    /// must be valid indexes in the task times list.
    pub fn calculate_total_completion_time(&self, task_list: &Vec<usize>) -> usize {
        task_list
            .iter()
            .zip(task_list.iter().skip(1))
            .enumerate()
            .fold(
                task_list.len()
                    * (self.task_times()[task_list[0]] + self.setup_times()[0][task_list[0] + 1]),
                |acc, (index, (&prev, &actual))| {
                    acc + (task_list.len() - index - 1)
                        * (self.task_times()[actual] + self.setup_times()[prev + 1][actual + 1])
                },
            )
    }

    /// Allows to get the number of machines
    pub fn number_of_machines(&self) -> usize {
        self.number_of_machines
    }

    /// Allows to get the vector of task times
    pub fn task_times(&self) -> &Vec<usize> {
        &self.task_times
    }

    /// Allows to get the setup times matrix
    pub fn setup_times(&self) -> &Vec<Vec<usize>> {
        &self.setup_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_usize_with_prefix_pass() {
        assert_eq!(
            ProblemInstance::parse_usize_with_prefix("n:\t45", "n:"),
            Some(45)
        );
    }
    #[test]
    fn parse_usize_with_prefix_fail() {
        assert_eq!(
            ProblemInstance::parse_usize_with_prefix("n:\t45", "m:"),
            None
        );
    }
    #[test]
    fn parse_usize_list_pass() {
        assert_eq!(
            ProblemInstance::parse_usize_list("45\t3\t23\t4\t\r\n", "\t"),
            Some(vec![45, 3, 23, 4])
        );
    }
    #[test]
    fn parse_usize_list_fail() {
        assert_eq!(
            ProblemInstance::parse_usize_list("45\t3\t23\ta", "\t"),
            None
        );
    }

    #[test]
    fn tct() {
        let instance = ProblemInstance {
            number_of_machines: 2,
            task_times: vec![1, 2, 4],
            setup_times: vec![
                vec![0, 0, 2, 3],
                vec![1, 0, 4, 3],
                vec![3, 2, 0, 2],
                vec![1, 0, 2, 0],
            ],
        };
        assert_eq!(instance.calculate_total_completion_time(&vec![0, 1, 2]), 21);
    }
}
