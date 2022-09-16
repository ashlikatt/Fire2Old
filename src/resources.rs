
#[derive(Eq)]
pub struct ResourceLoc {
    file: FileLoc,
    elem: ElementLoc
}
impl PartialEq for ResourceLoc {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file && self.elem == other.elem
    }
}

#[derive(Eq)]
pub struct FileLoc {
    loc: Vec<String>
}
impl PartialEq for FileLoc {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
    }
}
impl FileLoc {
    pub fn with(&self, next: String) -> FileLoc {
        let mut new = self.loc.clone();
        new.push(next);
        FileLoc {
            loc: new
        }
    }
}

type ElementLoc = FileLoc;