// SPDX-License-Identifier: GPL-2.0-or-later
/* Verify the signature on a PKCS#7 message.
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #define pr_fmt(fmt) "PKCS7: "fmt
// Dependencies supplied by the surrounding kernel translation.

/*
 * Digest the relevant parts of the PKCS#7 data
 */
unsafe fn pkcs7_digest(
    pkcs7: *mut pkcs7_message,
    sinfo: *mut pkcs7_signed_info,
) -> c_int {
    let sig = (*sinfo).sig;
    let mut tfm: *mut crypto_shash;
    let mut desc: *mut shash_desc;
    let desc_size: usize;
    let mut ret: c_int;

    kenter(",%u,%s", (*sinfo).index, (*sinfo).sig.hash_algo);

    if (*sinfo).authattrs.is_null() && (*sig).algo_takes_data {
        /* There's no intermediate digest and the signature algo
         * doesn't want the data prehashing.
         */
        (*sig).m = (*pkcs7).data as *mut c_void;
        (*sig).m_size = (*pkcs7).data_len;
        (*sig).m_free = false;
        return 0;
    }

    /* The digest was calculated already. */
    if !(*sig).m.is_null() {
        return 0;
    }

    if (*sinfo).sig.hash_algo.is_null() {
        return -ENOPKG;
    }

    /* Allocate the hashing algorithm we're going to need and find out how
     * big the hash operational data will be.
     */
    tfm = crypto_alloc_shash((*sinfo).sig.hash_algo, 0, 0);
    if IS_ERR(tfm) {
        return if PTR_ERR(tfm) == -ENOENT { -ENOPKG } else { PTR_ERR(tfm) };
    }

    desc_size = crypto_shash_descsize(tfm) + core::mem::size_of::<shash_desc>();
    (*sig).m_size = crypto_shash_digestsize(tfm);

    ret = -ENOMEM;
    (*sig).m = kmalloc(umax((*sinfo).authattrs_len, (*sig).m_size), GFP_KERNEL);
    if (*sig).m.is_null() {
        goto!(error_no_desc);
    }
    (*sig).m_free = true;

    desc = kzalloc(desc_size, GFP_KERNEL) as *mut shash_desc;
    if desc.is_null() {
        goto!(error_no_desc);
    }

    (*desc).tfm = tfm;

    /* Digest the message [RFC2315 9.3] */
    ret = crypto_shash_digest(desc, (*pkcs7).data, (*pkcs7).data_len, (*sig).m);
    if ret < 0 {
        goto!(error);
    }
    pr_devel!("MsgDigest = [%*ph]\n", 8, (*sig).m);

    /* However, if there are authenticated attributes, there must be a
     * message digest attribute amongst them which corresponds to the
     * digest we just calculated.
     */
    if !(*sinfo).authattrs.is_null() {
        if (*sinfo).msgdigest.is_null() {
            pr_warn!("Sig %u: No messageDigest\n", (*sinfo).index);
            ret = -EKEYREJECTED;
            goto!(error);
        }

        if (*sinfo).msgdigest_len != (*sig).m_size {
            pr_warn!("Sig %u: Invalid digest size (%u)\n", (*sinfo).index, (*sinfo).msgdigest_len);
            ret = -EBADMSG;
            goto!(error);
        }

        if memcmp((*sig).m, (*sinfo).msgdigest, (*sinfo).msgdigest_len) != 0 {
            pr_warn!("Sig %u: Message digest doesn't match\n", (*sinfo).index);
            ret = -EKEYREJECTED;
            goto!(error);
        }

        /* We then calculate anew, using the authenticated attributes
         * as the contents of the digest instead.  Note that we need to
         * convert the attributes from a CONT.0 into a SET before we
         * hash it.
         *
         * However, for certain algorithms, such as ML-DSA, the digest
         * is integrated into the signing algorithm.  In such a case,
         * we copy the authattrs, modifying the tag type, and set that
         * as the digest.
         */
        memcpy((*sig).m, (*sinfo).authattrs, (*sinfo).authattrs_len);
        *( (*sig).m as *mut u8) = ASN1_CONS_BIT | ASN1_SET;

        if (*sig).algo_takes_data {
            (*sig).m_size = (*sinfo).authattrs_len;
            ret = 0;
        } else {
            ret = crypto_shash_digest(desc, (*sig).m, (*sinfo).authattrs_len, (*sig).m);
            if ret < 0 {
                goto!(error);
            }
        }
        pr_devel!("AADigest = [%*ph]\n", 8, (*sig).m);
    }

    goto!(cleanup);
error:
    kfree(desc as *mut c_void);
error_no_desc:
    crypto_free_shash(tfm);
cleanup:
    kleave!(" = %d", ret);
    ret
}

