use std::env;

fn main() {
    let mut args = env::args();
    let exe = args.next().unwrap_or_else(|| "wc3-injector".to_string());

    let Some(cmd) = args.next() else {
        print_help(&exe);
        return;
    };

    match cmd.as_str() {
        "-h" | "--help" | "help" => print_help(&exe),
        "plan" => {
            println!(
                "\
Milestone plan (high level):
  1) Identify wc3.exe target + hash
  2) Launch under Wine (windowed, deterministic)
  3) Load shim DLL (proxy DLL or remote injection)
  4) Establish local IPC handshake
  5) Add first read-only state probe
  6) Add first safe command (e.g. move)
"
            );
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
{exe} - host-side launcher/injection helper (scaffold)

USAGE:
  {exe} <command>

COMMANDS:
  plan           Print next steps for first milestone
  help           Print this help
"
    );
}

