# MIDNOTE

Midnote is a terminal application that reads a MIDI file
and displays you its notes bar-by-bar, while playing it.

# Goals

As a blind musician myself, I found it very difficult to learn new songs on my own.

I've "seen" [Lunar Tabs][], and wanted to make something similar but for MIDI files
since good guitarpro5 tabs are not free but you can download many great MIDI files.

# Prerequisites
 
Midnote works by reading a MIDI file and parsing the messages within.
The display requires no MIDI device (software or hardware) but you need one for the playback.
 
Windows comes with a default MIDI device, which is pretty lame sounding and laggy.
[OmniMidi][] is highly recommended for Windows users.

On *NIX, you'll need to install your own MIDI synthesizer.
I personally recommend [Fluidsynth][].

On MacOS, [Fluidsynth][] is available and it should work as good as it works on *NIX systems.

# Build Requirements
You need a working [Rust][] installation along with the rust package manager, `cargo` (`cargo` ships with the rust toolchain).
On *nix, you also need alsa development libraries:
```sh
# debian / ubuntu
apt install libasound2-dev
# fedora / sentos
dnf install alsa-lib-devel
```

# Installation
Pre-built binaries can be found on the [releases](https://github.com/insomnimus/midnote/releases) page.

# Building The Project

To build the project, you only need a working rust environment and git:

```sh
git clone https://github.com/insomnimus/midnote
cd midnote
git checkout main
cargo install --path . --locked
```

# Usage

You start midnote by giving it a midi file as an argument and optionally, specifying a MIDI device.

```sh
# Open megalovania.mid, using the default MIDI device:
midnote ./megalovania.mid

# Specify another MIDI device:
midnote ./megalovania.mid --device 2

# List available MIDI devices:
midnote --list
```

For more options you can set, please run `midnote --help`.

# Configuration
Midnote accepts a config file (*.json) with the `--config` command line argument.
The default configuration is as follows:

```json
{
  "colors": true,
  "keys": {
    "next": "Right",
    "prev": "Left",
    "replay": {
      "Char": "r"
    },
    "solo": {
      "Char": "s"
    },
    "silence": {
      "Char": " "
    },
    "rewind": {
      "Char": "p"
    },
    "exit": "Esc",
    "help": {
      "Char": "h"
    }
  }
}
```

## Possible Keys

-	`Backspace`
-	`Enter`
-	`Left`
-	`Right`
-	`Up`
-	`Down`
-	`Home`
-	`End`
-	`PageUp`
-	`PageDown`
-	`Tab`
-	`BackTab`
-	`Delete`
-	`Insert`
-	`Esc`

Any letter key is also valid but needs to be wrapped in a `Char` object, see the default config above.

Function keys have the form `{"F": 1..=12 }`.

[Lunar Tabs]: https://github.com/ProjPossibility/Lunar-Tabs-Desktop
[OmniMidi]: https://github.com/KeppySoftware/OmniMIDI
[Fluidsynth]: https://github.com/FluidSynth/fluidsynth
[Rust]: https://github.com/rust-lang/rust
