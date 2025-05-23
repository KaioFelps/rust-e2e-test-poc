# e2e tests proof-of-concept
This repository contains the source code of a todo list. The goal is to write asynchronous end-to-end tests for each
of the endpoints, being it a simple view response or an action.

## Testing
```bash
docker compose --env-file .env.test up -d
npm ci && npm run build # needed because tests rely on generated manifest.json
cargo test
```

## Running
First of all, start the docker containers needed by the application:
```bash
docker compose up -d
```

In one terminal, start the front-end dev server:
```bash
npm install
npm run dev
```

In a second one, start the back-end dev server:
```bash
cargo run
```

## Tech Stack
These tests rely only on cargo test and rstest. For reaching tearup and teardrop, it uses rstest fixtures feature and rust
drop descontructors. It aint the most beautiful test syntax, but works just fine.

## Insights
An more advanced way of improving the tests look would be developing an attribute which takes some setup and teardrop
methods just like rstest's fixtures, but returning always a rust void (`()`). This attribute would bootstrap tokio's
runtime and run each setup sequentially, then run the test, then run each teardrop.

It's good to remember that each test is already executed as a binary apart.

## References
- [SeaORM tests](https://github.com/SeaQL/sea-orm/blob/master/tests/crud_tests.rs)
- [sqlx macro](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
- https://lik.ai/blog/async-setup-and-teardown-in-rust