use super::ast::{Arity, Arited, Word};

use std::convert::From;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Error {
    TypeError,
    ListError,
    PrintError,
    ArityError,
}

impl From<!> for Error {
    fn from(_: !) -> Error {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
enum Data {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(VecDeque<Data>),
    Quotation(Box<Arited>)
}

#[derive(Debug)]
pub struct Machine {
    stack: Vec<Data>,
    retained: Vec<Data>,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: vec![],
            retained: vec![],
        }
    }

    fn retain(&mut self, n: usize) {
        for _ in 0..n {
            let v = self.pop_one();
            self.retained.push(v);
        }
    }

    fn restore(&mut self, n: usize) {
        for _ in 0..n {
            let v = self.retained.pop().expect("IIE: Retained underflow");
            self.stack.push(v);
        }
    }

    fn pop_one(&mut self) -> Data {
        self.stack.pop().expect("IIE: Stack underflow")
    }

    fn pop_two(&mut self) -> (Data, Data) {
        let r = self.pop_one();
        let l = self.pop_one();
        (l, r)
    }

    fn push_integer(&mut self, int: i64) -> Result<(), !> {
        self.stack.push(Data::Integer(int));
        Ok(())
    }

    fn push_float(&mut self, float: f64) -> Result<(), !> {
        self.stack.push(Data::Float(float));
        Ok(())
    }

    fn push_string(&mut self, string: String) -> Result<(), !> {
        self.stack.push(Data::String(string));
        Ok(())
    }

    fn push_bool(&mut self, logic: bool) {
        self.stack.push(Data::Bool(logic));
    }

    fn push_zilde(&mut self) -> Result<(), !> {
        self.stack.push(Data::List(VecDeque::new()));
        Ok(())
    }

    fn push_quotation(&mut self, arited: Box<Arited>) -> Result<(), !> {
        self.stack.push(Data::Quotation(arited));
        Ok(())
    }

    fn greater_than(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_bool(l > r);
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_bool(l > r);
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn lesser_than(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_bool(l < r);
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_bool(l < r);
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn equals(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_bool(l == r);
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_bool(l == r);
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn plus(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_integer(l + r)?
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_float(l + r)?
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn minus(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_integer(l - r)?
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_float(l - r)?
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn prod(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_integer(l * r)?
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_float(l * r)?
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn div(&mut self) -> Result<(), Error> {
        let (left, right) = self.pop_two();
        match (left, right) {
            (Data::Integer(l), Data::Integer(r)) => {
                self.push_integer(l * r)?
            },
            (Data::Float(l), Data::Float(r)) => {
                self.push_float(l * r)?
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn swap(&mut self) -> Result<(), !> {
        let (l, r) = self.pop_two();
        self.stack.push(r);
        self.stack.push(l);
        Ok(())
    }

    fn dup(&mut self) -> Result<(), !> {
        let v = self.pop_one();
        self.stack.push(v.clone());
        self.stack.push(v);
        Ok(())
    }

    fn drop(&mut self) -> Result<(), !> {
        drop(self.pop_one());
        Ok(())
    }

    fn comma(&mut self) -> Result<(), Error> {
        let (list, val) = self.pop_two();
        match (list, val) {
            (Data::List(mut vs), v) => {
                vs.push_back(v);
                self.stack.push(Data::List(vs));
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn behead(&mut self) -> Result<(), Error> {
        let list = self.pop_one();
        match list {
            Data::List(mut vs) => {
                let v = vs.pop_front().ok_or(Error::ListError)?;
                self.stack.push(v);
                self.stack.push(Data::List(vs));
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    fn print(&mut self) -> Result<(), Error> {
        let val = self.pop_one();
        match val {
            Data::Integer(i) => print!("{}", i),
            Data::Float(f) => print!("{}", f),
            Data::Bool(b) => print!("{}", b),
            Data::String(s) => print!("{}", s),
            _ => return Err(Error::PrintError),
        }
        Ok(())
    }

    fn execute_word(&mut self, word: Word) -> Result<(), Error> {
        use self::Word::*;
        match word {
            Gt => self.greater_than()?,
            Eq => self.equals()?,
            Lt => self.lesser_than()?,
            Plus => self.plus()?,
            Minus => self.minus()?,
            Prod => self.prod()?,
            Div => self.div()?,
            Swap => self.swap()?,
            Dup => self.dup()?,
            Drop => self.drop()?,
            Id => (), // `id` is no-op
            Zilde => self.push_zilde()?,
            Comma => self.comma()?,
            Behead => self.behead()?,
            Print => self.print()?,
            Rec => self.recurse()?,
        }
        Ok(())
    }

    fn recurse(&mut self) -> Result<(), Error> {
        let (init, quote) = self.pop_two();
        match (init, quote) {
            (v, Data::Quotation(q)) => {
                if q.arity() != Arity(1, 2) { return Err(Error::ArityError) }

                self.stack.push(v);
                loop {
                    self.execute(&*q)?;
                    let cond = self.pop_one();
                    match cond {
                        Data::Bool(false) => break,
                        Data::Bool(true) => (),
                        _ => return Err(Error::TypeError),
                    }
                }
            },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }

    pub fn execute(&mut self, expr: &Arited) -> Result<(), Error> {
        use self::Arited::*;

        match expr {
            &Composition(ref comp, _) => {
                for e in comp {
                    self.execute(&e)?
                }
            },
            &Concatenation(ref conc, arity) => {
                self.retain(arity.0 as usize);
                for e in conc {
                    self.restore(e.arity().0 as usize);
                    self.execute(&e)?
                }
            },
            &Question(ref cons, ref alter, _) => {
                let cond = self.pop_one();
                match cond {
                    Data::Bool(true) => self.execute(cons)?,
                    Data::Bool(false) => self.execute(alter)?,
                    _ => return Err(Error::TypeError),
                }
            },
            &Word(w, _) => self.execute_word(w)?,
            &Integer(i) => self.push_integer(i)?,
            &Float(f) => self.push_float(f)?,
            &String(ref s) => self.push_string(s.clone())?,
            &Quotation(ref q) => self.push_quotation(q.clone())?,
            &IdN(_) => (),
        }
        Ok(())
    }

    pub fn execute_program(&mut self, expr: &Arited) -> Result<(), Error> {
        if expr.arity().0 != 0 { return Err(Error::ArityError) }
        self.execute(expr)
    }
}
