#!/bin/bash

# Command to run
COMMAND="./target/release/down_on_spot "
COMMAND+=""

# Track PID of the running command
PID=0

# Function to clean up and stop everything
cleanup() {
    echo "Stopping script..."
    if [[ $PID -ne 0 ]]; then
        kill $PID 2>/dev/null
    fi
    exit 0
}

# Catch Ctrl+C or kill signals
trap cleanup SIGINT SIGTERM

while true; do
    echo "Starting command..."
    $COMMAND &
    PID=$!

    # Wait up to 5 minutes for the command to finish
    for i in {1..300}; do
        if ! kill -0 $PID 2>/dev/null; then
            echo "Command finished on its own. Exiting script."
            wait $PID 2>/dev/null
            exit 0
        fi
        sleep 1
    done

    # If still running after 5 minutes, kill it
    echo "Command still running after 5 minutes. Stopping..."
    kill $PID 2>/dev/null
    wait $PID 2>/dev/null

    # Wait 10 minutes before restarting
    echo "Waiting 10 minutes before restarting..."
    sleep 600
done

