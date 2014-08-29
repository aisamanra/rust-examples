pub use re::compile::compile;
pub use re::instruction::{Instr,IChar,IMatch,IJmp,ISplit};
pub mod compile;
pub mod instruction;
pub mod recursive;
pub mod stack;
