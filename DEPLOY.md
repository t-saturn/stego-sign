# deployment guide

## prerequisites

- Docker + Docker Compose
- `make`
- AWS credentials (if using S3) or AIStor license (if using local storage)

## storage providers

stego-sign supports two storage backends, configured via `STORAGE_PROVIDER` in `.env`:

| provider | description |
|---|---|
| `aistor` | local MinIO/AIStor — self-hosted, requires `minio.license` |
| `aws` | AWS S3 — managed cloud storage, requires IAM credentials |

---

## first deploy

### 1. configure environment
```bash
cp .env.example .env
# edit .env with your values
```

minimum required values:
```env
POSTGRES_PASSWORD=your-secure-password
STORAGE_PROVIDER=aistor          # or aws
STORAGE_ACCESS_KEY=your-key
STORAGE_SECRET_KEY=your-secret
STORAGE_BUCKET_PREFIX=stego-yourname
APP_PORT=55548
APP_BASE_URL=http://your-domain-or-ip:55548
```

if using aws, also set:
```env
# STORAGE_ENDPOINT can be left empty for aws
STORAGE_ENDPOINT=
```

### 2. deploy infrastructure and app

**with aistor (local storage):**
```bash
make deploy-aistor
```

**with aws s3:**
```bash
make deploy-aws
```

### 3. apply database migrations

wait a few seconds for the database to be ready, then:
```bash
make migrate
```

at this point the server will be in a restart loop — this is expected until migrations are applied and keys are generated.

### 4. generate signing keys
```bash
make keygen
```

output:
```json
{
    "success": true,
    "data": {
        "note": "add these to your .env and remove this endpoint in production",
        "signing_key": "MFECAQEwBQYDK2VwBCIEI...",
        "verify_key": "WwS/EtkGvQQyiFA4hw/..."
    }
}
```

### 5. add keys to .env
```env
SIGNING_KEY=MFECAQEwBQYDK2VwBCIEI...
VERIFY_KEY=WwS/EtkGvQQyiFA4hw/...
```

### 6. force recreate server with new env vars
```bash
make recreate-server
```

verify the server is running:
```bash
make logs s=server
```

the app is now available at `http://localhost:${APP_PORT}`.

---

## subsequent deploys

for regular restarts or updates after the first deploy:
```bash
# pull latest changes and rebuild
make build
make up

# if env vars changed
make recreate-server
```

---

## switching storage provider

### from aistor to aws
```bash
# 1. update .env
STORAGE_PROVIDER=aws
STORAGE_ACCESS_KEY=your-aws-key
STORAGE_SECRET_KEY=your-aws-secret
STORAGE_ENDPOINT=   # leave empty

# 2. redeploy
make down
make deploy-aws
```

note: existing files in aistor will not be migrated automatically.

### from aws to aistor
```bash
# 1. update .env
STORAGE_PROVIDER=aistor
STORAGE_ENDPOINT=http://aistor:9000
STORAGE_ACCESS_KEY=your-aistor-key
STORAGE_SECRET_KEY=your-aistor-secret

# 2. redeploy
make down
make deploy-aistor
```

---

## full reset

to wipe everything and start from scratch:
```bash
make down-v       # removes containers and volumes
make deploy-aistor  # or deploy-aws
make migrate
make keygen
# add keys to .env
make recreate-server
```

---

## useful commands
```bash
make ps                  # show running containers
make logs s=server       # follow server logs
make logs s=app          # follow app logs
make migrate-reset       # clear all data, keep schema
make migrate-fresh       # drop and re-apply all schemas
make keygen              # regenerate signing keys
make recreate-server     # apply new env vars to server
```