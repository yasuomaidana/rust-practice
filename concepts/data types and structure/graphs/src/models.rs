use std::fmt;

#[derive!(Debug)]
pub(crate) struct Fighter{
    pub(crate) name: String
}

impl Fighter{
    pub(crate) fn new(name: String) -> Self{
        Self{
            name
        }
    }
}

impl fmt::Display for Fighter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}
