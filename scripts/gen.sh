#!/bin/bash

java -jar antlr4.jar -Dlanguage=Rust -visitor Luatic.g4 -o src/frontend/
