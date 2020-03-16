#!/bin/bash

set -e

cross build --target arm-unknown-linux-gnueabihf --release
rsync -az target/arm-unknown-linux-gnueabihf/release/home-station-2 pi@homestation:/home/pi/home-station-2 --info=progress2