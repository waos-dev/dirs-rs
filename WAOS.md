# dirs for WaOS

This fork adds WaOS to dirs 6.0.0: on a WaOS app (a WASI component, `target_os = "wasi"`) the standard folders are the app's own, where upstream answers none. The change is `src/waos.rs` and its switch in `src/lib.rs`; everything else is upstream's. It is kept for whoever ports an app to WaOS, and is not offered upstream.

## Using it

```toml
[patch.crates-io]
dirs = { git = "https://github.com/waos-dev/dirs-rs", branch = "waos" }
```

## What it answers

| Folder | On WaOS |
|---|---|
| `config_dir`, `config_local_dir`, `preference_dir`, `data_dir`, `data_local_dir`, `state_dir` | `/data`, what the app keeps |
| `cache_dir` | `/cache`, what the app can lose |
| `runtime_dir` | `/tmp`, emptied when the app ends |
| `home_dir`, `executable_dir`, and the user's own (`document_dir`, `download_dir`, `picture_dir` and the rest) | none |

An app has no home folder and none of the user's folders: it reaches one only once the user gives it, through a file dialog.
