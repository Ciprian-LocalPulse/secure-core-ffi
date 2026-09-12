const assert = require("node:assert/strict");
const { SecurityCore } = require("./security_core");

const core = new SecurityCore(Buffer.alloc(32, 7), process.env.SECURITY_CORE_LIB);
try {
  const plaintext = Buffer.from("node binding integration test");
  const encrypted = core.encrypt(plaintext);
  assert.ok(encrypted.length > plaintext.length);
  assert.deepEqual(core.decrypt(encrypted), plaintext);
  encrypted[encrypted.length - 1] ^= 1;
  assert.throws(() => core.decrypt(encrypted), /Decriptarea/);
  console.log("Node.js binding tests passed");
} finally {
  core.close();
}
