# security-core-ffi-nodejs-binding

Node.js binding for the Secure Core FFI AES-256-GCM native library, loaded with `koffi`.

## Install

```bash
npm install security-core-ffi-nodejs-binding
```

This package is a JavaScript binding; it does not embed platform binaries. Download the matching native archive for Linux x86_64, Windows x86_64, or macOS arm64 from the corresponding GitHub Release, verify its checksum, and expose the absolute library path through `SECURITY_CORE_LIB`.

```javascript
const crypto = require("node:crypto");
const { SecurityCore } = require("security-core-ffi-nodejs-binding");

const core = new SecurityCore(crypto.randomBytes(32), process.env.SECURITY_CORE_LIB);
try {
  const encrypted = core.encrypt(Buffer.from("sensitive data"));
  const plaintext = core.decrypt(encrypted);
  console.log(plaintext.toString());
} finally {
  core.close();
}
```

## Security boundary

The application owns key generation, storage, authorization, rotation, and incident response. Do not use browser-held keys for server-controlled secrets. See the project threat model and key-management guidance before production use.
