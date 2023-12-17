# About

Extract a personal access token from a [Woodpecker CI](https://github.com/woodpecker-ci/woodpecker)
sqlite database.

It can be used to intialise the `PICUS_WOODPECKER_TOKEN` environment variable
of [Picus](https://github.com/windsource/picus) when the Picus deamon run on
the same machine as the Woodpecker server.


# Usage

```
Extract a personal access token from a Woodpecker CI sqlite database

Usage: woodpecker-pat [OPTIONS] [SQLITE_FILE]

Arguments:
  [SQLITE_FILE]  Sets the SQLite file [default: woodpecker.sqlite]

Options:
  -u <USER_LOGIN>      Sets the username (default: the first created user)
  -h, --help           Print help
  -V, --version        Print version
```
