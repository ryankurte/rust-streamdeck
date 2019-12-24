
#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::{TermLogger, LevelFilter, TerminalMode};

extern crate structopt;
use structopt::StructOpt;

extern crate humantime;
use humantime::Duration;

use streamdeck::{StreamDeck, Error};

#[derive(StructOpt)]
#[structopt(name = "streamdeck-cli", about = "A CLI for the Elgato StreamDeck")]
struct Options {

    #[structopt(subcommand)]
    cmd: Commands,

    #[structopt(long, default_value="0fd9", parse(try_from_str=u16_parse_hex))]
    /// USB Device Vendor ID (VID) in hex
    vid: u16,

    #[structopt(long, default_value="0063", parse(try_from_str=u16_parse_hex))]
    /// USB Device Product ID (PID) in hex
    pid: u16,

    #[structopt(long)]
    /// USB Device Serial
    serial: Option<String>,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

fn u16_parse_hex(s: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(s, 16)
}

#[derive(StructOpt)]

pub enum Commands {
    Reset,
    Version,
    SetBrightness{
        /// Brightness value from 0 to 100
        brightness: u8,
    },
    GetButtons {
        #[structopt(long)]
        /// Timeout for button reading
        timeout: Option<Duration>,

        #[structopt(long)]
        /// Read continuously
        continuous: bool,
    },
    SetColour {
        /// Index of button to be set
        key: u8,

        #[structopt(short, long, default_value="0")]
        /// Red channel
        red: u8,

        #[structopt(short, long, default_value="0")]
        /// Blue channel
        blue: u8,

        #[structopt(short, long, default_value="0")]
        /// Green channel
        green: u8,
    }
}

fn main() {
    // Parse options
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(opts.level, simplelog::Config::default(), TerminalMode::Mixed).unwrap();

    // Connect to device
    let mut deck = match StreamDeck::connect(opts.vid, opts.pid, opts.serial) {
        Ok(d) => d,
        Err(e) => {
            error!("Error connecting to streamdeck: {:?}", e);
            return
        }
    };

    let serial = deck.serial().unwrap();
    info!("Connected to device (vid: {:04x} pid: {:04x} serial: {})", opts.vid, opts.pid, serial);

    // Run the command
    if let Err(e) = do_command(&mut deck, opts.cmd) {
        error!("Command error: {:?}", e);
    }
}

fn do_command(deck: &mut StreamDeck, cmd: Commands) -> Result<(), Error> {
    match cmd {
        Commands::Reset => {
            deck.reset()?;
        },
        Commands::Version => {
            let version = deck.version()?;
            info!("Firmware version: {}", version);
        }
        Commands::SetBrightness{brightness} => {
            deck.set_brightness(brightness)?;
        },
        Commands::GetButtons{timeout, continuous} => {
            loop {
                let buttons = deck.read_buttons(timeout.map(|t| *t ))?;
                info!("buttons: {:?}", buttons);

                if !continuous {
                    break
                }
            }
        },
        Commands::SetColour{key, red, green, blue} => {
            info!("Setting key {} colour to: (r: {} g: {} b: {})", key, red, green, blue);
            deck.set_button_rgb(key, red, green, blue)?;
        }
    }

    Ok(())
}
