# WebAssembly Integration in Next.js

🇷🇴 [Citește în Română](README.ro.md)

This folder documents the `Rust -> Wasm -> Next.js` pipeline illustrated in `assets/rust-wasm-integration-guide.jpg`.

## 1. Build Wasm

```bash
cargo install wasm-pack   # one-time
wasm-pack build --target web --out-dir bindings/wasm-nextjs/pkg
```

This produces the `pkg/` folder with `security_wasm.js` and `security_wasm_bg.wasm`.

## 2. `next.config.js` setup

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  experiments: {
    asyncWebAssembly: true,
  },
};

module.exports = nextConfig;
```

## 3. Example React component (`components/SecureComponent.jsx`)

```javascript
"use client";
import { useEffect, useState } from "react";

export default function SecureComponent({ data }) {
  const [encrypted, setEncrypted] = useState(null);

  useEffect(() => {
    const runSecure = async () => {
      const wasm = await import("../bindings/wasm-nextjs/pkg/security_wasm");
      await wasm.default(); // initialize the Wasm module

      const key = crypto.getRandomValues(new Uint8Array(32));
      const ctx = new wasm.SecurityContext(key);

      const encoded = new TextEncoder().encode(data);
      const encryptedBytes = ctx.encrypt(encoded);

      setEncrypted(Buffer.from(encryptedBytes).toString("hex"));
    };

    runSecure();
  }, [data]);

  return (
    <div>
      <h3>Secure Component</h3>
      <p>encrypted data</p>
      <code>{encrypted}</code>
    </div>
  );
}
```

## 4. Flow diagram (Rust -> Wasm -> Next.js)

```mermaid
flowchart LR
    A["Rust Security Core (lib.rs)"] -->|"cargo build --release"| B["wasm-pack build --target web"]
    B --> C["pkg/security_wasm.js + security_wasm_bg.wasm"]
    C --> D["next.config.js: asyncWebAssembly: true"]
    D --> E["React component: SecurityContext.encrypt(data)"]
    E --> F["Encrypted data rendered client-side"]
```

See the full diagram in [`../../assets/rust-wasm-integration-guide.jpg`](../../assets/rust-wasm-integration-guide.jpg).
