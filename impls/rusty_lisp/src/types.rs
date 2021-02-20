use crate::printer::PrintVisitor;

pub trait MalTypeTrait {
    fn accept_print_visitor(&self, visitor: &PrintVisitor) -> String;
}

pub type MalType = Box<dyn MalTypeTrait>;

pub struct MalList {
    pub list: Vec<MalType>,
} 
impl MalTypeTrait for MalList {
    fn accept_print_visitor(&self, visitor: &PrintVisitor) -> String {
        visitor.pr_list(self)
    }
}

pub struct MalNumber {
    pub value: i64,
}
impl MalTypeTrait for MalNumber {
    fn accept_print_visitor(&self, visitor: &PrintVisitor) -> String {
        visitor.pr_number(self)
    }
}

pub struct MalSymbol {
    pub name: String,
}
impl MalTypeTrait for MalSymbol {
    fn accept_print_visitor(&self, visitor: &PrintVisitor) -> String {
        visitor.pr_symbol(self)
    }
}
