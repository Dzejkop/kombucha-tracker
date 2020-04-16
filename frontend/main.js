import init, { run_app } from './pkg/kombucha_tracker_frontend.js';
async function main() {
   await init('/pkg/kombucha_tracker_frontend_bg.wasm');
   run_app();
}
main()