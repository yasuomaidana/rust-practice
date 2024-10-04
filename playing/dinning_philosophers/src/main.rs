use std::sync::Arc;
use std::thread;
use rayon::iter::{IntoParallelIterator,ParallelIterator};

mod philosophers;
mod table;

fn main() {
    let names_capacity_time = vec![
        ("Jürgen Habermas", 5, 1),
        ("Friedrich Engels", 5, 3),
        ("Karl Marx", 10, 3),
        ("Thomas Piketty", 5, 2),
        ("Michel Foucault", 5, 2),
        ("Socrates", 5, 2),
        ("Plato", 15, 2),
        ("Aristotle", 5, 2),
        ("Pythagoras", 25, 3),
        ("Heraclitus", 5, 2),
        ("Democritus", 6, 2),
        ("Diogenes", 5, 2),
        ("Epicurus", 5, 2),
        ("Zeno of Citium", 5, 2),
        ("Tales of Miletus", 5, 2),
    ];
    let names: Vec<&str> = names_capacity_time.iter().map(|(name, _, _)| *name).collect();
    let capacities: Vec<usize> = names_capacity_time.iter().map(|(_, capacity, _)| *capacity).collect();
    let times: Vec<usize> = names_capacity_time.iter().map(|(_, _, time)| *time).collect();
    let table = table::Table::from_names_and_forks(names, capacities, times, 5);

    let threads = table.philosophers.iter().map(|philosopher| {
        let philosopher = Arc::clone(philosopher);
        thread::spawn(move || {
            if philosopher.fork_status(){
                philosopher.eat();
            }
            else{
                println!("{}  is waiting for the forks", philosopher.name);
            }
            thread::sleep(std::time::Duration::from_secs(1));
        })
    }).collect::<Vec<_>>();
    threads.into_par_iter().for_each(|thread| {
        thread.join().unwrap();
    });
}
