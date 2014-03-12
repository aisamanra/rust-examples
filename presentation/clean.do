FILES="rust.tex rust.pdf"

for f in $FILES; do
    if [ -e $f ]; then rm $f; fi
done
