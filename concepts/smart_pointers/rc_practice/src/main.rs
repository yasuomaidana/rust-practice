use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Tool{
    name: String,
    owner: Rc<Owner>
}

struct Owner{
    name: String,
    tools: RefCell<Vec<Weak<Tool>>>,
}

impl Owner {
    fn new(name: String) -> Owner {
        Owner {
            name,
            tools: RefCell::new(vec![])
        }
    }
}

fn main() {
    let gandalf = Rc::new(Owner::new("Gandalf".to_string()));
    let legolas = Rc::new(Owner::new("Legolas".to_string()));

    let hammer = Rc::from(Tool {name:"hammer".to_string(), owner: Rc::clone(&gandalf)});
    let saw = Rc::from(Tool {name:"saw".to_string(), owner: gandalf.clone()});
    let elbow = Rc::from(Tool {name:"elbow".to_string(), owner: legolas.clone()});

    println!("Owner: {}", gandalf.name);
    println!("Tools: {} {}", hammer.owner.name, saw.owner.name);
    gandalf.tools.borrow_mut().push(Rc::downgrade(&hammer));
    gandalf.tools.borrow_mut().push(Rc::downgrade(&saw));
    gandalf.tools.borrow_mut().push(Rc::downgrade(&elbow));

    println!("{} has:", gandalf.name);
    for tool in gandalf.tools.borrow().iter() {
        let tool = tool.upgrade().unwrap();
        let owner = tool.owner.name.clone();
        let tool = tool.name.clone();
        println!("\t{} - owner: {}", tool, owner);
    }

}
