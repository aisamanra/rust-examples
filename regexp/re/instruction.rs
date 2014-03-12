/* A single instruction as used in the VM-based matcher */
#[deriving(Clone)]
pub enum Instr {
    IChar(char),        /* match a character or fail */
    IMatch,             /* match anything successfully */
    IJmp(uint) ,        /* jump to instr i */
    ISplit(uint, uint), /* try both instrs i and j */
}
