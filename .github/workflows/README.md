# GitHub Actions Workflow

This directory contains CI/CD pipeline configurations for the Zibelina Store project.

## Setup

### GitHub Repository Setup

1. Create these repository secrets:
   - `SSH_PRIVATE_KEY`: Your deployment server's SSH private key
   - `SERVER_HOST`: Your server's hostname or IP
   - `SERVER_USER`: SSH username on deployment server
   - `DEPLOY_PATH`: Deployment directory path (e.g., /opt/zibelina-store)

To add secrets:
1. Go to your repository on GitHub
2. Navigate to Settings > Secrets and variables > Actions
3. Click "New repository secret"
4. Add each secret listed above

### Branch Protection (Optional)

Set up branch protection rules for `main`:

1. Go to Settings > Branches
2. Add rule for `main` branch
3. Enable:
   - Require pull request reviews
   - Require status checks to pass
   - Require branches to be up to date

## Local Development Setup

For local development without triggering GitHub Actions:

1. Install dependencies:
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL
sudo apt install postgresql-16

# Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

2. Setup database:
```bash
make setup-local
```

3. Run migrations and seeds:
```bash
make migrate
make seed
```

4. Start the application:
```bash
make run
```

## Pipeline Stages

### 1. Test
- Runs on pull requests and pushes to main
- Sets up PostgreSQL service container
- Runs migrations
- Executes cargo tests

### 2. Build
- Builds Docker image
- Only runs if tests pass
- Caches dependencies

### 3. Deploy
- Only runs on main branch
- Deploys to server using SSH
- Runs migrations and restarts services

## Workflow File Structure

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    # Test job configuration
  build:
    # Build job configuration
  deploy:
    # Deploy job configuration
```

## Customization

### Adding New Jobs

1. Create new job in `main.yml`:
```yaml
new_job:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - name: Your Step
      run: echo "New job"
```

2. Add job dependency if needed:
```yaml
new_job:
  needs: [test, build]
```

### Environment Variables

Add new variables in two places:
1. GitHub repository secrets for sensitive data
2. `env:` section in workflow file for public values