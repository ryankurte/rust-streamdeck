#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use clap::{Parser, Subcommand};

extern crate humantime;
use humantime::Duration;

use streamdeck::{Colour, Error, Filter, ImageOptions, StreamDeck};

#[derive(Parser)]
#[command(name = "streamdeck-cli", about = "A CLI for the Elgato StreamDeck")]
struct Options {
    #[command(subcommand)]
    cmd: Commands,

    #[command(flatten)]
    filter: Filter,

    #[arg(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Reset the attached device
    Reset,
    /// Fetch the device firmware version
    Version,
    /// Search for connected streamdecks
    Probe,
    /// Set device display brightness
    SetBrightness {
        /// Brightness value from 0 to 100
        brightness: u8,
    },
    /// Fetch button states
    GetButtons {
        #[arg(long)]
        /// Timeout for button reading
        timeout: Option<Duration>,

        #[arg(long)]
        /// Read continuously
        continuous: bool,
    },
    /// Set button colours
    SetColour {
        /// Index of button to be set
        key: u8,

        #[command(flatten)]
        colour: Colour,
    },
    /// Set button images
    SetImage {
        /// Index of button to be set
        key: u8,

        /// Image file to be loaded
        file: String,

        #[command(flatten)]
        opts: ImageOptions,
    },
}

fn main() {
    // Parse options
    let opts = Options::parse();

    // Setup logging
    let mut config = simplelog::ConfigBuilder::new();
    config.set_time_level(LevelFilter::Off);

    TermLogger::init(
        opts.level,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    // Connect to device
    let mut deck = match StreamDeck::connect(opts.filter.vid, opts.filter.pid, opts.filter.serial) {
        Ok(d) => d,
        Err(e) => {
            error!("Error connecting to streamdeck: {:?}", e);
            return;
        }
    };

    let serial = deck.serial().unwrap();
    info!(
        "Connected to device (vid: {:04x} pid: {:04x} serial: {})",
        opts.filter.vid, opts.filter.pid, serial
    );

    // Run the command
    if let Err(e) = do_command(&mut deck, opts.cmd) {
        error!("Command error: {:?}", e);
    }
}

fn do_command(deck: &mut StreamDeck, cmd: Commands) -> Result<(), Error> {
    match cmd {
        Commands::Reset => {
            deck.reset()?;
        }
        Commands::Version => {
            let version = deck.version()?;
            info!("Firmware version: {}", version);
        }
        Commands::SetBrightness { brightness } => {
            deck.set_brightness(brightness)?;
        }
        Commands::GetButtons {
            timeout,
            continuous,
        } => loop {
            let buttons = deck.read_buttons(timeout.map(|t| *t))?;
            info!("buttons: {:?}", buttons);

            if !continuous {
                break;
            }
        },
        Commands::Probe => {
            let results = StreamDeck::probe()?;
            if results.is_empty() {
                info!("No devices found");
                return Ok(());
            }
            info!("Found {} devices", results.len());
            for res in results {
                match res {
                    Ok((device, pid)) => info!("Streamdeck: {:?} (pid: {:#x})", device, pid),
                    Err(_) => warn!("Found Elgato device with unsupported PID"),
                }
            }
        }
        Commands::SetColour { key, colour } => {
            info!("Setting key {} colour to: ({:?})", key, colour);
            deck.set_button_rgb(key, &colour)?;
        }
        Commands::SetImage { key, file, opts } => {
            info!("Setting key {} to image: {}", key, file);
            deck.set_button_file(key, &file, &opts)?;
        }
    }

    Ok(())
}
