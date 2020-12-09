# ecies-ed25519-wasm

A WASM binding for [`ecies-ed25519`](https://github.com/phayes/ecies-ed25519).

## Usage

```js
import * as ed25519 from "ecies-ed25519-wasm";

const data = Uint8Array.from([1, 2, 3, 4]);
const [sk, pk] = ed25519.generate_keypair();

const encrypted = ed25519.encrypt(pk, data);
const decrypted = ed25519.decrypt(sk, encrypted);

alert("ed25519 decrypted: " + decrypted);
```
