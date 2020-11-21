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
        .position(|l| l == "EDGE_WEIGHT_SECTION" || l == "NODE_COORD_SECTION")
        .unwrap()
        + 1;

    (is_explicit, starting_index)
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

fn find_dimension(lines: &[String]) -> u32 {
    lines
        .iter()
        .find(|&l| l.contains("DIMENSION"))
        .unwrap()
        .split(":")
        .map(|l| l.trim())
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn load_sol_file(file_name: Option<&str>) -> Option<Vec<i32>> {
    let file_name = file_name?;

    let lines: Vec<String> = read_lines(file_name)
        .map(|l| l.expect("could not parse line"))
        .collect();

    let dim = find_dimension(&lines);

    let starting_index = lines.iter().position(|l| l == "TOUR_SECTION").unwrap() + 1;

    let data = lines[starting_index..(starting_index + dim as usize + 1)]
        .iter()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    Some(data)
}

impl FileType {
    pub fn load_problem_file(file_name: &str) -> FileType {
        let lines: Vec<String> = read_lines(file_name)
            .map(|l| l.expect("could not parse line"))
            .collect();

        let dim = find_dimension(&lines);

        let (is_explicit, starting_index) = find_file_type(&lines);

        let file_type = construct_option(is_explicit, lines, starting_index, dim);

        file_type
    }
}
