use ants_tsp::tsp_solver;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Ants based TSP solver")
        .version("0.2.0")
        .author("Jan Vykruta <vykrutak@gmail.com>")
        .arg(
            Arg::with_name("PROBLEM_FILE")
                .help("File name of problem file. For example: bays29.tsp\nPlease note that the file needs to have following fields: 'DIMENSION', 'EDGE_WEIGHT_TYPE', 'EDGE_WEIGHT_SECTION' or 'NODE_COORD_SECTION' depending on 'EDGE_WEIGHT_TYPE'. 'EDGE_WEIGHT_TYPE' supports 'EXPLICIT' and 'EUC_2D' types. The format can be seen in data/bays29.tsp or data/berlin52.tsp. Please note that threre is no protection against invalid problem specification.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("SOLUTION_FILE")
                .help("Optional file name of optimal solution file. For example: bays29.opt.tour. Please note that the file needs to have following fields: 'DIMENSION', 'TOUR_SECTION'. The format can be seen in data/bays29.opt.tour. Please note that there is no protection against invalid solution specification.")
                .index(2),
        )
        .arg(
            Arg::with_name("pheromone_decay")
                .short("p")
                .default_value("0.1").help("as specified in ACS"),
        )
        .arg(
            Arg::with_name("evaporation_rate")
                .short("e")
                .default_value("0.1").help("as specified in ACS"),
        )
        .arg(Arg::with_name("ant_count").short("a").default_value("10").help("number of ants"))
        .arg(Arg::with_name("alpha").long("alpha").default_value("0.1").help("as specified in ACS"))
        .arg(Arg::with_name("beta").long("beta").default_value("2").help("as specified in ACS"))
        .arg(Arg::with_name("q0").long("q0").default_value("0.9").help("as specified in ACS"))
        .arg(
            Arg::with_name("iterations")
                .long("iter")
                .default_value("1000").help("number of iterations"),
        )
        .get_matches();

    let mut solver = tsp_solver::TspSolver::new(matches);
    solver.solve();
    solver.print_result();
}
