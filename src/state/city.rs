use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct City {
    pub name: String,
    pub x_axis: f32,
    pub y_axis: f32
}


impl City {
    /**
    * Create a new City.
    * name: name of country.
    * x_axis: position in X axis.
    * y_axis: position in Y axis.
    */
    pub fn new(name: String, x_axis: f32, y_axis: f32) -> City {
        City {name, x_axis, y_axis}
    }
}

impl PartialEq for City {
    fn eq(&self, other : &Self) -> bool {
        self.x_axis == other.x_axis &&
        self.y_axis == other.y_axis
    }
}

impl PartialOrd for City {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x_axis.partial_cmp(&other.x_axis)
    }
}

impl Eq for City{}

impl Ord for City{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_axis == other.x_axis &&
           self.y_axis == other.y_axis {
            return Ordering::Equal;
        } else if self.x_axis >= other.x_axis &&
                  self.y_axis >= other.y_axis {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}
