# embedded-holochain-runner

A library that makes it VERY easy to run Holochain as a library, from your own binary, with great defaults

## How it will work

`datastore_path` is most important. If existing persisted Holochain conductor files
are found in the given directory, it will simply re-use the `admin_ws_port` `app_ws_port` `app_id` and `dnas` from that configuration. Otherwise, it will create that directory, and setup your configuration as specified.

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
