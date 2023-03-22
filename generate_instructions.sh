#!/usr/bin/env bash

cd generator

ocamlopt -c cpplib.ml
ocamlopt -c instructions.ml
ocamlopt -o ../bin/instruction_generator cpplib.cmx instructions.cmx 

cd ..
./bin/instruction_generator
./clang_format.sh