pub unsafe fn pkcs7_get_digest(
    pkcs7: *mut pkcs7_message,
    buf: *mut *const u8,
    len: *mut u32,
    hash_algo: *mut hash_algo,
) -> c_int {
    let sinfo = (*pkcs7).signed_infos;
    let mut i: c_int;

    /*
     * This function doesn't support messages with more than one signature.
     */
    if sinfo.is_null() || !(*sinfo).next.is_null() {
        return -EBADMSG;
    }

    let ret = pkcs7_digest(pkcs7, sinfo);
    if ret != 0 { return ret; }
    if !(*sinfo).sig.m_free {
        pr_notice_once!("%s: No digest available\n", __func__);
        return -EINVAL; /* TODO: MLDSA doesn't necessarily calculate an
                         * intermediate digest. */
    }

    *buf = (*sinfo).sig.m;
    *len = (*sinfo).sig.m_size;

    i = match_string(hash_algo_name, HASH_ALGO__LAST, (*sinfo).sig.hash_algo);
    if i >= 0 { *hash_algo = i as hash_algo; }
    0
}

/* Find the key (X.509 certificate) to use to verify a PKCS#7 message. */
unsafe fn pkcs7_find_key(pkcs7: *mut pkcs7_message, sinfo: *mut pkcs7_signed_info) -> c_int {
    let mut x509 = (*pkcs7).certs;
    let mut certix: u32 = 1;
    kenter!("%u", (*sinfo).index);
    while !x509.is_null() {
        if !asymmetric_key_id_same((*x509).id, (*sinfo).sig.auth_ids[0]) {
            x509 = (*x509).next; certix += 1; continue;
        }
        pr_devel!("Sig %u: Found cert serial match X.509[%u]\n", (*sinfo).index, certix);
        (*sinfo).signer = x509;
        return 0;
    }
    pr_debug!("Sig %u: Issuing X.509 cert not found (#%*phN)\n", (*sinfo).index,
              (*sinfo).sig.auth_ids[0].len, (*sinfo).sig.auth_ids[0].data);
    0
}

/* Verify the internal certificate chain as best we can. */
unsafe fn pkcs7_verify_sig_chain(pkcs7: *mut pkcs7_message, sinfo: *mut pkcs7_signed_info) -> c_int {
    let mut x509 = (*sinfo).signer;
    let mut p;
    kenter!("");
    p = (*pkcs7).certs;
    while !p.is_null() { (*p).seen = false; p = (*p).next; }
    loop {
        pr_debug!("verify %s: %*phN\n", (*x509).subject, (*x509).raw_serial_size, (*x509).raw_serial);
        (*x509).seen = true;
        if (*x509).blacklisted {
            (*sinfo).blacklisted = true;
            p = (*sinfo).signer;
            while p != x509 { (*p).blacklisted = true; p = (*p).signer; }
            pr_debug!("- blacklisted\n"); return 0;
        }
        pr_debug!("- issuer %s\n", (*x509).issuer);
        let sig = (*x509).sig;
        if !(*sig).auth_ids[0].is_null() { pr_debug!("- authkeyid.id %*phN\n", (*sig).auth_ids[0].len, (*sig).auth_ids[0].data); }
        if !(*sig).auth_ids[1].is_null() { pr_debug!("- authkeyid.skid %*phN\n", (*sig).auth_ids[1].len, (*sig).auth_ids[1].data); }
        if (*x509).self_signed {
            if (*x509).unsupported_sig { goto!(unsupported_sig_in_x509); }
            (*x509).signer = x509; pr_debug!("- self-signed\n"); return 0;
        }
        let auth = if !(*sig).auth_ids[0].is_null() { (*sig).auth_ids[0] } else { (*sig).auth_ids[1] };
        if !auth.is_null() {
            p = (*pkcs7).certs;
            while !p.is_null() {
                if (!(*sig).auth_ids[0].is_null() && asymmetric_key_id_same((*p).id, auth)) ||
                   ((*sig).auth_ids[0].is_null() && !(*p).skid.is_null() && asymmetric_key_id_same((*p).skid, auth)) { break; }
                p = (*p).next;
            }
        }
        if p.is_null() { pr_debug!("- top\n"); return 0; }
        if !(*sig).auth_ids[1].is_null() && !asymmetric_key_id_same((*p).skid, (*sig).auth_ids[1]) {
            pr_warn!("Sig %u: X.509 chain contains auth-skid nonmatch (%u->%u)\n", (*sinfo).index, (*x509).index, (*p).index);
            return -EKEYREJECTED;
        }
        pr_debug!("- subject %s\n", (*p).subject);
        if (*p).seen { pr_warn!("Sig %u: X.509 chain contains loop\n", (*sinfo).index); return 0; }
        let ret = public_key_verify_signature((*p).pub, (*x509).sig);
        if ret < 0 { return ret; }
        (*x509).signer = p;
        if x509 == p { pr_debug!("- self-signed\n"); return 0; }
        x509 = p; might_sleep!();
    }
unsupported_sig_in_x509:
    0
}

