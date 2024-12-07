mod chip8;

use clap::Parser;
use std::ptr::NonNull;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// ROM file to load
    #[clap(short, long)]
    rom: Option<String>,

    /// Window width in pixels, default = 640. Ff not set to multiples of 64, it will be rounded down.
    #[clap(short, long)]
    width: Option<u32>,

    ///  Window size in pixels, default = 320. It's enough to set only one of the dimensions, the other will be calculated automatically.If not set to multiples of 32, it will be rounded down.
    #[clap(long)]
    height: Option<u32>,

    /// Foreground color. The background color will be the opposite of this color.
    #[clap(short, long, default_value = "0xFFFFFF")]
    fg_color: String,

    /// Background color. Default = inverse of fg_color
    #[clap(short, long)]
    bg_color: Option<String>,

    /// CPU Clock speed (Hz)
    #[clap(short, long, default_value = "500")]
    cpu_clock: u32,

    /// Timers rate (Hz). CPU to timers rate should be 500/60
    #[clap(short, long)]
    timers_rate: Option<u32>,
}

fn args_to_settings(args: Args) -> chip8::ChipSettings {
    let size: (u32, u32) = match (args.width, args.height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (width, 0),
        (None, Some(height)) => (0, height),
        (None, None) => (0, 0),
    };
    let fg_color = u32::from_str_radix(&args.fg_color[2..], 16).expect("Invalid foreground color");
    let bg_color = match args.bg_color {
        Some(bg_color) => {
            u32::from_str_radix(&bg_color[2..], 16).expect("Invalid background color")
        }
        None => !fg_color,
    };
    let cpu_rate = args.cpu_clock as u64;
    let timers_rate = match args.timers_rate {
        Some(rate) => {
            if rate == 0 {
                (cpu_rate as f32 / 500f32 * 60f32) as u64
            }
            else {
                rate as u64
            }
        }
        None => (cpu_rate as f32 / 500f32 * 60f32) as u64,
    };
    let beep = true;
    let rom = match args.rom {
        Some(rom) => {
            let roms_to_try = vec![
                rom.clone(),
                format!("{}.ch8", rom),
                format!("rom/{}", rom),
                format!("rom/{}.ch8", rom),
            ];
            let mut found_rom = String::new();
            for rom in roms_to_try {
                if std::path::Path::new(&rom).exists() {
                    found_rom = rom;
                    break;
                }
            }
            if found_rom.is_empty() {
                log::error!("ROM file not found: {}", rom);
                println!("ROM file not found: {}", rom);
                std::process::exit(1);
            }
            found_rom
        }
        None => {
            log::error!("No ROM file provided");
            println!("No ROM file provided");
            std::process::exit(1);
        }
    };

    chip8::ChipSettings::new(rom, size, cpu_rate, timers_rate, (fg_color, bg_color), beep)
}

// 6377
fn main() {
    let args = Args::parse();

    println!("Hello, Chip-8!");

    let mut chip8 = chip8::Chip8::new(args_to_settings(args));
    chip8.run();
}
