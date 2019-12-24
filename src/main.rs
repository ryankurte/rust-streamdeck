
#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::{TermLogger, LevelFilter, TerminalMode};

extern crate structopt;
use structopt::StructOpt;

extern crate humantime;
use humantime::Duration;

use streamdeck::{StreamDeck, Filter, Error};

#[derive(StructOpt)]
#[structopt(name = "streamdeck-cli", about = "A CLI for the Elgato StreamDeck")]
struct Options {

    #[structopt(subcommand)]
    cmd: Commands,

    #[structopt(flatten)]
    filter: Filter,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

#[derive(StructOpt)]
pub enum Commands {
    /// Reset the attached device
    Reset,
    /// Fetch the device firmware version
    Version,
    /// Set device display brightness
    SetBrightness{
        /// Brightness value from 0 to 100
        brightness: u8,
    },
    /// Fetch button states
    GetButtons {
        #[structopt(long)]
        /// Timeout for button reading
        timeout: Option<Duration>,

        #[structopt(long)]
        /// Read continuously
        continuous: bool,
    },
    /// Set button colours
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
    let mut config = simplelog::ConfigBuilder::new();
    config.set_time_level(LevelFilter::Off);

    TermLogger::init(opts.level, config.build(), TerminalMode::Mixed).unwrap();

    // Connect to device
    let mut deck = match StreamDeck::connect(opts.filter.vid, opts.filter.pid, opts.filter.serial) {
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
