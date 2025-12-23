# Bipa API

## Build tools & versions used

Required:
- **Rust**: 1.91 (as defined in `rust-toolchain`)
- **Task**: Task automation tool (alternative to Make)
- **Docker & Docker Compose**: For running infrastructure services (Postgres, LocalStack)
- **Cargo Nextest**: For optimized test execution

Optional:
- **Kubernetes**: For running the API and worker together locally

## Steps to run the app

> [!WARNING]
> This project uses [Task](https://taskfile.dev/) to manage commands. [Here](https://taskfile.dev/docs/installation) are the installation instructions.

### Kubernates

> [!WARNING]
> Required Kubernetes running locally.

To run the application locally with Kubernetes
```shell
task k8s:up
```

> [!INFO]
> When the application starts, it will take 1 minute to worker start fetching data from the external API.
> The API will return an empty array until the worker has finished.

[Link to fetch data from api running on k8s](http://localhost:30000/nodes)

To stop the application running on Kubernetes
```shell
task k8s:down
```

### Docker

> [!WARNING]
> Required Docker running locally.

To run the application api locally with Docker
```shell
task run:api
```

> [!INFO]
> The API will return an empty array until the worker be executed once.

[Link to fetch data from api standalone](http://localhost:9095/nodes)

[Link to swagger from api standalone](http://localhost:9095/swagger)

[Link to metrics from api standalone](http://localhost:9095/metrics)

To run the application worker locally with Docker
```shell
task run:worker
```

### Tests

```shell
task test
```

## What was the reason for your focus? What problems were you trying to solve?
The main focus was to establish a solid and standardized foundation for microservices in Rust using the `derust` library.
The solved problems include:
- **Observability**: Native integration with logs, metrics (Prometheus), and traces.
- **Configuration Management**: Dynamic loading of environment variables and secrets via AWS Secrets Manager (simulated via LocalStack).
- **Standardization**: Well-defined Cargo workspace structure, facilitating the separation between business logic (`core`) and entry interfaces (`api`, `worker`).

## How long did you spend on this project?

I spent approximately 12 hours on development and testing over the course of 3 days.

## Did you make any trade-offs for this project? What would you have done differently with more time?
**Trade-offs:**
- I chose to use my own library (`derust`) to speed up infrastructure setup, which introduces a specific external dependency.

## What do you think is the weakest part of your project?

- Dependency on my own library (derust) — which may introduce a steeper learning curve and is not a market standard. This was a deliberate choice to simplify development while ensuring standardization and observability.
- Lack of pagination when reading from the external API.
- Lack of pagination and filters in the exposed API.
- Kubernetes CronJob running every 1 minute — this could be an issue if a shorter interval is required.

## Is there any other information you’d like us to know?
The project uses the latest Rust editions (2024) and leverages the `tokio`/`axum` ecosystem for high asynchronous performance.