.PHONY: all setup dev build test clean deploy reset migrate seed check ui-dev api-dev install docs db-up db-down test-api test-ui build-api build-ui

include .env
export

# Default target
all: dev

# Create required directories
create-dirs:
	@mkdir -p postgres-data
	@chmod 777 postgres-data

# Clean generated directories by reading from .gitignore
clean-generated: clean
	@echo "Cleaning generated directories..."
	@echo "Finding directories from .gitignore..."
	@for dir in $$(grep -v '^#' .gitignore | grep '/' | tr -d '/'); do \
		echo "Cleaning $$dir..."; \
		find . -type d -name "$$dir" -exec rm -rf {} +; \
	done
	@echo "Cleaning specific nested directories..."
	@find . -type d -name "target" -exec rm -rf {} +
	@find . -type d -name "debug" -exec rm -rf {} +
	@find . -type d -name "dist" -exec rm -rf {} +
	@find . -name "*.rs.bk" -delete
	@find . -name "*.pdb" -delete
	@find . -name "cargo.log" -delete
	@find . -name ".DS_Store" -delete
	@echo "✨ Cleaned all generated directories and files"

# List all directories that will be cleaned
list-generated:
	@echo "Directories that will be cleaned:"
	@echo "From .gitignore:"
	@grep -v '^#' .gitignore | grep '/' | tr -d '/'
	@echo "\nAdditional directories:"
	@echo "- All 'target' directories"
	@echo "- All 'debug' directories"
	@echo "- All 'dist' directories"
	@echo "\nFiles that will be cleaned:"
	@echo "- *.rs.bk"
	@echo "- *.pdb"
	@echo "- cargo.log"
	@echo "- .DS_Store"

# Clean up
clean: db-down clean-generated
	@echo "Cleaning up..."
	@docker-compose down -v
	@rm -rf ui/dist
	@echo "✨ Cleanup complete"

# Start PostgreSQL
db-up: create-dirs
	@echo "Starting PostgreSQL..."
	@docker-compose up -d zibelina-store-pg
	@echo "Waiting for PostgreSQL to be ready..."
	@until docker-compose exec -T zibelina-store-pg pg_isready -U user -d zibelina; do \
		echo "PostgreSQL is unavailable - sleeping"; \
		sleep 1; \
	done
	@echo "PostgreSQL is ready!"

# Initialize scripts
init:
	@chmod +x scripts/main.lua
	@chmod +x scripts/setup.sh
	@./scripts/setup.sh

# Start PostgreSQL
db-up:
	@echo "Starting PostgreSQL..."
	@docker-compose up -d zibelina-store-pg
	@echo "Waiting for PostgreSQL to be ready..."
	@until docker-compose exec -T zibelina-store-pg pg_isready -U user -d zibelina; do \
		echo "PostgreSQL is unavailable - sleeping"; \
		sleep 1; \
	done
	@echo "PostgreSQL is ready!"

# Stop PostgreSQL
db-down:
	@echo "Stopping PostgreSQL..."
	@docker-compose stop zibelina-store-pg
	@docker-compose rm -f zibelina-store-pg

# Setup development environment
setup: init db-up
	@echo "🚀 Setting up development environment..."
	@command -v lua >/dev/null 2>&1 || { echo "Installing Lua..."; \
		brew install lua; }
	@./scripts/main.lua ui install
	@command -v diesel >/dev/null 2>&1 || { echo "Installing Diesel CLI..."; \
		cargo install diesel_cli --no-default-features --features postgres; }
	@make migrate
	@make seed

# Development mode (with UI)
dev: db-up
	@echo "Starting development environment..."
	@tmux new-session -d -s zibelina \
		'cd server && cargo run' \; \
		split-window -h 'cd ui && trunk serve' \; \
		attach || \
	{ echo "Starting services separately..."; \
		cd server && cargo run & \
		cd ui && trunk serve; }

