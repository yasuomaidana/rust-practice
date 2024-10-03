use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub enum PhilosopherState {
    Thinking,
    Hungry,
    Eating,
    Full,
}

pub struct Fork {
    pub(crate) id: usize,
    pub(crate) owner: Mutex<()>,
}


pub struct Philosopher {
    id: usize,
    pub(crate) name: String,
    pub state: Mutex<PhilosopherState>,
    stomach_capacity: Mutex<usize>,
    turn_time: usize,
    pub(crate) left_fork: Arc<Fork>,
    pub(crate) right_fork: Arc<Fork>,
}

impl PartialEq for PhilosopherState {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl Philosopher {
    pub fn new(id: usize, name: &str,
               stomach_capacity: usize, turn_time: usize,
               left_fork: Arc<Fork>, right_fork: Arc<Fork>,
    ) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            state: Mutex::from(PhilosopherState::Thinking),
            stomach_capacity: Mutex::from(stomach_capacity),
            turn_time,
            left_fork,
            right_fork,
        }
    }

    pub fn eat(&self) {
        if *self.stomach_capacity.lock().unwrap() <= 0 {
            println!("{} is full", self.name);
            *self.state.lock().unwrap() = PhilosopherState::Full;
            return;
        }

        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        *self.state.lock().unwrap() =  PhilosopherState::Eating;
        println!("{} is eating", self.name);
        let _first_fork = first_fork.owner.lock().unwrap();
        println!("picked first fork {}", first_fork.id);
        let _second_fork = second_fork.owner.lock().unwrap();
        println!("picked second fork {}", second_fork.id);

        *self.stomach_capacity.lock().unwrap() -= 1 * self.turn_time;
        sleep(Duration::from_micros(self.turn_time as u64));
        println!("releasing first fork {}", first_fork.id);
        println!("releasing second fork {}", second_fork.id);
        match *self.stomach_capacity.lock().unwrap() <= 0 {
            true => *self.state.lock().unwrap() =  PhilosopherState::Full,
            false => *self.state.lock().unwrap() =  PhilosopherState::Hungry,
        }
    }

    pub fn think(&mut self) {
        let state = self.state.lock().unwrap();

        if *state != PhilosopherState::Thinking {
            println!("{} is thinking", self.name);
            return;
        }
        *self.state.lock().unwrap() =  PhilosopherState::Thinking;
    }

    pub fn fork_status(&self) -> bool {
        let left_fork = match self.left_fork.owner.try_lock() {
            Ok(_) => true,
            Err(_) => false,
        };
        let right_fork = match self.right_fork.owner.try_lock() {
            Ok(_) => true,
            Err(_) => false,
        };
        left_fork && right_fork
    }
}