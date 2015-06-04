DEPS="rust.tex rust.md"
redo-ifchange $DEPS
DIR=$(mktemp -d)
xelatex --output-directory $DIR 1>&2 rust.tex
xelatex --output-directory $DIR 1>&2 rust.tex
mv $DIR/rust.pdf $3
rm -rf $DIR
