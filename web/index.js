import init, { generate_bech32_address } from "../pkg/bitcoin_wasm_bech32.js";

async function runWasm() {
    // Initialize the WASM module
    await init();

    // Define the function to generate a Bitcoin address from a public key
    window.generateBitcoinAddress = function() {
        const publicKeyHex = document.getElementById("input").value;

        // Call the WASM function to generate the Bitcoin address
        const bitcoinAddress = generate_bech32_address(publicKeyHex);

        document.getElementById("output").innerText = `Bitcoin Address:\n${bitcoinAddress}`;
    };
}

runWasm();


