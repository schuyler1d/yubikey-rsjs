# YubiKey Rust-nodeJS bridge experiment

How to create an API using [Rust yubikey crate](https://crates.io/crates/yubikey) to Node.

I'm specifically interested in using the smart-card features.

This is edited from the [hello-world example](https://github.com/neon-bindings/examples) of the [neon rust-node](https://neon-bindings.com/) bridge.

Eventually, we will want this to work in Electron ( [neon docs](https://neon-bindings.com/docs/electron-apps/) )

Run in the directory
```bash
rustup override set nightly
rustup target add wasm32-wasi
nvm use 12

npm install
```

which will build a file called `index.node` in the base directory.  You can then plug-in a YubiKey
and run

```js
x = require('.')
x.yubikeySerial()
```

and should get the serial number of your yubikey.
