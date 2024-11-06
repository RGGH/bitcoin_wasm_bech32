In Bech32 encoding, used for SegWit (P2WPKH) Bitcoin addresses, 
data must be converted from the standard 8-bit (or byte) representation 
into 5-bit "chunks" because Bech32 addresses use a base32 encoding scheme. 

This conversion is essential because the Bech32 alphabet can only handle values up to 31 
(which can be represented by 5 bits) rather than the typical 256 values that a full byte (8 bits) can represent.
