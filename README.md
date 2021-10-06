# embedded-holochain-runner

A library that makes it VERY easy to run Holochain as a library, from your own binary, with great defaults

You can find this crate on crates.io: https://crates.io/crates/embedded_holochain_runner

## How it will work

`datastore_path` is most important. If existing persisted Holochain conductor files
are found in the given directory, it will simply re-use the `admin_ws_port` `app_ws_port` `app_id` and `dnas` from that configuration. Otherwise, it will create that directory, and setup your configuration as specified.

It will pair nicely with structopt to make a configurable service. See [a simple demo](https://github.com/Sprillow/embedded-holochain-demo). For a more advanced application using the exported `async_main` function, shutdown signal, and `StateSignal` listeners, you can see it in use in the [Acorn Holochain application](https://github.com/h-be/acorn/blob/main/conductor/src/main.rs).

In either case,

- first run/installation
- second run/reboot

it will log this to the console when the interfaces are all ready and the app installed or running:

`EMBEDDED_HOLOCHAIN_IS_READY`

## Usage

`Cargo.toml`

```toml
[dependencies]
embedded_holochain_runner = "0.0.103"

[patch.crates-io]
rkv = { git = "https://github.com/holochain/rkv.git", branch = "master" }
lmdb-rkv = { git = "https://github.com/holochain/lmdb-rs.git" }
```

Assuming you have a compiled Holochain DNA file sitting around at `../dna/sample/sample.dna`...

`main.rs`

```rust
use embedded_holochain_runner::*;
const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");
fn main() {
    // String is like "CellNick"/"SlotId"
    let dnas: Vec<(Vec<u8>, String)> = vec![(SAMPLE_DNA.into(), "sample".into())];
    blocking_main(HcConfig {
        datastore_path: String::from("databases"),
        keystore_path: String::from("keystore"),
        app_id: String::from("my-app-id"),
        dnas,
        admin_ws_port: 1234,
        app_ws_port: 8888,
        proxy_url: String::from("kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"),
        event_channel: None,
    })
}
```

It will clearly log its configuration to the console.

RUST_LOG environment variable can be set to get details logs from Holochain. Those logs are by default suppressed.

## Events

if you pass an `event_channel`, which should be of type: `Option<tokio::sync::mpsc::Sender<StateSignal>>` where `StateSignal` can be imported via `use embedded_holochain_runner::StateSignal`, you can listen for the following events, to trigger external actions.

It looks like:

```rust
pub enum StateSignal {
    // will be only one or the other of these
    IsFirstRun,
    IsNotFirstRun,
    // are sub events after IsFirstRun
    CreatingKeys,
    RegisteringDna,
    InstallingApp,
    EnablingApp,
    AddingAppInterface,
    // Done/Ready Event, called when websocket interfaces and
    // everything else is ready
    IsReady,
}
```

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
hdk = "0.0.109"
```

Currently bundles Holochain version: [0.0.109 (October 6, 2021)](https://github.com/holochain/holochain/releases/tag/holochain-0.0.109).
