#!/bin/sh
set -eu

port=${PORT:-3187}
address=${ADDRESS:-0.0.0.0}
case "$port" in
    ''|*[!0-9]*) echo "PORT must be a numeric TCP port" >&2; exit 2 ;;
esac
if [ "$port" -lt 1 ] || [ "$port" -gt 65535 ]; then
    echo "PORT must be between 1 and 65535" >&2
    exit 2
fi
case "$address" in
    ''|*[!A-Za-z0-9.:_-]*) echo "ADDRESS must be an IP address or hostname" >&2; exit 2 ;;
esac
exec /app/app "$address:$port"
