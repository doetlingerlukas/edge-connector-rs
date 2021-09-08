# edge-connector-rs

Connects a [faasd](https://github.com/openfaas/faasd/) device to the Apollo platform.

## Dependencies

If you haven't got the Rust toolchain installed already, take a look at the official [Getting started page](https://www.rust-lang.org/learn/get-started).

Unless you are compiling on the target device directly, the following is required:

- [cross](https://github.com/rust-embedded/cross) is needed for cross compilation (install with `cargo install cross`).
- [Docker](https://www.docker.com/) or [Podman](https://podman.io/) is required for cross.
- [Ruby](https://www.ruby-lang.org/de/) to use the provided `Rakefile`.

Also take a look at the [dependencies section of cross](https://github.com/rust-embedded/cross#dependencies) and make sure you meet the requirements.

Currently `rake` tasks utilize `rsync` to deploy the compiled binaries to the target device, so make sure it's available at the path.

## Cross compile

The target device should be running [faasd](https://github.com/openfaas/faasd/) and be reachable over the network.
Make sure to have `ssh` access to the Raspberry Pi. If you are not using a certificate for connecting, you'll be asked to enter a password.

The `run` task will fetch the access key for faasd and compile the binary from it. It also automatically deploys the binary and runs it on the device. To execute the task, insert your Pi's address and run the following command:

```bash
RPI="<IP or DNS name>"; rake run
```

## Compile on device

Copy the faasd access key in a file named `faas-key`, which can be found at `/var/lib/faasd/secrets/basic-auth-password`.

Compile the code using the following command:

```bash
cargo build --release
```
