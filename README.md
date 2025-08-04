# License API

`License API` is a simple license server designed to protect software

> [!WARNING]
> Project is currently migrating from Python to Rust.
> The API has lots of changes compared to before

## Installation

```
diesel setup
diesel migration run
cargo run --release
```
> [!NOTE]
> Before running these commands, remember to fill out the .env file

## Client libraries

- [Python](https://github.com/awalki/license-api-py)
- [Rust](https://github.com/awalki/license-api/rs)
- JavaScript [in development]
- C# [in development]
- C++ [in development]

## Roadmap

- [x] Core API functionality
- [ ] API rate limiting
- [ ] Test coverage
- [ ] Web dashboard
- [ ] Docker support
- [ ] Metrics

### License
This project is licensed under the [MIT](https://github.com/awalki/license-api/blob/main/license-api/LICENSE) license.