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
                .short("pd")
                .default_value("0.5"),
        )
        .arg(
            Arg::with_name("evaporation_rate")
                .short("er")
                .default_value("0.5"),
        )
        .arg(Arg::with_name("ant_count").short("ac").default_value("5"))
        .arg(Arg::with_name("alpha").default_value("0.5"))
        .arg(Arg::with_name("beta").default_value("0.5"))
        .arg(Arg::with_name("q0").default_value("0.5"))
        .arg(Arg::with_name("iterations").default_value("10"))
        .get_matches();

    let mut solver = tsp_solver::TspSolver::new(matches);
    solver.solve();
    solver.print_result();
}
