use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod problem_instance_error;
pub use problem_instance_error::{
    ProblemInstanceError,
    ProblemInstanceError::{IOError, SyntaxError},
};

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

    pub fn calculate_total_completion_time(&self, task_index_list: Vec<usize>) -> usize {
        task_index_list
            .iter()
            .zip(task_index_list.iter().skip(1))
            .fold(
                self.task_times()[task_index_list[0]]
                    + self.setup_times()[0][task_index_list[0] + 1],
                |acc, (&prev, &actual)| {
                    acc * 2 + self.task_times()[actual] + self.setup_times()[prev + 1][actual + 1]
                },
            )
    }

    pub fn number_of_machines(&self) -> usize {
        self.number_of_machines
    }

    pub fn task_times(&self) -> &Vec<usize> {
        &self.task_times
    }

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
        assert_eq!(instance.calculate_total_completion_time(vec![0, 1, 2]), 22);
    }
}
