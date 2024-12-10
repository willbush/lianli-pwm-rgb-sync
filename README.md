# lianli-pwm-rgb-sync

A tiny hack that enables the following for the LianLi-UNI SL-Infinity:

- Motherboard PWM Sync
- LED ARGB Header Sync

Colors then be controlled via https://openrgb.org/.

For more features and device support check out [uni-sync](https://github.com/EightB1ts/uni-sync).

## build and run

Build dependencies `pkg-config` and `udev` are needed. A nix flake is provided with everything needed. Just `nix develop` or use `direnv`.

```sh
$ cargo build --release
$ sudo ./target/release/lianli-pwm-rgb-sync
```

Note the program has to run with elevated privileges.

# Future

I might replace this with a script that calls [hidapitester](https://github.com/todbot/hidapitester) in the future if it makes its way into [nixpkgs](https://github.com/NixOS/nixpkgs).