# API development only
api-dev: db-up
	@echo "Starting API server..."
	@cd server && cargo run

# UI development only
ui-dev:
	@echo "Starting UI server..."
	@cd ui && trunk serve

# Build everything
build: build-api build-ui

# Build API
build-api:
	@echo "Building API..."
	@cd server && cargo build --release

# Build UI
build-ui:
	@echo "Building UI..."
	@cd ui && trunk build --release

# Run all tests
test: test-api test-ui

# Run API tests
test-api: db-up
	@echo "Running API tests..."
	@cd server && cargo test

# Run UI tests
test-ui:
	@echo "Running UI tests..."
	@cd ui && cargo test

# Clean up
clean: db-down
	@echo "Cleaning up..."
	@docker-compose down -v
	@rm -rf target
	@rm -rf ui/dist
	@rm -rf postgres-data

# Reset everything
reset: clean setup

# Database operations
# Setup diesel
db-setup: db-up
	@echo "Setting up diesel..."
	@cd server/infrastructure && diesel setup

# Run migrations
migrate: db-up
	@echo "Running migrations..."
	@cd server/infrastructure && DATABASE_URL="postgres://user:password@localhost/zibelina" diesel migration run

# Create new migration
migrate-create:
	@read -p "Enter migration name: " name; \
	cd server/infrastructure && diesel migration generate $$name; \
	echo "Migration created in server/infrastructure/migrations/"

# Redo last migration
migrate-redo: db-up
	@echo "Redoing last migration..."
	@cd server/infrastructure && DATABASE_URL="postgres://user:password@localhost/zibelina" diesel migration redo

# Create new seed file
create-seed:
	@read -p "Enter seed name: " name; \
	timestamp=$$(date +%Y%m%d%H%M%S); \
	mkdir -p server/infrastructure/seeds; \
	file_path="server/infrastructure/seeds/$${timestamp}_$${name}.sql"; \
	echo "-- Seed: $$name\n-- Created at: $$(date)\n\nBEGIN;\n\n-- Add your seed data here\n-- Example:\n-- INSERT INTO table_name (column1, column2) VALUES ('value1', 'value2');\n\nCOMMIT;" > $$file_path; \
	echo "Created seed file: $$file_path"

# Run seeds
seed: db-up
	@echo "Running seeds..."
	@for file in server/infrastructure/seeds/*.sql; do \
		echo "Applying seed: $$file"; \
		PGPASSWORD=password psql -h localhost -U user -d zibelina -f $$file; \
	done

# Reset database (drop, create, migrate, seed)
db-reset: db-up
	@echo "Resetting database..."
	@cd server/infrastructure && DATABASE_URL="postgres://user:password@localhost/zibelina" diesel database reset
	@make migrate
	@make seed

# Database shell
db-shell: db-up
	@docker-compose exec zibelina-store-pg psql -U user -d zibelina

# Check environment
check-env:
	@echo "Checking environment..."
	@command -v docker >/dev/null 2>&1 || echo "❌ Docker not installed"
	@command -v docker-compose >/dev/null 2>&1 || echo "❌ Docker Compose not installed"
	@command -v psql >/dev/null 2>&1 || echo "❌ PostgreSQL client not installed"
	@command -v cargo >/dev/null 2>&1 || echo "❌ Rust not installed"
	@command -v lua >/dev/null 2>&1 || echo "❌ Lua not installed"
	@test -f .env || echo "❌ .env file missing"
	@echo "✅ Environment check complete"

# Deploy to production
deploy: build
	@docker-compose up -d

# Generate documentation
docs:
	@cd server && cargo doc --no-deps
	@cd ui && cargo doc --no-deps
	@echo "Documentation generated in target/doc"

# Install dependencies
install:
	@cargo install trunk wasm-bindgen-cli
	@rustup target add wasm32-unknown-unknown

# Development shell
shell:
	@docker-compose exec zibelina-store-pg psql -U user -d zibelina