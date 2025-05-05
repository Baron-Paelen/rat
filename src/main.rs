use std::{env, error::Error, fs, io::{self, Write}, path::PathBuf};
mod args;


fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let args: args::RatArgs = args::RatArgs::from(raw_args);

    if args.flags.contains(&args::Flag::Help) {
        args::RatArgs::display_help();
        return Ok(());
    }
    if args.flags.contains(&args::Flag::Version) {
        println!("0.0.1");
        return Ok(());
    }
    
    // loop through each operand
    'big: for op in args.operands {
        if &op.to_string() == "-" {
            // handle stdin
            loop {
                match read_stdin() {
                    Ok(x) => {
                        let (contents, n) = x;
                        if n == 0 {
                            continue 'big;
                        }
                        print!("{}", contents);
                        io::stdout().flush().unwrap();
                    },
                    Err(_) => panic!("rat: -: Input/output error"),
                }
            }
            
        } else if PathBuf::from(&op).is_file() {
            // handle files
            let mut s_out = read_file(&op)?;
            apply_flag_formatting(&mut s_out, &args.flags);
            print!("{}", s_out);
            io::stdout().flush().unwrap();
        } else {
            panic!("unsupported operand type!")
        }
        
    }

    Ok(())
}

// unbuffered text reader
fn read_file(p: &String) -> Result<String, &str> {
    match fs::read_to_string(p) {
        Ok(v) => Ok(v),
        Err(_) => Err("error in read_file()!"),
    }
}

// slightly buffered stdin reader
fn read_stdin() -> Result<(String, usize), &'static str> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(n) => Ok((String::from(buf), n)),
        Err(_) => Err("error in read_stdin()!"),
    }
}

fn apply_flag_formatting(s: &mut String, flags: &Vec<args::Flag>) {
    if flags.contains(&args::Flag::ShowTabs) {
        *s = s.replace("\t", "^I");
    }
    if flags.contains(&args::Flag::SqueezeBlanks) {
        let indices: Vec<usize> = s.rmatch_indices('\n').map(|(i, _)| i).collect();
        let mut prev_match = *indices.first().unwrap();
        let mut cons_nl: usize = 0;

        for i in indices {
            // println!("{} - {}", prev_match, i);
            if prev_match-i == 1 {
                cons_nl += 1;
                // s.replace_range(i..prev_match, "");
            } else if cons_nl > 1{
                // println!("\treplacing {}-{} with nothing", i+1, i+cons_nl);
                s.replace_range(prev_match..prev_match+cons_nl, "\n");
                cons_nl = 0;
            } else {
                cons_nl = 0;
            }
            prev_match = i;
        }
    }
    if flags.contains(&args::Flag::ShowEnds) {
        let indices: Vec<usize> = s.rmatch_indices('\n').map(|(i, _)| i).collect();
        for i in indices {
            s.insert(i, '$');
        }
    }
    if flags.contains(&args::Flag::ShowNonPrinting) {
        let mut out = String::new();

        // byte by byte for ASCII printable and nonprintables
        for byte in s.bytes() {
            match byte {
                0x0A => out.push('\n'),
                0x00..=0x1F => out.push_str(&format!("^{}", (byte + 0x40) as char)),
                0x7F => out.push_str("^?"), // DEL
                0x20..=0x7E => out.push(byte as char), //
                0x80..=0xFF => {
                    let printable = byte & 0x7F; // clears the leftmost/high bit. converts non ASCII into representative ASCII char
                    let display = match printable {
                        0x00..=0x1F => format!("M-^{}", (printable + 0x40) as char),
                        0x7F => "M-^?".to_string(), // DEL
                        _ => format!("M-{}", printable as char),
                    };
                    out.push_str(&display);
                }
            }
        }
        
        *s = out;
    }

    if flags.contains(&args::Flag::NumberNonBlankLines) {
        let lines = s.lines();
        let mut out:String = String::new();
        let mut line_num = 1;

        for l in lines {
            if l == "$" && flags.contains(&args::Flag::ShowEnds) { // this is to match cat's behavior when -b and -E are used
                out.push_str("$\n");
            } else if !l.is_empty() {
                // construct a string with the appropriate number of spaces
                let line_num_char_len = line_num.to_string().len();
                let num_spaces = 6 - line_num_char_len;
                let mut spaces = String::with_capacity(num_spaces);
                for _ in 0..num_spaces {
                    spaces.push(' ');
                }
    
                out.push_str(&format!("{}{}\t{}\n", spaces.as_str(), line_num, l));
                line_num += 1;
            } else {
                out.push_str("\n");
            }

        }

        *s = out;
    } 
    if flags.contains(&args::Flag::NumberLines) && !flags.contains(&args::Flag::NumberNonBlankLines) { // NNBL overrides NL
        let indices: Vec<usize> = s.rmatch_indices('\n').map(|(i, _)| i).collect();

        // this loop adds line numbers in reverse
        for (i, c_lc) in indices.iter().skip(1).enumerate() {
            let line_num = indices.len() - i;
            let line_num_char_len = line_num.to_string().len();
            let num_spaces = 6 - line_num_char_len;

            // construct a string with the appropriate number of spaces
            let mut spaces = String::with_capacity(num_spaces);
            for _ in 0..num_spaces {
                spaces.push(' ');
            }
            let line_num_form = format!("{}{}\t", spaces, line_num);
            s.insert_str(*c_lc+1, &line_num_form);
        }
        s.insert_str(0, "     1\t"); // add first line's line number
    }
}

