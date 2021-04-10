use super::ProblemInstance;

mod greedy_solution;
pub use greedy_solution::GreedySolution;

pub trait ProblemSolution {
    fn solve(instance: &ProblemInstance) -> Self;

    fn get_total_completion_time(&self) -> usize;

    fn get_tcts_by_machine(&self) -> &Vec<usize>;

    fn get_tasks_by_machine(&self) -> &Vec<Vec<usize>>;

    fn calculate_total_completion_time(
        instance: &ProblemInstance,
        task_index_list: &Vec<usize>,
    ) -> usize {
        task_index_list
            .iter()
            .zip(task_index_list.iter().skip(1))
            .fold(
                instance.task_times()[task_index_list[0]]
                    + instance.setup_times()[0][task_index_list[0] + 1],
                |acc, (&prev, &actual)| {
                    acc + instance.task_times()[actual]
                        + instance.setup_times()[prev + 1][actual + 1]
                },
            )
    }
}
