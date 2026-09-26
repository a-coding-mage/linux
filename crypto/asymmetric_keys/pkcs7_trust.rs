// SPDX-License-Identifier: GPL-2.0-or-later
/* Validate the trust chain of a PKCS#7 message.
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// pr_fmt(fmt) "PKCS7: " fmt
// Dependencies: linux/kernel.h, linux/export.h, linux/slab.h, linux/err.h,
// linux/asn1.h, linux/key.h, keys/asymmetric-type.h, crypto/public_key.h,
// "pkcs7_parser.h".

use core::ffi::c_int;
use core::ptr::null_mut;

/*
 * Check the trust on one PKCS#7 SignedInfo block.
 */
unsafe fn pkcs7_validate_trust_one(
    _pkcs7: *mut pkcs7_message,
    sinfo: *mut pkcs7_signed_info,
    trust_keyring: *mut key,
) -> c_int {
    let mut sig: *mut public_key_signature = (*sinfo).sig;
    let mut x509: *mut x509_certificate;
    let mut last: *mut x509_certificate = null_mut();
    let mut key: *mut key;

    kenter!(",%u,", (*sinfo).index);

    if (*sinfo).unsupported_crypto {
        kleave!(" = -ENOPKG [cached]");
        return -ENOPKG;
    }

    'verified: {
        'matched: {
            x509 = (*sinfo).signer;
            while !x509.is_null() {
                if (*x509).seen {
                    if (*x509).verified {
                        break 'verified;
                    }
                    kleave!(" = -ENOKEY [cached]");
                    return -ENOKEY;
                }
                (*x509).seen = true;

                /* Look to see if this certificate is present in the trusted
                 * keys.
                 */
                key = find_asymmetric_key(trust_keyring, (*x509).id, (*x509).skid, null_mut(), false);
                if !IS_ERR(key.cast()) {
                    /* One of the X.509 certificates in the PKCS#7 message
                     * is apparently the same as one we already trust.
                     * Verify that the trusted variant can also validate
                     * the signature on the descendant.
                     */
                    pr_devel!(
                        "PKCS7: sinfo {}: Cert {} as key {:x}\n",
                        (*sinfo).index,
                        (*x509).index,
                        key_serial(key)
                    );
                    break 'matched;
                }
                if key == ERR_PTR(-ENOMEM as _).cast() {
                    return -ENOMEM;
                }

                /* Self-signed certificates form roots of their own, and if we
                 * don't know them, then we can't accept them.
                 */
                if (*x509).signer == x509 {
                    kleave!(" = -ENOKEY [unknown self-signed]");
                    return -ENOKEY;
                }

                might_sleep();
                last = x509;
                sig = (*last).sig;
                x509 = (*x509).signer;
            }

            /* No match - see if the root certificate has a signer amongst the
             * trusted keys.
             */
            if !last.is_null()
                && (!(*(*last).sig).auth_ids[0].is_null() || !(*(*last).sig).auth_ids[1].is_null())
            {
                key = find_asymmetric_key(
                    trust_keyring,
                    (*(*last).sig).auth_ids[0],
                    (*(*last).sig).auth_ids[1],
                    null_mut(),
                    false,
                );
                if !IS_ERR(key.cast()) {
                    x509 = last;
                    pr_devel!(
                        "PKCS7: sinfo {}: Root cert {} signer is key {:x}\n",
                        (*sinfo).index,
                        (*x509).index,
                        key_serial(key)
                    );
                    break 'matched;
                }
                if PTR_ERR(key.cast()) != -(ENOKEY as i64) as _ {
                    return PTR_ERR(key.cast()) as c_int;
                }
            }

            /* As a last resort, see if we have a trusted public key that matches
             * the signed info directly.
             */
            key = find_asymmetric_key(trust_keyring, (*(*sinfo).sig).auth_ids[0], null_mut(), null_mut(), false);
            if !IS_ERR(key.cast()) {
                pr_devel!("PKCS7: sinfo {}: Direct signer is key {:x}\n", (*sinfo).index, key_serial(key));
                x509 = null_mut();
                sig = (*sinfo).sig;
                break 'matched;
            }
            if PTR_ERR(key.cast()) != -(ENOKEY as i64) as _ {
                return PTR_ERR(key.cast()) as c_int;
            }

            kleave!(" = -ENOKEY [no backref]");
            return -ENOKEY;
        }
        // matched:
        let ret: c_int = verify_signature(key, sig);
        key_put(key);
        if ret < 0 {
            if ret == -ENOMEM {
                return ret;
            }
            kleave!(" = -EKEYREJECTED [verify %d]", ret);
            return -EKEYREJECTED;
        }
    }
    // verified:
    if !x509.is_null() {
        (*x509).verified = true;
        let mut p = (*sinfo).signer;
        while p != x509 {
            (*p).verified = true;
            p = (*p).signer;
        }
    }
    kleave!(" = 0");
    0
}

/**
 * pkcs7_validate_trust - Validate PKCS#7 trust chain
 * @pkcs7: The PKCS#7 certificate to validate
 * @trust_keyring: Signing certificates to use as starting points
 *
 * Validate that the certificate chain inside the PKCS#7 message intersects
 * keys we already know and trust.
 *
 * Returns, in order of descending priority:
 *
 *  (*) -EKEYREJECTED if a signature failed to match for which we have a valid
 *	key, or:
 *
 *  (*) 0 if at least one signature chain intersects with the keys in the trust
 *	keyring, or:
 *
 *  (*) -ENOPKG if a suitable crypto module couldn't be found for a check on a
 *	chain.
 *
 *  (*) -ENOKEY if we couldn't find a match for any of the signature chains in
 *	the message.
 *
 * May also return -ENOMEM.
 */
#[no_mangle]
pub unsafe extern "C" fn pkcs7_validate_trust(pkcs7: *mut pkcs7_message, trust_keyring: *mut key) -> c_int {
    let mut cached_ret: c_int = -ENOKEY;

    let mut p: *mut x509_certificate = (*pkcs7).certs;
    while !p.is_null() {
        (*p).seen = false;
        p = (*p).next;
    }

    let mut sinfo: *mut pkcs7_signed_info = (*pkcs7).signed_infos;
    while !sinfo.is_null() {
        let ret = pkcs7_validate_trust_one(pkcs7, sinfo, trust_keyring);
        match -ret {
            ENOKEY => {}
            ENOPKG => {
                if cached_ret == -ENOKEY {
                    cached_ret = -ENOPKG;
                }
            }
            0 => cached_ret = 0,
            _ => return ret,
        }
        sinfo = (*sinfo).next;
    }

    cached_ret
}
// EXPORT_SYMBOL_GPL(pkcs7_validate_trust);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
