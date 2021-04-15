mod state;
mod heuristics;

use std::time::Instant;
use crate::state::city::City as City;
use crate::state::state::State as State;
use crate::heuristics::simulated_annealing as sim_ann;

fn main() {
    let start = Instant::now();
    let initial_state = get_initial_state();
    let iter = 600;
    let temp = 300.0;
    let attemp_level = 10;
    let decrease_temp = 0.80;
    let neighbors_level = 10;
    sim_ann::simulated_annealing(initial_state, iter, temp, attemp_level, decrease_temp, neighbors_level);
    let duration = start.elapsed();
    println!("Tiempo: {:?} segundos", duration);
}

fn get_initial_state() -> State {
    let a = City::new(String::from("a"), 11003.611100, 42102.500000);
    let b = City::new(String::from("b"), 11108.611100, 42373.888900);
    let c = City::new(String::from("c"), 11133.333300, 42885.833300);
    let d = City::new(String::from("d"), 11155.833300, 42712.500000);
    let e = City::new(String::from("e"), 11183.333300, 42933.333300);
    let f = City::new(String::from("f"), 11297.500000, 42853.333300);
    let g = City::new(String::from("g"), 11310.277800, 42929.444400);
    let h = City::new(String::from("h"), 11416.666700, 42983.333300);
    let i = City::new(String::from("i"), 11423.888900, 43000.277800);
    let j = City::new(String::from("j"), 11438.333300, 42057.222200);
    let k = City::new(String::from("k"), 11461.111100, 43252.777800);
    let l = City::new(String::from("l"), 11485.555600, 43187.222200);
    let m = City::new(String::from("m"), 11503.055600, 42855.277800);
    let n = City::new(String::from("n"), 11511.388900, 42106.388900);
    let o = City::new(String::from("o"), 11522.222200, 42841.944400);
    let p = City::new(String::from("p"), 11569.444400, 43136.666700);
    let q = City::new(String::from("q"), 11583.333300, 43150.000000);
    let r = City::new(String::from("r"), 11595.000000, 43148.055600);
    let s = City::new(String::from("s"), 11600.000000, 43150.000000);
    let t = City::new(String::from("t"), 11690.555600, 42686.666700);
    let u = City::new(String::from("u"), 11715.833300, 41836.111100);
    let v = City::new(String::from("v"), 11751.111100, 42814.444400);
    let w = City::new(String::from("w"), 11770.277800, 42651.944400);
    let x = City::new(String::from("x"), 11785.277800, 42884.444400);
    let y = City::new(String::from("y"), 11822.777800, 42673.611100);
    let z = City::new(String::from("z"), 11846.944400, 42660.555600);
    let a1 = City::new(String::from("a1"), 11963.055600, 43290.555600);
    let b1 = City::new(String::from("b1"), 11973.055600, 43026.111100);
    let c1 = City::new(String::from("c1"), 12058.333300, 42195.555600);
    let d1 = City::new(String::from("d1"), 12149.444400, 42477.500000);
    let e1 = City::new(String::from("e1"), 12286.944400, 43355.555600);
    let f1 = City::new(String::from("f1"), 12300.000000, 42433.333300);
    let g1 = City::new(String::from("g1"), 12355.833300, 43156.388900);
    let h1 = City::new(String::from("h1"), 12363.333300, 43189.166700);
    let i1 = City::new(String::from("i1"), 12372.777800, 42711.388900);
    let j1 = City::new(String::from("j1"), 12386.666700, 43334.722200);
    let k1 = City::new(String::from("k1"), 12421.666700, 42895.555600);
    let l1 = City::new(String::from("k1"), 12645.000000, 42973.333300);
    let cities = vec![a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,a1,b1,c1,d1,e1,f1,g1,h1,i1,j1,k1,l1];
    let initial = State::new(std::ptr::null(), cities);
    return initial;
}
