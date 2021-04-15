mod state;
mod heuristics;
use std::fs;
use std::time::Instant;
use crate::state::city::City as City;
use crate::state::state::State as State;
use crate::heuristics::simulated_annealing as sim_ann;

fn main() {
    let start = Instant::now();
    let initial_state = get_initial_state();
    let iter = 600;
    let temp = 500.0;
    let attemp_level = 10;
    let decrease_temp = 0.80;
    let neighbors_level = 10;
    sim_ann::simulated_annealing(initial_state, iter, temp, attemp_level, decrease_temp, neighbors_level);
    let duration = start.elapsed();
    println!("Tiempo: {:?} segundos", duration);
}

fn get_initial_state() -> State {
    let mut cities : Vec<City> = vec![];
    let content = fs::read_to_string("example.txt").expect("No se encontró el archivo");
    let chunks : Vec<String> = content.split("\n").map(str::to_string).collect();
    let chunks : Vec<String> = chunks.iter().map(|x|x.replace('\r',"")).collect();
    for chunk in chunks {
        if chunk.len() == 0 {
            continue;
        }
        let tokens : Vec<String> = chunk.split_whitespace().map(str::to_string).collect();
        let new_city = City::new(tokens[0].parse::<u32>().unwrap(),tokens[1].parse::<f32>().unwrap(),tokens[2].parse::<f32>().unwrap());
        cities.push(new_city);
    }
    let initial = State::new(std::ptr::null(), cities);
    return initial;
}
