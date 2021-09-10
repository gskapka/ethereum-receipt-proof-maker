pub static USAGE_INFO: &str = "
❍ Rusty Receipt Proof Maker ❍

    Copyright Provable 2019
    Questions: greg@oraclize.it

❍ Info ❍

This tool generates a merkle receipt proof of the receipt pertaining to the given transaction hash.

***

Usage:  rusty-receipt-proof-maker [--help]
        rusty-receipt-proof-maker <txhash> [--verbose | -v]

Options:

    --help              ❍ Show this message.

    -v, --verbose       ❍ Enable verbose mode for additional output.

    <txhash>            ❍ A transaction hash of an Ethereum transaction
                        ➔ Format: A 32-byte long, prefixed hex string.

";
