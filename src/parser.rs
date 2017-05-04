#![allow(dead_code)]
use oak_runtime::*;

use super::ast::*;

// Parsing is scary. Very scary.
// Be afraid!

grammar! obsc {
    program = spacing expression    // a program is, but, an expression

    // Damn left recursion
    expression
        = composition
        / concatenation
        / non_empty
        / plain
        / spacing > empty_expression

    non_empty
        = infix
        / concatenation
        / ensquared
        / enclosed

    plain
        = string
        / number
        / word > word_expression

    // This will parse ``a `b` `c` d`` as ``(a `b`) (`c` d)``. It actually should not, but it does.
    infix
        = simple (infixed simple)+ > infix_chain
        / simple infixed > infix_left
        / infixed simple > infix_right
    // A greater flexibility is lost to make parsing simplier
    infixed = gravis word gravis

    concatenation = simple (concat simple)+ > concatenation
    composition = (infix / concatenation / simple) (non_empty / plain)+ > composition

    // Simple as ``(⍬`,`·`,`·`,`·)``
    simple
        = ensquared
        / enclosed
        / string
        / number
        / word > word_expression

    enclosed
        = lparren expression rparren > enclosed_expression

    ensquared
        = lbracket expression rbracket > quotation_expression

    word
        = gt > gt_word
        / eq > eq_word
        / lt > lt_word
        / plus > plus_word
        / minus > minus_word
        / prod > prod_word
        / div > div_word
        / swap > swap_word
        / dup > dup_word
        / drop > drop_word
        / id > id_word
        / zilde > zilde_word
        / comma > comma_word
        / behead > behead_word
        / print > print_word
        / rec > rec_word

    // Only simplest shapes of numbers. Not like in your lovely javascript
    number
        = digit+ "." digit+ spacing > float_expression
        / digit+ spacing > integer_expression
    digit = ["0-9"]


    string = "'" (!eos .)* "'" spacing > string_expression
    eos = !"\\" "'"

    // “Grave accent”? Sorry, no burial here.
    // Why don't you just call it “groove”?
    gravis = "`" spacing -> ()

    // Sweet APL symbols... Too bad, fonts you use suck.
    // This is how they actually should look like: http://aplwiki.com/AplCharacters
    comment = "⍝" (!"\n" .)* -> (^)

    // I have no lexer and I must parse
    lparren = "(" spacing -> ()
    rparren = ")" spacing -> ()
    lbracket = "[" spacing -> ()
    rbracket = "]" spacing -> ()

    // Behold the unicode
    rec = "∇" spacing -> ()
    print = "⎕" spacing -> ()
    concat = ";" spacing -> ()

    zilde = "⍬" spacing -> ()
    comma = "," spacing -> ()
    behead = "⍘" spacing -> () // I have no idea what this symbol is for in APL

    swap = "↔" spacing -> ()
    dup = "⇈" spacing -> ()
    drop = "↓" spacing -> ()
    id = "·" spacing -> ()

    gt = ">" spacing -> ()
    eq = "=" spacing -> ()
    lt = "<" spacing -> ()
    plus = "+" spacing -> ()
    minus = "−" spacing -> ()
    prod = "×" spacing -> ()
    div = "÷" spacing -> ()

    // Defining things in reverse
    spacing = [" \n\r\t"]* comment* -> (^)

    // Super code use
    use super::*;

    // Hello, boilerplate!
    fn gt_word() -> Word { Word::Gt }
    fn eq_word() -> Word { Word::Eq }
    fn lt_word() -> Word { Word::Lt }
    fn plus_word() -> Word { Word::Plus }
    fn minus_word() -> Word { Word::Minus }
    fn prod_word() -> Word { Word::Prod }
    fn div_word() -> Word { Word::Div }

    fn swap_word() -> Word { Word::Swap }
    fn dup_word() -> Word { Word::Dup }
    fn drop_word() -> Word { Word::Drop }
    fn id_word() -> Word { Word::Id }

    fn zilde_word() -> Word { Word::Zilde }
    fn comma_word() -> Word { Word::Comma }
    fn behead_word() -> Word { Word::Behead }

    fn print_word() -> Word { Word::Print }
    fn rec_word() -> Word { Word::Rec }

    fn word_expression(w: Word) -> Expression { Expression::Word(w) }
    fn float_expression(left: Vec<char>, right: Vec<char>) -> Expression {
        use std::iter::once;
        let float: String =
            left.into_iter().chain(once('.')).chain(right.into_iter()).collect();
        Expression::Float(float.parse().unwrap())
    }
    fn integer_expression(int: Vec<char>) -> Expression {
        let int: String = int.into_iter().collect();
        Expression::Integer(int.parse().unwrap())
    }
    fn string_expression(s: Vec<char>) -> Expression { Expression::String(s.into_iter().collect()) }
    fn quotation_expression(q: Expression) -> Expression { Expression::Quotation(Box::new(q)) }
    fn enclosed_expression(e: Expression) -> Expression { e }
    fn empty_expression() -> Expression { Expression::Nop }

    fn infix_chain(exp: Expression, chain: Vec<(Word, Expression)>) -> Expression {
        let mut chain = chain.into_iter();
        let init = {
            let (w, e) = chain.next().unwrap();
            Expression::Composition(vec![
                Expression::Concatenation(vec![exp, e]),
                Expression::Word(w)
            ])
        };

        chain.fold(init, |e1, (w, e2)| {
            Expression::Composition(vec![
                Expression::Concatenation(vec![e1, e2]),
                Expression::Word(w)
            ])
        })
    }

    fn infix_left(exp: Expression, inf: Word) -> Expression {
        Expression::InfixLeft(
            Box::new(exp),
            Box::new(Expression::Word(inf)),
        )
    }

    fn infix_right(inf: Word, exp: Expression) -> Expression {
        Expression::InfixRight(
            Box::new(Expression::Word(inf)),
            Box::new(exp),
        )
    }

    fn composition(head: Expression, mut tail: Vec<Expression>) -> Expression {
        tail.insert(0, head);
        Expression::Composition(tail)
    }
    fn concatenation(head: Expression, mut tail: Vec<Expression>) -> Expression {
        tail.insert(0, head);
        Expression::Concatenation(tail)
    }
}

