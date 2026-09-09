// SPDX-License-Identifier: GPL-2.0-or-later
/* Instantiate a public key crypto key from an X.509 Certificate
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: crypto/hash.h, keys/asymmetric-parser.h,
// keys/asymmetric-subtype.h, keys/system_keyring.h, linux/hex.h,
// linux/module.h, linux/kernel.h, linux/slab.h, linux/string.h,
// asymmetric_keys.h, and x509_parser.h.

/*
 * Set up the signature parameters in an X.509 certificate.  This involves
 * digesting the signed data and extracting the signature.
 */
pub unsafe fn x509_get_sig_params(cert: *mut x509_certificate) -> i32 {
    let sig = (*cert).sig;
    let mut tfm: *mut crypto_shash;
    let mut desc: *mut shash_desc;
    let mut desc_size: usize;
    let mut ret: i32;

    pr_devel!("==>{}()\n", "x509_get_sig_params");

    /* Calculate a SHA256 hash of the TBS and check it against the
     * blacklist.
     */
    sha256((*cert).tbs, (*cert).tbs_size, (*cert).sha256.as_mut_ptr());
    ret = is_hash_blacklisted(
        (*cert).sha256.as_ptr(),
        core::mem::size_of_val(&(*cert).sha256),
        BLACKLIST_HASH_X509_TBS,
    );
    if ret == -EKEYREJECTED {
        pr_err!("Cert %*phN is blacklisted\n", core::mem::size_of_val(&(*cert).sha256) as i32, (*cert).sha256.as_ptr());
        (*cert).blacklisted = true;
        ret = 0;
    }

    (*sig).s = kmemdup((*cert).raw_sig, (*cert).raw_sig_size, GFP_KERNEL);
    if (*sig).s.is_null() {
        return -ENOMEM;
    }

    (*sig).s_size = (*cert).raw_sig_size;

    if (*sig).algo_takes_data {
        /* The signature algorithm does whatever passes for hashing. */
        (*sig).m = (*cert).tbs as *mut u8;
        (*sig).m_size = (*cert).tbs_size;
        (*sig).m_free = false;
        return ret;
    }

    /* Allocate the hashing algorithm we're going to need and find out how
     * big the hash operational data will be.
     */
    tfm = crypto_alloc_shash((*sig).hash_algo, 0, 0);
    if is_err(tfm) {
        if ptr_err(tfm) == -ENOENT {
            (*cert).unsupported_sig = true;
            return 0;
        }
        return ptr_err(tfm);
    }

    desc_size = crypto_shash_descsize(tfm) + core::mem::size_of::<shash_desc>();
    (*sig).m_size = crypto_shash_digestsize(tfm);

    ret = -ENOMEM;
    (*sig).m = kmalloc((*sig).m_size, GFP_KERNEL);
    if (*sig).m.is_null() {
        crypto_free_shash(tfm);
        return ret;
    }
    (*sig).m_free = true;

    desc = kzalloc(desc_size, GFP_KERNEL) as *mut shash_desc;
    if desc.is_null() {
        kfree((*sig).m as *mut core::ffi::c_void);
        crypto_free_shash(tfm);
        return ret;
    }

    (*desc).tfm = tfm;

    ret = crypto_shash_digest(desc, (*cert).tbs, (*cert).tbs_size, (*sig).m);
    if ret < 0 {
        kfree(desc as *mut core::ffi::c_void);
        crypto_free_shash(tfm);
        return ret;
    }

    kfree(desc as *mut core::ffi::c_void);
    crypto_free_shash(tfm);
    pr_devel!("<=={}() = {}\n", "x509_get_sig_params", ret);
    ret
}

/*
 * Check for self-signedness in an X.509 cert and if found, check the signature
 * immediately if we can.
 */
pub unsafe fn x509_check_for_self_signed(cert: *mut x509_certificate) -> i32 {
    let mut ret: i32 = 0;

    pr_devel!("==>{}()\n", "x509_check_for_self_signed");

    if (*cert).raw_subject_size != (*cert).raw_issuer_size
        || memcmp((*cert).raw_subject, (*cert).raw_issuer, (*cert).raw_issuer_size) != 0
    {
        pr_devel!("<=={}() = 0 [not]\n", "x509_check_for_self_signed");
        return 0;
    }

    if !(*(*cert).sig).auth_ids[0].is_null() || !(*(*cert).sig).auth_ids[1].is_null() {
        /* If the AKID is present it may have one or two parts.  If
         * both are supplied, both must match.
         */
        let a = asymmetric_key_id_same((*cert).skid, (*(*cert).sig).auth_ids[1]);
        let b = asymmetric_key_id_same((*cert).id, (*(*cert).sig).auth_ids[0]);

        if !a && !b {
            pr_devel!("<=={}() = 0 [not]\n", "x509_check_for_self_signed");
            return 0;
        }

        ret = -EKEYREJECTED;
        if ((a && !b) || (b && !a))
            && !(*(*cert).sig).auth_ids[0].is_null()
            && !(*(*cert).sig).auth_ids[1].is_null()
        {
            return ret;
        }
    }

    if (*cert).unsupported_sig {
        return 0;
    }

    ret = public_key_verify_signature((*cert).pub, (*cert).sig);
    if ret < 0 {
        if ret == -ENOPKG {
            (*cert).unsupported_sig = true;
            ret = 0;
        }
        return ret;
    }

    pr_devel!("Cert Self-signature verified");
    (*cert).self_signed = true;
    pr_devel!("<=={}() = {}\n", "x509_check_for_self_signed", ret);
    ret
}

