#!/bin/bash

path="./tests/ltc/"
files=$(ls $path)
for filename in $files
do
  if ([[ $filename == *".luac" ]]) then
    echo "executing" $filename "..."
    $(lua "./tests/ltc/"$filename) >> "./tests/ltc/"${filename//luac/check} &> "./tests/ltc/"${filename//luac/check}
    if ([[ $? -ne 0 ]]) then
      exit -1
    fi
  fi
done
