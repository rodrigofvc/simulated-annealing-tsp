use rand::Rng;
use crate::state::city::City as City;

#[derive(Clone, Debug)]
struct CityIndex(City, usize);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct State {
    pub parent : *const State,
    pub tour: Vec<City>
}

 impl State {
     pub fn new(parent: *const State, tour: Vec<City>) -> State {
         State { parent, tour }
     }

     /**
     * Get n neighbors from the current state.
     * Each new neighbor is a random swap between countries of the current city.
     */
     pub fn get_neighbors(&self, mut n : u32) -> Vec<State> {
         let mut neighbors : Vec<State> = Vec::new();
         while n != 0 {
             let new_neighbor = self.get_neighbor();
             neighbors.push(new_neighbor);
             n-=1;
         }
         neighbors.sort_by_key(|a| a.fitness() as u32);
         return neighbors;
     }

     /**
     * Get a new neighbor from the current state.
     * Search for paths that intersects each other and 'destroy' the intersection.
     * If no intersection found, then random swap .
     */
     fn get_neighbor (&self) -> State {
         let mut new_tour = self.tour.clone();
         let new_neighbor : State;
         for i in 0..self.tour.len() {
             if i + 1 != self.tour.len() {
                let city_1 = self.tour[i].clone();
                let city_2 = self.tour[i+1].clone();
                let city_index_1 = CityIndex(city_1, i);
                let city_index_2 = CityIndex(city_2, i+1);
                match self.intersect_path(city_index_1, city_index_2.clone()) {
                    Some ((city_index_3, _city_index_4)) => {
                        new_tour[i+1] = city_index_3.0;
                        new_tour[city_index_3.1] = city_index_2.0;
                        new_neighbor = State { parent: self, tour: new_tour};
                        return new_neighbor;
                    },
                    _ =>
                    {
                        continue;
                    }
                }
            }
        }
        /* No lines intersection found, random swap */
        let position_1 = rand::thread_rng().gen_range(1, self.tour.len());
        let mut position_2 = rand::thread_rng().gen_range(1, self.tour.len());
        while position_1 == position_2 {
            position_2 = rand::thread_rng().gen_range(1, self.tour.len());
        }
        let tmp = new_tour[position_1].clone();
        new_tour[position_1] = new_tour[position_2].clone();
        new_tour[position_2] = tmp;
        new_neighbor = State { parent: self, tour: new_tour};
        return new_neighbor;
     }

     /**
     * Given two cities with their index, search for cities
     * that makes a path which intersect with the path of the given cities.
     * city_1: Tuple with a city and its index.
     * city_2: Tuple with a city and its index.
     * return the cities with their index, otherwise return None.
     *
     */
     fn intersect_path (&self, city_1 : CityIndex, city_2 : CityIndex) -> Option<(CityIndex, CityIndex)> {
         for i in 0..self.tour.len() {
             if i + 1 != self.tour.len() {
                 if city_1.1 == i && city_2.1 == i + 1 ||
                    city_2.1 == i && city_1.1 == i + 1 {
                        continue;
                 }
                 let city_3 = self.tour[i].clone();
                 let city_4 = self.tour[i+1].clone();
                 let index_city_3 = i;
                 let index_city_4 = i + 1;
                 let y_diff_1 = city_2.0.y_axis - city_1.0.y_axis;
                 let x_diff_1 = city_1.0.x_axis - city_2.0.x_axis;

                 let y_diff_2 = city_4.y_axis - city_3.y_axis;
                 let x_diff_2 = city_3.x_axis - city_4.x_axis;

                 let delta = y_diff_1 * x_diff_2 - x_diff_1 * y_diff_2;

                 if delta == 0.0 {
                    let res = (CityIndex(city_3, index_city_3), CityIndex(city_4, index_city_4));
                    return Some(res);
                 }
             }
         }
         None
     }

     /**
     * Adds the distance between cities that are next, and distance between first and last city.
     * 1-2-3-4-5-..-n.
     * distance(1,2) + distance(2,3) + distance(3,4) +..+ distance(n-1,n) + distance(n,1)
     */
     pub fn fitness (&self) -> f32 {
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

     /**
     * Get a string with every coordenade of every city.
     * Firts column is x axis, second column is y axis.
     * #X #Y
     * 1.23 4.56 # First city
     * .........
     *
     */
     pub fn get_coordinates(&self) -> String {
         let mut content = String::new();
         for city in &self.tour {
             content.push_str(&city.to_string());
         }
         content.push_str(&self.tour[0].to_string());
         content
     }

 }


 #[cfg(test)]
 mod tests {
     use crate::state::city::City as City;
     use crate::state::state::State as State;
     /**
     * Test if neighbors of current state differ in position of 2 cities.
     */
     #[test]
     fn test_neighbors() {
         let initial = init_state();
         let neighbors = initial.get_neighbors(8);
         for neighbor in neighbors {
             // Never change origin
             assert_eq!(1, neighbor.tour[0].id);
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

     fn init_state() -> State {
         let a = City::new(1, 34.4, 54.6);
         // 1 -> 2 42.242277
         let b = City::new(2, 12.3, 18.6);
         // 2 -> 3 87.184001
         let c = City::new(3, 96.0, 43.0);
         // 3 -> 4 94.681044
         let d = City::new(4, 03.7, 21.9);
         // 4 -> 5 75.700066
         let e = City::new(5, 76.4, 43.0);
         // 5 -> 6 63.724799
         let f = City::new(6, 14.1, 29.6);
         // 6 -> 7  29.441637
         let g = City::new(7, 23.2, 01.6);
         // 7 -> 8 81.973715
         let h = City::new(8, 32.0, 83.1);
         // 8 -> 9 82.186374
         let i = City::new(9, 88.8, 23.7);
         // 9 -> 10 92.93374
         let j = City::new(10, 12.6, 76.9);
         // 10 -> 11 31.185413
         let cities = vec![a,b,c,d,e,f,g,h,i,j];
         let initial = State::new(std::ptr::null(), cities);
         return initial;
     }
 }
