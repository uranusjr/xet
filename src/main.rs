fn run(cmd: &str, args: Vec<String>, envs: Vec<(String, String)>) -> i32 {
    let result = std::process::Command::new(cmd)
        .args(args)
        .envs(envs)
        .status()
        .unwrap_or_else(|_| panic!("failed to run {}", cmd));

    // TODO: None means a signal. Extract with std::os::unix.
    result.code().unwrap_or(-1)
}

fn main() {
    let mut envs: Vec<(String, String)> = Vec::new();

    let mut args_iter = std::env::args().skip(1);
    while let Some(arg) = args_iter.next() {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        match parts.len() {
            1 => { std::process::exit(run(&arg, args_iter.collect(), envs)) },
            2 => { envs.push((parts[0].to_string(), parts[1].to_string())); },
            _ => { panic!("split by = does not return 1 or 2 parts"); },
        }
    }
}
