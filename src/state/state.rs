use rand::Rng;
use std::ptr;
use crate::state::city as city;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct State {
    pub parent : *const State,
    pub tour: Vec<city::City>
}

 impl State {
     pub fn new(parent: *const State, tour: Vec<city::City>) -> State {
         State { parent, tour }
     }

     /**
     * Get n neighbors from the current state.
     * Each new neighbor is a random swap between countries of the current city.
     */
     pub fn get_neighbors(&self, mut n : u32) -> Vec<State> {
         let mut neighbors : Vec<State> = Vec::new();
         let m = self.tour.len();
         let mut random_1;
         let mut random_2;
         while n != 0 {
             random_1  = rand::thread_rng().gen_range(1, m);
             random_2  = rand::thread_rng().gen_range(1, m);
             while random_1 == random_2 {
                 random_2 = rand::thread_rng().gen_range(1, m);
             }
             let new_neighbor = self.get_neighbor(random_1, random_2);
             neighbors.push(new_neighbor);
             n-=1;
         }
         //neighbors.sort_by_key(|a| a.get_collitions());
         return neighbors;
     }

     /**
     * Get a new neighbor from the current state.
     * Swap the countries in position_1 and position_2.
     * position_1, position_2: positions of countries in tour vector.
     */
     fn get_neighbor (&self, position_1 : usize, position_2 : usize) -> State {
         let mut new_tour = self.tour.clone();
         let tmp : crate::state::city::City = new_tour[position_1].clone();
         new_tour[position_1] = new_tour[position_2].clone();
         new_tour[position_2] = tmp;
         let new_neighbor = State { parent: self, tour: new_tour};
         return new_neighbor;
     }

     fn fitness (&self) -> f32 {
         let mut fitness = 0.0;
         let len = self.tour.len();
         let mut i = 0;
         while i != len {
             if i + 1 != len {
                 fitness += self.tour[i].get_distance(self.tour[i+1].clone());
                 i += 1;
                 continue;
             }
             break;
         }
         // Add the distance between origin city and last city
         fitness += self.tour[0].get_distance(self.tour[len-1].clone());
         return fitness;
     }

 }


 #[cfg(test)]
 mod tests {
     /**
     * Test if neighbors of current state differ in position of 2 cities.
     */
     #[test]
     fn test_neighbors() {
         let initial = init_state();
         let neighbors = initial.get_neighbors(8);
         for neighbor in neighbors {
             // Never change origin
             assert_eq!(String::from("a"), neighbor.tour[0].name);
             assert_eq!(10, neighbor.tour.len());
             let mut variations = 0;
             for (index, city) in neighbor.tour.iter().enumerate() {
                 if *city != initial.tour[index as usize] {
                     variations +=1;
                 }
             }
             // Only two different cities have changed of position.
             assert_eq!(2, variations);
         }
     }

     #[test]
     fn test_fitness(){
         let initial = init_state();
         let range = 681.0..682.0;
         assert!(range.contains(&initial.fitness()));
     }

     fn init_state() -> crate::state::state::State {
         let a = crate::state::city::City::new(String::from("a"), 34.4, 54.6);
         // a -> b 42.242277
         let b = crate::state::city::City::new(String::from("b"), 12.3, 18.6);
         // b -> c 87.184001
         let c = crate::state::city::City::new(String::from("c"), 96.0, 43.0);
         // c -> d 94.681044
         let d = crate::state::city::City::new(String::from("d"), 03.7, 21.9);
         // d -> e 75.700066
         let e = crate::state::city::City::new(String::from("e"), 76.4, 43.0);
         // e -> f 63.724799
         let f = crate::state::city::City::new(String::from("f"), 14.1, 29.6);
         // f -> g  29.441637
         let g = crate::state::city::City::new(String::from("g"), 23.2, 01.6);
         // g -> h 81.973715
         let h = crate::state::city::City::new(String::from("h"), 32.0, 83.1);
         // h -> i 82.186374
         let i = crate::state::city::City::new(String::from("i"), 88.8, 23.7);
         // i -> j 92.93374
         let j = crate::state::city::City::new(String::from("j"), 12.6, 76.9);
         // j -> a 31.185413
         let cities = vec![a,b,c,d,e,f,g,h,i,j];
         let initial = crate::state::state::State::new(std::ptr::null(), cities);
         return initial;
     }
 }
