/* A single instruction as used in the VM-based matcher */
#[deriving(Clone,Show)]
pub enum Instr {
    Char(char),        /* match a character or fail */
    Match,             /* match anything successfully */
    Jmp(uint) ,        /* jump to instr i */
    Split(uint, uint), /* try both instrs i and j */
}