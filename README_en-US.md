# maimai-android-touch-panel-rs

English | [简体中文](README.md)

A program that records Android device touch events using `adb shell getevent` and simulates maimai touch panel input (Rust version).

## Notice
This is a small project tested on Xiaomi Pad 5 Pro (Android 13) and Xiaomi Pad 6 Pro (Android 15), and only supports Linux multi-touch protocol Type B.

Known issues:

- Only supports Linux multi-touch protocol Type B, not Type A, which may cause older devices to be unsupported.
  For differences between the two types, see [documentation](https://www.kernel.org/doc/Documentation/input/multi-touch-protocol.txt)
- Touch Keys output but no key press detected (resolution issue)
- Double tap in-game only registers once (program not in running mode)
- Touch always shows as pressed in-game (unknown cause)

Better projects include:

- [KanadeDX](https://github.com/KanadeDX/Public) (Implementation of a certain 8-button program on Android/iOS)
- [AstroDX](https://github.com/2394425147/astrodx) (Android, Windows?)
- [MajdataPlay](https://github.com/LingFeng-bbben/MajdataPlay) (Windows, Android?)

These projects include complete implementations of Mai2 Chart Player, not just a touch input program.

## Usage

1. First, change the value of `DummyTouchPanel` in the game configuration file to `0`
2. Open any image editing tool and prepare an image with the same size as your device screen (e.g., 1600x2560). Place `./image/color_exp_panel.png` at the circular touch area position of the image. Save the edited image to the `image` directory and name it `image_monitor.png`
3. Edit the `config.toml` configuration file. Modify the `[zone_colors]` configuration to change the RGB channel color values of each zone to match your edited image's corresponding zone colors (usually no need to change, default is fine)
4. Install ADB debugging tools on your computer and add the installation path to system environment variables
5. Download the latest `maimai-touch-windows.zip` from the [Releases](../../releases) page and extract it to any directory. Or build from source (install the [Rust toolchain](https://rustup.rs), then run `cargo build --release`)
6. First, fill in the actual screen size into the `monitor_size` configuration in `config.toml`. Open a terminal, run `adb shell getevent -l`, tap the bottom-right corner of the screen, get the `ABS_MT_POSITION_X` and `ABS_MT_POSITION_Y` values from the terminal, convert hexadecimal to decimal, and fill the obtained data into the `input_size` configuration
7. Android devices with charging port facing down are generally in normal screen orientation. If you need to play with reversed screen, change the `reverse_monitor` configuration to `true`
8. Edit the `config.toml` configuration file and modify multiple configurations according to the instructions in the file
9. Download a `VSPD` virtual serial port tool and establish forwarding between `COM3` and `COM33`
10. Enable USB debugging on your phone. Strongly recommend using USB network tethering to connect to the computer, as streaming over WLAN may not be very stable and increase latency
11. You can stream the computer screen to Android devices using software like `Apollo`, `IddSampleDriver`, `Sunshine` and `Moonlight`, or the more convenient but higher latency `spacedesk`. Please find the detailed process on your own, as it's not within the scope of this discussion
12. Connect your phone to the computer, first double-click `start.bat`, then run the game. When the console outputs `Connected to game`, you're ready
13. Adjust the delay in-game. Generally, both judgment A/B need adjustment to work properly. For me, it's `A:-1.0/B:+0.5` to `A:-2.0/B:+2.0`
14. Play a round to see if you're missing any slides / if touch is responsive. Modify the `area_scope` value in `config.toml` based on your experience
15. If single-point latency is low but sliding has extremely high latency, change `sleep_mode` in `config.toml` to `false`, or you can decrease the value of `sleep_delay_us` (if it's still laggy, please submit an issue for feedback)

## Command List

If accidentally disconnected during gameplay, enter `start` in the console and press Enter to reconnect to the game

Enter `reverse` to adjust the touch device screen orientation

Enter `restart` to reload the configuration file/restart the program

Enter `exit` to fully exit the program (you can also use Ctrl+C)

## Building from Source

Install the [Rust toolchain](https://rustup.rs), then run from the project directory:

```bash
cargo build --release
```

The compiled executable will be at `target/release/maimai-touch-rs.exe` (Windows).

## Common Issues

For delay/other suggestions, refer to [#3](https://github.com/ERR0RPR0MPT/maimai-android-touch-panel/issues/3)

Q: On higher Android versions (13-15), the touch area is completely misaligned. Only tapping the top-left corner of the screen works. The image uses the tablet's actual resolution. Testing on an Android 10 device works normally.

A: Follow the steps to modify the `monitor_size` and `input_size` configurations in `config.toml`

Q: Error when closing and reopening

A: Directly closing the console window may cause background processes to remain. Please use Ctrl+C to terminate the program or enter `exit` before exiting.

## Notes

To add 2P, copy the program again and add forwarding from COM4 to COM44. In the configuration file `config.toml`, specify the device serial number obtained using `adb devices` in the `specified_device` field

This program is for testing purposes only. Currently, playing 12-13 difficulty is feasible and need better optimization in the future.

## License
