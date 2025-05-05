# What is this?
I decided to approach learning Rust through recreating common CLI tools. `rat` is a simple program that prints out files. `rat` supports the standard `cat` flags: 

    -A, --show-all           equivalent to -vET
    -b, --number-nonblank    number nonempty output lines, overrides -n
    -e                       equivalent to -vE
    -E, --show-ends          display $ at end of each line
    -n, --number             number all output lines
    -s, --squeeze-blank      suppress repeated empty output lines
    -t                       equivalent to -vT
    -T, --show-tabs          display TAB characters as ^I
    -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
        --help        display this help and exit
        --version     output version information and exit


# Things to redo/fix
- Make reading files/stdin buffered
- Make SqueezeBlanks replace chunks of newlines at once instead of one by one.
- *Maybe ignore this, unnecessary dedup:* Make flags apply all at once instead of constantly looping thru rmatch_indices.
- Make testing suite with random inputs that diff's cat and rat - maybe bash script.
