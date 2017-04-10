#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Composition(Vec<Expression>),      // `Comp`? Ugly. `Cmpstn`? Ugly as C. `Compose`? Meh.
    Concatenation(Vec<Expression>),    // no escape from long long names
    Word(Word),
    Integer(i64),
    Float(f64),
    String(String),
    Quotation(Box<Expression>),
    /// `a1 a2 ... an -> a1 a2 ... an`
    IdN(u32),   // this allows you to use just ``a `b` `` instead of ``a `b` (·;·;...;·)`
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Word {
    Gt,
    Eq,
    Lt,
    Plus,
    Minus,
    Prod,
    Div,
    Swap,
    Dup,
    Drop,
    Id,
    Zilde,
    Comma,
    Behead,
    Print,
    Rec,
}
