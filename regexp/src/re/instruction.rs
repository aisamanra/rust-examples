/* A single instruction as used in the VM-based matcher */
#[derive(Clone,Debug)]
pub enum Instr {
    Char(char),        /* match a character or fail */
    Match,             /* match anything successfully */
    Jmp(usize),          /* jump to instr i */
    Split(usize, usize),   /* try both instrs i and j */
}
