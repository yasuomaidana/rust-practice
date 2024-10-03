use std::sync::{Arc, Mutex};
use crate::philosophers::{Fork, Philosopher};

pub struct Table {
    pub(crate) philosophers: Vec<Arc<Philosopher>>,
    forks: Vec<Arc<Fork>>,
}

impl Table {
    pub fn from_names_and_forks(names: Vec<&str>,
                                stomach_capacities: Vec<usize>,
                                turn_times: Vec<usize>,
                                num_forks:usize)-> Self{
        let mut philosophers = Vec::new();
        let mut forks = Vec::new();

        for i in 0..num_forks{
            forks.push(Arc::new(Fork{
                id: i,
                owner: Mutex::new(()),
            }));
        }

        for (i,((name, stomach_capacity), turn_time) )in names
            .into_iter().zip(stomach_capacities).zip(turn_times)
            .enumerate(){

            let left_fork = Arc::clone(&forks[i % num_forks]);
            let right_fork = Arc::clone(&forks[(i + 1) % num_forks]);

            philosophers.push(Arc::new(Philosopher::new(i, name, stomach_capacity, turn_time, left_fork, right_fork)));
        }

        Table{
            philosophers,
            forks,
        }
    }

}