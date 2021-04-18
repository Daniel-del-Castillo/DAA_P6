use std::env;

mod problem_instance;
use problem_instance::ProblemInstance;
mod problem_solver;
use problem_solver::grasp::local_search::{
    InterMachineReinsertion, InterMachineSwap, IntraMachineReinsertion, IntraMachineSwap, NoSearch,
};
use problem_solver::grasp::stop_criterion::{IterationsWithoutChange, TotalIterations};
use problem_solver::{
    FastGreedySolver, GreedySolver, ProblemSolution, ProblemSolver, RandomizedGreedySolver, GRASP,
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
    let solver = RandomizedGreedySolver::new(3);
    let solution = solver.solve(&instance);
    println!("\nSolution with a randomized greedy algorithm:");
    print_solution(solution);
    let solver = GRASP::new(3, NoSearch::new(), TotalIterations::new(1000));
    let solution = solver.solve(&instance);
    println!("\nSolution with a (constructive only) GRASP algorithm with 1000 iterations:");
    print_solution(solution);
    let solver = GRASP::new(
        3,
        IntraMachineReinsertion::new(),
        TotalIterations::new(1000),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with intra machine reinsertion search with 1000 iterations:");
    print_solution(solution);
    let solver = GRASP::new(
        3,
        InterMachineReinsertion::new(),
        TotalIterations::new(1000),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with inter machine reinsertion search with 1000 iterations:");
    print_solution(solution);
    let solver = GRASP::new(3, IntraMachineSwap::new(), TotalIterations::new(1000));
    let solution = solver.solve(&instance);
    println!(
        "\nSolution with a GRASP algorithm with intra machine swap search with 1000 iterations:"
    );
    print_solution(solution);
    let solver = GRASP::new(3, InterMachineSwap::new(), TotalIterations::new(1000));
    let solution = solver.solve(&instance);
    println!(
        "\nSolution with a GRASP algorithm with inter machine swap search with 1000 iterations:"
    );
    print_solution(solution);

    let solver = GRASP::new(3, NoSearch::new(), TotalIterations::new(1000));
    let solution = solver.solve(&instance);
    println!(
        "\nSolution with a (constructive only) GRASP algorithm with 100 iterations without change:"
    );
    print_solution(solution);
    let solver = GRASP::new(
        3,
        IntraMachineReinsertion::new(),
        IterationsWithoutChange::new(100),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with intra machine reinsertion search with 100 iterations without change:");
    print_solution(solution);
    let solver = GRASP::new(
        3,
        InterMachineReinsertion::new(),
        IterationsWithoutChange::new(100),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with inter machine reinsertion search with 100 iterations without change:");
    print_solution(solution);
    let solver = GRASP::new(
        3,
        IntraMachineSwap::new(),
        IterationsWithoutChange::new(100),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with intra machine swap search with 100 iterations without change:");
    print_solution(solution);
    let solver = GRASP::new(
        3,
        InterMachineSwap::new(),
        IterationsWithoutChange::new(100),
    );
    let solution = solver.solve(&instance);
    println!("\nSolution with a GRASP algorithm with inter machine swap search with 100 iterations without change:");
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
