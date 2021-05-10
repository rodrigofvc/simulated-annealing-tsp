use crate::state::state::State as State;
use rand::Rng;


/**
* Simulated Annealing metaheuristic.
* initial_state: the initial state.
* iter: numer of iterations.
* temp: initial temperature.
* attemp_level: iterations per temperature level.
* decrease_temp: percent of temperature that will decrease.
* neighbors_level: number of neighbors to search from actual state.
* Return the best state found when iterations end or temperature is 0.
*/
pub fn simulated_annealing(initial_state : State, iter : i32, mut temp : f32, attemp_level : i32, decrease_temp : f32, neighbors_level: u32) -> State {
    let mut actual : State;
    actual = initial_state;
    for _ in 0..iter {
        for _ in 0..attemp_level {
            let best_neighbor = actual.get_neighbors(neighbors_level).clone();
            if best_neighbor.fitness() <= actual.fitness() {
                actual = best_neighbor;
            } else {
                let diff = (best_neighbor.fitness() as f32) - (actual.fitness() as f32);
                let prob = (diff/temp).exp();
                let mut rng = rand::thread_rng();
                let random = rng.gen::<f32>();
                if prob <= random {
                    actual = best_neighbor;
                }
            }
        }
        if temp <= 0.0 {
            break;
        }
        temp *= decrease_temp;
    }
    return actual;
}
