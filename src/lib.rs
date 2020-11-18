mod input_parser {
    use std::fs::File;
    use std::io::{self, BufRead};

    pub enum FileType {
        Explicit((Vec<f32>, u32)),
        Coordinates((Vec<(f32, f32)>, u32)),
    }

    fn read_lines(path: &str) -> io::Lines<io::BufReader<File>> {
        let file = File::open(path).expect(&format!("file '{}' could not be opened:", path));
        io::BufReader::new(file).lines()
    }

    fn find_file_type(lines: &[String]) -> (bool, usize) {
        let is_explicit = lines
            .iter()
            .any(|l| l.starts_with("EDGE_WEIGHT_TYPE") && l.contains("EXPLICIT"));

        let starting_index = lines
            .iter()
            .zip(0u32..)
            .find(|l| l.0 == "EDGE_WEIGHT_SECTION" || l.0 == "NODE_COORD_SECTION")
            .unwrap()
            .1
            + 1;

        (is_explicit, starting_index as usize)
    }

    fn construct_option(
        is_explicit: bool,
        lines: Vec<String>,
        starting_index: usize,
        dim: u32,
    ) -> FileType {
        let lines = &lines[starting_index..(starting_index + dim as usize)];

        if is_explicit {
            let data: Vec<f32> = lines
                .iter()
                .map(|l| l.trim().split_whitespace())
                .flatten()
                .map(|i| i.trim().parse::<f32>().unwrap())
                .collect();
            FileType::Explicit((data, dim))
        } else {
            let data: Vec<(f32, f32)> = lines
                .iter()
                .map(|l| l.trim().split_whitespace().skip(1))
                .map(|mut s| {
                    (
                        s.next().unwrap().parse::<f32>().unwrap(),
                        s.next().unwrap().parse::<f32>().unwrap(),
                    )
                })
                .collect();
            FileType::Coordinates((data, dim))
        }
    }

    impl FileType {
        pub fn load_file(file_name: &str) -> FileType {
            let lines: Vec<String> = read_lines(file_name)
                .map(|l| l.expect("could not parse line"))
                .collect();

            let dim = lines
                .iter()
                .find(|l| l.contains("DIMENSION"))
                .unwrap()
                .split(":")
                .map(|l| l.trim())
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let (is_explicit, starting_index) = find_file_type(&lines[..]);

            let file_type = construct_option(is_explicit, lines, starting_index, dim);

            file_type
        }
    }
}

mod tsp_instance {
    pub struct TspInstance {
        pub grid: Vec<f32>,
        pub dim: u32,
    }

    fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
        let x = (p2.0 - p1.0) * (p2.0 - p1.0) + (p2.1 - p1.1) * (p2.1 - p1.1);

        x.sqrt()
    }

    fn parse_coordinates(coord: Vec<(f32, f32)>, dim: u32) -> Vec<f32> {
        let mut grid: Vec<f32> = vec![];

        grid.resize(dim as usize * dim as usize, 0f32);

        for i in 0..dim {
            for j in 0..dim {
                grid[(i * dim + j) as usize] = distance(coord[i as usize], coord[j as usize]);
            }
        }

        grid
    }

    impl TspInstance {
        pub fn new(file: super::input_parser::FileType) -> TspInstance {
            let (grid, dim) = match file {
                super::input_parser::FileType::Coordinates((coord, dim)) => {
                    (parse_coordinates(coord, dim), dim)
                }
                super::input_parser::FileType::Explicit(expl) => expl,
            };

            TspInstance { grid, dim }
        }

        pub fn at(&self, x: usize, y: usize) -> f32 {
            self.grid[x * self.dim as usize + y]
        }
    }
}

pub mod tsp_solver {
    use rand::Rng;
    use std::cmp::{max, min};

    struct Config {
        ant_count: u32,
        pheromone_decay: f32,
        evaporation_rate: f32,
        alpha: f32,
        beta: f32,
        q0: f32,
        iterations: u32,
        tau0: f32,
    }

    struct Ant {
        visited: Vec<bool>,
        solution: Vec<u32>,
        is_done: bool,
        current_state: u32,
        orig_state: u32,
    }

