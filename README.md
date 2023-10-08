This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

Run the development server:

```bash
cargo tauri dev
```

## Building

First, to create the type bindings defined in the backend model, you need to run:

```sh
cd src-tauri && cargo 'test' '--package' 'surrealdb-starter-taurikit' '--bin' 'surrealdb-starter-taurikit' '--' 'model::types' '--nocapture'
```

Build the app

```bash
cargo tauri build
```

SurrealDB it is run as an in-memory database, by enabling the following feature:

`./src-tauri/Cargo.toml`

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

To use a server, we need to switch to:

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
