use super::{ProblemInstance, ProblemSolution};
use std::collections::hash_set::HashSet;

pub struct GreedySolution {
    tct: usize,
    task_assignment_matrix: Vec<Vec<usize>>,
}

impl ProblemSolution for GreedySolution {
    fn solve(instance: &ProblemInstance) -> Self {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        let mut solution = GreedySolution {
            tct: 0,
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        solution.choose_initial_tasks(instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            let (machine, task) =
                solution.get_best_next_machine_and_task(instance, &mut asigned_tasks);
            solution.task_assignment_matrix[machine].push(task);
            asigned_tasks.insert(task);
        }
        solution
    }
    fn get_total_completion_time(&self) -> usize {
        self.tct
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
                .min_by_key(|(_, cost)| *cost)
                .map(|(index, _)| index);
            let task = match task {
                Some(val) => val,
                None => break,
            };
            self.task_assignment_matrix.push(vec![task]);
            asigned_tasks.insert(task);
        }
    }

    fn get_best_next_machine_and_task(
        &self,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> (usize, usize) {
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
            .map(|(machine, (task, _))| (machine, task))
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
