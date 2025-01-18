# Zibelina Store

Full-stack e-commerce platform built with Rust, featuring a WebAssembly frontend and PostgreSQL database.

- 🚀 High-performance Rust backend
- 🌐 WebAssembly frontend with Yew
- 📦 PostgreSQL database
- 🐳 Docker containerization
- 🔄 Hot reload development
- 🔒 Role-based authentication

## Table of Contents
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Development Workflows](#development-workflows)
  - [Local Development](#local-development)
  - [Docker Development](#docker-development)
- [Development Commands](#development-commands)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Documentation](#documentation)
- [License](#license)

## Prerequisites

### Required Software
- Docker & Docker Compose
- Rust (1.70.0 or later)
- Lua 5.3+
- PostgreSQL client (for development tools)

### Installation

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Install other dependencies
brew install lua postgresql@16 docker docker-compose
```

#### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Install other dependencies
sudo apt update
sudo apt install -y lua5.3 postgresql-client-16 docker.io docker-compose
```

## Quick Start
```bash
# Clone repository
git clone git@github.com:chavychaze/zibeline.git
cd zibeline

# Setup environment
cp .env.example .env
make check-env     # Verify environment setup
make setup         # Setup development environment

# Start development environment
make dev
```

Access your applications:
- Frontend: http://localhost:8080
- API: http://localhost:8000
- API Docs: http://localhost:8000/docs

## Development Workflows

### Local Development

#### Option 1: Full-Stack (Single Command)
Best for full-stack development with hot reload:
```bash
make dev
```

#### Option 2: Separate Services
Better for focused development on specific components:
```bash
# Terminal 1: Start PostgreSQL
make db-up

# Terminal 2: Start API
make api-dev

# Terminal 3: Start UI
make ui-dev
```

### Docker Development
Run everything in containers:
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

Access your containerized applications:
- Frontend: http://localhost
- API: http://localhost:8000
- API Docs: http://localhost:8000/docs

## Development Commands

### Database Operations
```bash
# Initial setup
make db-setup        # Initial diesel setup
make migrate         # Run migrations
make seed           # Run all seeds

# Development
make db-up          # Start PostgreSQL
make db-down        # Stop PostgreSQL
make db-reset       # Reset database
make db-shell       # Database shell

# Create new files
make migrate-create  # Create new migration
make migrate-redo   # Redo last migration
make create-seed    # Create new seed file
```

### Building
```bash
# Production builds
make build          # Build everything
make build-api      # Build API only
make build-ui       # Build UI only

# Development builds
make dev            # Development mode with hot reload
```

### Testing
```bash
# Run all tests
make test           # All tests
make test-api       # API tests only
make test-ui        # UI tests only
```

### Maintenance
```bash
make clean          # Clean build artifacts
make reset          # Reset everything
make check-env      # Verify environment
make docs           # Generate documentation
```

## Project Structure
```
zibelina-store/
├── .env                    # Environment variables
├── .github/                # GitHub Actions workflows
├── docker-compose.yml      # Docker configuration
├── docs/                   # Documentation
├── migrations/ 
├── server/                 # Backend API (Rust)
│   ├── api/                # Source code
|       ├── src/                # Source code
│       └── tests/              # UI tests
│   └── tests/              # API tests
├── ui/                     # Frontend (Yew/WebAssembly)
│   ├── src/                # Source code
│   └── tests/              # UI tests
└── scripts/                # Management scripts
    ├── lib/                # Lua utilities
    └── commands/           # Lua command modules
    └── server_setup/       # Server setup fo Lua modules
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```env
# Database
DATABASE_URL=postgres://user:password@localhost/zibelina
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=zibelina

# API
API_PORT=8000
RUST_LOG=debug

# UI
UI_PORT=8080
```

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Documentation

- [Development Guide](docs/development.md)
- [Deployment Guide](docs/deployment.md)
- [Security Practices](docs/security.md)
- [Testing Strategy](docs/testing.md)
- [API Documentation](http://localhost:8000/docs)

## License

GNU v2.1 License - see [LICENSE](LICENSE) for details.