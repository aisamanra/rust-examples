use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};
// use std::vec::Vec;

/* A regular expression parse tree */
#[deriving(Show)]
enum Regexp {
    RChar(char),
    RSeq(Box<Regexp>, Box<Regexp>),
    RChc(Box<Regexp>, Box<Regexp>),
    RRep(Box<Regexp>),
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
                 (s2, RSeq(box r1, box r2)) },
        '|' => { let (s1, r1) = parse(s.slice_from(1));
                 let (s2, r2) = parse(s1);
                 (s2, RChc(box r1, box r2)) },
        '*' => { let (s1, r1) = parse(s.slice_from(1));
                 (s1, RRep(box r1)) },
        c   => (s.slice_from(1), RChar(c)),
    }
}

/* Compiling an AST for regexps to the instructions */
fn emit(r: &Regexp, i: uint) -> (uint, Vec<Instr>) {
    match *r {
        RChar(c) => { (i+1, vec![IChar(c)]) },
        RSeq(box ref a, box ref b) =>
            { let (ai, mut v1) = emit(a, i);
              let (bi, v2) = emit(b, ai);
              v1.push_all_move(v2);
              (bi, v1) },
        RChc(box ref a, box ref b) =>
            { let (ai, v1) = emit(a, i + 1);
              let (bi, v2) = emit(b, ai + 1);
              let mut spl = vec![ ISplit(i + 1, ai + 1) ];
              let jmp = vec![ IJmp(ai) ];
              spl.push_all_move(v1);
              spl.push_all_move(jmp);
              spl.push_all_move(v2);
              (bi, spl) },
        RRep(box ref a) =>
            { let (ai, v1) = emit(a, i + 1);
              let mut spl = vec![ ISplit(i + 1, ai + 1) ];
              let jmp = vec![ IJmp(i) ];
              spl.push_all_move(v1);
              spl.push_all_move(jmp);
              (ai + 1, spl) },
    }
}

/* A wrapper over these processes */
pub fn compile(s: &str) -> Vec<Instr> {
    let (_, re) = parse(s);
    println!("{}", re);
    let (_, ins) = emit(&re, 0);
    println!("{}", ins);
    return ins.append([IMatch]);
}
