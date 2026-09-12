# secure-core-ffi-python

Native Python bindings (PyO3) for `secure-core-ffi` — AES-256-GCM
encryption, backed by the same Rust core used by the C/C++, Node.js,
and WebAssembly bindings.

## Usage

```python
from secure_core_ffi import SecurityCore

core = SecurityCore(key)          # key must be exactly 32 bytes
ciphertext = core.encrypt(b"secret data")
plaintext = core.decrypt(ciphertext)
```

See the main [repository README](../../README.md) for more details.
