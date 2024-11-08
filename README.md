# Bitcoin bech32 segwit address from public key

[bitcoin wiki](https://en.bitcoin.it/wiki/Bech32)

In Bech32 encoding, used for SegWit (P2WPKH) Bitcoin addresses, 
data must be converted from the standard 8-bit (or byte) representation 
into 5-bit "chunks" because Bech32 addresses use a base32 encoding scheme. 

This conversion is essential because the Bech32 alphabet can only handle values up to 31 
(which can be represented by 5 bits) rather than the typical 256 values that a full byte (8 bits) can represent.
---

see : [reference implementation in Rust](https://github.com/sipa/bech32/tree/master/ref/rust)

---

A Bech32 address is created from a public key as follows:

1. Having a compressed public key (0x02 or 0x03 followed by 32 byte X coordinate): 0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798

2. Perform SHA-256 hashing on the public key: 0f715baf5d4c2ed329785cef29e562f73488c8a2bb9dbc5700b361d54b9b0554

3. Perform RIPEMD-160 hashing on the result of SHA-256: 751e76e8199196d454941c45d1b3a323f1433bd6

4. The result of step 3 is an array of 8-bit unsigned integers (base 2^8=256) and Bech32 encoding converts this to an array of 5-bit unsigned integers (base 2^5=32) so we “squash” the bytes to get:

in hex: 0e140f070d1a001912060b0d081504140311021d030c1d03040f1814060e1e16

in numbers: 14 20 15 07 13 26 00 25 18 06 11 13 08 21 04 20 03 17 02 29 03 12 29 03 04 15 24 20 06 14 30 22

5 bits binary: 01110 10100 01111 00111 01101 11010 00000 11001 10010 00110 01011 01101 01000 10101 00100 10100 00011 10001 00010 11101 00011 01100 11101 00011 00100 01111 11000 10100 00110 01110 11110 10110

5. Add the witness version byte in front of the step 4 result (current version is 0): 000e140f070d1a001912060b0d081504140311021d030c1d03040f1814060e1e16

6. Compute the checksum by using the data from step 5 and the H.R.P (bc for MainNet and tb for TestNet) 0c0709110b15

7. Append the checksum to result of step 5 (we now have an array of 5-bit integers): 000e140f070d1a001912060b0d081504140311021d030c1d03040f1814060e1e160c0709110b15

8. Map each value to its corresponding character in Bech32Chars (qpzry9x8gf2tvdw0s3jn54khce6mua7l) 00 -> q, 0e -> w,… qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4

9. A Bech32_encoded address consists of 3 parts: HRP + Separator + Data: bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4

The final result from step 9 is the same as example in BIP173. 
