# stego-sign — Makefile

# -- environment variables
include .env
export

# -- config
DB_CONTAINER     ?= stego-db
DB_USER          ?= $(POSTGRES_USER)
DB_NAME          ?= $(POSTGRES_DB)
SERVER_CONTAINER ?= stego-server
APP_CONTAINER    ?= stego-app

# -- detect if DB is running in Docker or local
# -- if DB_CONTAINER is running, use docker exec; otherwise use psql directly
DB_RUNNING := $(shell docker ps --format '{{.Names}}' 2>/dev/null | grep -w $(DB_CONTAINER))

ifeq ($(DB_RUNNING),$(DB_CONTAINER))
  PSQL = docker exec -i $(DB_CONTAINER) psql -U $(DB_USER) -d $(DB_NAME)
else
  PSQL = psql "$(DATABASE_URL)"
endif

MIGRATIONS_DIR = server/migrations

# -- help
.DEFAULT_GOAL := help

help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# -- docker compose
up: ## Start all services
	docker compose up -d

up-infra: ## Start only db and aistor (for local dev)
	docker compose up -d db aistor

down: ## Stop all services
	docker compose down

down-v: ## Stop all services and remove volumes
	docker compose down -v

restart: ## Restart all services
	docker compose restart

build: ## Build all Docker images
	docker compose build

build-server: ## Build server image only
	docker compose build server

build-app: ## Build app image only
	docker compose build app

logs: ## Follow logs (usage: make logs s=server)
	docker compose logs -f $(s)

ps: ## Show running containers
	docker compose ps

# -- database migrations
migrate: ## Apply all schemas (files first, then app)
	@echo "→ Applying schema_files..."
	@$(PSQL) < $(MIGRATIONS_DIR)/schema_files.sql
	@echo "→ Applying schema_app..."
	@$(PSQL) < $(MIGRATIONS_DIR)/schema_app.sql
	@echo "✓ Migrations applied"

migrate-reset: ## Reset all data (keeps schema)
	@echo "→ Resetting schema_app data..."
	@$(PSQL) < $(MIGRATIONS_DIR)/reset_schema_app.sql
	@echo "→ Resetting schema_files data..."
	@$(PSQL) < $(MIGRATIONS_DIR)/reset_schema_files.sql
	@echo "✓ Data reset"

migrate-drop: ## Drop all schemas completely
	@echo "→ Dropping schema_app..."
	@$(PSQL) < $(MIGRATIONS_DIR)/delete_schema_app.sql
	@echo "→ Dropping schema_files..."
	@$(PSQL) < $(MIGRATIONS_DIR)/delete_schema_files.sql
	@echo "✓ Schemas dropped"

migrate-fresh: migrate-drop migrate ## Drop and re-apply all schemas

# -- local development
dev-server: ## Run server locally (requires db + aistor running)
	cd server && cargo run

dev-app: ## Run app locally (requires server running)
	cd app && cargo leptos watch

dev: up-infra ## Start infra + run server and app locally
	@echo "→ Infrastructure ready"
	@echo "→ Run 'make dev-server' and 'make dev-app' in separate terminals"

# -- keys
keygen: ## Generate Ed25519 key pair and print to stdout
	@echo "→ Generating keys (server must be running on port $(SERVER_PORT))..."
	@curl -s http://localhost:$(SERVER_PORT)/api/v1/admin/keygen | python3 -m json.tool

# -- cleanup
clean: ## Remove build artifacts
	cd server && cargo clean
	cd app && cargo clean

prune: ## Remove unused Docker resources
	docker system prune -f