use super::{ProblemInstance, ProblemSolution};
use std::collections::hash_set::HashSet;

pub struct GreedySolution {
    tcts_by_machine: Vec<usize>,
    task_assignment_matrix: Vec<Vec<usize>>,
}

impl ProblemSolution for GreedySolution {
    fn solve(instance: &ProblemInstance) -> Self {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        let mut solution = GreedySolution {
            tcts_by_machine: Vec::with_capacity(instance.number_of_machines()),
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        solution.choose_initial_tasks(instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            solution.add_task(instance, &mut asigned_tasks);
        }
        solution
    }

    fn get_total_completion_time(&self) -> usize {
        self.tcts_by_machine.iter().sum()
    }

    fn get_tcts_by_machine(&self) -> &Vec<usize> {
        &self.tcts_by_machine
    }

    fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>> {
        &self.task_assignment_matrix
    }
}

impl GreedySolution {
    fn choose_initial_tasks(
        &mut self,
        instance: &ProblemInstance,
        asigned_tasks: &mut HashSet<usize>,
    ) {
        for _ in 0..instance.number_of_machines() {
            let task = instance.setup_times()[0]
                .iter()
                .skip(1) // Skip the starting setup times column
                .enumerate()
                .filter(|(index, _)| !asigned_tasks.contains(index))
                .map(|(index, setup)| (index, setup + instance.task_times()[index]))
                .min_by_key(|(_, cost)| *cost);
            let (task, tct_increment) = match task {
                Some(values) => values,
                None => break,
            };
            self.task_assignment_matrix.push(vec![task]);
            self.tcts_by_machine.push(tct_increment);
            asigned_tasks.insert(task);
        }
    }

    fn add_task(&mut self, instance: &ProblemInstance, asigned_tasks: &mut HashSet<usize>) {
        let (machine, task, tct_increment) =
            self.get_best_next_machine_and_task(instance, asigned_tasks);
        self.task_assignment_matrix[machine].push(task);
        asigned_tasks.insert(task);
        self.tcts_by_machine[machine] += tct_increment;
    }

    fn get_best_next_machine_and_task(
        &self,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> (usize, usize, usize) {
        self.task_assignment_matrix
            .iter()
            .enumerate()
            .map(|(index, machine_tasks)| {
                (
                    index,
                    GreedySolution::get_best_next_task(instance, machine_tasks, &asigned_tasks),
                )
            })
            .min_by_key(|(_, (_, cost))| *cost)
            .map(|(machine, (task, tct_increment))| (machine, task, tct_increment))
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }

    fn get_best_next_task(
        instance: &ProblemInstance,
        task_list: &Vec<usize>,
        asigned_tasks: &HashSet<usize>,
    ) -> (usize, usize) {
        let base_tct_increment =
            GreedySolution::calculate_total_completion_time(instance, &task_list);
        instance
            .task_times()
            .iter()
            .enumerate()
            .filter(|(index, _)| !asigned_tasks.contains(index))
            .map(|(task_index, task_time)| {
                (
                    task_index,
                    base_tct_increment
                        + task_time
                        + instance.setup_times()[task_list.last().unwrap() + 1][task_index + 1],
                )
            })
            .min_by_key(|(_, cost)| *cost)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }
}
