/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_PUBLIC_KEY_BINDINGS_H
#define LUPOS_PUBLIC_KEY_BINDINGS_H
#include <crypto/akcipher.h>
#include <crypto/public_key.h>
#include <crypto/sig.h>
#include <keys/asymmetric-subtype.h>
#include <linux/asn1.h>
#include <linux/err.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/seq_file.h>
#include <linux/slab.h>
#include <linux/string.h>

/* C inline and macro boundaries only; all public-key policy lives in Rust. */
void *rust_pkey_kmalloc(size_t size);
void rust_pkey_free_sig(struct crypto_sig *tfm);
unsigned int rust_pkey_sig_keysize(struct crypto_sig *tfm);
unsigned int rust_pkey_sig_digestsize(struct crypto_sig *tfm);
unsigned int rust_pkey_sig_maxsize(struct crypto_sig *tfm);
int rust_pkey_sig_set_pubkey(struct crypto_sig *tfm, const void *key, unsigned int keylen);
int rust_pkey_sig_set_privkey(struct crypto_sig *tfm, const void *key, unsigned int keylen);
int rust_pkey_sig_sign(struct crypto_sig *tfm, const void *src, unsigned int slen, void *dst, unsigned int dlen);
int rust_pkey_sig_verify(struct crypto_sig *tfm, const void *src, unsigned int slen, const void *digest, unsigned int dlen);
void rust_pkey_free_akcipher(struct crypto_akcipher *tfm);
unsigned int rust_pkey_akcipher_maxsize(struct crypto_akcipher *tfm);
int rust_pkey_akcipher_set_pub_key(struct crypto_akcipher *tfm, const void *key, unsigned int keylen);
int rust_pkey_akcipher_set_priv_key(struct crypto_akcipher *tfm, const void *key, unsigned int keylen);
bool rust_pkey_warn_verify(bool condition);
void rust_pkey_log_enter(const char *function);
void rust_pkey_log_exit(const char *function, int ret);
#endif
