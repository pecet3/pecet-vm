#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub offset: Option<u32>,
    pub symbol_type: SymbolType,
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

#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }
}
