// SPDX-License-Identifier: GPL-2.0-or-later
/* Signature verification with an asymmetric key
 *
 * See Documentation/crypto/asymmetric-keys.rst
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// pr_fmt(fmt) "SIG: " fmt
// External kernel declarations and build configuration are supplied elsewhere.

/*
 * Destroy a public key signature.
 */
pub unsafe fn public_key_signature_free(sig: *mut public_key_signature) {
    if !sig.is_null() {
        for i in 0..(*sig).auth_ids.len() {
            kfree((*sig).auth_ids[i]);
        }
        kfree((*sig).s);
        if (*sig).m_free {
            kfree((*sig).m);
        }
        kfree(sig);
    }
}

// EXPORT_SYMBOL_GPL(public_key_signature_free);

/**
 * query_asymmetric_key - Get information about an asymmetric key.
 * @params: Various parameters.
 * @info: Where to put the information.
 */
pub unsafe fn query_asymmetric_key(
    params: *const kernel_pkey_params,
    info: *mut kernel_pkey_query,
) -> c_int {
    let key: *mut key = (*params).key;

    // pr_devel("==>%s()\n", __func__);

    if (*key).type_ != &key_type_asymmetric {
        return -EINVAL;
    }
    let subtype: *const asymmetric_key_subtype = asymmetric_key_subtype(key);
    if subtype.is_null() || (*key).payload.data[0].is_null() {
        return -EINVAL;
    }
    if (*subtype).query.is_none() {
        return -ENOTSUPP;
    }

    let ret = ((*subtype).query.unwrap())(params, info);

    // pr_devel("<==%s() = %d\n", __func__, ret);
    ret
}

// EXPORT_SYMBOL_GPL(query_asymmetric_key);

/**
 * verify_signature - Initiate the use of an asymmetric key to verify a signature
 * @key: The asymmetric key to verify against
 * @sig: The signature to check
 *
 * Returns 0 if successful or else an error.
 */
pub unsafe fn verify_signature(
    key: *const key,
    sig: *const public_key_signature,
) -> c_int {
    // pr_devel("==>%s()\n", __func__);

    if (*key).type_ != &key_type_asymmetric {
        return -EINVAL;
    }
    let subtype: *const asymmetric_key_subtype = asymmetric_key_subtype(key);
    if subtype.is_null() || (*key).payload.data[0].is_null() {
        return -EINVAL;
    }
    if (*subtype).verify_signature.is_none() {
        return -ENOTSUPP;
    }

    let ret = ((*subtype).verify_signature.unwrap())(key, sig);

    // pr_devel("<==%s() = %d\n", __func__, ret);
    ret
}

// EXPORT_SYMBOL_GPL(verify_signature);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
