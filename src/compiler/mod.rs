use crate::resources::ResourceLoc;

#[derive(Debug)]
pub struct CompileError<'a> {
    info: &'a str,
    loc: ResourceLoc,
    line_num: usize,
    line_index: usize,
}
impl CompileError<'_> {
    pub fn new<'a>(loc: ResourceLoc, line_num: usize, line_index: usize, info: &'a str) -> CompileError<'a> {
        CompileError {
            info, loc, line_num, line_index 
        }
    }
    pub fn to_string(&self) -> String {
        format!("Exception occured at {}:{} in {}:\n{}", self.line_num, self.line_index, self.loc, self.line_index)
    }
}