Retired in favor of using [hidapitester](https://github.com/todbot/hidapitester) see [commit](https://github.com/willbush/system/commit/c70cc7466c7a8322ddb7bd7a3461be0a4d1d93c5)

# lianli-pwm-rgb-sync

A tiny hack that enables the following for the LianLi-UNI SL-Infinity:

- Motherboard PWM Sync
- LED ARGB Header Sync

Colors then be controlled via https://openrgb.org/.

For more features and device support check out [uni-sync](https://github.com/EightB1ts/uni-sync).

## build and run

Dependencies `pkg-config` and `udev` are needed at build / run-time. A nix flake is provided with everything needed. Just `nix develop` or use `direnv allow .`.

```sh
$ cargo build --release
$ sudo ./target/release/lianli-pwm-rgb-sync
```

Note the program has to run with elevated privileges.
