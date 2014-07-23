use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};

/* The state of a program can be unambiguously specified by
 * a current instruction and a current position in the string. */
struct EvalState { pc: uint, cc: uint }

/* An evaluator that maintains a manual, mutable stack for doing
 * regular-expression matching. */
pub fn eval(instrs: &[Instr], input: &str) -> bool {
    let mut stack = vec![ EvalState {pc: 0, cc: 0} ];

    /* Every time we find that a possibility is impossible, we
     * remove it from the stack. If we have completed a match,
     * we'll short-circuit out of this loop; otherwise, an empty
     * stack means we have failed every possible branch and can
     * return false. */
    while stack.len() > 0 {
        /* This call to .unwrap() is safe because we've already
         * manually checked the stack length. */
        let st = stack.pop().unwrap();
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
