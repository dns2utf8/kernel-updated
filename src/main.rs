extern crate notify_rust;
extern crate rustc_serialize;
extern crate docopt;


use std::process::Command;

use notify_rust::Notification;
use docopt::Docopt;

/// Parse the output of `pacman -Q linux`
fn parse_pacman_output(pacman_ouput: &str) -> Option<&str> {
    pacman_ouput.split_whitespace()
        .skip(1)
        .next()
}

/// Parse the output of `uname -r`
fn parse_uname_output(uname_output: &str) -> Option<&str> {
    uname_output.split("-ARCH")
        .next()
}

const USAGE: &'static str = "
Check the currently installed kernel against the currently running one.

Usage:
  kernel-updated
  kernel-updated (-h | --help)
  kernel-updated --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("kernel-updated: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    let output_pacman = Command::new("pacman")
        .arg("-Q")
        .arg("linux")
        .output()
        .expect("Could not execute pacman");
    // pacman output is in the form "linux version"
    let output_pacman = String::from_utf8_lossy(&output_pacman.stdout);
    let output_pacman = parse_pacman_output(&output_pacman)
        .expect("Could not parse pacman output");

    // uname output is in the form version-ARCH
    let output_uname = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Could not execute uname");
    let output_uname = String::from_utf8_lossy(&output_uname.stdout);
    let output_uname = parse_uname_output(&output_uname)
        .expect("Could not parse uname output");

    println!("installed: {}", output_pacman);
    println!("running:   {}", output_uname);

    let should_reboot = output_pacman != output_uname;

    if should_reboot {
        println!("You should reboot your system!");
        Notification::new()
            .summary("Reboot needed")
            .body("Kernel got updated! You should reboot your system!")
            .timeout(6000) //milliseconds
            .show().unwrap();
    }
}


#[cfg(test)]
mod test {
    use super::{parse_pacman_output, parse_uname_output};

    #[test]
    fn test_parse_pacman_output() {
        assert_eq!(Some("4.5.4-1"), parse_pacman_output("linux 4.5.4-1"));
    }

    #[test]
    fn test_parse_uname_output() {
        assert_eq!(Some("4.5.4-1"), parse_uname_output("4.5.4-1-ARCH"));
    }
}

