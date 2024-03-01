use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

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
    Ok(i32::from_str_radix(&line, 10).unwrap())
}
fn main() -> std::io::Result<()> {
    let file_path = "/sys/class/backlight/intel_backlight/brightness";
    let max_brightness_file_path = "/sys/class/backlight/intel_backlight/max_brightness";
    let current_brightness_file = File::open(file_path)?;
    let max_brightness_file = File::open(max_brightness_file_path)?;
    let max_brightness = read_brightness_file(max_brightness_file)?;
    let mut current_brightness = read_brightness_file(current_brightness_file)?;
    let args: Vec<String> = env::args().collect();
    if args[1].eq("-inc") {
        current_brightness = min(
            ((1 as f32 + args[2].parse::<f32>().unwrap() / (100 as f32))
                * current_brightness as f32) as i32,
            max_brightness,
        );
        let mut f = File::create(file_path)?;
        f.write_all(current_brightness.to_string().as_bytes())?;
    } else if args[1].eq("-dec") {
        current_brightness = Ord::max(
            ((1 as f32 - args[2].parse::<f32>().unwrap() / (100 as f32))
                * current_brightness as f32) as i32,
            0,
        );
        let mut f = File::create(file_path)?;
        f.write_all(current_brightness.to_string().as_bytes())?;
    } else if args[1].eq("-get") {
        println!(
            "{}",
            ((current_brightness as f32 / max_brightness as f32) * 100.0) as i32
        );
    }
    Ok({})
}