/* Verify one signed information block from a PKCS#7 message. */
unsafe fn pkcs7_verify_one(pkcs7: *mut pkcs7_message, sinfo: *mut pkcs7_signed_info) -> c_int {
    kenter!(",%u", (*sinfo).index);
    let ret = pkcs7_digest(pkcs7, sinfo); if ret < 0 { return ret; }
    let ret = pkcs7_find_key(pkcs7, sinfo); if ret < 0 { return ret; }
    if (*sinfo).signer.is_null() { return 0; }
    pr_devel!("Using X.509[%u] for sig %u\n", (*sinfo).signer.index, (*sinfo).index);
    if test_bit(sinfo_has_signing_time, &(*sinfo).aa_set) &&
       ((*sinfo).signing_time < (*sinfo).signer.valid_from || (*sinfo).signing_time > (*sinfo).signer.valid_to) {
        pr_warn!("Message signed outside of X.509 validity window\n"); return -EKEYREJECTED;
    }
    let ret = public_key_verify_signature((*sinfo).signer.pub, (*sinfo).sig); if ret < 0 { return ret; }
    pr_devel!("Verified signature %u\n", (*sinfo).index);
    pkcs7_verify_sig_chain(pkcs7, sinfo)
}

pub unsafe fn pkcs7_verify(pkcs7: *mut pkcs7_message, usage: key_being_used_for) -> c_int {
    let mut actual_ret = -ENOPKG;
    match usage {
        VERIFYING_MODULE_SIGNATURE => {
            if (*pkcs7).data_type != OID_data { pr_warn!("Invalid module sig (not pkcs7-data)\n"); return -EKEYREJECTED; }
            if (*pkcs7).have_authattrs { pr_warn!("Invalid module sig (has authattrs)\n"); return -EKEYREJECTED; }
        }
        VERIFYING_FIRMWARE_SIGNATURE => {
            if (*pkcs7).data_type != OID_data { pr_warn!("Invalid firmware sig (not pkcs7-data)\n"); return -EKEYREJECTED; }
            if !(*pkcs7).have_authattrs { pr_warn!("Invalid firmware sig (missing authattrs)\n"); return -EKEYREJECTED; }
        }
        VERIFYING_KEXEC_PE_SIGNATURE => if (*pkcs7).data_type != OID_msIndirectData { pr_warn!("Invalid kexec sig (not Authenticode)\n"); return -EKEYREJECTED; },
        VERIFYING_UNSPECIFIED_SIGNATURE | VERIFYING_BPF_SIGNATURE => if (*pkcs7).data_type != OID_data { pr_warn!("Invalid unspecified sig (not pkcs7-data)\n"); return -EKEYREJECTED; },
        _ => return -EINVAL,
    }
    let mut sinfo = (*pkcs7).signed_infos;
    while !sinfo.is_null() {
        let ret = pkcs7_verify_one(pkcs7, sinfo);
        if (*sinfo).blacklisted { if actual_ret == -ENOPKG { actual_ret = -EKEYREJECTED; } sinfo = (*sinfo).next; continue; }
        if ret < 0 { if ret == -ENOPKG { (*sinfo).unsupported_crypto = true; sinfo = (*sinfo).next; continue; } kleave!(" = %d", ret); return ret; }
        actual_ret = 0; sinfo = (*sinfo).next;
    }
    kleave!(" = %d", actual_ret); actual_ret
}

pub unsafe fn pkcs7_supply_detached_data(pkcs7: *mut pkcs7_message, data: *const c_void, datalen: usize) -> c_int {
    if !(*pkcs7).data.is_null() { pr_warn!("Data already supplied\n"); return -EINVAL; }
    (*pkcs7).data = data; (*pkcs7).data_len = datalen; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
