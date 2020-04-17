#!/usr/bin/env sh

# Requires browser-sync
# > sudo npm install -g browser-sync
cd target-www
browser-sync start -s -f . --no-notify --host 127.0.0.1 --port 8080