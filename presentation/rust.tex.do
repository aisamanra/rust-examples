redo-ifchange rust.md
pandoc -f markdown -t beamer --standalone --highlight-style haddock \
  -V theme=Boadilla -V colortheme=beaver <rust.md >$3
