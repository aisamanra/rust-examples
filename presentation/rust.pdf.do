DEPS="rust.tex rust.md"
redo-ifchange $DEPS
xelatex --output-directory build 1>&2 rust.tex
mv build/rust.pdf $3
