# hello-rusthdl

## Requirements

    pyenv virtualenv 3.11.1 gsg-luna
    pyenv local gsg-luna
    cd ~/GreatScott/luna.git && pip install -e .

    brew install libusb
    sudo mkdir /usr/local/lib
    sudo ln -s /opt/homebrew/lib/libusb.dylib /usr/local/lib

## Run

    cargo run --release --bin blinky
