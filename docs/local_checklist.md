# Local Testing Checklist

## Initial Setup Verification

1. Environment Setup
```bash
# Check environment
make check-env

# Expected Output:
✅ PostgreSQL is running
✅ Required tools installed
✅ Environment variables set
```

2. Database Setup
```bash
# Reset database
make reset

# Expected Output:
🗑️ Cleaning up...
🔄 Running migrations...
🌱 Seeding database...
✅ Database ready
```

3. Service Health
```bash
# Check all services
make health

# Expected Output:
✅ Database connection successful
✅ API responding
✅ UI server running
```

## API Testing

1. Authentication
```bash
# Test admin login
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@zibelina.com","password":"admin123"}'

# Expected: JWT token response
```

2. API Documentation
- Browse to: http://localhost:8000/docs
- Check all endpoints are documented
- Verify request/response examples
- Test endpoints directly from Swagger UI

3. API Health
```bash
# Check API endpoints
curl http://localhost:8000/health
curl http://localhost:8000/api/products
curl http://localhost:8000/api/users
```

## UI Development

1. Development Server
```bash
# Start UI development
make ui-dev

# Verify in browser:
- http://localhost:8080
- Hot reload working
- No console errors
```

2. Build Process
```bash
# Build UI
make build-ui

# Check build output:
- ui/dist/ directory created
- No build errors
- Assets optimized
```

3. Type Checking
```bash
# Check types
cd ui && cargo check
```

## Database Operations

1. Migration Testing
```bash
# Create test migration
diesel migration generate test_migration

# Apply migration
make migrate

# Verify in database
psql $DATABASE_URL -c "\dt"
```

2. Seed Data
```bash
# Run seeder
make seed

# Verify in database:
psql $DATABASE_URL -c "SELECT * FROM users;"
psql $DATABASE_URL -c "SELECT * FROM products;"
```

3. Backup/Restore
```bash
# Create backup
make backup

# Restore from backup
make restore
```

## Docker Testing

1. Container Build
```bash
# Build containers
docker-compose build

# Expected Output:
✅ zibelina-store-ui built
✅ zibelina-store-api built
✅ zibelina-store-pg built
```

2. Container Runtime
```bash
# Start containers
docker-compose up -d

# Check status
docker-compose ps
docker-compose logs
```

3. Container Health
```bash
# Check container health
docker-compose ps
docker-compose exec zibelina-store-pg pg_isready
```

## Security Checks

1. Dependencies
```bash
# Check for vulnerabilities
cargo audit
npm audit (if using Node.js tools)
```

2. Code Analysis
```bash
# Run security lints
cargo clippy
make security-check
```

3. SSL/TLS
```bash
# Check SSL configuration
curl -vI https://localhost
```

## Performance Testing

1. Load Testing
```bash
# Run basic load test
make load-test

# Expected Results:
- Response time < 100ms
- No errors under load
```

2. Memory Usage
```bash
# Check memory usage
docker stats
```

3. Database Performance
```bash
# Check query performance
make db-analyze
```

## Pre-commit Checks

1. Code Quality
```bash
# Format code
make fmt

# Run lints
make lint
```

2. Tests
```bash
# Run all tests
make test

# Expected Output:
✅ Unit tests passed
✅ Integration tests passed
```

3. Documentation
```bash
# Generate docs
make docs

# Check coverage
make doc-coverage
```

## URLs to Check

- UI: http://localhost:8080
  - [ ] Home page loads
  - [ ] Login works
  - [ ] Products display
  - [ ] Cart functions

- API: http://localhost:8000
  - [ ] /health returns OK
  - [ ] /docs is accessible
  - [ ] /api/products returns data

- Admin: http://localhost:8080/admin
  - [ ] Login works
  - [ ] Dashboard loads
  - [ ] CRUD operations work

## Common Issues & Fixes

1. Database Connection
```bash
# Reset database
make reset-db

# Check logs
make logs-db
```

2. API Issues
```bash
# Clear API cache
make clear-cache

# Restart API
make restart-api
```

3. UI Issues
```bash
# Clear dependencies
make clean-ui
make install-ui

# Rebuild UI
make rebuild-ui
```

## Ready for Development Checklist

- [ ] Environment setup complete
- [ ] Database migrations applied
- [ ] Seed data loaded
- [ ] API running and healthy
- [ ] UI development server working
- [ ] Docker containers running
- [ ] Documentation accessible
- [ ] Test suite passing
- [ ] Security checks passed
- [ ] Performance baseline established