use std::collections::HashMap;

pub trait Container<T> {
    fn get_inner(&self, s: &String) -> Option<&T>;
}
    
pub struct Program {
    packages: HashMap<String, FireDir>
}
impl Program {
    pub fn new() -> Program {
        Program {
            packages: HashMap::new(),
        }
    }
}
impl Container<FireDir> for Program {
    fn get_inner(&self, s: &String) -> Option<&FireDir> {
        self.packages.get(s)
    }
}


enum FireFileType {
    Dir(FireDir),
    File(FireFile)
}
    
struct FireDir {
    files: HashMap<String, FireFileType>
}
impl Container<FireFileType> for FireDir {
    fn get_inner(&self, s: &String) -> Option<&FireFileType> {
        self.files.get(s)
    }
}
    
struct FireFile {
    declarations: HashMap<String, TopLevelDeclaration>
}
impl Container<TopLevelDeclaration> for FireFile {
    fn get_inner(&self, s: &String) -> Option<&TopLevelDeclaration> {
        self.declarations.get(s)
    }
}
    
enum TopLevelDeclaration {
    Var(GlobalVar),
    Func(Func),
    Proc(Proc),
    Struct(Struct),
    Trait(Trait),
    Enum(Enum)
}
    
enum StructInner {
    Func(Func),
    Property(Property)
}
    
struct GlobalVar();
struct Func();
struct Proc();
struct Struct {
    inner: HashMap<String,StructInner>
}
impl Container<StructInner> for Struct {
    fn get_inner(&self, s: &String) -> Option<&StructInner> {
        self.inner.get(s)
    }

}
struct Property();
struct Trait {
    funcs: HashMap<String,Func>
}
impl Container<Func> for Trait {
    fn get_inner(&self, s: &String) -> Option<&Func> {
        self.funcs.get(s)
    }
}
struct Enum {
    vals: HashMap<String,GlobalVar>
}
impl Container<GlobalVar> for Enum {
    fn get_inner(&self, s: &String) -> Option<&GlobalVar> {
        self.vals.get(s)
    }
}