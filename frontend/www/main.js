import init, { run_app } from '../pkg/kombucha_tracker_frontend.js';
async function main() {
   await init('/main.wasm');
   run_app();
}
main()