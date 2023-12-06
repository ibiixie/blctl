#!/bin/bash

args=("$@")

echo "Building..."

cargo build --release

if [ $? -ne 0 ]; then
  echo "Build failed!"
  exit 1
fi

echo "Installing..."

sudo cp -f ./target/release/blctl /usr/bin/

if [ $? -ne 0 ]; then
    echo "Could not copy blctl binary to /usr/bin/"
    exit 1
fi

if [[ $args == *"--runit"* ]]; then
    echo "Copying Runit service configuration..."
    
    # Make sure blctl is not alreadu running as a service
    sudo sv down blctl &> /dev/null

    sudo cp -f -r ./service-configs/runit/blctl/ /etc/sv/
    
    if [ $? -ne 0 ]; then
        echo "Could not copy Runit service onfiguration files"
        exit 1
    fi

    echo "Runit service configuration copied!"
else
    echo "No init system specified; skipping service configuration"
fi

echo "Installation complete!"

exit 0
