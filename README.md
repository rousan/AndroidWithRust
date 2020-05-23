# Android with Rust

An example Android app showing two-way communication bridge with Rust backed by Tokio.rs as worker which makes it possible to write async Rust code.

## Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

## Development

Setup the project by installing all the required dev tools:

```sh
tusk setup
```

Start the dev hot rebuild:

```sh
tusk dev
```

Build release library files:

```sh
tusk build:prod
```

Generate jni files:

```sh
tusk jni
```

Please refer to `tusk.yml` file for more commands.

## Reference link

For more [info](https://medium.com/visly/rust-on-android-19f34a2fb43).