# Bipa API

## Build tools & versions used
- **Rust**: 1.91 (as defined in `rust-toolchain`)
- **Task**: Task automation tool (alternative to Make)
- **Docker & Docker Compose**: For running infrastructure services (Postgres, LocalStack)
- **Cargo Nextest**: For optimized test execution

## Steps to run the app

### 1. Task Installation
This project uses [Task](https://taskfile.dev/) to manage commands. [Here](https://taskfile.dev/docs/installation) are the installation instructions.

### 2. Initial Setup
Run the command below to install Rust helper tools:
```bash
task setup
```

### 3. Run the Application
The command below will spin up the necessary containers (Postgres and LocalStack) and start the API:
```bash
task run
```
The API will be available by default on port `9095`. Swagger can be accessed at `http://localhost:9095/swagger`.

### 4. Run Tests
```bash
task test
```

## What was the reason for your focus? What problems were you trying to solve?
The main focus was to establish a solid and standardized foundation for microservices in Rust using the `derust` library.
The solved problems include:
- **Observability**: Native integration with logs, metrics (Prometheus), and traces.
- **Configuration Management**: Dynamic loading of environment variables and secrets via AWS Secrets Manager (simulated via LocalStack).
- **Standardization**: Well-defined Cargo workspace structure, facilitating the separation between business logic (`core`) and entry interfaces (`api`).

## How long did you spend on this project?
// TODO

## Did you make any trade-offs for this project? What would you have done differently with more time?
**Trade-offs:**
- I chose to use my own library (`derust`) to speed up infrastructure setup, which introduces a specific external dependency.

## What do you think is the weakest part of your project?
// TODO

## Is there any other information you’d like us to know?
The project uses the latest Rust editions (2024) and leverages the `tokio`/`axum` ecosystem for high asynchronous performance.