use super::{NewTask, ProblemInstance, ProblemSolution, ProblemSolver};
use std::collections::HashSet;

pub struct FastGreedySolver {
    solution: ProblemSolution,
}

impl ProblemSolver for FastGreedySolver {
    fn solve(mut self, instance: &ProblemInstance) -> ProblemSolution {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        self.solution = ProblemSolution {
            tcts_by_machine: Vec::with_capacity(instance.number_of_machines()),
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        self.choose_initial_tasks(instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            self.add_task(instance, &mut asigned_tasks);
        }
        self.solution
    }
}

impl FastGreedySolver {
    pub fn new() -> Self {
        FastGreedySolver {
            solution: ProblemSolution {
                task_assignment_matrix: Vec::new(),
                tcts_by_machine: Vec::new(),
            },
        }
    }

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
            self.solution.task_assignment_matrix.push(vec![task]);
            self.solution.tcts_by_machine.push(tct_increment);
            asigned_tasks.insert(task);
        }
    }

    fn add_task(&mut self, instance: &ProblemInstance, asigned_tasks: &mut HashSet<usize>) {
        let new_task = self.get_best_new_task(instance, asigned_tasks);
        self.solution.task_assignment_matrix[new_task.machine].push(new_task.task);
        asigned_tasks.insert(new_task.task);
        self.solution.tcts_by_machine[new_task.machine] += new_task.tct_increment;
    }

    fn get_best_new_task(
        &self,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> NewTask {
        (0..instance.number_of_machines())
            .map(|machine| self.get_best_new_task_by_machine(instance, machine, &asigned_tasks))
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }

    fn get_best_new_task_by_machine(
        &self,
        instance: &ProblemInstance,
        machine: usize,
        asigned_tasks: &HashSet<usize>,
    ) -> NewTask {
        (0..instance.task_times().len())
            .filter(|index| !asigned_tasks.contains(index))
            .map(|task| {
                let mut task_list = self.solution.task_assignment_matrix[machine].clone();
                task_list.push(task);
                let tct_increment = instance.calculate_total_completion_time(task_list)
                    - self.solution.tcts_by_machine[machine];
                NewTask {
                    task,
                    machine,
                    tct_increment,
                    position: 0,
                }
            })
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }
}
