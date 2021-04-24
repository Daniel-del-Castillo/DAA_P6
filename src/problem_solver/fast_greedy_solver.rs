use super::{NewTask, ProblemInstance, ProblemSolution, ProblemSolver};
use std::collections::HashSet;

/// A greedy algorithm that creates a solution for the problem by evaluating
/// in each step the best machine to add a determined task to the
/// solution. It always adds the new tasks at the end, unlike [GreedySolver](super::GreedySolver)
pub struct FastGreedySolver {}

impl ProblemSolver for FastGreedySolver {
    fn solve(&mut self, instance: &ProblemInstance) -> ProblemSolution {
        let mut asigned_tasks = HashSet::with_capacity(instance.task_times().len());
        let mut solution = ProblemSolution {
            tcts_by_machine: Vec::with_capacity(instance.number_of_machines()),
            task_assignment_matrix: Vec::with_capacity(instance.number_of_machines()),
        };
        FastGreedySolver::choose_initial_tasks(&mut solution, instance, &mut asigned_tasks);
        while asigned_tasks.len() < instance.task_times().len() {
            FastGreedySolver::add_task(&mut solution, instance, &mut asigned_tasks);
        }
        solution
    }
}

impl FastGreedySolver {
    /// Creates a new solver
    pub fn new() -> Self {
        FastGreedySolver {}
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
        solution: &mut ProblemSolution,
        instance: &ProblemInstance,
        asigned_tasks: &mut HashSet<usize>,
    ) {
        let new_task = FastGreedySolver::get_best_new_task(solution, instance, asigned_tasks);
        solution.task_assignment_matrix[new_task.machine].push(new_task.task);
        asigned_tasks.insert(new_task.task);
        solution.tcts_by_machine[new_task.machine] += new_task.tct_increment;
    }

    fn get_best_new_task(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        asigned_tasks: &HashSet<usize>,
    ) -> NewTask {
        (0..instance.number_of_machines())
            .flat_map(|machine| {
                (0..instance.task_times().len())
                    .filter(|index| !asigned_tasks.contains(index))
                    .map(move |task| {
                        FastGreedySolver::get_new_solution(solution, instance, task, machine)
                    })
            })
            .min_by_key(|new_task| new_task.tct_increment)
            // Panics if all the tasks have been asigned. This function shouldn't be called in such cases
            .unwrap()
    }

    fn get_new_solution(
        solution: &ProblemSolution,
        instance: &ProblemInstance,
        task: usize,
        machine: usize,
    ) -> NewTask {
        let mut task_list = solution.task_assignment_matrix[machine].clone();
        task_list.push(task);
        let tct_increment = instance.calculate_total_completion_time(&task_list)
            - solution.tcts_by_machine[machine];
        NewTask {
            task,
            machine,
            tct_increment,
            position: 0,
        }
    }
}
