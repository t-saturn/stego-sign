# stego-app

SSR/WASM frontend for StegoSign, built with [Leptos](https://leptos.dev) 0.8 and Axum.

## Stack

- **Leptos 0.8** — full-stack Rust framework with SSR + hydration
- **Axum** — HTTP server for SSR and API proxy
- **WASM** — client compiled to WebAssembly via `wasm32-unknown-unknown`
- **Tailwind CSS** — styles via CDN browser build
- **gloo-net** — HTTP client for browser-side requests

## Architecture
```
browser
  │
  └─► localhost:3000  (stego-app — Leptos SSR)
        │
        └─► /api/*  →  internal proxy  →  localhost:4000  (stego-server)
```

The browser **never talks directly to the server**. All `/api/*` requests go through the app's internal proxy, which forwards them to the server on the internal network.

### Key environment variables

| Variable | Purpose | Example |
|---|---|---|
| `API_BASE_URL` | SSR → server (Rust process only) | `http://localhost:4000` |
| `PUBLIC_API_BASE_URL` | CSR → app proxy (injected into meta tag) | `http://localhost:3000` |
| `LEPTOS_SITE_ADDR` | Address the app listens on | `127.0.0.1:3000` |

## Local development

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

### Setup
```bash
cp .env.example .env
# edit .env with your values
```

Make sure the server is running on `localhost:4000`:
```bash
# in /server
cargo run
```

### Run
```bash
cargo leptos watch
```

Open [http://localhost:3000](http://localhost:3000).

## Production build
```bash
cargo leptos build --release
```

Binary at `target/release/stego-app`, assets at `target/site/`.

## Docker

Build args are provided by the root `docker-compose.yml`:
```yaml
args:
  API_BASE_URL:        http://server:4000
  PUBLIC_API_BASE_URL: http://localhost:${APP_PORT}
```
```bash
# from project root
docker compose up -d app
```

## Project structure
```
src/
├── main.rs          # SSR entry point + /api/* proxy
├── lib.rs           # WASM entry point (hydrate)
├── app.rs           # HTML shell + main router
├── config.rs        # api_base_url() — SSR reads env, CSR reads meta tag
├── features/
│   ├── home/        # home page + stats
│   ├── sign/        # document signing
│   ├── verify/      # integrity verification
│   └── documents/   # document list and download
└── shared/
    ├── components/  # navbar, footer
    └── models.rs    # shared types
```

## API Proxy

`main.rs` registers a `/api/*` route that forwards all requests to the server:
```
GET  /api/v1/stats      →  http://server:4000/api/v1/stats
POST /api/v1/sign       →  http://server:4000/api/v1/sign
POST /api/v1/verify     →  http://server:4000/api/v1/verify
GET  /api/v1/documents  →  http://server:4000/api/v1/documents
```

This removes the need for CORS configuration and keeps the server off the public network.