    fn eq_4(
        current_state: u32,
        possible_moves: &[u32],
        world: &super::tsp_instance::TspInstance,
        pheromones: &[f32],
        config: &Config,
    ) -> u32 {
        let sum = possible_moves.iter().fold(0f32, |a, v| {
            let smaller = min(current_state, *v);
            let bigger = max(current_state, *v);
            a + (pheromones[(smaller * world.dim + bigger) as usize].powf(config.alpha)
                * (1f32 / world.at(smaller as usize, bigger as usize)).powf(config.beta))
        });

        let move_prob = possible_moves.iter().map(|m| {
            let smaller = min(current_state, *m);
            let bigger = max(current_state, *m);

            let nom = pheromones[(smaller * world.dim + bigger) as usize].powf(config.alpha)
                * (1f32 / world.at(smaller as usize, bigger as usize)).powf(config.beta);

            nom / sum
        });

        let random = rand::thread_rng().gen::<f32>();
        let mut prob_sum = 0f32;

        for (mov, prob) in possible_moves.iter().zip(move_prob) {
            prob_sum += prob;
            if prob_sum > random {
                return *mov;
            }
        }

        panic!(format!(
            "impossible to reach -> random: {}; prob_sum: {}\n",
            random, prob_sum
        ));
    }

    fn argmax(
        current_state: u32,
        possible_moves: &[u32],
        world: &super::tsp_instance::TspInstance,
        pheromones: &[f32],
        config: &Config,
    ) -> u32 {
        let mut best_move = possible_moves[0];
        let smaller = min(current_state, best_move);
        let bigger = max(current_state, best_move);
        let mut best_val = pheromones[(smaller * world.dim + bigger) as usize]
            * (1f32 / world.at(smaller as usize, bigger as usize)).powf(config.beta);

        for mov in &possible_moves[1..] {
            let smaller = min(current_state, *mov);
            let bigger = max(current_state, *mov);
            let val = pheromones[(smaller * world.dim + bigger) as usize]
                * (1f32 / world.at(smaller as usize, bigger as usize)).powf(config.beta);
            if val > best_val {
                best_val = val;
                best_move = *mov;
            }
        }

        best_move
    }

    impl Ant {
        fn new(world: &super::tsp_instance::TspInstance) -> Ant {
            let start_state = rand::thread_rng().gen_range(0, world.dim);
            let solution: Vec<u32> = vec![start_state];
            let mut visited = vec![false; world.dim as usize];
            visited[start_state as usize] = true;

            Ant {
                visited,
                solution,
                is_done: false,
                current_state: start_state,
                orig_state: start_state,
            }
        }
        fn make_move(
            &mut self,
            world: &super::tsp_instance::TspInstance,
            pheromones: &mut [f32],
            config: &Config,
        ) {
            if self.is_done {
                return;
            }

            let possible_moves: Vec<u32> = self
                .visited
                .iter()
                .zip(0u32..)
                .filter(|v| !v.0)
                .map(|v| v.1)
                .collect();

            if possible_moves.is_empty() {
                self.is_done = true;
                self.solution.push(self.orig_state);
                return;
            }

            let next_move = if rand::thread_rng().gen::<f32>() < config.q0 {
                argmax(
                    self.current_state,
                    &possible_moves,
                    world,
                    pheromones,
                    config,
                )
            } else {
                eq_4(
                    self.current_state,
                    &possible_moves,
                    world,
                    pheromones,
                    config,
                )
            };

            self.solution.push(next_move);
            self.visited[next_move as usize] = true;

            let smaller = min(self.current_state, next_move);
            let bigger = max(self.current_state, next_move);
            let old_val = pheromones[(smaller * world.dim + bigger) as usize];
            pheromones[(smaller * world.dim + bigger) as usize] =
                (1f32 - config.pheromone_decay) * old_val + config.pheromone_decay * config.tau0;

            self.current_state = next_move;
        }

        pub fn get_solution(&self, world: &super::tsp_instance::TspInstance) -> f32 {
            let mut state = self.solution[0];
            let mut dist = 0f32;

            for next_state in &self.solution[1..] {
                dist += world.at(state as usize, *next_state as usize);
                state = *next_state;
            }

            dist
        }