/*
 * Attempt to parse a data blob for a key as an X509 certificate.
 */
unsafe extern "C" fn x509_key_preparse(prep: *mut key_preparsed_payload) -> i32 {
    let mut cert: *mut x509_certificate = x509_cert_parse((*prep).data, (*prep).datalen);
    let mut kids: *mut asymmetric_key_ids;
    let mut p: *mut i8;
    let mut desc: *mut i8;
    let mut q: *const i8;
    let srlen: usize;
    let sulen: usize;

    if is_err(cert) {
        return ptr_err(cert);
    }

    pr_devel!("Cert Issuer: %s\n", (*cert).issuer);
    pr_devel!("Cert Subject: %s\n", (*cert).subject);
    pr_devel!("Cert Key Algo: %s\n", (*(*cert).pub_).pkey_algo);
    pr_devel!("Cert Valid period: %lld-%lld\n", (*cert).valid_from, (*cert).valid_to);

    (*(*cert).pub_).id_type = b"X509\0".as_ptr() as *const i8;

    if (*cert).unsupported_sig {
        public_key_signature_free((*cert).sig);
        (*cert).sig = core::ptr::null_mut();
    } else {
        pr_devel!("Cert Signature: %s + %s\n", (*(*cert).sig).pkey_algo, (*(*cert).sig).hash_algo);
    }

    if (*cert).blacklisted {
        return -EKEYREJECTED;
    }

    sulen = strlen((*cert).subject);
    if !(*cert).raw_skid.is_null() {
        srlen = (*cert).raw_skid_size;
        q = (*cert).raw_skid;
    } else {
        srlen = (*cert).raw_serial_size;
        q = (*cert).raw_serial;
    }

    desc = kmalloc(sulen + 2 + srlen * 2 + 1, GFP_KERNEL) as *mut i8;
    if desc.is_null() {
        return -ENOMEM;
    }
    p = memcpy(desc as *mut core::ffi::c_void, (*cert).subject as *const core::ffi::c_void, sulen) as *mut i8;
    *p = b':' as i8; p = p.add(1);
    *p = b' ' as i8; p = p.add(1);
    p = bin2hex(p, q, srlen);
    *p = 0;

    kids = kmalloc_obj::<asymmetric_key_ids>();
    if kids.is_null() {
        return -ENOMEM;
    }
    (*kids).id[0] = (*cert).id;
    (*kids).id[1] = (*cert).skid;
    (*kids).id[2] = asymmetric_key_generate_id((*cert).raw_subject, (*cert).raw_subject_size, core::ptr::null(), 0);
    if is_err((*kids).id[2]) {
        return ptr_err((*kids).id[2]);
    }

    __module_get(public_key_subtype.owner);
    (*prep).payload.data[asym_subtype] = &mut public_key_subtype as *mut _ as *mut core::ffi::c_void;
    (*prep).payload.data[asym_key_ids] = kids as *mut core::ffi::c_void;
    (*prep).payload.data[asym_crypto] = (*cert).pub_ as *mut core::ffi::c_void;
    (*prep).payload.data[asym_auth] = (*cert).sig as *mut core::ffi::c_void;
    (*prep).description = desc;
    (*prep).quotalen = 100;

    (*cert).pub_ = core::ptr::null_mut();
    (*cert).id = core::ptr::null_mut();
    (*cert).skid = core::ptr::null_mut();
    (*cert).sig = core::ptr::null_mut();
    0
}

static mut x509_key_parser: asymmetric_key_parser = asymmetric_key_parser {
    owner: THIS_MODULE,
    name: b"x509\0".as_ptr() as *const i8,
    parse: Some(x509_key_preparse),
};

/* Module stuff */
unsafe extern "C" fn x509_key_init() -> i32 {
    register_asymmetric_key_parser(&mut x509_key_parser)
}

unsafe extern "C" fn x509_key_exit() {
    unregister_asymmetric_key_parser(&mut x509_key_parser);
}

module_init!(x509_key_init);
module_exit!(x509_key_exit);

module_description!("X.509 certificate parser");
module_author!("Red Hat, Inc.");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
