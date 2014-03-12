use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};
mod instruction;

/* The state of a program can be unambiguously specified by
 * a current instruction and a current position in the string. */
struct EvalState { pc: uint, cc: uint }

/* An evaluator that maintains a manual, mutable stack for doing
 * regular-expression matching. */
pub fn eval(instrs: &[Instr], input: &str) -> bool {
    let mut stack = ~[ EvalState {pc: 0, cc: 0} ];

    while stack.len() > 0 {
        let st = stack.pop();
        match instrs[st.pc] {
            IChar(_) if st.cc >= input.len() =>
                continue,
            IChar(c) if c == input.char_at(st.cc) =>
                stack.push(EvalState { pc: st.pc + 1, cc: st.cc + 1 }),
            IChar(_)     =>
                continue,
            IMatch       =>
                return true,
            IJmp(i)      =>
                stack.push(EvalState { pc: i, cc: st.cc }),
            ISplit(i, j) => {
                stack.push(EvalState { pc: j, cc: st.cc });
                stack.push(EvalState { pc: i, cc: st.cc });
            },
        }
    }
    return false;
}
