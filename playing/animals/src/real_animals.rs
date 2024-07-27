#[derive(Debug)]
pub(crate) struct LivingStatus{
    pub(crate) age: u8,
    pub(crate) is_alive: bool
}

pub(crate) struct Lion{
    pub(crate) name: String,
    pub(crate) status: LivingStatus
}

impl Lion {
    pub(crate) fn new(p0: String) -> Lion {
        Lion {
            name: p0,
            status: LivingStatus {
                age: 0,
                is_alive: true,
            },
        }
    }
}

pub struct Elephant{
    name: String,
    status: LivingStatus
}

impl Elephant{
    pub(crate) fn new(p0: String) -> Elephant{
        Elephant{ name: p0, status: LivingStatus{age: 0, is_alive: true} }
    }
}

pub struct Human{
    name: String,
    status: LivingStatus,
}

impl Human{
    pub(crate) fn new(name: String) -> Human{
        Human{ name, status: LivingStatus{age: 0, is_alive: true} }
    }

}
pub(crate) trait Animal{
    fn get_name(&self) -> &String;
    fn get_status(&self) -> &LivingStatus;
    fn set_status(&mut self, status: LivingStatus);
}

pub(crate) trait LivingSpeaker {
    fn make_sound(&self);
}

impl LivingSpeaker for Lion{
    fn make_sound(&self){
        println!("Roar!");
    }
}

impl LivingSpeaker for Elephant{
    fn make_sound(&self){
        println!("Trumpet!");
    }
}

impl LivingSpeaker for Human{
    fn make_sound(&self){
        println!("Hello!");
    }
}


impl Animal for Lion{
    fn get_name(&self) -> &String{
        &self.name
    }

    fn get_status(&self) -> &LivingStatus{
        &self.status
    }

    fn set_status(&mut self, status: LivingStatus){
        self.status = status;
    }
}

impl Animal for Elephant{
    fn get_name(&self) -> &String{
        &self.name
    }

    fn get_status(&self) -> &LivingStatus{
        &self.status
    }

    fn set_status(&mut self, status: LivingStatus){
        self.status = status;
    }
}

impl Animal for Human{
    fn get_name(&self) -> &String{
        &self.name
    }

    fn get_status(&self) -> &LivingStatus{
        &self.status
    }

    fn set_status(&mut self, status: LivingStatus){
        self.status = status;
    }
}