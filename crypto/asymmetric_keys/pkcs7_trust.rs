// SPDX-License-Identifier: GPL-2.0-or-later
/* Validate the trust chain of a PKCS#7 message.
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/*
 * Check the trust on one PKCS#7 SignedInfo block.
 */
unsafe fn pkcs7_validate_trust_one(
    pkcs7: *mut pkcs7_message,
    sinfo: *mut pkcs7_signed_info,
    trust_keyring: *mut key,
) -> i32 {
    let mut sig: *mut public_key_signature = (*sinfo).sig;
    let mut x509: *mut x509_certificate;
    let mut last: *mut x509_certificate = core::ptr::null_mut();
    let mut p: *mut x509_certificate;
    let mut key: *mut key;
    let ret: i32;

    kenter(",%u,", (*sinfo).index);

    if (*sinfo).unsupported_crypto {
        kleave(" = -ENOPKG [cached]");
        return -ENOPKG;
    }

    x509 = (*sinfo).signer;
    while !x509.is_null() {
        if (*x509).seen {
            if (*x509).verified {
                goto_verified: {
                    if !x509.is_null() {
                        (*x509).verified = true;
                        p = (*sinfo).signer;
                        while p != x509 {
                            (*p).verified = true;
                            p = (*p).signer;
                        }
                    }
                    kleave(" = 0");
                    return 0;
                }
            }
            kleave(" = -ENOKEY [cached]");
            return -ENOKEY;
        }
        (*x509).seen = true;

        /* Look to see if this certificate is present in the trusted keys. */
        key = find_asymmetric_key(trust_keyring, (*x509).id, (*x509).skid,
                                  core::ptr::null_mut(), false);
        if !IS_ERR(key) {
            /* Verify that the trusted variant can also validate the
             * signature on the descendant. */
            pr_devel("sinfo %u: Cert %u as key %x\n",
                     (*sinfo).index, (*x509).index, key_serial(key));
            goto_matched: {
                ret = verify_signature(key, sig);
                key_put(key);
                if ret < 0 {
                    if ret == -ENOMEM { return ret; }
                    kleave(" = -EKEYREJECTED [verify %d]", ret);
                    return -EKEYREJECTED;
                }
                if !x509.is_null() {
                    (*x509).verified = true;
                    p = (*sinfo).signer;
                    while p != x509 {
                        (*p).verified = true;
                        p = (*p).signer;
                    }
                }
                kleave(" = 0");
                return 0;
            }
        }
        if key == ERR_PTR(-ENOMEM) { return -ENOMEM; }
        /* Self-signed certificates form roots of their own. */
        if (*x509).signer == x509 {
            kleave(" = -ENOKEY [unknown self-signed]");
            return -ENOKEY;
        }
        might_sleep();
        last = x509;
        sig = (*last).sig;
        x509 = (*x509).signer;
    }

    /* No match - see if the root certificate has a signer amongst the
     * trusted keys. */
    if !last.is_null() && ((*(*last).sig).auth_ids[0] || (*(*last).sig).auth_ids[1]) {
        key = find_asymmetric_key(trust_keyring, (*last).sig.auth_ids[0],
                                  (*last).sig.auth_ids[1], core::ptr::null_mut(), false);
        if !IS_ERR(key) {
            x509 = last;
            pr_devel("sinfo %u: Root cert %u signer is key %x\n",
                     (*sinfo).index, (*x509).index, key_serial(key));
            // Continue with the common verification operation.
            ret = verify_signature(key, sig);
            key_put(key);
            if ret < 0 { if ret == -ENOMEM { return ret; } return -EKEYREJECTED; }
            (*x509).verified = true;
            kleave(" = 0");
            return 0;
        }
        if PTR_ERR(key) != -ENOKEY { return PTR_ERR(key); }
    }

    /* As a last resort, see if a trusted public key matches directly. */
    key = find_asymmetric_key(trust_keyring, (*sinfo).sig.auth_ids[0],
                              core::ptr::null_mut(), core::ptr::null_mut(), false);
    if !IS_ERR(key) {
        pr_devel("sinfo %u: Direct signer is key %x\n", (*sinfo).index, key_serial(key));
        ret = verify_signature(key, (*sinfo).sig);
        key_put(key);
        if ret < 0 { if ret == -ENOMEM { return ret; } return -EKEYREJECTED; }
        kleave(" = 0");
        return 0;
    }
    if PTR_ERR(key) != -ENOKEY { return PTR_ERR(key); }
    kleave(" = -ENOKEY [no backref]");
    -ENOKEY
}

/** Validate PKCS#7 trust chain. */
pub unsafe fn pkcs7_validate_trust(pkcs7: *mut pkcs7_message, trust_keyring: *mut key) -> i32 {
    let mut sinfo: *mut pkcs7_signed_info;
    let mut p: *mut x509_certificate = (*pkcs7).certs;
    let mut cached_ret = -ENOKEY;
    while !p.is_null() { (*p).seen = false; p = (*p).next; }
    sinfo = (*pkcs7).signed_infos;
    while !sinfo.is_null() {
        let ret = pkcs7_validate_trust_one(pkcs7, sinfo, trust_keyring);
        match ret {
            -ENOKEY => {},
            -ENOPKG => { if cached_ret == -ENOKEY { cached_ret = -ENOPKG; } },
            0 => { cached_ret = 0; },
            _ => return ret,
        }
        sinfo = (*sinfo).next;
    }
    cached_ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
