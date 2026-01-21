use std::env;

fn main() {
    let mut args = env::args();
    let exe = args.next().unwrap_or_else(|| "wc3".to_string());

    let Some(cmd) = args.next() else {
        print_help(&exe);
        return;
    };

    match cmd.as_str() {
        "-h" | "--help" | "help" => print_help(&exe),
        "--version" | "version" => {
            println!("wc3 {}", env!("CARGO_PKG_VERSION"));
        }
        "status" => {
            // Stub: later this will talk to the injector/shim over IPC.
            println!("status: not connected (scaffold)");
        }
        "resources" => {
            println!("Reference repos live in resources/README.md");
        }
        other => {
            eprintln!("Unknown command: {other}\n");
            print_help(&exe);
            std::process::exit(2);
        }
    }
}

fn print_help(exe: &str) {
    println!(
        "\
{exe} - Warcraft III CLI (scaffold)

USAGE:
  {exe} <command>

COMMANDS:
  status         Show connection + target info (stub)
  resources      Show where reference repos are listed
  version        Print version
  help           Print this help
"
    );
}

