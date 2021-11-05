# Tonic Test repo

This repo can be used to reproduce a curious problem with tonic:

When starting a server with *both* TLS and `serve_with_incoming`, clients will hang when performing RPC calls.

To reproduce this, run the following commands in terminal A:

    cargo run --bin=server -- --incoming --tls

In terminal B, run the following commands:

    cargo run --bin=client -- --tls


The client will hang after `Connection established, making RPC call...`.

If you instead call the server without `--tls` or without `--incoming`, the client will not hang.  (Obivously if you omit the `--tls` flag on the server, also omit it from the client.)

# TLS keys
Since this test requires TLS, you can run `make` from within the `ssl` directory to generate the keys.