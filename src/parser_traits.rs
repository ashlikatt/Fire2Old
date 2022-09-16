use std::collections::{HashMap, HashSet};
use crate::{expression_parser::Value, resources::ResourceLoc};

// Program contains packages
pub struct Program {
    pub resources: HashMap<ResourceLoc, TopLevelResource>
}
pub struct ProgramResources {
    pub resources: HashSet<ResourceLoc>
}

pub enum Variable {
    GlobalVariable(GlobalVariable),
    LocalVariable(LocalVariable)
}
pub enum DataType {
    Struct(),
    Enum(),

}

pub enum ResourceHolder {
    Dir(Dir),
    FireFile(FireFile)
}

pub enum Function {
    ConcreteFunction(ConcreteFunction),
    AbstractFunction(AbstractFunction)
}
pub enum TopLevelResource {
    ConcreteFunction(ConcreteFunction),
    AbstractFunction(AbstractFunction),
    Process(Process),
    GlobalVariable(GlobalVariable),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
}

pub struct Package(HashMap<String, ResourceHolder>);
pub struct Dir(HashMap<String,Box<ResourceHolder>>);
pub struct FireFile(HashMap<String,TopLevelResource>);
pub struct ConcreteFunction{ annotations: Vec<Annotation>, parameters: Vec<LocalVariable>, return_type: DataType, code: CodeBlock }
pub struct AbstractFunction{ annotations: Vec<Annotation>, parameters: Vec<LocalVariable>, return_type: DataType }
pub struct Process{ annotations: Vec<Annotation>, parameters: Vec<LocalVariable>, code: CodeBlock }
pub struct GlobalVariable{ annotations: Vec<Annotation>, var_type: DataType }
pub struct LocalVariable{ name: String, var_type: DataType }
pub struct Struct{ annotations: Vec<Annotation>, properties: HashMap<String, LocalVariable>, traits: Vec<Trait>, methods: HashMap<String, ConcreteFunction> }
pub struct Enum{ annotations: Vec<Annotation>, fields: HashMap<String, GlobalVariable>}
pub struct Trait{ annotations: Vec<Annotation>, supertraits: Vec<Trait>, methods: HashMap<String, Function> }


pub struct CodeBlock(Vec<Statement>);
pub enum Statement {
    Assignment(Variable, Value),
    Break, Continue, Raise(Value), Return(Value),
    If{expr: Value, block: CodeBlock, otherwise: Option<CodeBlock>},
    While{expr: Value, block: CodeBlock},
    For{var: LocalVariable, val: Value, block: CodeBlock},
}
pub struct Annotation{ name: String, params: Vec<Value> }