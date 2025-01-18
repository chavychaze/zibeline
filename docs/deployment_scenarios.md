# Deployment Scenarios

## Local Development
```mermaid
graph LR
    Dev[Developer] --> Local[Local Environment]
    Local --> UI[UI :8080]
    Local --> API[API :8000]
    Local --> DB[(PostgreSQL :5432)]
```

### Setup
```bash
# Start local environment
make dev

# Start with hot reload
make watch
```

## Staging Environment
```mermaid
graph LR
    Dev --> GitHub
    GitHub --> Actions[GitHub Actions]
    Actions --> Staging[Staging Server]
    Staging --> SUI[UI :80]
    Staging --> SAPI[API :8000]
    Staging --> SDB[(PostgreSQL)]
```

### Configuration
```yaml
# staging.yml
environment:
  name: staging
  domain: staging.zibelina.com
  ssl: true
  monitoring: basic
```

## Production Environment
```mermaid
graph TB
    Users --> CDN
    CDN --> LB[Load Balancer]
    LB --> UI1[UI Server 1]
    LB --> UI2[UI Server 2]
    LB --> API1[API Server 1]
    LB --> API2[API Server 2]
    API1 --> DB[(Primary DB)]
    API2 --> DB
    DB --> Replica[(DB Replica)]
```

### High Availability Setup
```yaml
# docker-compose.prod.yml
services:
  api:
    deploy:
      replicas: 2
      update_config:
        order: start-first
        failure_action: rollback
      restart_policy:
        condition: any
```

## Microservices Architecture
```mermaid
graph TB
    Gateway --> Auth
    Gateway --> Orders
    Gateway --> Products
    Gateway --> Users
    Orders --> NATS
    Products --> NATS
    Users --> NATS
```

### Service Mesh Configuration
```yaml
# istio-config.yml
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: zibelina-routing
spec:
  hosts:
  - "*.zibelina.com"
  http:
  - match:
    - uri:
        prefix: /api/auth
    route:
    - destination:
        host: auth-service
```

## Cloud Deployment

### AWS Setup
```terraform
# main.tf
provider "aws" {
  region = "us-west-2"
}

module "zibelina" {
  source = "./modules/zibelina"
  environment = "production"
  domain = "zibelina.com"
}
```

### Digital Ocean Setup
```terraform
# digitalocean.tf
resource "digitalocean_kubernetes_cluster" "zibelina" {
  name    = "zibelina-cluster"
  region  = "nyc1"
  version = "1.24.4-do.0"
  
  node_pool {
    name       = "worker-pool"
    size       = "s-2vcpu-4gb"
    node_count = 3
  }
}
```

## Bare Metal Deployment

### Server Setup
```bash
# Install requirements
apt update && apt install -y \
    docker.io \
    docker-compose \
    nginx \
    certbot

# Setup certificates
certbot --nginx -d zibelina.com
```

### Monitoring Setup
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'zibelina'
    static_configs:
      - targets: ['localhost:9090']
```

## Development Pipeline
```mermaid
graph LR
    Dev[Development] --> Test[Testing]
    Test --> Staging[Staging]
    Staging --> Prod[Production]
```

### Deployment Checks
```bash
# Pre-deployment checks
make check-security
make check-performance
make check-dependencies

# Post-deployment checks
make verify-deployment
make monitor-errors
```

## Rollback Procedures

### Quick Rollback
```bash
# Rollback to previous version
make rollback

# Verify rollback
make health-check
```

### Database Rollback
```bash
# Revert last migration
diesel migration revert

# Restore from backup
make db-restore backup=backup_20240118.sql
```