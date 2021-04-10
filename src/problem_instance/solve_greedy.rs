use super::ProblemInstance;
use std::{collections::hash_set::HashSet, usize};

impl ProblemInstance {
    pub fn solve_greedy(&self) -> Vec<Vec<usize>> {
        let mut asigned_tasks = HashSet::with_capacity(self.task_times.len());
        let mut solution = self.choose_initial_tasks(&mut asigned_tasks);
        while asigned_tasks.len() < self.task_times.len() {
            let (machine, task) =
                self.get_best_next_machine_and_task(&solution, &mut asigned_tasks);
            solution[machine].push(task);
            asigned_tasks.insert(task);
        }
        solution
    }

    fn choose_initial_tasks(&self, asigned_tasks: &mut HashSet<usize>) -> Vec<Vec<usize>> {
        let mut solution = Vec::with_capacity(self.number_of_machines);
        for _ in 0..self.number_of_machines {
            let task = self.setup_times[0]
                .iter()
                .skip(1) // Skip the starting setup times column
                .enumerate()
                .filter(|(index, _)| !asigned_tasks.contains(index))
                .map(|(index, setup)| (index, setup + self.task_times[index]))
                .min_by_key(|(_, cost)| *cost)
                .map(|(index, _)| index);
            let task = match task {
                Some(val) => val,
                None => break,
            };
            solution.push(vec![task]);
            asigned_tasks.insert(task);
        }
        solution
    }

    fn get_best_next_machine_and_task(
        &self,
        solution: &Vec<Vec<usize>>,
        asigned_tasks: &HashSet<usize>,
    ) -> (usize, usize) {
        solution
            .iter()
            .enumerate()
            .map(|(index, machine_tasks)| {
                (
                    index,
                    self.get_best_next_task(machine_tasks, &asigned_tasks),
                )
            })
            .min_by_key(|(_, (_, cost))| *cost)
            .map(|(machine, (task, _))| (machine, task))
            // Panics if all the tasks have been asigned. This function shouldn't be called in such a case
            .unwrap()
    }

    fn get_best_next_task(
        &self,
        task_list: &Vec<usize>,
        asigned_tasks: &HashSet<usize>,
    ) -> (usize, usize) {
        let base_tct_increment = self.calculate_total_completion_time(&task_list);
        self.task_times
            .iter()
            .enumerate()
            .filter(|(index, _)| !asigned_tasks.contains(index))
            .map(|(task_index, task_time)| {
                (
                    task_index,
                    base_tct_increment
                        + task_time
                        + self.setup_times[task_list.last().unwrap() + 1][task_index + 1],
                )
            })
            .min_by_key(|(_, cost)| *cost)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such a case
            .unwrap()
    }

    fn calculate_total_completion_time(&self, task_index_list: &Vec<usize>) -> usize {
        task_index_list
            .iter()
            .zip(task_index_list.iter().skip(1))
            .fold(
                self.task_times[task_index_list[0]] + self.setup_times[0][task_index_list[0] + 1],
                |acc, (&prev, &actual)| {
                    acc + self.task_times[actual] + self.setup_times[prev + 1][actual + 1]
                },
            )
    }
}
