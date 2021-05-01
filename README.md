# ingress-status-sync

A Kubernetes controller. It watches for ingresses with the annotation `ingress-status-sync.wiaph.one/enabled: 'true'`. For each one, it populates `.status.loadBalancer.ingress` with the IPs of the nodes running a target service.

Usage:

```sh
ingress-status-sync \
    --forever \
    --target-service-namespace=ingress-nginx \
    --target-service-name=ingress-nginx-controller
```

## Development

### Install prerequisites

- [Rust]
- [pre-commit]

[Rust]: https://www.rust-lang.org/
[pre-commit]: https://pre-commit.com/

### Install the pre-commit hook

```sh
pre-commit install
```

This installs a Git hook that runs a quick sanity check before every commit.

### Run the app

```sh
cargo run
```

### Run the tests

```sh
cargo test
```
