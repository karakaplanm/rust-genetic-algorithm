use clap::Parser;

mod problem;
use crate::problem::Problem;

mod parameter;
use crate::parameter::Parameters;

mod algorithm;
mod individual;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 10, help = "Dimension")]
    dim: usize,
    #[arg(short, long, default_value_t = 0.001_f64, help = "Tolerance")]
    tol: f64,
    #[arg(short, long, default_value_t = 100, help = "Population Size")]
    pop: usize,
    #[arg(short, long, default_value_t = 200000, help = "Iteration")]
    iter: usize,
    #[arg(short, long, default_value_t = 1, help = "Verbosity")]
    verbosity: usize,
    #[arg(short, long, default_value_t = 0.8, help = "Mutation Rate")]
    mutat: f64,
    #[arg(short, long, default_value_t = 0.8, help = "Crossover Rate")]
    cross: f64,
    #[arg(short, long, default_value_t = 4, help = "Selection")]
    select: usize,
    #[arg(short, long, default_value_t = 2, help = "Elit")]
    elit: usize,
    #[arg(short, long, default_value_t = 1000, help = "Verbosity step")]
    interval: usize,
    #[arg(short, value_enum, default_value_t = Problem::Schwefel, help="Problem Type")]
    func: Problem,
}

/// Setup and run algorithm to search for solution
fn main() {
    let args = Args::parse();

    let parameters = Parameters {
        tolerance: args.tol,
        dimension: args.dim,
        population: args.pop,
        iterations: args.iter,
        elitism: args.elit,
        selection: args.select,
        mutation: args.mutat,
        crossover: args.cross,
        verbosity: args.verbosity,
        interval: args.interval,
    };

    let problem = args.func;
    let result = algorithm::search(problem, parameters);
    println!(
        "{:?} converged to {} after {} generations in {} seconds.",
        result.problem, result.individual.fitness, result.iterations, result.duration
    );
    for (i, res) in result.individual.solution.iter().enumerate() {
        println!("{} {}", i, res);
    }
}
