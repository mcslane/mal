use crate::types::{MalType, MalList, MalSymbol, MalNumber};

pub fn pr_str(obj: MalType) -> String {
    let visitor = PrintVisitor{};
    (*obj.accept_print_visitor(&visitor)).to_string()
}

pub struct PrintVisitor {}

impl PrintVisitor {
    pub fn pr_list(&self, list: &MalList) -> String {
        format!(
            "({})",
            (&list.list).into_iter()
                .map(|boxed_obj| (*boxed_obj.accept_print_visitor(self)).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ).to_string()
    }
    
    pub fn pr_number(&self, number: &MalNumber) -> String {
        number.value.to_string()
    }
    
    pub fn pr_symbol(&self, symbol: &MalSymbol) -> String {
        String::from(&symbol.name)
    }
}
