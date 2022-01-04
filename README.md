# edge-connector-rs

Connects an Raspberry Pi running [faasd](https://github.com/openfaas/faasd/) to the Apollo platform.

## Dependencies

If you haven't got the Rust toolchain installed already, take a look at the official [Getting started page](https://www.rust-lang.org/learn/get-started).

Unless you are compiling on the target device directly, the following is required:

- [cross](https://github.com/rust-embedded/cross) is needed for cross compilation (install with `cargo install cross`).
- [Docker](https://www.docker.com/) or [Podman](https://podman.io/) is required for cross.
- [Ruby](https://www.ruby-lang.org/de/) to use the provided `Rakefile`.

Also take a look at the [dependencies section of cross](https://github.com/rust-embedded/cross#dependencies) and make sure you meet the requirements.

Currently `rake` tasks uses `ssh` to deploy the compiled binaries to the target device, so make sure the device is accessible.

## Setup

Choose between the following options to set up the edge-connector on a Raspberry Pi.

### Option 1: Cross compile

The recommended option is to cross compile the binary on you current development setup for the architecture of the target device.

The target device, Raspberry Pi or similar, should be running [faasd](https://github.com/openfaas/faasd/) and be reachable over the network.
Make sure to have `ssh` access to the device. If you are not using a certificate for connecting, you'll be asked to enter a password.

The `run` task will fetch the access key for faasd and compile the binary. Note that the value of the key is compiled into the binary and must match the device running it, otherwise the edge-connector will not function correctly. It also automatically deploys the binary and runs it on the device. To execute the task, insert your Pi's address and run the following command:

```bash
RPI="<IP or DNS name>" rake run
```

### Option 2: Compile on device

If you chose to compile the project directly on the device, Raspberry Pi or similar, make sure you have got the Rust installed. The device should be running [faasd](https://github.com/openfaas/faasd/).

Copy the faasd access key in a file named `faas-key`, which can be found at `/var/lib/faasd/secrets/basic-auth-password`. You can use the following command to create the file: `sudo cat /var/lib/faasd/secrets/basic-auth-password > faas-key`.

Compile the code using the following command:

```bash
cargo build --release
```

The finished binary can be found in the `target` folder. Alternatively, you can use `cargo run` to directly execute the program.

## Usage

Refer to the [rpi-serverless repository](https://github.com/doetlingerlukas/rpi-serverless) on how to utilize device running edge-connector for the [Apollo Platform](https://github.com/Apollo-Core).
