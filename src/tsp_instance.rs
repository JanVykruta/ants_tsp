pub struct TspInstance {
    pub grid: Vec<f32>,
    pub dim: u32,
    pub solution: Option<Vec<u32>>,
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

fn parse_solution(sol: Option<Vec<i32>>) -> Option<Vec<u32>> {
    let sol = sol?;

    let ret = sol.iter().map(|&n| (n.abs() - 1i32) as u32).collect();

    Some(ret)
}

impl TspInstance {
    pub fn new(problem_file: &str, solution_file: Option<&str>) -> TspInstance {
        let problem = super::input_parser::FileType::load_problem_file(problem_file);

        let (grid, dim) = match problem {
            super::input_parser::FileType::Coordinates((coord, dim)) => {
                (parse_coordinates(coord, dim), dim)
            }
            super::input_parser::FileType::Explicit(expl) => expl,
        };

        let solution = parse_solution(super::input_parser::load_sol_file(solution_file));

        TspInstance {
            grid,
            dim,
            solution,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
        self.grid[x * self.dim as usize + y]
    }

    pub fn get_optimal_solution(&self) -> Option<(&[u32], f32)> {
        match &self.solution {
            Some(sol) => Some((sol, self.evaluate_solution(sol))),
            None => None,
        }
    }

    pub fn evaluate_solution(&self, sol: &[u32]) -> f32 {
        let mut state = sol[0];
        let mut dist = 0f32;

        for &next_state in &sol[1..] {
            dist += self.at(state as usize, next_state as usize);
            state = next_state;
        }

        dist
    }
}
