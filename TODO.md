 # TODO List

### General

 [] Makefile
 [] Scripts for local development
 [] Set Up a Seeder for DB
 [] Fix Docker
 [] Add roles / Auth microservice
 [] move to gRpc
 [] Admin space / microservice
 [] NATS / microservice
 [] k8s - something similar but cheep
 [] Skaffold - ?? not sure if I need it at all

#### Late development
 
 [] Add Blockchain and token for transactions /microservice
 [] Add Blockchain for purchasing products
 [] Add purchaise in USDT
 [] Add LLM in order support (search for free one)
 [] Research for using a LLM in this kind of interaction 

## Infrastructure & Setup
- [ ] Makefile
  - [ ] Add watch mode with hot reload
  - [ ] Add performance profiling targets
  - [ ] Add security scanning targets
  - [ ] Add documentation generation targets
  - [ ] Add database backup/restore targets

- [ ] Scripts for Local Development
  - [ ] Add development environment validation
  - [ ] Add automated dependency checks
  - [ ] Add local SSL setup
  - [ ] Add development database backup/restore
  - [ ] Add log rotation

- [ ] Set Up DB Seeder
  - [ ] Add faker data generation
  - [ ] Add different seed profiles (minimal, full, test)
  - [ ] Add seed data versioning
  - [ ] Add seed data cleanup

- [ ] Fix Docker
  - [ ] Optimize container sizes
  - [ ] Add multi-stage builds
  - [ ] Implement proper health checks
  - [ ] Add Docker layer caching
  - [ ] Setup development containers

## Architecture & Services

- [ ] Auth Microservice
  - [ ] Implement JWT authentication
  - [ ] Add OAuth2 providers
  - [ ] Add role-based access control
  - [ ] Add 2FA support
  - [ ] Add session management

- [ ] Move to gRPC
  - [ ] Design service protocols
  - [ ] Implement service discovery
  - [ ] Add protocol buffers
  - [ ] Setup bidirectional streaming
  - [ ] Add gRPC gateway for REST compatibility

- [ ] Admin Space Microservice
  - [ ] Implement dashboard
  - [ ] Add analytics
  - [ ] Add user management
  - [ ] Add content management
  - [ ] Add reporting system

- [ ] NATS Microservice
  - [ ] Setup message queues
  - [ ] Implement event sourcing
  - [ ] Add message persistence
  - [ ] Setup pub/sub patterns
  - [ ] Add message replay capability

- [ ] Kubernetes Alternative
  - [ ] Research k3s
  - [ ] Look into Nomad
  - [ ] Consider Docker Swarm
  - [ ] Evaluate Podman
  - [ ] Research cost-effective orchestration

- [ ] Skaffold Evaluation
  - [ ] Research benefits for project
  - [ ] Compare with alternatives
  - [ ] Evaluate cost-benefit ratio
  - [ ] Consider simpler alternatives

## Testing & Quality
- [ ] Add comprehensive testing
  - [ ] Unit tests
  - [ ] Integration tests
  - [ ] E2E tests
  - [ ] Performance tests
  - [ ] Security tests

- [ ] Setup CI/CD
  - [ ] Add automated testing
  - [ ] Add deployment pipelines
  - [ ] Add security scanning
  - [ ] Add performance monitoring
  - [ ] Add automated rollbacks

## Late Development Features

- [ ] Blockchain Integration
  - [ ] Research optimal blockchain platform
  - [ ] Design smart contracts
  - [ ] Implement token system
  - [ ] Add wallet integration
  - [ ] Setup transaction monitoring

- [ ] Cryptocurrency Payments
  - [ ] Add USDT integration
  - [ ] Implement multi-currency support
  - [ ] Add payment verification
  - [ ] Setup exchange rate management
  - [ ] Add payment reconciliation

- [ ] LLM Integration
  - [ ] Research open-source LLMs
    - [ ] Look into Llama 2
    - [ ] Evaluate Mistral AI
    - [ ] Consider OpenAssistant
  - [ ] Order support implementation
    - [ ] Add product recommendations
    - [ ] Implement chatbot support
    - [ ] Add order status queries
  - [ ] Cost analysis
    - [ ] Compare hosting options
    - [ ] Evaluate inference costs
    - [ ] Consider hybrid approaches

## Additional Best Practices

- [ ] Testing Improvements
  - [ ] Add integration tests between UI and API
  - [ ] Add E2E tests with Playwright
  - [ ] Implement API contract testing
  - [ ] Add performance benchmarks
  - [ ] Setup mutation testing

- [ ] Documentation
  - [ ] Add OpenAPI/Swagger docs
  - [ ] Create architecture decision records
  - [ ] Add user documentation
  - [ ] Setup automated doc generation
  - [ ] Add code examples and tutorials

- [ ] Monitoring & Observability
  - [ ] Add Prometheus metrics
  - [ ] Setup Grafana dashboards
  - [ ] Implement error tracking with Sentry
  - [ ] Add distributed tracing
  - [ ] Setup log aggregation

- [ ] Development Experience
  - [ ] Add git hooks for quality checks
  - [ ] Setup IDE configurations
  - [ ] Add development containers
  - [ ] Implement automated code review
  - [ ] Add development guidelines

- [ ] Security
  - [ ] Add security headers
  - [ ] Implement rate limiting
  - [ ] Add CSRF protection
  - [ ] Setup security scanning
  - [ ] Implement audit logging