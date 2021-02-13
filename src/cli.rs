use crate::get_and_parse::GlitterRc;
use crate::logger;
use std::env;

fn help() {
    println!(
        "
    See https://github.com/Milo123459/glitter for information.
    Make sure to star the repo if you like Glitter!
    "
    );
}

fn push(config: GlitterRc, args: Vec<String>) {
    let splitted = config.commit_message.split('$').skip(1);
    let mut result = String::from(&config.commit_message);
    for val in splitted {
        if !String::from(val).starts_with("RAW_GIT_PARAMS") {
            let val_ = &args[val.split("").nth(1).unwrap().parse::<usize>().unwrap()];
            result = result.replace(
                &format!("${}", String::from(val).split("").collect::<Vec<_>>()[1]),
                &val_,
            );
        } else {
            result = args.join(" ");
            logger::info("Using commit message provided. No template found.");
            return;
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
pub fn cli(config: std::result::Result<GlitterRc, serde_json::Error>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let conf = config.unwrap();

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
}
