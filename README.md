# Esobsc: the esoteric version of ␣;

Esobsc is an implementation of [␣;](https://suhr.github.io/obsc/) for the esolang community.

## How to use it

Install Rust and complile esobsc with `cargo build --release`.

To make your first esobsc program, create a source code file named `hello.c` and write this into it:

```
'Hello world!\n'⎕    ⍝ This prints “Hello world”
```

Run your program with `cargo run -- hello.c`

## Syntax

- `+` `−` `×` `÷` `>` `=` `<` — arithmetics
- `13 ·` ⇒ `13`
- `666 13 ↔` ⇒ `13 666`
- `42 19 ↓` ⇒ `42`
- `9 ⇈` ⇒ `9 9`
- `⍬` — creates an empty list
- `,` — appends an element to a list
- `⍬1,⍘` ⇒ `⍬1`
- `⎕` — print
- `∇` — fixed point combinator. It takes an init value and `(a -> a bool)` quotation and runs it while true
- `` 1`=`0?'OH MY GOD JC A BUG!\n':'My branching is argumentated\n'.⎕ ``
- `()`, `[]` — grouping, quotation
- ` ` `;` — composition and concatenation
- `⍝` — a lamp

## FAQ

- **Q:** How do I type all these symbols? I'm using `ed` and there's no `∇` on my keyboard.
- **A:** It is hard to write esobsc code in `ed`. Consider using a text editor.
- **Q:** I see strange squares instead of esobsc symbols. Why?
- **A:** Probably your system lacks Unicode fonts required to display esobsc symbols correctly. Install fonts. Alternatively, your software may not have Unicode support. Never use software that doesn't support Unicode.
- **Q:** Is esobsc turing-complete?
- **A:** Yes. Proving it is left to the reader as an exercise.
