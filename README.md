# RustyChip

This is a simple Chip8 emulator written in Rust for my VSB-TUO Rust course.\
Runs basic Chip-8 programs. Chip-48 and SuperChip roms will be implemented later.\

## Features

- [x] Basic Chip-8 emulation
- [x] Display using `raylib`
- [x] Command line arguments
    - [x] CPU clock speed and timers rate configuration
    - [x] Window size configuration
    - [x] Foreground and background color configuration
    - [x] ROM loading
- [ ] Chip-48 and SuperChip support
- [ ] Keybindings configuration
- [ ] Sound
- [ ] Args improvements

## Usage

To run RustyChip, use the following command:

```bash
cargo run --release -- [OPTIONS]
```

For example:

```bash
cargo run --release -- -r <path_to_rom>
```

To build the project, use the following command:

```bash
cargo build --release
```

To see all available options, use the `--help` flag:

```
Usage: RustyChip [OPTIONS]

Options:
  -r, --rom <ROM>                  ROM file to load
  -w, --width <WIDTH>              Window width in pixels, default = 640. Ff not set to multiples of 64, it will be rounded down
      --height <HEIGHT>            Window size in pixels, default = 320. It's enough to set only one of the dimensions, the other will be calculated automatically. If not set to multiples of 32, it will be rounded down
  -f, --fg-color <FG_COLOR>        Foreground color. The background color will be the opposite of this color [default: 0xFFFFFF]
  -b, --bg-color <BG_COLOR>        Background color. Default = inverse of fg_color
  -c, --cpu-clock <CPU_CLOCK>      CPU Clock speed (Hz) [default: 500]
      --timers-rate <TIMERS_RATE>  Timers rate (Hz). CPU to timers rate should be 500/60
  -z, --swap-yz                    Swap Y and Z keys
  -h, --help                       Print help
```

### ROMs

Place your ROMs in the `rom` directory.\
The emulator will look for the ROMs both in your working directory and in the `rom` directory by default (you don't need
to include
`rom/` to the path specification.\
It is recommended for the ROMs to have the `.ch8` extension, but you don't need to type the `.ch8` extension, the
emulator will try to find the file with and without it.

Keep in mind that there is no support for Chip-48 and SuperChip ROMs yet - there might be some weird behavior or crashes
when running them.

### Window size

You can adjust the window size. In ~~most~~ all cases, you probably want to set just the width or the height. The other
will be calculated automatically. _(If you want some obscure window size, Feel free to set both dimensions)_

The window size should be set to multiples of 64x32. You don't need to worry about it - you can set any size and it will
be rounded down to the nearest multiple ...unless you explicitly set both dimensions.

### Colors

You can adjust the foreground and background colors. If you set just the foreground color, the background color will be
the opposite of the foreground color.\
The colors should be set in hexadecimal format (e.g. `0xAABBCC`).\
_If you set the colors to the same value, you won't see anything :)_

### Timers

Adjust the CPU clock speed to your liking - each game may require a different speed to feel right.\
Timers rate will be adjusted automatically, you do NOT need to set them manually.\
The default CPU clock speed is 500 Hz. The timers rate should be 60 Hz.\
You should not change the timers rate unless you want some weird behavior.

