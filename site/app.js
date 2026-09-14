(() => {
  "use strict";

  const $ = (id) => document.getElementById(id);
  const state = { payload: null, runs: 0 };
  const keyField = $("key");
  const plaintextField = $("plaintext");
  const payloadField = $("payload-output");
  const statusField = $("lab-status");

  function toBase64(bytes) {
    let binary = "";
    for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]);
    return btoa(binary);
  }

  function fromBase64(value) {
    const binary = atob(value.trim());
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
    return bytes;
  }

  function toHex(bytes) {
    return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  }

  function setStatus(title, detail, kind) {
    statusField.className = "lab-status status-" + kind;
    statusField.innerHTML = '<span class="status-icon">●</span><span><b></b><small></small></span>';
    statusField.querySelector("b").textContent = title;
    statusField.querySelector("small").textContent = detail;
  }

  function generateKey() {
    keyField.value = toBase64(crypto.getRandomValues(new Uint8Array(32)));
    setStatus("Fresh key generated", "The 256-bit key exists only in this browser tab.", "success");
  }

  async function importKey() {
    const key = fromBase64(keyField.value);
    if (key.length !== 32) throw new Error("The key must decode to exactly 32 bytes (256 bits).");
    return crypto.subtle.importKey("raw", key, { name: "AES-GCM" }, false, ["encrypt", "decrypt"]);
  }

  function updateMetrics(operation, length, elapsed) {
    $("operation-output").textContent = operation;
    $("length-output").textContent = length + " bytes";
    $("time-output").textContent = elapsed.toFixed(2) + " ms";
  }

  async function encrypt() {
    const started = performance.now();
    try {
      const key = await importKey();
      const nonce = crypto.getRandomValues(new Uint8Array(12));
      const data = new TextEncoder().encode(plaintextField.value);
      const encrypted = new Uint8Array(await crypto.subtle.encrypt(
        { name: "AES-GCM", iv: nonce, tagLength: 128 },
        key,
        data
      ));
      state.payload = new Uint8Array(nonce.length + encrypted.length);
      state.payload.set(nonce);
      state.payload.set(encrypted, nonce.length);
      $("nonce-output").textContent = toHex(nonce);
      payloadField.value = toBase64(state.payload);
      $("decrypted-output").value = "";
      state.runs += 1;
      updateMetrics("encrypt", state.payload.length, performance.now() - started);
      setStatus("Encryption authenticated", "Run " + state.runs + " · nonce generated with the browser CSPRNG.", "success");
    } catch (error) {
      setStatus("Experiment could not run", error.message, "error");
    }
  }

  async function decrypt() {
    const started = performance.now();
    try {
      const payload = fromBase64(payloadField.value);
      if (payload.length < 28) throw new Error("Payload is too short: nonce, ciphertext and tag are required.");
      const key = await importKey();
      const nonce = payload.slice(0, 12);
      const encrypted = payload.slice(12);
      const plain = await crypto.subtle.decrypt(
        { name: "AES-GCM", iv: nonce, tagLength: 128 },
        key,
        encrypted
      );
      $("nonce-output").textContent = toHex(nonce);
      $("decrypted-output").value = new TextDecoder().decode(plain);
      state.payload = payload;
      updateMetrics("decrypt", payload.length, performance.now() - started);
      setStatus("Authentication verified", "The tag is valid; plaintext was released after verification.", "success");
    } catch (error) {
      $("decrypted-output").value = "";
      setStatus("Authentication rejected", "The key or payload was changed. No plaintext was returned.", "error");
    }
  }

  function tamper() {
    try {
      const payload = state.payload ? new Uint8Array(state.payload) : fromBase64(payloadField.value);
      if (payload.length < 1) throw new Error("Encrypt a sample before tampering with the payload.");
      payload[payload.length - 1] ^= 1;
      state.payload = payload;
      payloadField.value = toBase64(payload);
      setStatus("Payload modified", "One bit was flipped. Decryption should now fail authentication.", "warning");
    } catch (error) {
      setStatus("No payload to tamper", error.message, "error");
    }
  }

  async function copyPayload() {
    if (!payloadField.value) return setStatus("Nothing to copy", "Encrypt a sample first.", "warning");
    try {
      await navigator.clipboard.writeText(payloadField.value);
      setStatus("Payload copied", "The observation record is ready to paste into another test.", "success");
    } catch {
      setStatus("Copy unavailable", "Select the payload manually from the observation record.", "warning");
    }
  }

  $("generate-key").addEventListener("click", generateKey);
  $("encrypt").addEventListener("click", encrypt);
  $("decrypt").addEventListener("click", decrypt);
  $("tamper").addEventListener("click", tamper);
  $("copy-payload").addEventListener("click", copyPayload);
  generateKey();
  setStatus("Ready for an experiment", "Generate a key or encrypt the sample to begin.", "ready");
})();
