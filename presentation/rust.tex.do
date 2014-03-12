redo-ifchange rust.md
pandoc -f markdown -t beamer --standalone --highlight-style haddock \
  -V theme=Boadilla -V colortheme=beaver --template=my.beamer <rust.md >$3
