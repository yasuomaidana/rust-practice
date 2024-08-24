use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub(crate) struct Fighter{
    pub(crate) name: String
}

pub(crate) struct FighterStats{
    pub(crate) name: String,
    pub(crate) degree: f32
}

impl Eq for FighterStats {}

impl PartialEq<Self> for FighterStats {
    fn eq(&self, other: &Self) -> bool {
        self.degree == other.degree
    }
}

impl PartialOrd<Self> for FighterStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.degree < other.degree {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for FighterStats{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        self.degree.partial_cmp(&other.degree).unwrap()
    }
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
