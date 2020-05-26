# Android with Rust

An example app showing the integration of Rust [Tokio.rs](https://tokio.rs/) with `Android` and it uses an event-based bridge for `two-way communication`. It allows you
to write async rust code (thanks to [Tokio.rs](https://tokio.rs/) runtime) in `Android` for better performance and it can also be used as a worker alternative of `Java thread pool`.

It creates a bridge wrapper between `Java` and `Rust` environment so that you don't have to deal with [`jni-rs`](https://github.com/jni-rs/jni-rs/) frequently.

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

Start the dev hot rebuild process:

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

See logcat via `ADB`:

```sh
tusk log
```

Please refer to `tusk.yml` file for more commands.

## Reference links

1. For more [info](https://medium.com/visly/rust-on-android-19f34a2fb43).

2. Array signature encoding: https://stackoverflow.com/questions/28780623/sending-jbyte-array-to-java-method-failed-using-jni

3. Cache the method id or field id to call a method or to access a field repeatedly from java object for better performance:
https://docs.oracle.com/en/java/javase/11/docs/specs/jni/design.html#accessing-fields-and-methods