# src-tauri

### DEVELOPMENT OVERVIEW

```bash
.
├── build.rs
├── Cargo.toml
├── README.md
├── src
│   ├── error.rs
│   ├── ipc
│   │   ├── mod.rs
│   │   ├── params.rs
│   │   ├── person.rs
│   │   └── response.rs
│   ├── macros.rs
│   ├── main.rs
│   └── model
│       ├── mod.rs
│       ├── store.rs
│       ├── try_from.rs
│       └── types.rs
└── tauri.conf.json
```

The modules are organised as followed:

- In `model` we define:
  - types
  - types conversions
  - database store methods
- In `ipc` we define the Tauri IPC commands to bridge the Frontend to the store implementations, following the "JSON-RPC 2.0" format:
  - method definitions (tauri commands)
  - params and state arguments
  - IpcResponse with the JSON-RPC 2.0 result/error

Further modularization should be conducted, specially if extending the functionality of the application when adding more tables and corresponding types conversions.

### TEST SURREAL STORE IMPLEMENTATIONS

```sh
cargo test --package surrealdb-starter-taurikit --bin surrealdb-starter-taurikit -- model::store::tests --nocapture
```
