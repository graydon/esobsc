#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Composition(Vec<Expression>),      // `Comp`? Ugly. `Cmpstn`? Ugly as C. `Compose`? Meh.
    Concatenation(Vec<Expression>),    // no escape from long long names
    Word(Word),
    Integer(i64),
    Float(f64),
    String(String),
    Quotation(Box<Expression>),
    Nop,    // all right, doing nothing is important
    /// `` foo `bar` ``
    InfixLeft(Box<Expression>, Box<Expression>),
    /// `` `foo` bar ``
    InfixRight(Box<Expression>, Box<Expression>)
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

fn word_arity(w: &Word) -> Arity {
    use self::Word::*;
    match *w {
        Gt | Eq | Lt
        | Plus | Minus
        | Prod | Div => Arity(2, 1),
        Swap => Arity(2, 2),
        Dup => Arity(1, 2),
        Drop => Arity(1, 0),
        Id => Arity(1, 1),
        Zilde => Arity(0, 1),
        Comma => Arity(2, 1),
        Behead => Arity(1, 2),
        Print => Arity(1, 0),
        Rec => Arity(2, 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arity(u32, u32);

impl Arity {
    fn concat(self, other: Arity) -> Self {
        Arity(self.0 + other.0, self.1 + other.1)
    }

    fn compose(self, other: Arity) -> Self {
        let ar_in = self.0 + if other.0 > self.1 { other.0 - self.1 } else { 0 };
        let ar_out = other.1 + if self.1 > other.0 { self.1 - other.0 } else { 0 };
        Arity(ar_in, ar_out)
    }
}

// Feels like unnecessary boilerplate. But whatever.
#[derive(Debug, Clone, PartialEq)]
pub enum Arited {
    Composition(Vec<Arited>, Arity),
    Concatenation(Vec<Arited>, Arity),
    Word(Word, Arity),
    Integer(i64),
    Float(f64),
    String(String),
    Quotation(Box<Arited>),
    /// `a1 a2 ... an -> a1 a2 ... an`
    IdN(u32),
}

impl Arited {
    pub fn from_expression(e: Expression) -> Self {
        use self::Expression::*;

        match e {
            Composition(v) => {
                let comp: Vec<Arited> = v.into_iter().map(Arited::from_expression).collect();
                let arity = comp.iter().fold(Arity(0, 0), |ar, e| ar.compose(e.arity()));
                Arited::Composition(comp, arity)
            },
            // That's code duplication. I basically just used copy-paste here
            // No, I'm not going to refactor this
            Concatenation(v) => {
                let conc: Vec<Arited> = v.into_iter().map(Arited::from_expression).collect();
                let arity = conc.iter().fold(Arity(0, 0), |ar, e| ar.concat(e.arity()));
                Arited::Concatenation(conc, arity)
            },
            InfixLeft(e, op) => {
                let e = Arited::from_expression(*e);
                let op = Arited::from_expression(*op);

                let id_n = Arited::infix_id(&op, &e);
                let conc_ar = e.arity().concat(id_n.arity());
                let comp_ar = conc_ar.compose(op.arity());

                Arited::Composition(vec![
                    Arited::Concatenation(vec![e, id_n], conc_ar), op
                ], comp_ar)
            },
            InfixRight(op, e) => {
                let e = Arited::from_expression(*e);
                let op = Arited::from_expression(*op);

                let id_n = Arited::infix_id(&op, &e);
                let conc_ar = id_n.arity().concat(e.arity());
                let comp_ar = conc_ar.compose(op.arity());

                Arited::Composition(vec![
                    Arited::Concatenation(vec![id_n, e], conc_ar), op
                ], comp_ar)
            },
            Word(w) => {
                let ar = word_arity(&w);
                Arited::Word(w, ar)
            },
            Quotation(q) => {
                let q_ar = Box::new(Arited::from_expression(*q));
                Arited::Quotation(q_ar)
            },
            Integer(i) => Arited::Integer(i),
            Float(f) => Arited::Float(f),
            String(s) => Arited::String(s),
            Nop => Arited::IdN(0),
        }
    }

    fn infix_id(infix: &Arited, expr: &Arited) -> Self {
        let ar_inf = infix.arity();
        let ar_exp = expr.arity();
        let n = if ar_inf.0 > ar_exp.1 { ar_inf.0 - ar_exp.1 } else { 0 };
        Arited::IdN(n)
    }

    pub fn arity(&self) -> Arity {
        use self::Arited::*;
        match *self {
            Composition(_, ar)
            | Concatenation(_, ar)
            | Word(_, ar) => ar,
            IdN(n) => Arity(n, n),
            _ => Arity(0, 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser;
    use super::{Arited, Arity, Word};

    #[test] fn test_simple() {
        let expr = Arited::from_expression(parser::parse("×⎕".into()).unwrap());
        assert_eq!(
            expr,
            Arited::Composition(
                vec![
                    Arited::Word(Word::Prod, Arity(2, 1)),
                    Arited::Word(Word::Print, Arity(1, 0))
                ],
                Arity(2, 0)
            )
        );
    }

    #[test] fn test_infix() {
        let expr = Arited::from_expression(parser::parse("×`+`×".into()).unwrap());
        assert_eq!(
            expr,
            Arited::Composition(
                vec![
                    Arited::Concatenation(
                        vec![
                            Arited::Word(Word::Prod, Arity(2, 1)),
                            Arited::Word(Word::Prod, Arity(2, 1))
                        ],
                        Arity(4, 2)
                    ),
                    Arited::Word(Word::Plus, Arity(2, 1))
                ],
                Arity(4, 1)
            )
        );
    }

    #[test] fn test_zero() {
        let expr = Arited::from_expression(parser::parse("2 2 3 3 ×`+`×⎕".into()).unwrap());
        match expr {
            Arited::Composition(_, Arity(0, 0)) => (),
            _ => panic!("Expr is not composition of arity null: {:?}", expr),
        }
    }
}
