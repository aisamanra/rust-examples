use re::instruction::Instr;
// use std::vec::Vec;

/* A regular expression parse tree */
#[deriving(Show)]
enum Regexp {
    Char(char),
    Seq(Box<Regexp>, Box<Regexp>),
    Chc(Box<Regexp>, Box<Regexp>),
    Rep(Box<Regexp>),
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
                 (s2, Regexp::Seq(box r1, box r2)) },
        '|' => { let (s1, r1) = parse(s.slice_from(1));
                 let (s2, r2) = parse(s1);
                 (s2, Regexp::Chc(box r1, box r2)) },
        '*' => { let (s1, r1) = parse(s.slice_from(1));
                 (s1, Regexp::Rep(box r1)) },
        c   => (s.slice_from(1), Regexp::Char(c)),
    }
}

/* Compiling an AST for regexps to the instructions.
 * The return values correspond to the length of the
 * vector (so that subsequent instructions to be added
 * know what pc to use) and the vector of instructions.
 */
fn emit(regexp: &Regexp, pc: uint) -> (uint, Vec<Instr>) {
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
        Regexp::Seq(box ref first, box ref second) =>
            { let (first_pc, mut v1) = emit(first, pc);
              let (second_pc, v2) = emit(second, first_pc);
              v1.push_all(v2.as_slice());
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
        Regexp::Chc(box ref first, box ref second) =>
            { let (first_pc, v1) = emit(first, pc + 1);
              let (second_pc, v2) = emit(second, first_pc + 1);
              let mut split_instr = vec![ Instr::Split(pc + 1, first_pc + 1) ];
              let jmp_instr = vec![ Instr::Jmp(second_pc) ];
              split_instr.push_all(v1.as_slice());
              split_instr.push_all(jmp_instr.as_slice());
              split_instr.push_all(v2.as_slice());
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
        Regexp::Rep(box ref expr) =>
            { let (expr_pc, v1) = emit(expr, pc + 1);
              let mut spl = vec![ Instr::Split(pc + 1, expr_pc + 1) ];
              let jmp = vec![ Instr::Jmp(pc) ];
              spl.push_all(v1.as_slice());
              spl.push_all(jmp.as_slice());
              (expr_pc + 1, spl)
            },
    }
}

/* A wrapper over these processes */
pub fn compile(s: &str) -> Vec<Instr> {
    let (_, re) = parse(s);
    println!("{}", re);
    let (_, mut ins) = emit(&re, 0);
    println!("{}", ins);
    /* If we get to the end of a compiled regular expression,
     * that means it hasn't aborted and we can match.
     */
    ins.push(Instr::Match);
    return ins;
}
