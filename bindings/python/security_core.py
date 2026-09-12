"""
Python binding for the Rust security core (security_core).
Cryptographic core author: Ciprian Stefan Plesca

Requires libsecurity_core.so / .dylib / .dll built from the Rust sources
(cargo build --release).
"""

import ctypes
import os
import platform


def _lib_name() -> str:
    system = platform.system()
    if system == "Windows":
        return "security_core.dll"
    if system == "Darwin":
        return "libsecurity_core.dylib"
    return "libsecurity_core.so"


class SecurityCore:
    def __init__(self, key: bytes, lib_path: str | None = None):
        if len(key) != 32:
            raise ValueError("The key must be exactly 32 bytes (256 bits)")

        path = lib_path or os.path.join(
            os.path.dirname(__file__), "..", "..", "target", "release", _lib_name()
        )
        self._lib = ctypes.CDLL(path)
        self._configure_signatures()

        self._ctx = self._lib.init_security_context(key, len(key))
        if not self._ctx:
            raise RuntimeError("Could not initialize the security context")

    def _configure_signatures(self):
        self._lib.init_security_context.restype = ctypes.c_void_p
        self._lib.init_security_context.argtypes = [ctypes.c_char_p, ctypes.c_size_t]

        self._lib.encrypt_payload.restype = ctypes.POINTER(ctypes.c_ubyte)
        self._lib.encrypt_payload.argtypes = [
            ctypes.c_void_p,
            ctypes.c_char_p,
            ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_size_t),
        ]

        self._lib.decrypt_payload.restype = ctypes.POINTER(ctypes.c_ubyte)
        self._lib.decrypt_payload.argtypes = [
            ctypes.c_void_p,
            ctypes.POINTER(ctypes.c_ubyte),
            ctypes.c_size_t,
            ctypes.POINTER(ctypes.c_size_t),
        ]

        self._lib.free_buffer.argtypes = [ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t]
        self._lib.free_security_context.argtypes = [ctypes.c_void_p]

    def encrypt(self, plaintext: bytes) -> bytes:
        out_len = ctypes.c_size_t()
        ptr = self._lib.encrypt_payload(self._ctx, plaintext, len(plaintext), ctypes.byref(out_len))
        if not ptr:
            raise RuntimeError("Encryption failed")
        try:
            return bytes(ptr[: out_len.value])
        finally:
            self._lib.free_buffer(ptr, out_len.value)

    def decrypt(self, ciphertext: bytes) -> bytes:
        buf = (ctypes.c_ubyte * len(ciphertext)).from_buffer_copy(ciphertext)
        out_len = ctypes.c_size_t()
        ptr = self._lib.decrypt_payload(self._ctx, buf, len(ciphertext), ctypes.byref(out_len))
        if not ptr:
            raise RuntimeError("Decryption failed (possibly corrupted data or wrong key)")
        try:
            return bytes(ptr[: out_len.value])
        finally:
            self._lib.free_buffer(ptr, out_len.value)

    def close(self):
        if getattr(self, "_ctx", None):
            self._lib.free_security_context(self._ctx)
            self._ctx = None

    def __del__(self):
        self.close()


if __name__ == "__main__":
    key = os.urandom(32)
    core = SecurityCore(key)
    enc = core.encrypt(b"Highly sensitive system data")
    print("Encrypted:", enc.hex())
    print("Decrypted:", core.decrypt(enc))
