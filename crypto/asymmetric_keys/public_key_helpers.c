// SPDX-License-Identifier: GPL-2.0-or-later
/* Header inline/macro and module-metadata boundaries only. */
#define pr_fmt(fmt) "PKEY: " fmt
#include "public_key_bindings.h"

void *rust_pkey_kmalloc(size_t size) { return kmalloc(size, GFP_KERNEL); }
void rust_pkey_free_sig(struct crypto_sig *tfm) { crypto_free_sig(tfm); }
unsigned int rust_pkey_sig_keysize(struct crypto_sig *tfm) { return crypto_sig_keysize(tfm); }
unsigned int rust_pkey_sig_digestsize(struct crypto_sig *tfm) { return crypto_sig_digestsize(tfm); }
unsigned int rust_pkey_sig_maxsize(struct crypto_sig *tfm) { return crypto_sig_maxsize(tfm); }
int rust_pkey_sig_set_pubkey(struct crypto_sig *tfm, const void *key, unsigned int keylen) { return crypto_sig_set_pubkey(tfm, key, keylen); }
int rust_pkey_sig_set_privkey(struct crypto_sig *tfm, const void *key, unsigned int keylen) { return crypto_sig_set_privkey(tfm, key, keylen); }
int rust_pkey_sig_sign(struct crypto_sig *tfm, const void *src, unsigned int slen, void *dst, unsigned int dlen) { return crypto_sig_sign(tfm, src, slen, dst, dlen); }
int rust_pkey_sig_verify(struct crypto_sig *tfm, const void *src, unsigned int slen, const void *digest, unsigned int dlen) { return crypto_sig_verify(tfm, src, slen, digest, dlen); }
void rust_pkey_free_akcipher(struct crypto_akcipher *tfm) { crypto_free_akcipher(tfm); }
unsigned int rust_pkey_akcipher_maxsize(struct crypto_akcipher *tfm) { return crypto_akcipher_maxsize(tfm); }
int rust_pkey_akcipher_set_pub_key(struct crypto_akcipher *tfm, const void *key, unsigned int keylen) { return crypto_akcipher_set_pub_key(tfm, key, keylen); }
int rust_pkey_akcipher_set_priv_key(struct crypto_akcipher *tfm, const void *key, unsigned int keylen) { return crypto_akcipher_set_priv_key(tfm, key, keylen); }
bool rust_pkey_warn_verify(bool condition) { return WARN_ON_ONCE(condition); }
void rust_pkey_log_enter(const char *function) { pr_devel("==>%s()\n", function); }
void rust_pkey_log_exit(const char *function, int ret) { pr_devel("<==%s() = %d\n", function, ret); }

MODULE_DESCRIPTION("In-software asymmetric public-key subtype");
MODULE_AUTHOR("Red Hat, Inc.");
MODULE_LICENSE("GPL");
