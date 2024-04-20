#!/bin/bash

java -jar antlr4.jar -Dlanguage=Rust luatic.g4 -o src/frontend/
