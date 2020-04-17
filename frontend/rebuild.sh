#!/usr/bin/env bash
set -euo pipefail

wasm-pack build --target web

rollup ./www/main.js --format iife --file ./pkg/bundle.js

mkdir -p target-www

cp ./pkg/bundle.js ./target-www/bundle.js
cp ./pkg/kombucha_tracker_frontend_bg.wasm ./target-www/main.wasm
cp ./www/index.html ./target-www/index.html
cp ./www/index.css ./target-www/index.css
