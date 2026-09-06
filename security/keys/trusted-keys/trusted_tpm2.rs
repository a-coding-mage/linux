// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 IBM Corporation
 * Copyright (C) 2014 Intel Corporation
 */

// Rust translation of keys/trusted-keys/trusted_tpm2.c.
// C include dependencies are represented as external declarations below.

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type i32 = core::ffi::c_int;
type size_t = usize;
type off_t = isize;

const PAGE_SIZE: i32 = 4096;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const E2BIG: i32 = 7;
const EFAULT: i32 = 14;
const TPM_HEADER_SIZE: off_t = 10;
const TPM_BUFSIZE: size_t = 4096;
const TPM_DIGEST_SIZE: u16 = 32;
const MAX_BLOB_SIZE: u32 = 512;
const MIN_KEY_SIZE: u16 = 32;
const MAX_KEY_SIZE: u16 = 128;
const TPM_BUF_INVALID: u32 = 1;
const TPM2_ST_SESSIONS: u16 = 0x8002;
const TPM2_ST_NO_SESSIONS: u16 = 0x8001;
const TPM2_CC_CREATE: u32 = 0x00000153;
const TPM2_CC_LOAD: u32 = 0x00000157;
const TPM2_CC_UNSEAL: u32 = 0x0000015e;
const TPM2_SA_DECRYPT: u8 = 0x20;
const TPM2_SA_ENCRYPT: u8 = 0x40;
const TPM_ALG_KEYEDHASH: u16 = 0x0008;
const TPM_ALG_NULL: u16 = 0x0010;
const TPM2_OA_USER_WITH_AUTH: u32 = 0x00000040;
const TPM2_OA_FIXED_TPM: u32 = 0x00000002;
const TPM2_OA_FIXED_PARENT: u32 = 0x00000010;
const OID_TPMSealedData: OID = 0;

#[repr(C)]
pub struct trusted_key_payload {
    pub blob: [u8; MAX_BLOB_SIZE as usize],
    pub blob_len: u32,
    pub key: [u8; MAX_KEY_SIZE as usize],
    pub key_len: u16,
    pub migratable: u8,
    pub old_format: u8,
}

#[repr(C)]
pub struct trusted_key_options {
    pub blobauth_len: u16,
    pub blobauth: *const u8,
    pub keyhandle: u32,
    pub hash: u32,
    pub keyauth: *const u8,
    pub policydigest_len: u16,
    pub policydigest: *const u8,
    pub policyhandle: u32,
}

#[repr(C)]
pub struct tpm_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_buf {
    pub data: *mut u8,
    pub length: size_t,
    pub flags: u32,
    pub handles: i32,
}

#[repr(C)]
pub struct tpm_header {
    pub tag: u16,
}

type OID = i32;

