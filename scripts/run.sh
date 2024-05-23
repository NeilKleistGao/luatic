#!/bin/bash

path="./tests/ltc/"
files=$(ls $path)
for filename in $files
do
  if ([[ $filename == *".lynx" ]]) then
    echo "executing" $filename "..." # check compatibility
    $(lua "./tests/ltc/"$filename)
    if ([[ $? -ne 0 ]]) then
      exit -1
    fi
  fi
done
