#!/usr/bin/sh
#set -e

for test in tests/*; do
    if [ -f "$test" ]; then
        cargo run "$test"
    fi
done
