use re::instruction::Instr;
use std::str::Chars;

/* A regular expression parse tree */
#[derive(Debug)]
enum Regexp {
    Char(char),
    Seq(Box<Regexp>, Box<Regexp>),
    Chc(Box<Regexp>, Box<Regexp>),
    Rep(Box<Regexp>),
}

fn chr(c: char) -> Box<Regexp> {
    Box::new(Regexp::Char(c))
}

fn seq(l: Box<Regexp>, r: Box<Regexp>) -> Box<Regexp> {
    Box::new(Regexp::Seq(l, r))
}

fn chc(l: Box<Regexp>, r: Box<Regexp>) -> Box<Regexp> {
    Box::new(Regexp::Chc(l, r))
}

fn rep(x: Box<Regexp>) -> Box<Regexp> {
    Box::new(Regexp::Rep(x))
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
fn parse<'a>(s: &'a mut Chars<'a>) -> (&'a mut Chars<'a>, Box<Regexp>) {
    match s.next() {
        Some('.') => { let (s1, r1) = parse(s);
                       let (s2, r2) = parse(s1);
                       (s2, seq(r1, r2)) },
        Some('|') => { let (s1, r1) = parse(s);
                       let (s2, r2) = parse(s1);
                       (s2, chc(r1, r2)) },
        Some('*') => { let (s1, r1) = parse(s);
                       (s1, rep(r1)) },
        Some(c)   => (s, chr(c)),
        None      => panic!("Unexpected EOF"),
    }
}

/* This should eventually be added to a stable API, but right now
 * isn't available in the stable stdlib. */
fn push_all<'a, A, I>(target: &mut Vec<A>, source: I)
    where A: Clone + 'a, I: Iterator<Item=&'a A> {
    for x in source {
        target.push(x.clone());
    }
}

/* Compiling an AST for regexps to the instructions.
 * The return values correspond to the length of the
 * vector (so that subsequent instructions to be added
 * know what pc to use) and the vector of instructions.
 */
fn emit(regexp: &Regexp, pc: usize) -> (usize, Vec<Instr>) {
    match *regexp {
        /* For a match, we produce this code:
         *   ---- <- pc
         *   | IChar(chr)
         *   ---- <- pc + 1
         */
        Regexp::Char(chr) => { (pc+1, vec![Instr::Char(chr)]) },
        /* For a sequencing, we produce this code:
         *   ---- <- pc
         *   |   [[ first ]]
         *   ---- <- first_pc
         *   |   [[ second ]]
         *   ---- <- second_pc
         */
        Regexp::Seq(ref first, ref second) =>
            { let (first_pc, mut v1) = emit(first, pc);
              let (second_pc, v2) = emit(second, first_pc);
              push_all(&mut v1, v2.iter());
              (second_pc, v1)
            },
        /* For a choice, we produce this code:
         *   ---- <- pc
         *   | ISplit(pc+1, first_pc+1)
         *   ---- <- pc + 1
         *   |   [[ first ]]
         *   ---- <- first_pc
         *   | IJmp(second_pc)
         *   ---- <- first_pc + 1
         *   |   [[ second ]]
         *   ---- <- second_pc
         */
        Regexp::Chc(ref first, ref second) =>
            { let (first_pc, v1) = emit(first, pc + 1);
              let (second_pc, v2) = emit(second, first_pc + 1);
              let mut split_instr = vec![ Instr::Split(pc + 1, first_pc + 1) ];
              let jmp_instr = vec![ Instr::Jmp(second_pc) ];
              push_all(&mut split_instr, v1.iter());
              push_all(&mut split_instr, jmp_instr.iter());
              push_all(&mut split_instr, v2.iter());
              (second_pc, split_instr)
            },
        /* For a repetition, we produce this code:
         *   ---- <- pc
         *   | ISplit(pc+1, expr_pc + 1)
         *   ---- <- pc + 1
         *   |   [[ expr ]]
         *   ---- <- expr_pc
         *   | IJmp(pc)
         *   ---- <- expr_pc + 1
         */
        Regexp::Rep(ref expr) =>
            { let (expr_pc, v1) = emit(expr, pc + 1);
              let mut spl = vec![ Instr::Split(pc + 1, expr_pc + 1) ];
              let jmp = vec![ Instr::Jmp(pc) ];
              push_all(&mut spl, v1.iter());
              push_all(&mut spl, jmp.iter());
              (expr_pc + 1, spl)
            },
    }
}

/* A wrapper over these processes */
pub fn compile(s: &str) -> Vec<Instr> {
    let (_, re) = parse(&mut s.chars());
    println!("{:?}", re);
    let (_, mut ins) = emit(&re, 0);
    println!("{:?}", ins);
    /* If we get to the end of a compiled regular expression,
     * that means it hasn't aborted and we can match.
     */
    ins.push(Instr::Match);
    return ins;
}
