use crate::get_and_parse::GlitterRc;
use crate::logger;
use std::env;
use std::result::Result;
use std::convert::TryInto;

fn help() {
    println!(
        "
    See https://github.com/Milo123459/glitter for information.
    Make sure to star the repo if you like Glitter!
    "
    );
}

fn push(config: GlitterRc, args: Vec<String>) {
    if config.commit_message == "$RAW_COMMIT_MSG" {
        logger::error("No template provided. A template has to be provided for Glitter to run the command push.");
        std::process::exit(1);
    }

    let splitted = config.commit_message.split('$').skip(1);

    let mut result = String::from(&config.commit_message);


    for val in splitted {
        if val.len() >= 2 && String::from(val.chars().nth(1).unwrap()) == String::from("+") {
            let idx = val.chars().nth(0).unwrap().to_digit(10).unwrap();
            let rest =  &args[idx as usize..];
            result = result.replace(
                &format!("${}+", String::from(val).split("").collect::<Vec<_>>()[1]),
                &rest.join(" ")
            );
        
        } else {
            let val_ = &args[val.split("").nth(1).unwrap().parse::<usize>().unwrap()];
            result = result.replace(
                &format!("${}", String::from(val).split("").collect::<Vec<_>>()[1]),
                &val_,
            );
        }
    }

    println!("{}", result);
}

fn match_cmds(args: Vec<String>, config: GlitterRc) {
    let cmd = &args[0];
    match &*cmd.to_lowercase() {
        "push" => push(config, args),
        _ => help(),
    }
}

pub fn cli(config: Result<GlitterRc, serde_json::Error>) {
    let args: Vec<String> = env::args().skip(1).collect();

    match config {
        Ok(conf) => {
            match args.len() {
                1 => match_cmds(args, conf),
                _ => {
                    if args.len() > 1 {
                        match_cmds(args, conf);
                    } else {
                        help()
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("we messed up :(. Err category: {:?}. line {} column {}", e.classify(), e.line(), e.column());
            std::process::exit(1)
        }
    }

}
