pub static USAGE_INFO: &'static str = "
❍ Rusty Receipt Proof Maker ❍

    Copyright Provable 2019
    Questions: greg@oraclize.it

❍ Info ❍

This tool generates a merkle receipt proof of the receipt pertaining to the given transaction hash.

***

Usage:  rusty-receipt-proof-maker [-h | --help]
        rusty-receipt-proof-maker <txhash> [-v | --verbose]

Options:

    -h, --help          ❍ Show this message.

    -b, --verbose       ❍ Enable verbose mode for additional output.

    <txhash>            ❍ A transaction hash of an Ethereum transaction
                        ➔ Format: A 32-byte long, prefixed hex string.

";
