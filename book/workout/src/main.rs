use std::{thread, time::Duration, collections::HashMap, hash::Hash};

fn main() {
    let simulated_user_specified_value: u32 = 10;
    let simulated_random_number: u32 = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |num: &u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    };
    let mut expensive_closure = Cacher::new(expensive_closure);
    if intensity < 25 {
        // println!("Today, do {} pushups!", expensive_result);
        // println!("Next, do {} situps!", expensive_result);
        // println!("Today, do {} pushups!", expensive_closure(intensity));
        // println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Today, do {} pushups!", expensive_closure.value(&intensity));
        println!("Next, do {} situps!", expensive_closure.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // println!("Today, run for {} minutes!", expensive_result);
            // println!("Today, run for {} minutes!", expensive_closure(intensity));
            println!("Today, run for {} minutes!", expensive_closure.value(&intensity));
        }
    }
}
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<'a, T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq
{
    calculation: T,
    map: HashMap<&'a U, V>
}

impl<'a, T, U, V> Cacher<'a, T, U, V>
where
    T: Fn(&U) -> V,
    U: Hash + Eq
{
    fn new(calculation: T) -> Cacher<'a, T, U, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }
    fn value(&mut self, arg: &'a U) -> &V {
        let v = (self.calculation)(&arg);
        self.map.entry(arg).or_insert(v)
        // match self.value.get(&arg) {
        //     Some(v) => v,
        //     None => {
        //         let v = ;
        //         self.value.insert(arg, v);
        //         self.value.get(&arg).unwrap()
        //     }
        // }
    }
}
