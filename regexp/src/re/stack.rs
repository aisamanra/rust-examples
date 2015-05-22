use re::instruction::Instr;

/* The state of a program can be unambiguously specified by
 * a current instruction and a current position in the string. */
struct EvalState { pc: usize, cc: usize }

/* An evaluator that maintains a manual, mutable stack for doing
 * regular-expression matching. */
pub fn eval(instrs: &[Instr], input: &[char]) -> bool {
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
            Instr::Char(_) if st.cc >= input.len() =>
                continue,
            Instr::Char(c) if c == input[st.cc] =>
                stack.push(EvalState { pc: st.pc + 1, cc: st.cc + 1 }),
            Instr::Char(_)     =>
                continue,
            Instr::Match       =>
                return true,
            Instr::Jmp(i)      =>
                stack.push(EvalState { pc: i, cc: st.cc }),
            Instr::Split(i, j) => {
                stack.push(EvalState { pc: j, cc: st.cc });
                stack.push(EvalState { pc: i, cc: st.cc });
            },
        }
    }
    return false;
}
