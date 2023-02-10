#!/usr/bin/env bash

set -uo pipefail

git ls-files -- '*.cc' '*.h' ':!:*/thirdparty/*' |
while read -r f; do
  clang-format-13 -style=file --Wno-error=unknown -i "$f"
done

diff=$(git diff --color)
echo "$diff"
