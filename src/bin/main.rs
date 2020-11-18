use ants_tsp::tsp_solver;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Ants based TSP solver")
        .version("0.2.0")
        .author("Jan Vykruta <vykrutak@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("File name of input file.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("pheromone_decay")
                .short("p")
                .default_value("0.1"),
        )
        .arg(
            Arg::with_name("evaporation_rate")
                .short("e")
                .default_value("0.1"),
        )
        .arg(Arg::with_name("ant_count").short("a").default_value("100"))
        .arg(Arg::with_name("alpha").long("alpha").default_value("0.1"))
        .arg(Arg::with_name("beta").long("beta").default_value("2"))
        .arg(Arg::with_name("q0").long("q0").default_value("0.9"))
        .arg(
            Arg::with_name("iterations")
                .long("iter")
                .default_value("10000"),
        )
        .get_matches();

    let mut solver = tsp_solver::TspSolver::new(matches);
    solver.solve();
    solver.print_result();
}
