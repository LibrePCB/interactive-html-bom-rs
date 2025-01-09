#!/usr/bin/env bash

# set shell settings (see https://sipb.mit.edu/doc/safe-shell/)
set -euv -o pipefail

# fix git error
git config --global --add safe.directory $PWD

# check if all files have Unix line endings
(git grep -Il $'\r' -- ':/') && exit 1

# check if no file contains trailing spaces
(git grep -Il ' $' -- ':/') && exit 1

# check if no file contains tabulators (with some exceptions)
(git grep -Il $'\t' -- ':/' ':!/LICENSES/') && exit 1

# check rust code formatting
for f in $(git ls-files -- '*.rs'); do
  (rustfmt --check "$f") || exit 1
done

# check formatting of .reuse/dep5
(debian-copyright-sorter --iml -s casefold -o ".reuse/dep5" ".reuse/dep5") || exit 1
(git diff --exit-code -- ".reuse/dep5") || exit 1
