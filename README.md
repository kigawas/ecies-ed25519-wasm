# ecies-ed25519-wasm

A WASM binding for [`ecies-ed25519`](https://github.com/phayes/ecies-ed25519).

## Install

```bash
npm install ecies-ed25519-wasm
```

## Usage

```js
import * as ed25519 from "ecies-ed25519-wasm";

const data = Uint8Array.from([1, 2, 3, 4]);
const [sk, pk] = ed25519.generate_keypair();

const encrypted = ed25519.encrypt(pk, data);
const decrypted = ed25519.decrypt(sk, encrypted);

alert("ed25519 decrypted: " + decrypted);
```

Check [this example](https://github.com/ecies/wasm-example) for more details.

## API

```ts
function generate_keypair(): Array<any>;
function encrypt(receiver_pub: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
function decrypt(receiver_sec: Uint8Array, msg: Uint8Array): Uint8Array | undefined;
```
