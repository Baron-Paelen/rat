use std::fmt;

pub struct RatArgs {
    pub flags:          Vec<Flag>,
    pub operands:       Vec<String>,
}

impl RatArgs {
    pub fn display_help() {
        println!(
            r#"
            Usage: rat [OPTION]... [FILE]...
            Conratenate FILE(s) to standard ratput.

            With no FILE, or when FILE is '-', read standard input.
            
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

            Examples:
            rat f - g  Output f's contents, then standard input, then g's contents.
            rat        Copy standard input to standard output.

            Disclaimer: This message was largely copied from "cat".
            "#
        );
    }
}

impl From<Vec<String>> for RatArgs {
    fn from(clargs: Vec<String>) -> RatArgs {
        let mut out = RatArgs {
            flags: Vec::new(),
            operands: Vec::new(),
        };

        // validate each flag and compound flags
        for arg in clargs {
            if arg.starts_with("--") {
                // long flags

                let fs = Flag::f_reduce(&arg).unwrap_or_else(|| panic!("invalid flag {}!", arg));
                for f in fs {
                    out.flags.push(f);
                }
            
            } else if arg == "-" {
                // read in from stdin
                out.operands.push(String::from("-"));
            } 
            else if arg.starts_with("-") {
                // single and potentially combined short flags
                
                for c in arg.chars().skip(1) { // skip the '-'
                    // make sure each flag is a real one
                    let fs = Flag::f_reduce(&format!("-{}", c))
                        .unwrap_or_else(|| panic!("invalid flag '{}' in {}!", c, arg));
                    
                    for f in fs {
                        out.flags.push(f);
                    }
                }
            } else {
                // anything else must be an operand (file, stream, socket, etc)
                out.operands.push(arg);
            }
        }

        if out.operands.is_empty() {
            out.operands.push(String::from("-"));
        }

        out
    }
}

// thank you gipity
impl fmt::Display for RatArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Flags:")?;
        for flag in &self.flags {
            writeln!(f, "  - {:?}", flag)?;
        }

        writeln!(f, "Operands:")?;
        for operand in &self.operands {
            writeln!(f, "  - {}", operand)?;
        }

        Ok(())
    }
}


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Flag {
    NumberNonBlankLines,
    ShowEnds,
    NumberLines,
    SqueezeBlanks,
    ShowTabs,
    ShowNonPrinting,
    Help,
    Version,
}

impl Flag {
    // parse a String flag into a Flag, expanding where possible, returning None when proper
    fn f_reduce(f: &String) -> Option<Vec<Flag>> {
        match f.as_str() {
            "-b" | "--number-nonblank"  => Some(vec![Flag::NumberNonBlankLines]),
            "-E" | "--show-ends"        => Some(vec![Flag::ShowEnds]),
            "-n" | "--number"           => Some(vec![Flag::NumberLines]),
            "-s" | "--squeeze-blank"    => Some(vec![Flag::SqueezeBlanks]),
            "-T" | "--show-tabs"        => Some(vec![Flag::ShowTabs]),
            "-v" | "--show-nonprinting" => Some(vec![Flag::ShowNonPrinting]),
            "-A" | "--show-all"         => Some(vec![
                Flag::ShowNonPrinting,
                Flag::ShowEnds,
                Flag::ShowTabs,
            ]),
            "-e"        => Some(vec![
                Flag::ShowNonPrinting,
                Flag::ShowEnds,
            ]),
            "-t"        => Some(vec![
                Flag::ShowNonPrinting,
                Flag::ShowTabs
            ]),
            "--help"    => Some(vec![Flag::Help]),
            "--version" => Some(vec![Flag::Version]),
            _ => None
        }
    }

}


