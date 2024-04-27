use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::ops::Mul;

trait Command {
    fn inc() -> Self;
}
fn read_brightness_file(file: File) -> Result<i32, std::io::Error> {
    let line = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap();
    Ok(line.parse::<i32>().unwrap())
}

fn main() -> std::io::Result<()> {
    let file_path = "/sys/class/backlight/intel_backlight/brightness";
    let max_brightness_file_path = "/sys/class/backlight/intel_backlight/max_brightness";
    let current_brightness_file = File::open(file_path)?;
    let max_brightness_file = File::open(max_brightness_file_path)?;
    let max_brightness = read_brightness_file(max_brightness_file)?;
    let current_brightness = read_brightness_file(current_brightness_file)?;
    let current_brightness_percent =
        ((current_brightness as f32 / max_brightness as f32).mul(100.0)).round() as i32;
    let args: Vec<String> = env::args().collect();
    if args[1].eq("-inc") {
        let mut f = File::create(file_path)?;
        let brightness_perc = min(current_brightness_percent + 1, 100);
        let brightness = (brightness_perc * max_brightness) / 100;
        f.write_all(brightness.to_string().as_bytes())?;
    } else if args[1].eq("-dec") {
        let mut f = File::create(file_path)?;
        let brightness_perc = max(current_brightness_percent - 1, 0);
        let brightness = (brightness_perc * max_brightness) / 100;
        f.write_all(brightness.to_string().as_bytes())?;
    } else if args[1].eq("-get") {
        print!("{}", current_brightness_percent);
    }
    Ok(())
}