unsafe extern "C" {
    static tpm2key_decoder: u8;

    fn kmalloc(size: size_t, flags: u32) -> *mut u8;
    fn kzalloc(size: size_t, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn memset(s: *mut core::ffi::c_void, c: i32, n: size_t) -> *mut core::ffi::c_void;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: size_t,
    ) -> *mut core::ffi::c_void;

    fn asn1_oid_len(oid: *const u32) -> i32;
    fn asn1_encode_oid(
        data: *mut u8,
        end_data: *mut u8,
        oid: *const u32,
        oid_len: i32,
    ) -> *mut u8;
    fn asn1_encode_boolean(data: *mut u8, end_data: *mut u8, val: bool) -> *mut u8;
    fn asn1_encode_tag(
        data: *mut u8,
        end_data: *mut u8,
        tag: u32,
        string: *const u8,
        len: size_t,
    ) -> *mut u8;
    fn asn1_encode_integer(data: *mut u8, end_data: *mut u8, integer: u32) -> *mut u8;
    fn asn1_encode_octet_string(
        data: *mut u8,
        end_data: *mut u8,
        string: *const u8,
        len: size_t,
    ) -> *mut u8;
    fn asn1_encode_sequence(
        data: *mut u8,
        end_data: *mut u8,
        string: *const u8,
        len: size_t,
    ) -> *mut u8;
    fn asn1_ber_decoder(
        decoder: *const u8,
        context: *mut core::ffi::c_void,
        data: *const u8,
        datalen: u32,
    ) -> i32;

    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn WARN(condition: bool, fmt: *const i8, ...) -> bool;
    fn pr_err(fmt: *const i8, ...);
    fn pr_debug(fmt: *const i8, ...);

    fn get_unaligned_be16(p: *const u8) -> u16;
    fn get_unaligned_be32(p: *const u8) -> u32;
    fn be32_to_cpup(p: *const u32) -> u32;
    fn be16_to_cpup(p: *const u16) -> u16;
    fn cpu_to_be16(x: u16) -> u16;

    fn look_up_OID(data: *const core::ffi::c_void, datasize: size_t) -> OID;
    fn sprint_oid(
        data: *const core::ffi::c_void,
        datasize: size_t,
        buffer: *mut i8,
        bufsize: size_t,
    );

    fn tpm2_find_hash_alg(hash: u32) -> i32;
    fn tpm_try_get_ops(chip: *mut tpm_chip) -> i32;
    fn tpm_put_ops(chip: *mut tpm_chip);
    fn tpm_ret_to_err(rc: i32) -> i32;
    fn tpm2_start_auth_session(chip: *mut tpm_chip) -> i32;
    fn tpm2_end_auth_session(chip: *mut tpm_chip);
    fn tpm2_chip_auth(chip: *mut tpm_chip) -> bool;
    fn tpm2_flush_context(chip: *mut tpm_chip, handle: u32);
    fn tpm_transmit_cmd(chip: *mut tpm_chip, buf: *mut tpm_buf, min_rsp_body_length: size_t, desc: *const i8) -> i32;

    fn tpm_buf_init(buf: *mut tpm_buf, size: size_t);
    fn tpm_buf_init_sized(buf: *mut tpm_buf, size: size_t);
    fn tpm_buf_reset(buf: *mut tpm_buf, tag: u16, ordinal: u32);
    fn tpm_buf_reset_sized(buf: *mut tpm_buf);
    fn tpm_buf_length(buf: *mut tpm_buf) -> size_t;
    fn tpm_buf_append(buf: *mut tpm_buf, new_data: *const u8, new_len: size_t);
    fn tpm_buf_append_u8(buf: *mut tpm_buf, value: u8);
    fn tpm_buf_append_u16(buf: *mut tpm_buf, value: u16);
    fn tpm_buf_append_u32(buf: *mut tpm_buf, value: u32);
    fn tpm_buf_append_name(chip: *mut tpm_chip, buf: *mut tpm_buf, handle: u32, name: *mut core::ffi::c_void) -> i32;
    fn tpm_buf_append_hmac_session(chip: *mut tpm_chip, buf: *mut tpm_buf, attributes: u8, auth: *const u8, auth_len: u16);
    fn tpm_buf_fill_hmac_session(chip: *mut tpm_chip, buf: *mut tpm_buf) -> i32;
    fn tpm_buf_check_hmac_response(chip: *mut tpm_chip, buf: *mut tpm_buf, rc: i32) -> i32;
    fn tpm_buf_read_u32(buf: *mut tpm_buf, offset: *mut off_t) -> i32;
}

static mut tpm2key_oid: [u32; 6] = [2, 23, 133, 10, 1, 5];

