pub mod random_writer {

    // Let's write a class that can take any sort of ordered data and apply Shannon's method.
    // So instead of counting the number of times Char c follows seed String s,
    // count the number of times datum c follows [seed array of type E] s.
    // This will be a fun and interesting class to use in short programs, and
    // will also be a good introduction to true programming over generic types.

    
    use drawable::drawable::Drawable;
    
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::cmp::Eq;
    use std::fmt::Debug;
    use std::fmt::Display;
    
    pub struct RandomWriter<'a, T: 'a> {
        input: &'a [T],
        k: u32,
        len: u32,
    }

    type DistroMap<'a, T> = HashMap<&'a T, f32>;
    type CountsMapMap<'a, T> = HashMap<&'a [T], HashMap<&'a T, u32>>;
    type DistroMapMap<'a, T> = HashMap<&'a [T], DistroMap<'a, T>>;
    
    impl<'a, T> RandomWriter<'a, T> {
        
        pub fn new(input: &'a [T], k: u32, len: u32) -> RandomWriter<'a, T> {
            RandomWriter { input: input, k: k, len: len }
        }        

        pub fn display_nested_hash_map(&self, map: DistroMapMap<'a, T>) 
            where CountsMapMap<'a, T>: IntoIterator,
                  &'a [T]: Eq + Hash,
                  &'a T: Eq + Hash,
                  T: Debug + Display
        {
            for (seed, element_counts) in map.iter() {
                print!("{:?}| ", seed);
                for (element, counts) in element_counts.iter() {
                    print!("{}: ", element);
                    print!("{}|", counts);
                }
                println!();
            }
        }

        pub fn get_prefix_distribution(&self) -> DistroMapMap<'a, T>
            where T: Eq + Hash
        {
            let prefix_counts = &self.get_prefix_counts();
            let mut distribution: DistroMapMap<'a, T> = HashMap::new();
            for (seed, element_counts) in prefix_counts.iter() {
                let seed_distribution = distribution.entry(seed).or_insert(HashMap::new());
                let total_instances: u32 = element_counts.values().sum();
                for (element, counts) in element_counts.iter() {
                    seed_distribution.insert(element, (*counts as f32) / (total_instances as f32));
                }
            }
            distribution
        }

        pub fn create_drawables(&self) -> HashMoap<&'a [T], Drawable<T>> where T: Eq + Hash + Copy {
            let prefix_distribution_maps = &self.get_prefix_distribution();
            let mut drawables: HashMap<&'a [T], Drawable<T>> = HashMap::new();
            for (prefix, distribution_map) in prefix_distribution_maps.into_iter() {
                drawables.insert(prefix, Drawable::new(distribution_map).unwrap());
            }

            //let drawables: HashMap<&'a [T], Drawable<T>> = prefix_distribution_maps.
            //    into_iter().
            //    map(|(k, v)| HashMap::new(k, Drawable::new(v).unwrap())).
            //    collect();
            drawables
            // unimplemented!()
        }
        
        pub fn get_prefix_counts(&self) -> CountsMapMap<'a, T>
            where T: Hash + Eq
        {
            let vals = &self.input;
            let mut followers_counts = HashMap::new();
            for i in 0..(vals.len() - (self.k as usize) - 1) {
                let end_of_prefix_pos = i + (self.k as usize) - 1;
                let seed = &vals[i..(end_of_prefix_pos + 1)];
                let next = &vals[end_of_prefix_pos + 1];
                let mut counts_map = followers_counts.entry(seed).
                                                      or_insert(HashMap::new());
                *counts_map.entry(next).or_insert(0) += 1;
            }
            followers_counts
        }
    }
}
