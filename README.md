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

# Future

I might replace this with a script that calls [hidapitester](https://github.com/todbot/hidapitester) in the future if it makes its way into [nixpkgs](https://github.com/NixOS/nixpkgs).

# NixOS systemd service

[Here](https://github.com/willbush/system/commit/91c8e1b76852125406ce0c2b152f94592d07ca8a) is an example of how to run this with systemd in NixOS.
