# Axum Tutorial Follow Along

Following along this excellent tutorial - [Rust Axum Full Course](https://youtu.be/XZtlD_m59sM)

## Development Commands

NOTE: Need --poll for my Fedora Linux setup.

```sh
# Run Server and Tests below in their own terminal window

# Server
$ . ./env  # Sets required DATABASE_URL environment variable
$ cargo watch -q -c --poll -w src/ -x "run --bin main"

# Tests
$ . ./env  # Sets required DATABASE_URL environment variable
$ cargo watch -q -c --poll -w tests/ -x "test -q quick_dev -- --nocapture"
```
