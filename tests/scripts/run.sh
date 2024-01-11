#!/bin/bash

path="./tests/ltc/"
files=$(ls $path)
for filename in $files
do
  if ([[ $filename == *".luac" ]]) then
    echo "executing" $filename "..."
    $(lua "./tests/ltc/"$filename) >> "./tests/ltc/"${filename//luac/check}
  fi
done
