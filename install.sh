#!/bin/bash

git pull
cargo build -qr
sudo cp target/release/kkit /bin/kkit
sudo ln -s /bin/kkit /bin/kk
sudo ln -s /bin/kkit /bin/k

 kkit v