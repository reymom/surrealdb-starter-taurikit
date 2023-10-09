# src-tauri

### MAIN COMMANDS

- Build

```sh
cargo build
```

- Generate frontend type bindings

```sh
cargo 'test' '--package' 'surrealdb-starter-taurikit' '--bin' 'surrealdb-starter-taurikit' '--' 'model::types'
```

- Test store implementations

```sh
cargo test --package surrealdb-starter-taurikit --bin surrealdb-starter-taurikit -- model::store::tests --nocapture
```

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
│   ├── main.rs
│   └── model
│       ├── mod.rs
│       ├── store.rs
│       └── types
│           ├── mod.rs
│           ├── person.rs
│           └── general.rs
└── tauri.conf.json
```

The modules are organised as followed:

- In `model` we define:
  - `types` module: general and specific-type implementations (like person) and mapping
  - `store`: database store methods and store wrapper
- In `ipc` we define the Tauri IPC commands to bridge the Frontend to the store implementations, following the "JSON-RPC 2.0" format:
  - method definitions (tauri commands)
  - params and state arguments
  - IpcResponse with the JSON-RPC 2.0 result/error

Further modularization should be conducted, specially if extending the functionality of the application when adding more tables and corresponding type conversions.

### PERSISTENT STORAGE

SurrealDB is run as an [in-memory database](https://surrealdb.com/docs/embedding/rust), by enabling the following feature:

`./Cargo.toml`

```toml
[dependencies]
surrealdb = { version = "1.0.0", default-features = false, features = ["kv-mem"] }
```

And using:

```rust
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<Mem>(()).await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;
}
```

To use a server, we need to enable all the surrealdb features:

`./Cargo.toml`

```toml
[dependencies]
surrealdb = "1.0.0"
```

Thus:

```rust
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;
}
```
