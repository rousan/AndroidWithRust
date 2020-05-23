# Android with Rust

An example app showing how to integrate Rust [Tokio.rs](https://tokio.rs/) runtime as worker to run async rust code in Android for better performance.

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

## Reference links

For more [info](https://medium.com/visly/rust-on-android-19f34a2fb43).

1. https://stackoverflow.com/questions/28780623/sending-jbyte-array-to-java-method-failed-using-jni