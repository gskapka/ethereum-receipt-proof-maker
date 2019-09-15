## :herb: Merkle-Proofs for Transactions Receipts in Ethereum

&nbsp;

A Maker of Merkle-Proofs for Transaction Receipts in Ethereum using Rust.

&nbsp;

***

&nbsp;

### :point_right: Usage:

__>__ To generate a proof for a given transaction hash:

__`❍ rusty-receipt-proof-maker <your-tx-hash-here>`__


```

rusty-receipt-proof-maker -v 0x5d761b001c4d69bf14c94b8e8a604d97e008a8a7dfb74a6459823b2178ffc033`

 ✔ CLI Args parsed successfully!
 ✔ Verbose mode: true
 ✔ Transaction hash: 0x5d76…c033
 ✔ Validating CLI args...
 ✔ Initializing state from CLI args...
 ✔ Getting RPC endpoint from environment variables...
 ✔ Endpoint retrieved: http://localhost:8545/
 ✔ Connecting to node...
 ✔ Connection successful! Latest block number: 8556245
 ✔ Getting block from transaction hash: 0x5d76…c033
 ✔ Getting all receipts from block...
 ✔ Getting transaction index of hash: 0x5d76…c033
 ✔ Building merkle-patricia trie from receipts...
 ✔ Pulling branch from trie...
 ✔ Hex encoding proof from nodes in branch...
 ✔ Hex Proof:

f90264f8b1a0dc1a1b7bc9f38fc6710af9abdf1da874fa708573431381908b8b6a25bd1f2b55a0701833ef4ede796d9c96b1ae7c66830b0126f106ea0bfe99864e679e8b0dfeafa00db067f1c8ee75d8a563038903eb5129ee1d73beeea9380ca58d1920c15fe84ba0a55d0852ba77dbcc2824337150c123ccde62699b0c6eaddf735c1b2638bb85ba80808080a04937e058ea19511f92f803c23f4e9a4eceaaf2246c86da0f1b2241316be8a34d8080808080808080f901ae30b901aaf901a701837ab31ab9010000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000002000000080000000000000000200000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000020000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000004000000000000010200000000000000000000000000000000000000000000000000000000000f89df89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a0000000000000000000000000539efb706852838c51905d3d31966c296e034000a0000000000000000000000000a2881a90bf33f03e7a3f803765cd2ed5c8928dfba0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff

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
        rusty-receipt-proof-maker <txhash> [-t | --trace]
        rusty-receipt-proof-maker <txhash> [-v | --verbose]

Options:

    -h, --help          ❍ Show this message.

    -v, --verbose       ❍ Enable verbose mode for additional output.

    -t, --trace         ❍ Enable tracing for debugging/bug reporting.

    <txhash>            ❍ A transaction hash of an Ethereum transaction
                        ➔ Format: A 32-byte long, prefixed hex string.

❍ Rusty Receipt Proof Maker ❍

    Copyright Provable 2019
    Questions: greg@oraclize.it

❍ Info ❍

This tool generates a merkle receipt proof of the receipt pertaining to the given transaction hash.

***

Usage:  rusty-receipt-proof-maker [-h | --help]
        rusty-receipt-proof-maker <txhash> [-t | --trace]
        rusty-receipt-proof-maker <txhash> [-v | --verbose]

Options:

    -h, --help          ❍ Show this message.

    -v, --verbose       ❍ Enable verbose mode for additional output.

    -t, --trace         ❍ Enable tracing for debugging/bug reporting.

    <txhash>            ❍ A transaction hash of an Ethereum transaction
                        ➔ Format: A 32-byte long, prefixed hex string.


```

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
rusty-receipt-proof-maker -v 0x5d761b001c4d69bf14c94b8e8a604d97e008a8a7dfb74a6459823b2178ffc033`

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

### :guardsman: Tests:

To run the tests:

__`❍ cargo +nightly test`__

__Note:__ Some expensive tests are ignored by default. To run all test, including those ignored, add the __`--ignored`__ flag.

__:radioactive: CAUTION:__ Some tests rely on access to a full node at __`http://localhost:8545`__. If one can't be reached at that endpoint, many of the tests will fail.

&nbsp;

***

&nbsp;

### :black_nib: Notes

__❍__ The current memory database usage is pure in that we clone the db and return the new, updated copy back to state. Efficiency sacrificed for immutability. Depending on performance of final tool (whose bottleneck is guaranteed to be the fetching of the potentially many receipts for the transactions in a block from the RPC endpoint) this will be changed.

***

&nbsp;

### :clipboard: To-Do

 - [x] Allow configurable endpoint.
 - [x] Have flag to suppress logging.
 - [x] Have timeout error on reqwests.
 - [x] Have method to convert hex string of even/odd length to offset/non- nibbles.
 - [x] Need a node rlp-decoder!
 - [ ] Benchmark it and maybe don't clone the db per above note?
 - [ ] Spinners for when it's doing the bits that take a while...?
 - [ ] Factor out log level stuff into own module (from cli arg parser!)
 - [ ] Remove unused fxns
