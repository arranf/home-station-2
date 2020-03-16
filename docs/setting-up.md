# Setting up HomeStation 2

## Requirements
- [Rust 1.38.0 or newer](https://rustup.rs/)
- A Raspberry Pi
- Docker daemon (for cross compilation)


## Setup.
### Configuration
To configure the application you'll need to create a `config.toml` file. This will be read at runtime, not compile time so will need to exist on the Raspberry Pi.
An example configuration is included `config.example.toml`, you should copy it (`cp config.example.toml config.toml`) and edit it. It by default uses dummy data.

To use weather services you'll need to sign up for a weather API, for example [Dark Sky](https://darksky.net/dev) or [Open Weather Map](https://openweathermap.org/api).

### Compilation
You can preview the application by running `cargo run`.

To compile it for the Raspberry Pi you'll need to have [cross](https://github.com/rust-embedded/cross) installed, which you can install with `cargo install cross`.
You can then compile for the Pi using `cross build --target arm-unknown-linux-gnueabihf --release`.

### Pi Setup
Homestation 2 will work with a basic headless Raspbian Buster Lite setup combined with xorg server (`sudo apt-get install xserver-xorg libxcursor-dev xinit x11-xserver-utils`). If you're using the [HyperPixel 4](https://shop.pimoroni.com/products/hyperpixel-4?variant=12569485443155) you will need to also run its installer script `curl https://get.pimoroni.com/hyperpixel4 | bash`. To get the screen to appear in landscape you will need to run `hyperpixel4-rotate right`.

Ensure your Pi boot into the console logged in as the Pi user by using `sudo raspi-config` and selecting the correct option under 'Boot Options'.

You'll need to transfer a few files to the device so it can be useful to mount the Pi's file system using `ssfs`. Theres an example script under `./scripts/pi-mount.example.sh` to do this.

As well as the built binary (`/target/arm-unknown-linux-gnueabihf/release/home-station-2`), you'll need to transfer a `config.toml` file, and the `assets` folder.
Transfer the compiled binary to the device, there's an example which uses `ssfs`.

Finally xorg needs to be configured. Create an entry at the end of your your `~/.bash_login` that looks like the code below to start xorg if you have a display:

```bash
if [[ ! $DISPLAY && $XDG_VTNR -eq 1 ]]; then
  exec startx
fi
```

Then create an `~/.xinitrc` file that looks like: 

```bash
/home/pi/home-station-2
```