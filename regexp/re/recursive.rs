use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};

/* We wrap the real evaluation function, as we're always going to
 * start executing instruction 0 with no string matched. */
pub fn eval(instrs: &[Instr], input: &str) -> bool {
    eval1(instrs, input, 0, 0)
}

/* We use the Rust stack as our stack in this naive recursive
 * implementation. */
fn eval1(instrs: &[Instr], input: &str, pc: uint, cc: uint) -> bool {
    match instrs[pc] {
        IChar(_) if cc >= input.len() => return false,
        IChar(c) if c == input.char_at(cc) =>
            eval1(instrs, input, pc + 1, cc + 1),
        IChar(_)     => return false,
        IMatch       => return true,
        IJmp(i)      => eval1(instrs, input, i, cc),
        ISplit(i, _) if eval1(instrs, input, i, cc) => true,
        ISplit(_, j) => eval1(instrs, input, j, cc),
    }
}
