use daap7::{
    problem_solver::{
        grasp::{
            local_search::{
                InterMachineSwap, IntraMachineReinsertion, IntraMachineSwap, LocalSearch,
            },
            stop_criterion::TotalIterations,
        },
        ProblemSolver, GVNS,
    },
    ProblemInstance,
};
use std::fs::File;
use std::io::{Result, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let mut output = File::create("results/mod3.md")?;
    print_headers(&mut output)?;
    for (file, n, m) in vec![
        ("problem_instances/I40j_2m_S1_1.txt", 40, 2),
        ("problem_instances/I40j_4m_S1_1.txt", 40, 4),
        ("problem_instances/I40j_6m_S1_1.txt", 40, 6),
        ("problem_instances/I40j_8m_S1_1.txt", 40, 8),
    ] {
        let instance = match ProblemInstance::from_file(file) {
            Ok(instance) => instance,
            Err(err) => {
                println!("{}: {}", file, err);
                return Ok(());
            }
        };
        write!(output, "|{}|{}|{}|", file, n, m)?;
        print_results(&mut output, &instance)?;
    }
    Ok(())
}

fn print_results(output: &mut File, instance: &ProblemInstance) -> Result<()> {
    let environments: Vec<Box<dyn LocalSearch>> = vec![
        Box::new(IntraMachineSwap::new()),
        Box::new(IntraMachineReinsertion::new()),
        Box::new(InterMachineSwap::new()),
    ];
    let mut solver = GVNS::new(5, TotalIterations::new(500), environments);
    let instant = Instant::now();
    let solution = solver.solve(instance);
    let duration = instant.elapsed();
    write!(
        output,
        "{}|{}|\n",
        solution.get_total_completion_time(),
        duration.as_millis()
    )?;
    Ok(())
}

fn print_headers(output: &mut File) -> Result<()> {
    write!(output, "|Problem|n|m|f|CPU|\n")?;
    write!(output, "|---|---|---|---|---|\n")
}
