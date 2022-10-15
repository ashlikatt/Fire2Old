use core::fmt;


#[derive(Eq)]
#[derive(Debug)]
pub struct ResourceLoc {
    pub loc: Vec<String>
}
impl ResourceLoc {
    pub fn new() -> ResourceLoc {
        ResourceLoc { loc: Vec::new() }
    }
    pub fn with(&self, other: String) -> ResourceLoc {
        let mut new = self.loc.clone();
        new.push(other);
        ResourceLoc { loc: new }
    }
}
impl PartialEq for ResourceLoc {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
    }
}
impl fmt::Display for ResourceLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.loc.join("::"))
    }
}