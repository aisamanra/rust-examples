use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};
use std::vec::append;
mod instruction;

/* A regular expression parse tree */
enum Regexp {
    RChar(char),
    RSeq(~Regexp, ~Regexp),
    RChc(~Regexp, ~Regexp),
    RRep(~Regexp),
}

/* We're assuming a prefix regexp here. That means that we have
 * the following operators:
 *   .ab => ab
 *   |ab => a|b
 *   *a  => a*
 * but these nest, so (ab|c)* would become
 *   *|c.ab
 * This is easier to parse. Deal with it.
 */
fn parse<'a>(s: &'a str) -> (&'a str, Regexp) {
    match s.char_at(0) {
        '.' => { let (s1, r1) = parse(s.slice_from(1));
                 let (s2, r2) = parse(s1);
                 (s2, RSeq(~r1, ~r2)) },
        '|' => { let (s1, r1) = parse(s.slice_from(1));
                 let (s2, r2) = parse(s1);
                 (s2, RChc(~r1, ~r2)) },
        '*' => { let (s1, r1) = parse(s.slice_from(1));
                 (s1, RRep(~r1)) },
        c   => (s.slice_from(1), RChar(c)),
    }
}

/* Compiling an AST for regexps to the instructions */
fn emit(r: &Regexp, i: uint) -> (uint, ~[Instr]) {
    match *r {
        RChar(c) => { (i+1, ~[IChar(c)]) },
        RSeq(ref a, ref b) =>
            { let (ai, v1) = emit(*a, i);
              let (bi, v2) = emit(*b, ai);
              (bi, append(v1, v2)) },
        RChc(ref a, ref b) =>
            { let (ai, v1) = emit(*a, i + 1);
              let (bi, v2) = emit(*b, ai + 1);
              let spl = ~[ ISplit(i + 1, ai + 1) ];
              let jmp = ~[ IJmp(ai) ];
              (bi, append(spl, append(v1, append(jmp, v2)))) },
        RRep(ref a) =>
            { let (ai, v1) = emit(*a, i + 1);
              let spl = ~[ ISplit(i + 1, ai + 1) ];
              let jmp = ~[ IJmp(i) ];
              (ai + 1, append(spl, append(v1, jmp))) },
    }
}

/* A wrapper over these processes */
pub fn compile(s: &str) -> ~[Instr] {
    let (_, re) = parse(s);
    println!("{:?}", re);
    let (_, ins) = emit(&re, 0);
    println!("{:?}", ins);
    return append(ins, [IMatch]);
}
