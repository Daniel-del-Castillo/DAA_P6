use std::env;

mod problem_instance;
use problem_instance::ProblemInstance;
mod problem_solver;
use problem_solver::{
    FastGreedySolver, GreedySolver, ProblemSolution, ProblemSolver, RandomizedGreedySolver,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of arguments. Use -h or --h to get help");
        return;
    }
    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }
    let instance = match ProblemInstance::from_file(&args[1]) {
        Ok(instance) => instance,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let solver = GreedySolver::new();
    let solution = solver.solve(&instance);
    println!("\nSolution with a greedy algorithm:");
    print_solution(solution);
    let solver = FastGreedySolver::new();
    let solution = solver.solve(&instance);
    println!("\nSolution with a greedier algorithm:");
    print_solution(solution);
    let solver = RandomizedGreedySolver::new(2);
    let solution = solver.solve(&instance);
    println!("\nSolution with a randomized greedy algorithm:");
    print_solution(solution);
}

fn print_solution(solution: ProblemSolution) {
    println!(
        "The solution found has a total completion time of {}",
        solution.get_total_completion_time()
    );
    let tcts_by_machine = solution.get_tcts_by_machine();
    for (i, task_list) in solution.get_tasks_by_machine().iter().enumerate() {
        println!(
            "The machine {} spends {} units of time executing the following tasks and in the following order: ", 
            i + 1,
            tcts_by_machine[i]
        );
        println!(
            "{{ {} }}",
            task_list
                .iter()
                .map(|num| (num + 1).to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn print_help() {
    println!("You should call this program providing as argument the path to a problem instance");
    print!("The file should have the following format");
    println!("you should substitute the {{}} with the correct values and use a tab as separator):");
    println!("n:\t{{number of tasks}}");
    println!("m:\t{{number of machines}}");
    println!("{{whatever but without have tabs}}\t{{list of task times separated by tabs}}");
    println!("{{a line, you can put here whatever you want}}");
    println!("{{list of setup times to go from inactive to each task}}");
    println!("{{list of setup times to go from task 1 to each task}}");
    println!("{{list of setup times to go from task 2 to each task}}");
    println!("Continues...\n");
    println!("- The first column and row of the matrix represent the inactive state");
    println!("- The matrix must be MxM, being M equal to th number of tasks + 1");
    println!("- The task times list must have an element for each task");
}
