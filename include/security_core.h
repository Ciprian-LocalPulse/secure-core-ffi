#ifndef SECURITY_CORE_H
#define SECURITY_CORE_H

#include <stddef.h>

#if defined(_WIN32) && !defined(SECURITY_CORE_STATIC)
#  if defined(SECURITY_CORE_BUILD)
#    define SECURITY_CORE_API __declspec(dllexport)
#  else
#    define SECURITY_CORE_API __declspec(dllimport)
#  endif
#else
#  define SECURITY_CORE_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SecurityContext SecurityContext;

SECURITY_CORE_API SecurityContext *init_security_context(
    const unsigned char *key_ptr, size_t key_len);
SECURITY_CORE_API unsigned char *encrypt_payload(
    SecurityContext *ctx, const unsigned char *data_ptr, size_t data_len,
    size_t *out_len);
SECURITY_CORE_API unsigned char *decrypt_payload(
    SecurityContext *ctx, const unsigned char *data_ptr, size_t data_len,
    size_t *out_len);
SECURITY_CORE_API void free_buffer(unsigned char *ptr, size_t len);
SECURITY_CORE_API void free_security_context(SecurityContext *ctx);

#ifdef __cplusplus
}
#endif

#endif
