# Rust Elgato StreamDeck Driver and Utility

An [hidapi](https://crates.io/crates/hidapi) based driver for direct interaction with Elgato StreamDeck devices, this is intended to allow applications to use these devices directly and on arbitrary platforms (without the use of the Elgato SDK).


## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-streamdeck.svg)](https://github.com/ryankurte/rust-streamdeck)
[![Travis Build Status](https://travis-ci.org/ryankurte/rust-streamdeck.svg?branch=master)](https://travis-ci.org/ryankurte/rust-streamdeck)
[![Crates.io](https://img.shields.io/crates/v/streamdeck.svg)](https://crates.io/crates/streamdeck)
[![Docs.rs](https://docs.rs/streamdeck/badge.svg)](https://docs.rs/streamdeck)

WIP. Pull requests more than welcome!

Features:

- [ ] Connecting to devices
  - [x] Connecting by VID/PID/Serial
  - [ ] Matching device _types_ (Mini etc.)
- [ ] Reading buttons
  - [x] Poll based mode (w/ blocking / non-blocking selection and timeouts)
  - [ ] Multi-threaded / async / callback driven mode
- [x] Writing brightness
- [ ] Setting buttons
  - [x] Writing colours
  - [ ] Writing images (untested)
- [ ] Devices
  - [x] Stream Deck Mini
  - [x] Stream Deck Original (untested)
  - [ ] Stream Deck Original V2
  - [ ] Stream Deck XL


## Getting started

- `cargo add streamdeck` to add this library to your project (with [cargo-edit](https://github.com/killercup/cargo-edit))
- `cargo install streamdeck` to install the utility only
- `git clone git@github.com:ryankurte/rust-streamdeck.git` to clone the repo

Building requires `libusb` and `hidapi` packages.

### Setting up permissions on linux

- `cp 40-streamdeck.rules /etc/udev/rules.d/` to allow user access to streamdeck devices
  - note this may need to be edited with other vid/pid combinations for other devices
- `sudo udevadm control --reload-rules` to reload udev rules


## Related Works

This library stands on the shoulders of giants (who had already done all the reversing work)...

You might also like to look at:

- [streamdeck-rs](https://crates.io/crates/streamdeck-rs) for writing plugins to interact with the official Elgato SDK
- [stream_deck_rs](https://crates.io/crates/stream_deck_rs) another project with similar goals
- [@cliffrowley's streamdeck protocol notes](https://gist.github.com/cliffrowley/d18a9c4569537b195f2b1eb6c68469e0)
- [python streamdeck library](https://github.com/abcminiuser/python-elgato-streamdeck)
- [node-elgato-stream-deck](https://github.com/Lange/node-elgato-stream-deck/blob/master/NOTES.md)
