#!/bin/bash

set -euxo pipefail

APP_NAME="csvq"

cargo build --release
sudo cp target/release/$APP_NAME /usr/bin/

