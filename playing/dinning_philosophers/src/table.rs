// use std::collections::{BTreeSet, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use crate::philosophers::{Fork, Philosopher, PhilosopherState};

pub struct Table {
    pub(crate) philosophers: Vec<Arc<Philosopher>>,
    // forks: Vec<Arc<Fork>>,
}

impl Table {
    pub fn from_names_and_forks(names: Vec<&str>,
                                stomach_capacities: Vec<usize>,
                                turn_times: Vec<usize>,
                                num_forks: usize) -> Self {
        let mut philosophers = Vec::new();
        let mut forks = Vec::new();

        for i in 0..num_forks {
            forks.push(Arc::new(Fork {
                id: i,
                owner: Mutex::new(()),
            }));
        }

        for (i, ((name, stomach_capacity), turn_time)) in names
            .into_iter().zip(stomach_capacities).zip(turn_times)
            .enumerate() {
            let left_fork = Arc::clone(&forks[i % num_forks]);
            let right_fork = Arc::clone(&forks[(i + 1) % num_forks]);

            philosophers.push(Arc::new(Philosopher::new(i, name, stomach_capacity, turn_time, left_fork, right_fork)));
        }

        Table {
            philosophers,
            // forks,
        }
    }

    pub fn dine(&self) {
        let total_philosopher = self.philosophers.len();
        let mut full_philosophers = 0;
        while total_philosopher > full_philosophers {
            let threads = self.philosophers.par_iter().map(
                |philosopher| {
                    let philosopher = Arc::clone(&philosopher);
                    thread::spawn(move || {
                        if *philosopher.state.lock().unwrap() == PhilosopherState::Full {
                            return 1;
                        }
                        if philosopher.fork_status() {
                            philosopher.eat();
                            0
                        } else {
                            println!("{} is waiting for the forks", philosopher.name);
                            0
                        }

                    })
                }
            ).collect::<Vec<_>>();
            full_philosophers = threads.into_par_iter().map(|thread| {
                thread.join().unwrap()
            }).sum();
            println!("full philosophers: {}", full_philosophers);
        }
    }
}