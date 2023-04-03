#!/usr/bin/env bash

cd generator

ocamlopt -c cpplib.ml
ocamlopt -c instructions.ml
ocamlopt -o ../bin/instruction_generator cpplib.cmx instructions.cmx 

cd ..
./bin/instruction_generator

git ls-files -- '*.cc' '*.h' ':!:*/libraries/*' |
while read -r f; do
  clang-format-13 -style=file --Wno-error=unknown -i "$f"
done
