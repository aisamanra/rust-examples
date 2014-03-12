DEPS="regexp.rs re/compile.rs re/instruction.rs re/mod.rs re/recursive.rs re/stack.rs"
redo-ifchange $DEPS
rustc regexp.rs -o $3
