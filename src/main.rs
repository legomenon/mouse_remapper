use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use sudo::RunningAs;

const DEVICE_EVENT: &str = "type 2 (EV_REL), code 6 (REL_HWHEEL), value";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if sudo::check() != RunningAs::Root {
        return match sudo::escalate_if_needed() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    let child = Command::new("evtest")
        .arg("/dev/input/event6")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut last_exec = 0.0;
    if let Some(stdout) = child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            if line.contains(DEVICE_EVENT) {
                let (time, val) = parse(&line)?;
                if (time - last_exec) < 0.15 {
                    continue;
                }
                last_exec = time;
                execute_command(val);
            }
        }
    }
    Ok(())
}

fn parse(s: &str) -> Result<(f64, i32), Box<dyn std::error::Error>> {
    let line = s.split_whitespace().collect::<Vec<_>>();
    let val = line[10].parse::<i32>()?;
    let time = line[2].replace(',', "");
    let time = time.parse::<f64>()?;

    Ok((time, val))
}

fn execute_pageup() {
    Command::new("sh")
        .arg("sudo")
        .arg("-c")
        .arg(
            r#"
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_LEFTCTRL --value 1 --sync;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_PAGEUP --value 1 --sync ;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_PAGEUP --value 0 --sync;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_LEFTCTRL --value 0 --sync
            "#,
        )
        .output()
        .expect("Failed to execute evemu-event");
}

fn execute_pagedown() {
    Command::new("sh")
        .arg("-c")
        .arg(
            r#"
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_LEFTCTRL --value 1 --sync;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_PAGEDOWN --value 1 --sync;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_PAGEDOWN --value 0 --sync;
            evemu-event /dev/input/event5 --type EV_KEY --code KEY_LEFTCTRL --value 0 --sync
            "#,
        )
        .output()
        .expect("Failed to execute evemu-event");
}

fn execute_command(i: i32) {
    if i < 0 {
        execute_pageup()
    } else {
        execute_pagedown()
    }
}
