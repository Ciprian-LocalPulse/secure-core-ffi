import os
import unittest

from security_core import SecurityCore


class SecurityCoreTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.core = SecurityCore(bytes(range(32)), os.environ.get("SECURITY_CORE_LIB"))

    @classmethod
    def tearDownClass(cls):
        cls.core.close()

    def test_roundtrip(self):
        plaintext = b"python binding integration test"
        self.assertEqual(self.core.decrypt(self.core.encrypt(plaintext)), plaintext)

    def test_tampering_fails(self):
        encrypted = bytearray(self.core.encrypt(b"tamper me"))
        encrypted[-1] ^= 1
        with self.assertRaises(RuntimeError):
            self.core.decrypt(bytes(encrypted))

    def test_invalid_key_is_rejected(self):
        with self.assertRaises(ValueError):
            SecurityCore(b"short", os.environ.get("SECURITY_CORE_LIB"))


if __name__ == "__main__":
    unittest.main()
