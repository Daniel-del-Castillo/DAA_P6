use super::{NewTask, ProblemInstance, ProblemSolution, ProblemSolver};
use std::collections::{BinaryHeap, HashSet};

/// A greedy algorithm that creates a solution for the problem by evaluating
/// in each step the best position and machine to add a determined task to the
/// solution and taking the k bests possible insertions. It then chooses randomly between those
pub struct RandomizedGreedySolver {
    size_to_choose_from: usize,
}

impl ProblemSolver for RandomizedGreedySolver {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        let mut solution = ProblemSolution {
            tcts_by_machine: Vec::with_capacity(instance.number_of_machines()),
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        RandomizedGreedySolver::choose_initial_tasks(&mut solution, instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            self.add_task(&mut solution, instance, &mut asigned_tasks);
        }
        solution
    }
}

impl RandomizedGreedySolver {
    /// Creates a new solver that will use the argument passed as k
    pub fn new(size_to_choose_from: usize) -> Self {
        RandomizedGreedySolver {
            size_to_choose_from,
        }
    }

    fn choose_initial_tasks(
        solution: &mut ProblemSolution,
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
            solution.task_assignment_matrix.push(vec![task]);
            solution.tcts_by_machine.push(tct_increment);
            asigned_tasks.insert(task);
        }
    }

    fn add_task(
        &self,
        solution: &mut ProblemSolution,
        instance: &ProblemInstance,
        asigned_tasks: &mut HashSet<usize>,
    ) {
        let possible_tasks = self.get_best_new_tasks(solution, instance, asigned_tasks);
        let election = rand::random::<usize>() % possible_tasks.len();
        solution.task_assignment_matrix[possible_tasks[election].machine].insert(
            possible_tasks[election].position,
            possible_tasks[election].task,
        );
        asigned_tasks.insert(possible_tasks[election].task);
        solution.tcts_by_machine[possible_tasks[election].machine] +=
            possible_tasks[election].tct_increment;
    }

    fn get_best_new_tasks(
        &self,
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> Vec<NewTask> {
        let mut new_tasks = (0..instance.number_of_machines())
            .flat_map(move |machine| {
                (0..instance.task_times().len())
                    .filter(move |index| !asigned_tasks.contains(index))
                    .flat_map(move |task| {
                        (0..=solution.task_assignment_matrix[machine].len()).map(move |position| {
                            RandomizedGreedySolver::get_new_solution(
                                solution, instance, task, machine, position,
                            )
                        })
                    })
            })
            .collect::<BinaryHeap<NewTask>>();
        (0..self.size_to_choose_from)
            .map(|_| new_tasks.pop())
            .filter(|task| task.is_some())
            .map(|task| task.unwrap())
            .collect()
    }

    fn get_new_solution(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        task: usize,
        machine: usize,
        position: usize,
    ) -> NewTask {
        let mut task_list = solution.task_assignment_matrix[machine].clone();
        task_list.insert(position, task);
        let tct_increment = instance.calculate_total_completion_time(&task_list)
            - solution.tcts_by_machine[machine];
        NewTask {
            task,
            position,
            machine,
            tct_increment,
        }
    }
}
