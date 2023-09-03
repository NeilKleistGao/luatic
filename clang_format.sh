#!/usr/bin/env bash

set -uo pipefail

type clang-format
if [ $? -eq 0 ]; then
  git ls-files -- '*.cc' '*.h' ':!:*/libraries/*' |
  while read -r f; do
    clang-format -style=file --Wno-error=unknown -i "$f"
  done
else
  git ls-files -- '*.cc' '*.h' ':!:*/libraries/*' |
  while read -r f; do
    clang-format-13 -style=file --Wno-error=unknown -i "$f" # clang-format-13 is used in CI
  done
fi

diff=$(git diff --color)
echo "$diff"
