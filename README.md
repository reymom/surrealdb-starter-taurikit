# SurrealDB-Tauri Starter Kit

This is a Starter Kit for [SurrealDB](https://surrealdb.com/) using [Tauri App](https://tauri.app/) with [Next.js](https://nextjs.org/). This app is intended to provide a built-in environment with this technologies combinations, with some assumptions on the structure which should be overriden by personal preferences.

It initializes a store interface with some basic implementations for a simple struct, with some mapping and organization layers. It also sets up the bindings to export the type interfaces to the frontend and be able to communicate back-and-forth through an ICP layer.

Finally, it sets up a simple react frontend without any UI framework, but with all the necessari connections ready-to use. The user can add and delete `Person`s and see the list of them.

## Requirements:

- Be sure to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

- Please refer to the prerequisites for using [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/).

- Node.js > v18

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

## Persistent storage

> Note: SurrealDB is run as an in-memory database. To enable persitent storage, edit the `src-tauri/Cargo.toml` to enable all `surrealdb` features.

> Refer to [src-tauri/Readme.md](https://github.com/reymom/surrealdb-starter-taurikit/tree/develop/src-tauri#persistent-storage) for further information.
