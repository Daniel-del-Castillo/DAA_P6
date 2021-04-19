use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::time::Instant;

use clap::{App, Arg};

mod problem_instance;
use problem_instance::ProblemInstance;
mod problem_solver;
use problem_solver::grasp::stop_criterion::{
    IterationsWithoutChange, StopCriterion, TotalIterations,
};
use problem_solver::{
    grasp::local_search::{
        InterMachineReinsertion, InterMachineReinsertionAnxious, InterMachineSwap,
        InterMachineSwapAnxious, IntraMachineReinsertion, IntraMachineReinsertionAnxious,
        IntraMachineSwap, IntraMachineSwapAnxious, NoSearch,
    },
    FastGreedySolver, GreedySolver, RandomizedGreedySolver,
};
use problem_solver::{ProblemSolver, GRASP};

const FILE_EXPLANATION: &'static str =
    "The file with the problem instance. It should have the following format:
(You should substitute the {} with the correct values and use a tab as separator):
n:\t{number of tasks}
m:\t{number of machines}
{whatever but without have tabs}\t{list of task times separated by tabs}
{a line, you can put here whatever you want}
{list of setup times to go from inactive to each task}
{list of setup times to go from task 1 to each task}
{list of setup times to go from task 2 to each task}
Continues...\n
- The first column and row of the matrix represent the inactive state
- The matrix must be MxM, being M equal to th number of tasks + 1
- The task times list must have an element for each task";

fn main() -> std::io::Result<()> {
    let matches = App::new("parallel-machine-scheduling-problem-with-dependent-setup-times")
        .arg(
            Arg::with_name("problem_file")
                .required(true)
                .help(FILE_EXPLANATION)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_file")
                .required(true)
                .help("The file in which the CSV output will be written")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("without_change")
                .short("w")
                .long("without_change")
                .help("The stop criterion used is iterations without change instead of plain iterations"),
        )
        .arg(
            Arg::with_name("k")
                .required(true)
                .short("k")
                .takes_value(true)
                .help("The number of candidates to choose from in the Randomized greedy algorithm"),
        )
        .get_matches();
    let instance = match ProblemInstance::from_file(&matches.value_of("problem_file").unwrap()) {
        Ok(instance) => instance,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };
    let mut output_file = File::create(&matches.value_of("output_file").unwrap())?;
    let k = matches.value_of("k").unwrap().parse().unwrap();
    print_headers(&mut output_file)?;
    for &iterations in [50, 100, 200, 500, 1000, 2000, 5000, 10000].iter() {
        if matches.is_present("without_change") {
            print_results(
                &instance,
                &mut output_file,
                IterationsWithoutChange::new(iterations),
                k,
                iterations,
            )?;
        } else {
            print_results(
                &instance,
                &mut output_file,
                TotalIterations::new(iterations),
                k,
                iterations,
            )?;
        }
    }
    Ok(())
}

fn print_results<S: StopCriterion>(
    instance: &ProblemInstance,
    output_file: &mut File,
    stop_criterion: S,
    k: usize,
    iterations: usize,
) -> Result<()> {
    write!(output_file, "{},", iterations)?;
    write!(output_file, "Greedy,")?;
    print_result(&instance, output_file, GreedySolver::new())?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "Greedier,")?;
    print_result(&instance, output_file, FastGreedySolver::new())?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "Randomized Greedy,")?;
    print_result(&instance, output_file, RandomizedGreedySolver::new(k))?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP without search,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, NoSearch::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP inter machine reinsertion,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, InterMachineReinsertion::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP intra machine reinsertion,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, IntraMachineReinsertion::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP inter machine swap,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, InterMachineSwap::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP intra machine swap,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, IntraMachineSwap::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP anxious inter machine reinsertion,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(
            k,
            InterMachineReinsertionAnxious::new(),
            stop_criterion.clone(),
        ),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP anxious intra machine reinsertion,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(
            k,
            IntraMachineReinsertionAnxious::new(),
            stop_criterion.clone(),
        ),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP anxious inter machine swap,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, InterMachineSwapAnxious::new(), stop_criterion.clone()),
    )?;
    write!(output_file, "{},", iterations)?;
    write!(output_file, "GRASP anxious intra machine swap,")?;
    print_result(
        &instance,
        output_file,
        GRASP::new(k, IntraMachineSwapAnxious::new(), stop_criterion.clone()),
    )?;
    Ok(())
}

fn print_result<P: ProblemSolver>(
    instance: &ProblemInstance,
    output_file: &mut File,
    solver: P,
) -> Result<()> {
    let instant = Instant::now();
    let tct = solver.solve(&instance).get_total_completion_time();
    let duration = instant.elapsed();
    write!(output_file, "{},{}\n", tct, duration.as_millis())
}

fn print_headers(file: &mut File) -> Result<()> {
    write!(file, "k,")?;
    write!(file, "algorithm,")?;
    write!(file, "tct,")?;
    write!(file, "time\n")
}
