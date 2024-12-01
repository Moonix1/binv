use std::{env::args, fs::File, io::Read, process::exit};

use log::{error, info};

mod viewer;
mod logb;

use viewer::*;
use logb::*;

fn shift(args: &mut Vec<String>) -> String {
    args.reverse();
    let a = args.pop().unwrap();
    args.reverse();

    a
}

fn usage(program: &String) {
    info!("usage: {} <ARGS>", program);
    println!("ARGS:");
    println!("   - h  <INPUT> : displays the file bytes in hex");
    println!("   - ha <INPUT> : displays the file bytes in hex and ascii alpha");
    println!();
    println!("   - b  <INPUT> : displays the file bytes in binary");
    println!("   - ba <INPUT> : displays the file bytes in binary and ascii alpha");
    println!();
    println!("   - help       : displays this message");
}

fn main() {
    Log::init();

    let mut args: Vec<String> = args().collect();

    let program = shift(&mut args);

    if args.len() < 1 {
        usage(&program);
        error!("expected argument[s]!");
        exit(1);
    }

    let mut line_size = 10;
    let mut buffer = Vec::<u8>::new();

    let mut vmode = 0;
    let mut file: Option<File> = None;
    while args.len() > 0 {
        let flag = shift(&mut args);

        match flag.as_str() {
            "-s" => {
                let line_s = shift(&mut args);
                match line_s.parse() {
                    Ok(val) => {
                        line_size = val;
                    },

                    Err(_) => {
                        error!("`{}` expected a number!", flag);
                        exit(1);
                    }
                }
            },
            
            "-h" => {
                let input = shift(&mut args);
                file = Some(match File::open(input.clone()) {
                    Ok(file) => file,
                    Err(_) => {
                        error!("invalid file path `{}`", input);
                        exit(1);
                    }
                });

                vmode = 1;
            },
            "-ha" => {
                let input = shift(&mut args);
                file = Some(match File::open(input.clone()) {
                    Ok(file) => file,
                    Err(_) => {
                        error!("invalid file path `{}`", input);
                        exit(1);
                    }
                });

                vmode = 2;
            },
            "-b" => {
                let input = shift(&mut args);
                file = Some(match File::open(input.clone()) {
                    Ok(file) => file,
                    Err(_) => {
                        error!("invalid file path `{}`", input);
                        exit(1);
                    }
                });

                vmode = 3;
            },
            "-ba" => {
                let input = shift(&mut args);
                file = Some(match File::open(input.clone()) {
                    Ok(file) => file,
                    Err(_) => {
                        error!("invalid file path `{}`", input);
                        exit(1);
                    }
                });

                vmode = 4;
            },

            "--help" => {
                usage(&program);
                exit(0);
            },

            _ => {
                usage(&program);
                error!("invalid flag `{flag}`");
                exit(1);
            }
        }
    }

    match vmode {
        1 => {
            file.unwrap().read_to_end(&mut buffer).unwrap();
            Viewer::dump_hex(buffer, line_size);
        },
        2 => {
            file.unwrap().read_to_end(&mut buffer).unwrap();
            Viewer::dump_hex_a(buffer, line_size);
        },
        3 => {
            file.unwrap().read_to_end(&mut buffer).unwrap();
            Viewer::dump_binary(buffer, line_size);
        },
        4 => {
            file.unwrap().read_to_end(&mut buffer).unwrap();
            Viewer::dump_binary_a(buffer, line_size);
        },

        _ => {
            error!("\n{} {} {}\n{}\n{}",
            "viewmode", vmode, "not found, if you're getting",
            "this error then its probably an internal",
            "issue that isn't your fault!");
        }
    }
}
