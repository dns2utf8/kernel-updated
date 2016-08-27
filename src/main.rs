extern crate notify_rust;
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

use notify_rust::Notification;

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

fn main() {
    // Every 10 minutes
    let check_interval = Duration::from_secs(600);

    loop {
        check_system();
        sleep(check_interval);
    }
}

fn check_system() {
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


    if output_pacman != output_uname {
        println!("You should reboot your system!");
        Notification::new()
            .summary("Reboot needed")
            .body("Kernel was updated! You should reboot your system!")
            .timeout(0) //milliseconds
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