        fn update_pheromones(
            &self,
            world: &super::tsp_instance::TspInstance,
            pheromones: &mut [f32],
            config: &Config,
        ) {
            let mut state = self.solution[0];
            let sol_cost = self.get_solution(world);

            for next_state in &self.solution[1..] {
                let smaller = min(state, *next_state);
                let bigger = max(state, *next_state);

                let old_val = pheromones[(smaller * world.dim + bigger) as usize];
                pheromones[(smaller * world.dim + bigger) as usize] =
                    (1f32 - config.evaporation_rate) * old_val
                        + config.evaporation_rate * (1f32 / sol_cost);

                state = *next_state;
            }
        }
    }
    pub struct TspSolver {
        problem_instance: super::tsp_instance::TspInstance,
        pheromone_dist: Vec<f32>,
        config: Config,
        best_sol_cost: f32,
        best_sol: Vec<u32>,
    }

    fn approximate_sol(world: &super::tsp_instance::TspInstance) -> f32 {
        let mut state = 0u32;
        let mut dist = 0f32;

        let mut visited = vec![false; world.dim as usize];
        visited[0] = true;

        for _ in 0..(world.dim - 1) {
            let next_move = visited
                .iter()
                .zip(0u32..)
                .filter(|m| !m.0)
                .map(|m| m.1)
                .min_by(|x, y| {
                    world
                        .at(state as usize, *x as usize)
                        .partial_cmp(&world.at(state as usize, *y as usize))
                        .unwrap()
                })
                .unwrap();

            dist += world.at(state as usize, next_move as usize);
            visited[next_move as usize] = true;
            state = next_move;
        }

        dist += world.at(state as usize, 0);

        dist
    }

    impl TspSolver {
        pub fn new(args: clap::ArgMatches) -> TspSolver {
            let file = super::input_parser::FileType::load_file(args.value_of("INPUT").unwrap());
            let problem_instance = super::tsp_instance::TspInstance::new(file);
            let pheromone_dist: Vec<f32> = vec![
                1f32 / (problem_instance.dim as f32);
                problem_instance.dim as usize
                    * problem_instance.dim as usize
            ];

            let tau0 = approximate_sol(&problem_instance);
            println!("approximate solution: {}", tau0);

            TspSolver {
                problem_instance,
                pheromone_dist,
                best_sol: vec![],
                best_sol_cost: f32::MAX,
                config: Config {
                    ant_count: args.value_of("ant_count").unwrap().parse().unwrap(),
                    pheromone_decay: args.value_of("pheromone_decay").unwrap().parse().unwrap(),
                    evaporation_rate: args.value_of("evaporation_rate").unwrap().parse().unwrap(),
                    alpha: args.value_of("alpha").unwrap().parse().unwrap(),
                    beta: args.value_of("beta").unwrap().parse().unwrap(),
                    q0: args.value_of("q0").unwrap().parse().unwrap(),
                    iterations: args.value_of("iterations").unwrap().parse().unwrap(),
                    tau0,
                },
            }
        }

        pub fn solve(&mut self) {
            for iteration in 0..self.config.iterations {
                let mut ants: Vec<Ant> = (0..self.config.ant_count)
                    .map(|_| Ant::new(&self.problem_instance))
                    .collect();

                while ants.iter().any(|a| !a.is_done) {
                    ants.iter_mut().for_each(|a| {
                        a.make_move(
                            &self.problem_instance,
                            &mut self.pheromone_dist,
                            &self.config,
                        )
                    });
                }

                let best_ant = ants
                    .iter()
                    .min_by(|l, r| {
                        l.get_solution(&self.problem_instance)
                            .partial_cmp(&r.get_solution(&self.problem_instance))
                            .unwrap()
                    })
                    .unwrap();

                let best_ant_sol = best_ant.get_solution(&self.problem_instance);
                println!("iteration {}: {}", iteration, best_ant_sol);

                best_ant.update_pheromones(
                    &self.problem_instance,
                    &mut self.pheromone_dist,
                    &self.config,
                );

                if self.best_sol_cost > best_ant_sol {
                    self.best_sol_cost = best_ant_sol;
                    self.best_sol = best_ant.solution.clone();
                }
            }
        }

        pub fn print_result(&self) {
            println!("----------- best solution -----------");
            println!("approximate solution cost: {}", self.config.tau0);
            println!("cost: {}", self.best_sol_cost);
            println!("solution: {:?}", self.best_sol);
        }
    }
}
