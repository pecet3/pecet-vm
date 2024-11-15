#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    offset: Option<u32>,
    symbol_type: SymbolType,
}
impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            offset: None,
            symbol_type,
        }
    }
    pub fn new_with_offset(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            offset: Some((offset)),
            symbol_type,
        }
    }
}
#[derive(Debug, Clone)]
pub enum SymbolType {
    Label,
    Integer,
    IrString,
}