pub fn parse(code: String) -> Result<Expression, ()> {
    obsc::parse_program(code.into_state()).data.ok_or(())
}

// I'm not a TDD programmer. But when I make my code compiling, I really doubt whether my code even works
// So I make small examples to check if results produced by my code acutally make any sense
// And it usually finds out, they doesn't
#[cfg(test)]
mod tests {
    // Testing in rust is extremely nice.
    // You don't have to put magical files in voodoo places or decide between hunit vs hspec vs tasty vs etc
    // You just make test functions in the module of tests. Simple as that.

    use oak_runtime::*;

    use super::obsc;
    use ast::Expression::*;
    use ast::Word::*;

    #[test] fn simple_postfix() {
        let ast = obsc::parse_program("2 2 + 3 −".into_state());
        assert_eq!(
            Some(Composition(vec![Integer(2), Integer(2), Word(Plus), Integer(3), Word(Minus)])),
            ast.data
        );
    }

    #[test] fn simple_concat() {
        let ast = obsc::parse_program("2 2 3 3 ×;× +".into_state());
        assert_eq!(
            Some(Composition(vec![
                Integer(2), Integer(2), Integer(3), Integer(3),
                Concatenation(vec![Word(Prod), Word(Prod)]),
                Word(Plus)
            ])),
            ast.data
        );
    }

    #[test] fn simple_infix() {
        let ast = obsc::parse_program("⍬`,`·`,`·`,`·".into_state());
        let should_be = obsc::parse_program("((⍬;· ,);· ,);· ,".into_state());
        assert_eq!(
            ast.data,
            should_be.data
        );
    }
}