unsafe fn tpm2_key_encode(
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
    mut src: *mut u8,
    len: u32,
) -> i32 {
    let SCRATCH_SIZE: i32 = PAGE_SIZE;
    let scratch: *mut u8 = kmalloc(SCRATCH_SIZE as size_t, GFP_KERNEL);
    let mut work: *mut u8 = scratch;
    let mut work1: *mut u8;
    let end_work: *mut u8 = scratch.add(SCRATCH_SIZE as usize);
    let priv_: *mut u8;
    let pub_: *mut u8;
    let priv_len: u16;
    let pub_len: u16;
    let mut ret: i32;

    priv_len = get_unaligned_be16(src).wrapping_add(2);
    priv_ = src;

    src = src.add(priv_len as usize);

    pub_len = get_unaligned_be16(src).wrapping_add(2);
    pub_ = src;

    if scratch.is_null() {
        return -ENOMEM;
    }

    work = asn1_encode_oid(
        work,
        end_work,
        core::ptr::addr_of!(tpm2key_oid) as *const u32,
        asn1_oid_len(core::ptr::addr_of!(tpm2key_oid) as *const u32),
    );

    if (*options).blobauth_len == 0 {
        let mut bool_: [u8; 3] = [0; 3];
        let mut w: *mut u8 = bool_.as_mut_ptr();
        /* tag 0 is emptyAuth */
        w = asn1_encode_boolean(w, unsafe { w.add(core::mem::size_of_val(&bool_)) }, true);
        if WARN(IS_ERR(w as *const core::ffi::c_void), c"BUG: Boolean failed to encode".as_ptr()) {
            ret = PTR_ERR(w as *const core::ffi::c_void);
            goto_err(scratch);
            return ret;
        }
        work = asn1_encode_tag(work, end_work, 0, bool_.as_ptr(), w.offset_from(bool_.as_ptr()) as size_t);
    }

    /*
     * Assume both octet strings will encode to a 2 byte definite length
     *
     * Note: For a well behaved TPM, this warning should never
     * trigger, so if it does there's something nefarious going on
     */
    if WARN(
        work.offset_from(scratch) + pub_len as isize + priv_len as isize + 14 > SCRATCH_SIZE as isize,
        c"BUG: scratch buffer is too small".as_ptr(),
    ) {
        ret = -EINVAL;
        goto_err(scratch);
        return ret;
    }

    work = asn1_encode_integer(work, end_work, (*options).keyhandle);
    work = asn1_encode_octet_string(work, end_work, pub_, pub_len as size_t);
    work = asn1_encode_octet_string(work, end_work, priv_, priv_len as size_t);

    work1 = (*payload).blob.as_mut_ptr();
    work1 = asn1_encode_sequence(
        work1,
        work1.add(core::mem::size_of_val(&(*payload).blob)),
        scratch,
        work.offset_from(scratch) as size_t,
    );
    if IS_ERR(work1 as *const core::ffi::c_void) {
        ret = PTR_ERR(work1 as *const core::ffi::c_void);
        pr_err(c"BUG: ASN.1 encoder failed with %d\n".as_ptr(), ret);
        goto_err(scratch);
        return ret;
    }

    kfree(scratch as *mut core::ffi::c_void);
    work1.offset_from((*payload).blob.as_mut_ptr()) as i32
}

unsafe fn goto_err(scratch: *mut u8) {
    kfree(scratch as *mut core::ffi::c_void);
}

#[repr(C)]
struct tpm2_key_context {
    parent: u32,
    pub_: *const u8,
    pub_len: u32,
    priv_: *const u8,
    priv_len: u32,
}

