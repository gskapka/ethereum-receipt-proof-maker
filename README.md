## :herb: Merkle-Proofs for Transactions Receipts in Ethereum

&nbsp;

A Maker of Merkle-Proofs for Transaction Receipts in Ethereum using Rust.

&nbsp;

***

&nbsp;

### :nut_and_bolt: Setup:

The tool requires access to a full ethereum node on whichever network you wish to generate receipt proofs for.

You can configure an endpoint for that node by creating a __`.env`__ file in the root of the repo thusly:

```

# At path: ./rusty-receipt-proof-maker/.env

ENDPOINT="<your-endpoint-here>"

```

This allows you to use for example an __[Infura](https://infura.io/)__ endpoint without risking exposing your API key. Another optional endpoint if you are not running your own node is __[Slock.It](http://rpc.slock.it/)__.

If you do not provide an endpoint, the tool will default to __`https://localhost:8545`__, and fail to run at all if it can't connect to a node at that location:

```

✔ Getting RPC endpoint from environment variables...
✔ Endpoint retrieved: http://localhost:8545/

✔ Connecting to node...
✘ HTTP Reqwest Error!
✘ http://localhost:8545/ timed out
✘ Please check your node & port settings and retry.

```

&nbsp;

***

&nbsp;

### :point_right: Usage:

__>__ To generate a proof for a given transaction hash:

__`❍ rusty-receipt-proof-maker <your-tx-hash-here>`__


```

rusty-receipt-proof-maker --verbose 0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6

✔ CLI Args parsed successfully!
✔ Verbose mode: true
✔ Transaction hash: 0xd6f5…2cc6

✔ Getting RPC endpoint from environment variables...
✔ Endpoint retrieved: http://localhost:8545/

✔ Connecting to node...
✔ Connection successful! Latest block number: 8316169

```

__>__ For usage notes, run the tool thusly:

__`❍ rusty-receipt-proof-maker --help`__

```

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

✔ Exiting, goodbye!

```

&nbsp;

***

&nbsp;

### :guardsman: Tests:

To run the tests:

__`❍ cargo +nightly test`__

__:radioactive: CAUTION:__ Some tests rely on access to a full node at __`http://localhost:8545`__. If one can't be reached at that endpoint, many of the tests will fail.

&nbsp;

***

&nbsp;

### :clipboard: To-Do

 - [x] Allow configurable endpoint.
 - [x] Have flag to suppress logging.
 - [x] Have timeout error on reqwests.
