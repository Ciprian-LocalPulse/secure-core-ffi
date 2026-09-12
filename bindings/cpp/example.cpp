// Example integration of the Rust security core into C++
// Cryptographic core author: Ciprian Ștefan Pleșca

#include <iostream>
#include <vector>
#include <cstring>
#include <string>

extern "C" {
    struct SecurityContext;
    SecurityContext* init_security_context(const unsigned char* key, size_t key_len);
    unsigned char* encrypt_payload(SecurityContext* ctx, const unsigned char* data, size_t data_len, size_t* out_len);
    unsigned char* decrypt_payload(SecurityContext* ctx, const unsigned char* data, size_t data_len, size_t* out_len);
    void free_buffer(unsigned char* ptr, size_t len);
    void free_security_context(SecurityContext* ctx);
}

int main() {
    std::vector<unsigned char> key(32, 0x01); // In production: generate from a CSPRNG
    std::string secret = "Highly sensitive system data";

    SecurityContext* ctx = init_security_context(key.data(), key.size());
    if (!ctx) {
        std::cerr << "Failed to initialize the security context" << std::endl;
        return 1;
    }

    size_t enc_len = 0;
    unsigned char* encrypted = encrypt_payload(ctx, (const unsigned char*)secret.data(), secret.size(), &enc_len);
    if (!encrypted) {
        std::cerr << "Encryption failed" << std::endl;
        free_security_context(ctx);
        return 1;
    }
    std::cout << "Secured payload. Byte size: " << enc_len << std::endl;

    size_t dec_len = 0;
    unsigned char* decrypted = decrypt_payload(ctx, encrypted, enc_len, &dec_len);
    if (decrypted) {
        std::string result((char*)decrypted, dec_len);
        std::cout << "Decrypted text: " << result << std::endl;
        free_buffer(decrypted, dec_len);
    }

    free_buffer(encrypted, enc_len);
    free_security_context(ctx);
    return 0;
}