unsafe fn tpm2_key_decode(
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
    buf: *mut *mut u8,
) -> i32 {
    let mut ret: i32;
    let mut ctx: tpm2_key_context = core::mem::zeroed();
    let mut blob: *mut u8;

    memset(
        &mut ctx as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&ctx),
    );

    ret = asn1_ber_decoder(
        &tpm2key_decoder as *const u8,
        &mut ctx as *mut _ as *mut core::ffi::c_void,
        (*payload).blob.as_ptr(),
        (*payload).blob_len,
    );
    if ret < 0 {
        return ret;
    }

    if ctx.priv_len + ctx.pub_len > MAX_BLOB_SIZE {
        return -EINVAL;
    }

    blob = kmalloc((ctx.priv_len + ctx.pub_len + 4) as size_t, GFP_KERNEL);
    if blob.is_null() {
        return -ENOMEM;
    }

    *buf = blob;
    (*options).keyhandle = ctx.parent;

    memcpy(blob as *mut core::ffi::c_void, ctx.priv_ as *const core::ffi::c_void, ctx.priv_len as size_t);
    blob = blob.add(ctx.priv_len as usize);

    memcpy(blob as *mut core::ffi::c_void, ctx.pub_ as *const core::ffi::c_void, ctx.pub_len as size_t);

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpm2_key_parent(
    context: *mut core::ffi::c_void,
    hdrlen: size_t,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: size_t,
) -> i32 {
    let ctx: *mut tpm2_key_context = context as *mut tpm2_key_context;
    let v: *const u8 = value as *const u8;
    let mut i: size_t;

    (*ctx).parent = 0;
    i = 0;
    while i < vlen {
        (*ctx).parent <<= 8;
        (*ctx).parent |= *v.add(i) as u32;
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpm2_key_type(
    context: *mut core::ffi::c_void,
    hdrlen: size_t,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: size_t,
) -> i32 {
    let oid: OID = look_up_OID(value, vlen);

    if oid != OID_TPMSealedData {
        let mut buffer: [i8; 50] = [0; 50];

        sprint_oid(value, vlen, buffer.as_mut_ptr(), core::mem::size_of_val(&buffer));
        pr_debug(
            c"OID is \"%s\" which is not TPMSealedData\n".as_ptr(),
            buffer.as_mut_ptr(),
        );
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpm2_key_pub(
    context: *mut core::ffi::c_void,
    hdrlen: size_t,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: size_t,
) -> i32 {
    let ctx: *mut tpm2_key_context = context as *mut tpm2_key_context;

    (*ctx).pub_ = value as *const u8;
    (*ctx).pub_len = vlen as u32;

    0
}

#[no_mangle]
pub unsafe extern "C" fn tpm2_key_priv(
    context: *mut core::ffi::c_void,
    hdrlen: size_t,
    tag: u8,
    value: *const core::ffi::c_void,
    vlen: size_t,
) -> i32 {
    let ctx: *mut tpm2_key_context = context as *mut tpm2_key_context;

    (*ctx).priv_ = value as *const u8;
    (*ctx).priv_len = vlen as u32;

    0
}

/**
 * tpm2_buf_append_auth() - append TPMS_AUTH_COMMAND to the buffer.
 *
 * @buf: an allocated tpm_buf instance
 * @session_handle: session handle
 * @nonce: the session nonce, may be NULL if not used
 * @nonce_len: the session nonce length, may be 0 if not used
 * @attributes: the session attributes
 * @hmac: the session HMAC or password, may be NULL if not used
 * @hmac_len: the session HMAC or password length, maybe 0 if not used
 */
unsafe fn tpm2_buf_append_auth(
    buf: *mut tpm_buf,
    session_handle: u32,
    nonce: *const u8,
    nonce_len: u16,
    attributes: u8,
    hmac: *const u8,
    hmac_len: u16,
) {
    tpm_buf_append_u32(buf, 9 + nonce_len as u32 + hmac_len as u32);
    tpm_buf_append_u32(buf, session_handle);
    tpm_buf_append_u16(buf, nonce_len);

    if !nonce.is_null() && nonce_len != 0 {
        tpm_buf_append(buf, nonce, nonce_len as size_t);
    }

    tpm_buf_append_u8(buf, attributes);
    tpm_buf_append_u16(buf, hmac_len);

    if !hmac.is_null() && hmac_len != 0 {
        tpm_buf_append(buf, hmac, hmac_len as size_t);
    }
}

/**
 * tpm2_seal_trusted() - seal the payload of a trusted key
 *
 * @chip: TPM chip to use
 * @payload: the key data in clear and encrypted form
 * @options: authentication values and other options
 *
 * Return: < 0 on error and 0 on success.
 */
#[no_mangle]
pub unsafe extern "C" fn tpm2_seal_trusted(
    chip: *mut tpm_chip,
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
) -> i32 {
    let mut offset: off_t = TPM_HEADER_SIZE;
    let mut buf: *mut tpm_buf = core::ptr::null_mut();
    let mut sized: *mut tpm_buf = core::ptr::null_mut();
    let mut blob_len: i32 = 0;
    let hash: i32;
    let mut flags: u32;
    let mut rc: i32;

    hash = tpm2_find_hash_alg((*options).hash);
    if hash < 0 {
        return hash;
    }

    if (*options).keyhandle == 0 {
        return -EINVAL;
    }

    rc = tpm_try_get_ops(chip);
    if rc != 0 {
        return rc;
    }

    rc = tpm2_start_auth_session(chip);
    if rc != 0 {
        tpm_put_ops(chip);
        return tpm_ret_to_err(rc);
    }

    buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if buf.is_null() {
        rc = -ENOMEM;
        tpm2_end_auth_session(chip);
        tpm_put_ops(chip);
        return tpm_ret_to_err(rc);
    }

    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_CREATE);

    sized = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if sized.is_null() {
        rc = -ENOMEM;
        tpm2_end_auth_session(chip);
        kfree(buf as *mut core::ffi::c_void);
        tpm_put_ops(chip);
        return tpm_ret_to_err(rc);
    }

    tpm_buf_init_sized(sized, TPM_BUFSIZE);

    rc = tpm_buf_append_name(chip, buf, (*options).keyhandle, core::ptr::null_mut());
    if rc == 0 {
        tpm_buf_append_hmac_session(chip, buf, TPM2_SA_DECRYPT, (*options).keyauth, TPM_DIGEST_SIZE);

        /* sensitive */
        tpm_buf_append_u16(sized, (*options).blobauth_len);

        if (*options).blobauth_len != 0 {
            tpm_buf_append(sized, (*options).blobauth, (*options).blobauth_len as size_t);
        }

        tpm_buf_append_u16(sized, (*payload).key_len);
        tpm_buf_append(sized, (*payload).key.as_ptr(), (*payload).key_len as size_t);
        tpm_buf_append(buf, (*sized).data, (*sized).length);

        /* public */
        tpm_buf_reset_sized(sized);
        tpm_buf_append_u16(sized, TPM_ALG_KEYEDHASH);
        tpm_buf_append_u16(sized, hash as u16);

        /* key properties */
        flags = 0;
        flags |= if (*options).policydigest_len != 0 { 0 } else { TPM2_OA_USER_WITH_AUTH };
        flags |= if (*payload).migratable != 0 {
            0
        } else {
            TPM2_OA_FIXED_TPM | TPM2_OA_FIXED_PARENT
        };
        tpm_buf_append_u32(sized, flags);

        /* policy */
        tpm_buf_append_u16(sized, (*options).policydigest_len);
        if (*options).policydigest_len != 0 {
            tpm_buf_append(sized, (*options).policydigest, (*options).policydigest_len as size_t);
        }

        /* public parameters */
        tpm_buf_append_u16(sized, TPM_ALG_NULL);
        tpm_buf_append_u16(sized, 0);

        tpm_buf_append(buf, (*sized).data, (*sized).length);

        /* outside info */
        tpm_buf_append_u16(buf, 0);

        /* creation PCR */
        tpm_buf_append_u32(buf, 0);

        if ((*buf).flags & TPM_BUF_INVALID) != 0 {
            rc = -E2BIG;
            tpm2_end_auth_session(chip);
        } else {
            rc = tpm_buf_fill_hmac_session(chip, buf);
            if rc == 0 {
                rc = tpm_transmit_cmd(chip, buf, 4, c"sealing data".as_ptr());
                rc = tpm_buf_check_hmac_response(chip, buf, rc);
                if rc == 0 {
                    blob_len = tpm_buf_read_u32(buf, &mut offset);
                    if blob_len > MAX_BLOB_SIZE as i32 || ((*buf).flags & TPM_BUF_INVALID) != 0 {
                        rc = -E2BIG;
                    } else if (*buf).length - offset as usize < blob_len as usize {
                        rc = -EFAULT;
                    } else {
                        blob_len = tpm2_key_encode(payload, options, (*buf).data.add(offset as usize), blob_len as u32);
                        if blob_len < 0 {
                            rc = blob_len;
                        }
                    }
                }
            }
        }
    }

    if rc == 0 {
        (*payload).blob_len = blob_len as u32;
    }

    kfree(sized as *mut core::ffi::c_void);
    kfree(buf as *mut core::ffi::c_void);
    tpm_put_ops(chip);
    tpm_ret_to_err(rc)
}

/**
 * tpm2_load_cmd() - execute a TPM2_Load command
 *
 * @chip: TPM chip to use
 * @payload: the key data in clear and encrypted form
 * @options: authentication values and other options
 * @blob_handle: returned blob handle
 *
 * Return: 0 on success.
 *        -E2BIG on wrong payload size.
 *        -EPERM on tpm error status.
 *        < 0 error from tpm_send.
 */
unsafe fn tpm2_load_cmd(
    chip: *mut tpm_chip,
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
    blob_handle: *mut u32,
) -> i32 {
    let mut blob_ref: *mut u8 = core::ptr::null_mut();
    let mut buf: *mut tpm_buf = core::ptr::null_mut();
    let private_len: u32;
    let public_len: u32;
    let blob_len: u32;
    let mut blob: *mut u8 = core::ptr::null_mut();
    let pub_: *mut u8;
    let mut rc: i32;
    let attrs: u32;

    rc = tpm2_key_decode(payload, options, &mut blob);
    if rc != 0 {
        /* old form */
        blob = (*payload).blob.as_mut_ptr();
        (*payload).old_format = 1;
    } else {
        /* Bind for cleanup: */
        blob_ref = blob;
    }

    /* new format carries keyhandle but old format doesn't */
    if (*options).keyhandle == 0 {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -EINVAL;
    }

    /* must be big enough for at least the two be16 size counts */
    if (*payload).blob_len < 4 {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -EINVAL;
    }

    private_len = get_unaligned_be16(blob) as u32;

    /* must be big enough for following public_len */
    if private_len + 2 + 2 > (*payload).blob_len {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -E2BIG;
    }

    public_len = get_unaligned_be16(blob.add((2 + private_len) as usize)) as u32;
    if private_len + 2 + public_len + 2 > (*payload).blob_len {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -E2BIG;
    }

    pub_ = blob.add((2 + private_len + 2) as usize);
    /* key attributes are always at offset 4 */
    attrs = get_unaligned_be32(pub_.add(4));

    if (attrs & (TPM2_OA_FIXED_TPM | TPM2_OA_FIXED_PARENT))
        == (TPM2_OA_FIXED_TPM | TPM2_OA_FIXED_PARENT)
    {
        (*payload).migratable = 0;
    } else {
        (*payload).migratable = 1;
    }

    blob_len = private_len + public_len + 4;
    if blob_len > (*payload).blob_len {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -E2BIG;
    }

    rc = tpm2_start_auth_session(chip);
    if rc != 0 {
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return rc;
    }

    buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if buf.is_null() {
        tpm2_end_auth_session(chip);
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -ENOMEM;
    }

    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_LOAD);

    rc = tpm_buf_append_name(chip, buf, (*options).keyhandle, core::ptr::null_mut());
    if rc != 0 {
        kfree(buf as *mut core::ffi::c_void);
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return rc;
    }

    tpm_buf_append_hmac_session(chip, buf, 0, (*options).keyauth, TPM_DIGEST_SIZE);

    tpm_buf_append(buf, blob, blob_len as size_t);

    if ((*buf).flags & TPM_BUF_INVALID) != 0 {
        tpm2_end_auth_session(chip);
        kfree(buf as *mut core::ffi::c_void);
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return -E2BIG;
    }

    rc = tpm_buf_fill_hmac_session(chip, buf);
    if rc != 0 {
        kfree(buf as *mut core::ffi::c_void);
        if !blob_ref.is_null() {
            kfree(blob_ref as *mut core::ffi::c_void);
        }
        return rc;
    }

    rc = tpm_transmit_cmd(chip, buf, 4, c"loading blob".as_ptr());
    rc = tpm_buf_check_hmac_response(chip, buf, rc);
    if rc == 0 {
        *blob_handle = be32_to_cpup((*buf).data.add(TPM_HEADER_SIZE as usize) as *const u32);
    }

    kfree(buf as *mut core::ffi::c_void);
    if !blob_ref.is_null() {
        kfree(blob_ref as *mut core::ffi::c_void);
    }
    tpm_ret_to_err(rc)
}

/**
 * tpm2_unseal_cmd() - execute a TPM2_Unseal command
 *
 * @chip: TPM chip to use
 * @payload: the key data in clear and encrypted form
 * @options: authentication values and other options
 * @blob_handle: blob handle
 *
 * Return: 0 on success
 *         -EPERM on tpm error status
 *         < 0 error from tpm_send
 */
unsafe fn tpm2_unseal_cmd(
    chip: *mut tpm_chip,
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
    blob_handle: u32,
) -> i32 {
    let mut head: *mut tpm_header;
    let mut buf: *mut tpm_buf = core::ptr::null_mut();
    let data_len: u16;
    let mut offset: i32;
    let data: *mut u8;
    let mut rc: i32;

    rc = tpm2_start_auth_session(chip);
    if rc != 0 {
        return rc;
    }

    buf = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if buf.is_null() {
        tpm2_end_auth_session(chip);
        return -ENOMEM;
    }

    tpm_buf_init(buf, TPM_BUFSIZE);
    tpm_buf_reset(buf, TPM2_ST_SESSIONS, TPM2_CC_UNSEAL);

    rc = tpm_buf_append_name(chip, buf, blob_handle, core::ptr::null_mut());
    if rc != 0 {
        kfree(buf as *mut core::ffi::c_void);
        return rc;
    }

    if (*options).policyhandle == 0 {
        tpm_buf_append_hmac_session(chip, buf, TPM2_SA_ENCRYPT, (*options).blobauth, (*options).blobauth_len);
    } else {
        /*
         * FIXME: The policy session was generated outside the
         * kernel so we don't known the nonce and thus can't
         * calculate a HMAC on it.  Therefore, the user can
         * only really use TPM2_PolicyPassword and we must
         * send down the plain text password, which could be
         * intercepted.  We can still encrypt the returned
         * key, but that's small comfort since the interposer
         * could repeat our actions with the exfiltrated
         * password.
         */
        tpm2_buf_append_auth(
            buf,
            (*options).policyhandle,
            core::ptr::null(), /* nonce */
            0,
            0,
            (*options).blobauth,
            (*options).blobauth_len,
        );
        if tpm2_chip_auth(chip) {
            tpm_buf_append_hmac_session(chip, buf, TPM2_SA_ENCRYPT, core::ptr::null(), 0);
        } else {
            offset = (*buf).handles * 4 + TPM_HEADER_SIZE as i32;
            head = (*buf).data as *mut tpm_header;
            if tpm_buf_length(buf) == offset as size_t {
                (*head).tag = cpu_to_be16(TPM2_ST_NO_SESSIONS);
            }
        }
    }

    rc = tpm_buf_fill_hmac_session(chip, buf);
    if rc != 0 {
        kfree(buf as *mut core::ffi::c_void);
        return rc;
    }

    rc = tpm_transmit_cmd(chip, buf, 6, c"unsealing".as_ptr());
    rc = tpm_buf_check_hmac_response(chip, buf, rc);

    if rc == 0 {
        data_len = be16_to_cpup((*buf).data.add(TPM_HEADER_SIZE as usize + 4) as *const u16);
        if data_len < MIN_KEY_SIZE || data_len > MAX_KEY_SIZE {
            kfree(buf as *mut core::ffi::c_void);
            return -EFAULT;
        }

        if tpm_buf_length(buf) < TPM_HEADER_SIZE as size_t + 6 + data_len as size_t {
            kfree(buf as *mut core::ffi::c_void);
            return -EFAULT;
        }
        data = (*buf).data.add(TPM_HEADER_SIZE as usize + 6);

        if (*payload).old_format != 0 {
            /* migratable flag is at the end of the key */
            memcpy(
                (*payload).key.as_mut_ptr() as *mut core::ffi::c_void,
                data as *const core::ffi::c_void,
                (data_len - 1) as size_t,
            );
            (*payload).key_len = data_len - 1;
            (*payload).migratable = *data.add(data_len as usize - 1);
        } else {
            /*
             * migratable flag already collected from key
             * attributes
             */
            memcpy(
                (*payload).key.as_mut_ptr() as *mut core::ffi::c_void,
                data as *const core::ffi::c_void,
                data_len as size_t,
            );
            (*payload).key_len = data_len;
        }
    }

    kfree(buf as *mut core::ffi::c_void);
    tpm_ret_to_err(rc)
}

/**
 * tpm2_unseal_trusted() - unseal the payload of a trusted key
 *
 * @chip: TPM chip to use
 * @payload: the key data in clear and encrypted form
 * @options: authentication values and other options
 *
 * Return: Same as with tpm_send.
 */
#[no_mangle]
pub unsafe extern "C" fn tpm2_unseal_trusted(
    chip: *mut tpm_chip,
    payload: *mut trusted_key_payload,
    options: *mut trusted_key_options,
) -> i32 {
    let mut blob_handle: u32 = 0;
    let mut rc: i32;

    rc = tpm_try_get_ops(chip);
    if rc != 0 {
        return rc;
    }

    rc = tpm2_load_cmd(chip, payload, options, &mut blob_handle);
    if rc == 0 {
        rc = tpm2_unseal_cmd(chip, payload, options, blob_handle);
        tpm2_flush_context(chip, blob_handle);
    }

    tpm_put_ops(chip);
    tpm_ret_to_err(rc)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
