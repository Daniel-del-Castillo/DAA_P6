use super::{NewTask, ProblemInstance, ProblemSolution, ProblemSolver};
use std::collections::{BinaryHeap, HashSet};

pub struct RandomizedGreedySolver {
    size_to_choose_from: usize,
    solution: ProblemSolution,
}

impl ProblemSolver for RandomizedGreedySolver {
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

impl RandomizedGreedySolver {
    pub fn new(size_to_choose_from: usize) -> Self {
        RandomizedGreedySolver {
            size_to_choose_from,
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
        let possible_tasks = self.get_best_new_task(instance, asigned_tasks);
        let election = rand::random::<usize>() % possible_tasks.len();
        self.solution.task_assignment_matrix[possible_tasks[election].machine].insert(
            possible_tasks[election].position,
            possible_tasks[election].task,
        );
        asigned_tasks.insert(possible_tasks[election].task);
        self.solution.tcts_by_machine[possible_tasks[election].machine] +=
            possible_tasks[election].tct_increment;
    }

    fn get_best_new_task(
        &self,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> Vec<NewTask> {
        let mut new_tasks = (0..instance.number_of_machines())
            .map(|machine| {
                self.get_best_new_task_by_machine(instance, machine, &asigned_tasks)
                    .into_iter()
            })
            .flatten()
            .collect::<BinaryHeap<NewTask>>();
        (0..self.size_to_choose_from)
            .map(|_| new_tasks.pop())
            .filter(|task| task.is_some())
            .map(|task| task.unwrap())
            .collect()
    }

    fn get_best_new_task_by_machine(
        &self,
        instance: &ProblemInstance,
        machine: usize,
        asigned_tasks: &HashSet<usize>,
    ) -> Vec<NewTask> {
        let mut new_tasks = (0..instance.task_times().len())
            .filter(|index| !asigned_tasks.contains(index))
            .map(|task| {
                self.get_best_new_task_by_task_and_machine(instance, task, machine)
                    .into_iter()
            })
            .flatten()
            .collect::<BinaryHeap<NewTask>>();
        (0..self.size_to_choose_from)
            .map(|_| new_tasks.pop())
            .filter(|task| task.is_some())
            .map(|task| task.unwrap())
            .collect()
    }

    fn get_best_new_task_by_task_and_machine(
        &self,
        instance: &ProblemInstance,
        task: usize,
        machine: usize,
    ) -> Vec<NewTask> {
        let mut new_tasks = (0..=self.solution.task_assignment_matrix[machine].len())
            .map(|position| {
                let mut task_list = self.solution.task_assignment_matrix[machine].clone();
                task_list.insert(position, task);
                let tct_increment = instance.calculate_total_completion_time(&task_list)
                    - self.solution.tcts_by_machine[machine];
                NewTask {
                    task,
                    position,
                    machine,
                    tct_increment,
                }
            })
            .collect::<BinaryHeap<NewTask>>();
        (0..self.size_to_choose_from)
            .map(|_| new_tasks.pop())
            .filter(|task| task.is_some())
            .map(|task| task.unwrap())
            .collect()
    }
}
