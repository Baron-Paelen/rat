#!/bin/bash

# basic script that checks if cat and rat have 
# the same outputs with the same set of randomly 
# selected flags.

tr -dc "A-Za-z 0-9" < /dev/urandom | fold -w100|head -n 100000 > bigfile.txt

FILE="bigfile.txt"

# check if file exists
if [ ! -f "$FILE" ]; then
    echo "Error: File '$FILE' does not exist."
    exit 1
fi

# list of cat-compatible flags
flags=(
  "-A"
  "-b"
  "-e"
  "-E"
  "-n"
  "-s"
  "-t"
  "-T"
  "-v"
  "--show-all"
  "--number-nonblank"
  "--show-ends"
  "--number"
  "--squeeze-blank"
  "--show-tabs"
  "--show-nonprinting"
  # "--help"    excluded because intentionally different outputs
  #"--version"  excluded because intentionally different outputs
)

# generate a random subset of flags
selected_flags=()
for flag in "${flags[@]}"; do
  if (( RANDOM % 5 == 0 )); then
    selected_flags+=("$flag")
  fi
done

# # run `cat` and `rat` on the file and compare their outputs
# CAT_OUTPUT=$(cat "${selected_flags[@]}" "$FILE")
# RAT_OUTPUT=$(cargo run -- "${selected_flags[@]}" "$FILE")

# if [ "$CAT_OUTPUT" == "$RAT_OUTPUT" ]; then
#     echo "The outputs of 'cat' and 'rat' are identical."
# else
#     echo "The outputs of 'cat' and 'rat' differ."
# fi

diff <(cat "${selected_flags[@]}" "$FILE") <(cargo run -q --release -- "${selected_flags[@]}" "$FILE")
DIFF_OUTPUT=$?

echo -e "\n"

if [ "$DIFF_OUTPUT" -eq "0" ]; then
    echo "✅ Success: The outputs of 'cat' and 'rat' are identical."
    echo "    Flags: ${selected_flags[@]}"
elif [ "$DIFF_OUTPUT" -eq "1" ]; then
    echo "❌ Failure: The outputs of 'cat' and 'rat' differ."
    echo "    Flags: ${selected_flags[@]}"
else
    echo "☠️ Error: diff failed with exit code $DIFF_OUTPUT."
    echo "    Flags: ${selected_flags[@]}"
fi

rm bigfile.txt