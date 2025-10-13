#!/bin/bash
# Reload hyprmarker daemon
# This will kill the old daemon and start a new one with updated config

echo "Stopping hyprmarker daemon..."
pkill hyprmarker

# Wait a moment for clean shutdown
sleep 0.5

echo "Starting hyprmarker daemon..."
hyprmarker --daemon &

# Wait to verify it started
sleep 0.5

if pgrep -x hyprmarker > /dev/null; then
    echo "✓ Daemon restarted successfully (PID: $(pgrep -x hyprmarker))"
    echo "Press Super+D to toggle overlay"
else
    echo "✗ Failed to start daemon"
    exit 1
fi
