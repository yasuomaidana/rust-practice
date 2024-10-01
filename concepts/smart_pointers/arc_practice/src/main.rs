use std::sync::{Arc, Mutex};
use std::thread;
use crate::tools::{Owner, Tool};

mod tools;

fn main() {
    println!("Starting arc_practice...");
    let gandalf = Arc::from(Owner::new("Gandalf".to_string()));
    let legolas = Arc::from(Owner::new("Legolas".to_string()));
    let tools = vec!["hammer", "saw", "elbow"];
    let mut threads = vec![];
    let tool_arcs = Arc::new(Mutex::new(vec![]));

    for (i,tool) in tools.into_iter().enumerate() {
        let original_owner = if i < 2 {&gandalf} else {&legolas};
        let original_owner = Arc::clone(original_owner);
        let gandalf = Arc::clone(&gandalf);
        let tools = Arc::clone(&tool_arcs);
        threads.push(thread::spawn(move || {
            let tool = Tool{name: tool.to_string(), owner: original_owner};
            let tool = Arc::from(tool);
            gandalf.tools.lock().unwrap().push(Arc::downgrade(&tool));
            tools.lock().unwrap().push(tool.clone());
            println!("{} has: {}", tool.owner.name, tool.name);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{} has:", gandalf.name);
    for tool in gandalf.tools.lock().unwrap().iter() {
        match tool.upgrade() {
            Some(tool) => {
                let owner = tool.owner.name.clone();
                let tool = tool.name.clone();
                println!("\t{} - owner: {}", tool, owner);
            },
            None => println!("Tool was dropped"),
        }
    }

}
