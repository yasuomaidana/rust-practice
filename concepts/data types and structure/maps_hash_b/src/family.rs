use std::cmp::Ordering;
use crate::person::Person;

pub(crate) struct Family<'a>{
    pub(crate) name: String,
    pub(crate) members: Vec<&'a Person>,
}

impl<'a> Family<'a> {
    pub(crate) fn new(name: String) -> Family<'a> {
        Family {
            name,
            members: Vec::new(),
        }
    }

    pub(crate) fn add_member(&mut self, person: &'a Person) {
        self.members.push(person);
    }
}

impl Eq for Family<'_> {

}

impl PartialEq<Self> for Family<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.members.len() == other.members.len()
    }
}

impl PartialOrd<Self> for Family<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Family<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.members.len().cmp(&other.members.len())
    }
}