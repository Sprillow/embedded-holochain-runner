# embedded-holochain-runner

A library that makes it VERY easy to run Holochain as a library, from your own binary, with great defaults

## How it will work

`datastore_path` is most important. If existing persisted Holochain conductor files
are found in the given directory, it will simply re-use the `admin_ws_port` `app_ws_port` `app_id` and `dnas` from that configuration. Otherwise, it will create that directory, and setup your configuration as specified.

It will pair nicely with structopt to make a configurable service. See [demo](https://github.com/Sprillow/embedded-holochain-demo).

In either case, 
- first run/installation
- second run/reboot
it will log this to the console when the interfaces are all ready and the app installed or running:

`EMBEDDED_HOLOCHAIN_IS_READY`

## Usage

`Cargo.toml`

```toml
[dependencies]
embedded_holochain_runner = { git = "https://github.com/Sprillow/embedded-holochain-runner.git" }
```

Assuming you have a compiled Holochain DNA file sitting around at `../dna/sample/sample.dna`...

`main.rs`

```rust
use embedded_holochain_runner::*;
const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");
fn main() {
    // String is like "CellNick"/"SlotId"
    let dnas: Vec<(Vec<u8>, String)> = vec![(SAMPLE_DNA.into(), "sample".into())];
    async_main(HcConfig {
        datastore_path: String::from("databases"),
        keystore_path: String::from("keystore"),
        app_id: String::from("my-app-id"),
        dnas,
        admin_ws_port: 1234,
        app_ws_port: 8888,
        proxy_url: String::from("kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"),
    })
}
```

It will clearly log its configuration to the console.

RUST_LOG environment variable can be set to get details logs from Holochain. Those logs are by default suppressed.

## Bootstrap Networking Service

This library is currently pointed at the `https://bootstrap-staging.holo.host` node discovery service.

## Holochain Version

The HDK used for your DNA should match the version used in this library, which is listed below.
Such as:

Zome `Cargo.toml`
```toml
[dependencies]
# use whatever hdk uses
serde = "*"
hdk = {git = "https://github.com/holochain/holochain.git", rev = "a6ac0439670ba367c723a80d3b8bc7c419aa5f6e", package = "hdk"}
```

Currently bundles Holochain version: [a6ac0439670ba367c723a80d3b8bc7c419aa5f6e (Apr 23, 2021)](https://github.com/holochain/holochain/commit/a6ac0439670ba367c723a80d3b8bc7c419aa5f6e).