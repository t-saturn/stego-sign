# stego-server

HTTP backend for StegoSign, built with [Axum](https://github.com/tokio-rs/axum) and [SeaORM](https://www.sea-ql.org/SeaORM/).

## Stack

- **Axum 0.8** — async HTTP server
- **SeaORM** — async ORM for PostgreSQL
- **aws-sdk-s3** — S3-compatible client for AIStor and AWS S3
- **ed25519-dalek** — cryptographic signing and verification
- **sqlx / migrations** — database migrations

## API

### Health

| Method | Route | Description |
|--------|-------|-------------|
| `GET` | `/health` | Server, DB and storage status |

### Documents

| Method | Route | Description |
|--------|-------|-------------|
| `POST` | `/api/v1/sign` | Sign a document (multipart: `file`, `author`, `watermark_position`) |
| `POST` | `/api/v1/verify` | Verify document integrity (multipart: `file`) |
| `GET` | `/api/v1/documents` | List all documents (max 100) |
| `GET` | `/api/v1/documents/{id}` | Get document by UUID |
| `GET` | `/api/v1/documents/{id}/audit` | Audit log for a document |
| `GET` | `/api/v1/documents/{id}/download` | Download signed file |
| `GET` | `/api/v1/verify/code/{code}` | Verify by QR verification code |

### Storage

| Method | Route | Description |
|--------|-------|-------------|
| `GET` | `/api/v1/storage/{bucket}/{*key}` | Download an object from storage |

### Stats & Registry

| Method | Route | Description |
|--------|-------|-------------|
| `GET` | `/api/v1/stats` | Global statistics |
| `GET` | `/api/v1/registry` | Full document registry |

### Admin (dev only)

| Method | Route | Description |
|--------|-------|-------------|
| `GET` | `/api/v1/admin/keygen` | Generate an Ed25519 key pair |

## Signing flow
```
POST /api/v1/sign  (multipart: file + author)
  1. Apply QR watermark to PDF with verify_url
  2. Compute SHA-256 of watermarked content
  3. Sign hash with Ed25519 (SIGNING_KEY)
  4. Embed steganographic payload (hash + signature + author)
  5. Upload original   → {prefix}-uploads/{doc_id}/{filename}
  6. Upload signed     → {prefix}-signatures/{doc_id}/signed_{filename}
  7. Register in DB    → app.documents
  8. Write audit entry → app.audit_logs
```

## Verification flow
```
POST /api/v1/verify  (multipart: file)
  1. Extract steganographic payload from file
  2. Compute SHA-256 of content (without payload)
  3. Look up document in DB by original hash
  4. Compare current hash vs registered hash
  5. Verify Ed25519 signature (VERIFY_KEY)
  → VALID / TAMPERED / UNREGISTERED / INVALID
  If TAMPERED: upload to {prefix}-corrupted and update status in DB
```

## Local development

### Prerequisites

- Rust 1.91+
- PostgreSQL running
- AIStor/MinIO or AWS S3 credentials

### Quick setup with Docker
```bash
# PostgreSQL
docker run -d --name stego-db \
  -e POSTGRES_USER=sre \
  -e POSTGRES_PASSWORD=sre \
  -e POSTGRES_DB=stegosign \
  -p 55432:5432 postgres:17

# AIStor (from project root)
docker compose up -d aistor
```

### Configuration
```bash
cp .env.example .env
# edit .env with your values
```

### Generate signing keys

On first run, start without keys and generate a pair:
```bash
cargo run
# server starts with a warning about missing keys
curl http://localhost:4000/api/v1/admin/keygen
# copy SIGNING_KEY and VERIFY_KEY into .env and restart
```

### Run
```bash
cargo run
```

Server available at `http://localhost:4000`.

### Migrations

Migrations run automatically on startup from `migrations/`.

## Storage

Two providers supported via `STORAGE_PROVIDER`:

**AIStor / MinIO (local)**
```env
STORAGE_PROVIDER=aistor
STORAGE_ENDPOINT=http://localhost:9000
STORAGE_ACCESS_KEY=admin
STORAGE_SECRET_KEY=your-secret
```

**AWS S3 (production)**
```env
STORAGE_PROVIDER=aws
STORAGE_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE
STORAGE_SECRET_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

On startup the server automatically creates the three buckets if they don't exist:
`{prefix}-uploads`, `{prefix}-signatures`, `{prefix}-corrupted`.

## Docker
```bash
# from project root
docker compose up -d server
```

The server is not exposed to the host — it is only accessible internally from the app via proxy.

## Project structure
```
src/
├── main.rs              # entry point, DB and storage initialization
├── router.rs            # route definitions
├── config/
│   ├── env.rs           # environment variable loading
│   └── db.rs            # PostgreSQL connection
├── handlers/
│   ├── health.rs        # GET /health
│   ├── sign.rs          # POST /api/v1/sign
│   ├── verify.rs        # POST /api/v1/verify
│   ├── verify_code.rs   # GET /api/v1/verify/code/{code}
│   ├── documents.rs     # GET /api/v1/documents/*
│   ├── stats.rs         # GET /api/v1/stats
│   ├── registry.rs      # GET /api/v1/registry
│   ├── storage_download.rs # GET /api/v1/storage/*
│   └── admin.rs         # GET /api/v1/admin/keygen
├── services/
│   ├── storage.rs       # S3 client (AIStor / AWS)
│   ├── crypto.rs        # SHA-256, Ed25519 signing
│   ├── stego.rs         # steganographic payload embed/extract
│   ├── watermark.rs     # QR insertion into PDF
│   └── qr.rs            # QR PNG generation
├── repositories/
│   ├── document.rs
│   ├── audit_log.rs
│   └── object.rs
└── models/
    ├── document.rs
    ├── audit_log.rs
    └── response.rs
```