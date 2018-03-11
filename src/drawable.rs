pub mod drawable {

    extern crate rand;
    use self::rand::Rng;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::cmp::Eq;    
    
    struct Pair<T> {
        upper_bound: f32,
        data: T,
    }
    
    pub struct Drawable<T> {
        distribution: Vec<Pair<T>>,
    }

    impl<T> Drawable<T> where T: Hash + Eq + Copy {

        pub fn new(data_to_probability_map: HashMap<T, f32>) -> Result<Drawable<T>, &'static str>
        {
            let mut running_p_sum: f32 = 0.0;
            let mut distribution = vec![];
            for (&data, p) in data_to_probability_map.iter() {
                running_p_sum += *p;
                distribution.push(Pair { upper_bound: running_p_sum, data: data });
            }
            if running_p_sum == 1.0 {
                Ok(Drawable { distribution: distribution })
            } else {
                Err("values of HashMap don't sum to 1")
            }
        }

        pub fn draw(&self) -> T {
            let mut r: f32 = rand::random();
            for pair in &self.distribution {
                if r < pair.upper_bound {
                    return pair.data;
                }
                r += pair.upper_bound;
            }
            panic!("failed to draw; random number not found in distribution")
        }
    }
}