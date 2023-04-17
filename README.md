# Axum Tutorial Follow Along

Following along this excellent tutorial - [Rust Axum Full Course](https://youtu.be/XZtlD_m59sM)

## Development Commands

NOTE: Need --poll for my Fedora Linux setup.

```sh
# Server
$ cargo watch -q -c --poll -w src/ -x run

# Tests
$ cargo watch -q -c --poll -w tests/ -x "test -q quick_dev -- --nocapture"
```
