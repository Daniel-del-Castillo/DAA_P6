use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod problem_instance_error;
pub use problem_instance_error::ProblemInstanceError;

const SEPARATOR: &'static str = "\t";

pub struct ProblemInstance {
    setup_times: Vec<Vec<usize>>,
    task_times: Vec<usize>,
    number_of_machines: usize,
}

impl ProblemInstance {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ProblemInstanceError> {
        let mut file_reader = BufReader::new(File::open(path)?);
        let mut line = String::new();
        file_reader.read_line(&mut line)?;
        let number_of_tasks = match ProblemInstance::parse_usize_with_prefix(&line, "n:") {
            Some(s) => s,
            None => return Err(ProblemInstanceError::SyntaxError(1)),
        };
        file_reader.read_line(&mut line)?;
        let number_of_machines = match ProblemInstance::parse_usize_with_prefix(&line, "m:") {
            Some(s) => s,
            None => return Err(ProblemInstanceError::SyntaxError(2)),
        };
        file_reader.read_line(&mut line)?;
        let task_times = match ProblemInstance::parse_usize_list(&line, SEPARATOR) {
            Some(times) if times.len() == number_of_tasks => times,
            _ => return Err(ProblemInstanceError::SyntaxError(3)),
        };
        file_reader.read_line(&mut line)?; // separator line
        let mut setup_times = Vec::new();
        for i in 0..=number_of_tasks {
            file_reader.read_line(&mut line)?;
            setup_times.push(match ProblemInstance::parse_usize_list(&line, SEPARATOR) {
                Some(times) if times.len() == number_of_tasks => times,
                _ => return Err(ProblemInstanceError::SyntaxError(i + 5)),
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
        str.split(delimiter).map(|str| str.parse().ok()).collect()
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
            ProblemInstance::parse_usize_list("45\t3\t23\t4", "\t"),
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
}
