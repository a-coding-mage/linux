// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 IBM Corporation
 * Copyright (c) 2019-2021, Linaro Limited
 *
 * See Documentation/security/keys/trusted-encrypted.rst
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void, VaListImpl};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type uint16_t = u16;
type uint32_t = u32;
type size_t = usize;
type gfp_t = c_uint;
type match_table_t = [match_token; 10];

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EPERM: c_int = 1;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const CAP_SYS_ADMIN: c_int = 21;
const DUMP_PREFIX_NONE: c_int = 0;

const SHA1_DIGEST_SIZE: usize = 20;
const TPM_NONCE_SIZE: usize = 20;
const TPM_DIGEST_SIZE: usize = 20;
const TPM_BUFSIZE: usize = 4096;
const MAX_OPT_ARGS: usize = 3;
const MAX_PCRINFO_SIZE: usize = 64;
const HASH_ALGO_SHA1: c_int = 2;
const HASH_ALGO_SHA256: c_int = 4;
const HASH_ALGO__LAST: c_int = 16;
const SRKHANDLE: uint32_t = 0x40000000;

const TPM_TAG_RQU_COMMAND: uint16_t = 193;
const TPM_TAG_RQU_AUTH1_COMMAND: uint16_t = 194;
const TPM_TAG_RQU_AUTH2_COMMAND: uint16_t = 195;
const TPM_TAG_RSP_COMMAND: uint16_t = 196;
const TPM_TAG_RSP_AUTH1_COMMAND: uint16_t = 197;
const TPM_TAG_RSP_AUTH2_COMMAND: uint16_t = 198;
const TPM_ORD_OSAP: uint32_t = 0x0000000b;
const TPM_ORD_OIAP: uint32_t = 0x0000000a;
const TPM_ORD_SEAL: uint32_t = 0x00000017;
const TPM_ORD_UNSEAL: uint32_t = 0x00000018;

static mut chip: *mut tpm_chip = ptr::null_mut();
static mut digests: *mut tpm_digest = ptr::null_mut();

/* implementation specific TPM constants */
const TPM_SIZE_OFFSET: usize = 2;
const TPM_RETURN_OFFSET: usize = 6;
const TPM_DATA_OFFSET: usize = 10;

unsafe fn LOAD32(buffer: *const c_uchar, offset: usize) -> uint32_t {
    ntohl(ptr::read_unaligned(buffer.add(offset) as *const uint32_t))
}

unsafe fn LOAD32N(buffer: *const c_uchar, offset: usize) -> uint32_t {
    ptr::read_unaligned(buffer.add(offset) as *const uint32_t)
}

unsafe fn LOAD16(buffer: *const c_uchar, offset: usize) -> uint16_t {
    ntohs(ptr::read_unaligned(buffer.add(offset) as *const uint16_t))
}

type c_uchar = u8;

#[repr(C)]
struct osapsess {
    handle: uint32_t,
    secret: [c_uchar; SHA1_DIGEST_SIZE],
    enonce: [c_uchar; TPM_NONCE_SIZE],
}

/* discrete values, but have to store in uint16_t for TPM use */
const SEAL_keytype: c_int = 1;
const SRK_keytype: c_int = 4;

/* CONFIG_TRUSTED_KEYS_DEBUG: debug versions print trusted key options and TPM buffers. */
unsafe fn dump_options(_o: *mut trusted_key_options) {}

unsafe fn dump_sess(_s: *mut osapsess) {}

unsafe fn dump_tpm_buf(_buf: *mut c_uchar) {}

unsafe extern "C" fn TSS_rawhmac(
    digest: *mut c_uchar,
    key: *const c_uchar,
    keylen: c_uint,
    mut args: ...
) -> c_int {
    let mut hmac_ctx: hmac_sha1_ctx = core::mem::zeroed();
    let mut ret: c_int = 0;

    hmac_sha1_init_usingrawkey(&mut hmac_ctx, key, keylen);

    loop {
        let dlen: c_uint = args.arg();
        if dlen == 0 {
            break;
        }
        let data: *mut c_uchar = args.arg();
        if data.is_null() {
            ret = -EINVAL;
            break;
        }
        hmac_sha1_update(&mut hmac_ctx, data, dlen);
    }
    if ret == 0 {
        hmac_sha1_final(&mut hmac_ctx, digest);
    }
    ret
}

/*
 * calculate authorization info fields to send to TPM
 */
unsafe extern "C" fn TSS_authhmac(
    digest: *mut c_uchar,
    key: *const c_uchar,
    keylen: c_uint,
    h1: *mut c_uchar,
    h2: *mut c_uchar,
    h3: c_uint,
    mut args: ...
) -> c_int {
    let mut paramdigest = [0u8; SHA1_DIGEST_SIZE];
    let mut sha_ctx: sha1_ctx = core::mem::zeroed();
    let mut ret: c_int = 0;
    let c: c_uchar = (h3 != 0) as c_uchar;

    if chip.is_null() {
        return -ENODEV;
    }

    sha1_init(&mut sha_ctx);
    loop {
        let dlen: c_uint = args.arg();
        if dlen == 0 {
            break;
        }
        let data: *mut c_uchar = args.arg();
        if data.is_null() {
            ret = -EINVAL;
            break;
        }
        sha1_update(&mut sha_ctx, data, dlen);
    }
    if ret == 0 {
        sha1_final(&mut sha_ctx, paramdigest.as_mut_ptr());
    }
    if ret == 0 {
        ret = TSS_rawhmac(
            digest,
            key,
            keylen,
            SHA1_DIGEST_SIZE as c_uint,
            paramdigest.as_mut_ptr(),
            TPM_NONCE_SIZE as c_uint,
            h1,
            TPM_NONCE_SIZE as c_uint,
            h2,
            1u32,
            &c as *const c_uchar as *mut c_uchar,
            0u32,
            0usize,
        );
    }
    ret
}

/*
 * verify the AUTH1_COMMAND (Seal) result from TPM
 */
unsafe extern "C" fn TSS_checkhmac1(
    buffer: *mut c_uchar,
    command: uint32_t,
    ononce: *const c_uchar,
    key: *const c_uchar,
    keylen: c_uint,
    mut args: ...
) -> c_int {
    let bufsize = LOAD32(buffer, TPM_SIZE_OFFSET);
    let tag = LOAD16(buffer, 0);
    let ordinal = command;
    let result = LOAD32N(buffer, TPM_RETURN_OFFSET);
    let mut testhmac = [0u8; SHA1_DIGEST_SIZE];
    let mut paramdigest = [0u8; SHA1_DIGEST_SIZE];
    let mut sha_ctx: sha1_ctx = core::mem::zeroed();

    if chip.is_null() {
        return -ENODEV;
    }
    if tag == TPM_TAG_RSP_COMMAND {
        return 0;
    }
    if tag != TPM_TAG_RSP_AUTH1_COMMAND {
        return -EINVAL;
    }

    let authdata = buffer.add(bufsize as usize - SHA1_DIGEST_SIZE);
    let continueflag = authdata.sub(1);
    let enonce = continueflag.sub(TPM_NONCE_SIZE);

    sha1_init(&mut sha_ctx);
    sha1_update(&mut sha_ctx, &result as *const uint32_t as *const u8, size_of::<uint32_t>() as c_uint);
    sha1_update(&mut sha_ctx, &ordinal as *const uint32_t as *const u8, size_of::<uint32_t>() as c_uint);
    loop {
        let dlen: c_uint = args.arg();
        if dlen == 0 {
            break;
        }
        let dpos: c_uint = args.arg();
        sha1_update(&mut sha_ctx, buffer.add(dpos as usize), dlen);
    }
    sha1_final(&mut sha_ctx, paramdigest.as_mut_ptr());

    let ret = TSS_rawhmac(
        testhmac.as_mut_ptr(),
        key,
        keylen,
        SHA1_DIGEST_SIZE as c_uint,
        paramdigest.as_mut_ptr(),
        TPM_NONCE_SIZE as c_uint,
        enonce,
        TPM_NONCE_SIZE as c_uint,
        ononce as *mut c_uchar,
        1u32,
        continueflag,
        0u32,
        0usize,
    );
    if ret < 0 {
        return ret;
    }
    if crypto_memneq(testhmac.as_mut_ptr(), authdata, SHA1_DIGEST_SIZE) != 0 {
        return -EINVAL;
    }
    0
}

/*
 * verify the AUTH2_COMMAND (unseal) result from TPM
 */
unsafe extern "C" fn TSS_checkhmac2(
    buffer: *mut c_uchar,
    command: uint32_t,
    ononce: *const c_uchar,
    key1: *const c_uchar,
    keylen1: c_uint,
    key2: *const c_uchar,
    keylen2: c_uint,
    mut args: ...
) -> c_int {
    let bufsize = LOAD32(buffer, TPM_SIZE_OFFSET);
    let tag = LOAD16(buffer, 0);
    let ordinal = command;
    let result = LOAD32N(buffer, TPM_RETURN_OFFSET);
    let mut testhmac1 = [0u8; SHA1_DIGEST_SIZE];
    let mut testhmac2 = [0u8; SHA1_DIGEST_SIZE];
    let mut paramdigest = [0u8; SHA1_DIGEST_SIZE];
    let mut sha_ctx: sha1_ctx = core::mem::zeroed();

    if tag == TPM_TAG_RSP_COMMAND {
        return 0;
    }
    if tag != TPM_TAG_RSP_AUTH2_COMMAND {
        return -EINVAL;
    }
    let authdata1 = buffer.add(bufsize as usize - (SHA1_DIGEST_SIZE + 1 + SHA1_DIGEST_SIZE + SHA1_DIGEST_SIZE));
    let authdata2 = buffer.add(bufsize as usize - SHA1_DIGEST_SIZE);
    let continueflag1 = authdata1.sub(1);
    let continueflag2 = authdata2.sub(1);
    let enonce1 = continueflag1.sub(TPM_NONCE_SIZE);
    let enonce2 = continueflag2.sub(TPM_NONCE_SIZE);

    sha1_init(&mut sha_ctx);
    sha1_update(&mut sha_ctx, &result as *const uint32_t as *const u8, size_of::<uint32_t>() as c_uint);
    sha1_update(&mut sha_ctx, &ordinal as *const uint32_t as *const u8, size_of::<uint32_t>() as c_uint);
    loop {
        let dlen: c_uint = args.arg();
        if dlen == 0 {
            break;
        }
        let dpos: c_uint = args.arg();
        sha1_update(&mut sha_ctx, buffer.add(dpos as usize), dlen);
    }
    sha1_final(&mut sha_ctx, paramdigest.as_mut_ptr());

    let mut ret = TSS_rawhmac(testhmac1.as_mut_ptr(), key1, keylen1, SHA1_DIGEST_SIZE as c_uint,
        paramdigest.as_mut_ptr(), TPM_NONCE_SIZE as c_uint, enonce1, TPM_NONCE_SIZE as c_uint,
        ononce as *mut c_uchar, 1u32, continueflag1, 0u32, 0usize);
    if ret < 0 {
        return ret;
    }
    if crypto_memneq(testhmac1.as_mut_ptr(), authdata1, SHA1_DIGEST_SIZE) != 0 {
        return -EINVAL;
    }
    ret = TSS_rawhmac(testhmac2.as_mut_ptr(), key2, keylen2, SHA1_DIGEST_SIZE as c_uint,
        paramdigest.as_mut_ptr(), TPM_NONCE_SIZE as c_uint, enonce2, TPM_NONCE_SIZE as c_uint,
        ononce as *mut c_uchar, 1u32, continueflag2, 0u32, 0usize);
    if ret < 0 {
        return ret;
    }
    if crypto_memneq(testhmac2.as_mut_ptr(), authdata2, SHA1_DIGEST_SIZE) != 0 {
        return -EINVAL;
    }
    0
}

/*
 * For key specific tpm requests, we will generate and send our
 * own TPM command packets using the drivers send function.
 */
unsafe fn trusted_tpm_send(buf: *mut tpm_buf) -> c_int {
    let mut rc: c_int;

    if chip.is_null() {
        return -ENODEV;
    }

    rc = tpm_try_get_ops(chip);
    if rc != 0 {
        return rc;
    }

    dump_tpm_buf((*buf).data);
    rc = tpm_transmit_cmd(chip, buf, 4, c"sending data".as_ptr());
    dump_tpm_buf((*buf).data);

    if rc > 0 {
        /* TPM error */
        rc = -EPERM;
    }

    tpm_put_ops(chip);
    rc
}

/*
 * Lock a trusted key, by extending a selected PCR.
 *
 * Prevents a trusted key that is sealed to PCRs from being accessed.
 * This uses the tpm driver's extend function.
 */
unsafe fn pcrlock(pcrnum: c_int) -> c_int {
    if capable(CAP_SYS_ADMIN) == 0 {
        return -EPERM;
    }

    if tpm_pcr_extend(chip, pcrnum, digests) != 0 { -EINVAL } else { 0 }
}

/*
 * Create an object specific authorisation protocol (OSAP) session
 */
unsafe fn osap(tb: *mut tpm_buf, s: *mut osapsess, key: *const c_uchar, type_: uint16_t, handle: uint32_t) -> c_int {
    let mut enonce = [0u8; TPM_NONCE_SIZE];
    let mut ononce = [0u8; TPM_NONCE_SIZE];
    let mut ret = tpm_get_random(chip, ononce.as_mut_ptr(), TPM_NONCE_SIZE);
    if ret < 0 {
        return ret;
    }
    if ret != TPM_NONCE_SIZE as c_int {
        return -EIO;
    }

    tpm_buf_reset(tb, TPM_TAG_RQU_COMMAND, TPM_ORD_OSAP);
    tpm_buf_append_u16(tb, type_);
    tpm_buf_append_u32(tb, handle);
    tpm_buf_append(tb, ononce.as_mut_ptr(), TPM_NONCE_SIZE as c_uint);

    ret = trusted_tpm_send(tb);
    if ret < 0 {
        return ret;
    }

    (*s).handle = LOAD32((*tb).data, TPM_DATA_OFFSET);
    memcpy((*s).enonce.as_mut_ptr(), (*tb).data.add(TPM_DATA_OFFSET + size_of::<uint32_t>()), TPM_NONCE_SIZE);
    memcpy(enonce.as_mut_ptr(), (*tb).data.add(TPM_DATA_OFFSET + size_of::<uint32_t>() + TPM_NONCE_SIZE), TPM_NONCE_SIZE);
    TSS_rawhmac((*s).secret.as_mut_ptr(), key, SHA1_DIGEST_SIZE as c_uint,
        TPM_NONCE_SIZE as c_uint, enonce.as_mut_ptr(), TPM_NONCE_SIZE as c_uint, ononce.as_mut_ptr(), 0u32, 0usize)
}

/*
 * Create an object independent authorisation protocol (oiap) session
 */
unsafe fn oiap(tb: *mut tpm_buf, handle: *mut uint32_t, nonce: *mut c_uchar) -> c_int {
    if chip.is_null() {
        return -ENODEV;
    }

    tpm_buf_reset(tb, TPM_TAG_RQU_COMMAND, TPM_ORD_OIAP);
    let ret = trusted_tpm_send(tb);
    if ret < 0 {
        return ret;
    }

    *handle = LOAD32((*tb).data, TPM_DATA_OFFSET);
    memcpy(nonce, (*tb).data.add(TPM_DATA_OFFSET + size_of::<uint32_t>()), TPM_NONCE_SIZE);
    0
}

#[repr(C)]
struct tpm_digests {
    encauth: [c_uchar; SHA1_DIGEST_SIZE],
    pubauth: [c_uchar; SHA1_DIGEST_SIZE],
    xorwork: [c_uchar; SHA1_DIGEST_SIZE * 2],
    xorhash: [c_uchar; SHA1_DIGEST_SIZE],
    nonceodd: [c_uchar; TPM_NONCE_SIZE],
}

/*
 * Have the TPM seal(encrypt) the trusted key, possibly based on
 * Platform Configuration Registers (PCRs). AUTH1 for sealing key.
 */
unsafe fn tpm_seal(
    tb: *mut tpm_buf,
    keytype: uint16_t,
    keyhandle: uint32_t,
    keyauth: *const c_uchar,
    data: *const c_uchar,
    datalen: uint32_t,
    blob: *mut c_uchar,
    bloblen: *mut uint32_t,
    blobauth: *const c_uchar,
    pcrinfo: *const c_uchar,
    pcrinfosize: uint32_t,
) -> c_int {
    let mut sess: osapsess = core::mem::zeroed();
    let td = kmalloc(size_of::<tpm_digests>(), GFP_KERNEL) as *mut tpm_digests;
    let mut ret: c_int;

    /* alloc some work space for all the hashes */
    if td.is_null() {
        return -ENOMEM;
    }

    /* get session for sealing key */
    ret = osap(tb, &mut sess, keyauth, keytype, keyhandle);
    if ret < 0 {
        kfree_sensitive(td as *mut c_void);
        return ret;
    }
    dump_sess(&mut sess);

    /* calculate encrypted authorization value */
    memcpy((*td).xorwork.as_mut_ptr(), sess.secret.as_mut_ptr(), SHA1_DIGEST_SIZE);
    memcpy((*td).xorwork.as_mut_ptr().add(SHA1_DIGEST_SIZE), sess.enonce.as_mut_ptr(), SHA1_DIGEST_SIZE);
    sha1((*td).xorwork.as_mut_ptr(), (SHA1_DIGEST_SIZE * 2) as c_uint, (*td).xorhash.as_mut_ptr());

    ret = tpm_get_random(chip, (*td).nonceodd.as_mut_ptr(), TPM_NONCE_SIZE);
    if ret < 0 {
        kfree_sensitive(td as *mut c_void);
        return ret;
    }
    if ret != TPM_NONCE_SIZE as c_int {
        kfree_sensitive(td as *mut c_void);
        return -EIO;
    }

    let ordinal = htonl(TPM_ORD_SEAL);
    let datsize = htonl(datalen);
    let pcrsize = htonl(pcrinfosize);
    let cont: c_uchar = 0;

    /* encrypt data authorization key */
    for i in 0..SHA1_DIGEST_SIZE {
        (*td).encauth[i] = (*td).xorhash[i] ^ *blobauth.add(i);
    }

    /* calculate authorization HMAC value */
    if pcrinfosize == 0 {
        /* no pcr info specified */
        ret = TSS_authhmac((*td).pubauth.as_mut_ptr(), sess.secret.as_ptr(), SHA1_DIGEST_SIZE as c_uint,
            sess.enonce.as_mut_ptr(), (*td).nonceodd.as_mut_ptr(), cont as c_uint,
            size_of::<uint32_t>() as c_uint, &ordinal as *const uint32_t as *mut c_uchar,
            SHA1_DIGEST_SIZE as c_uint, (*td).encauth.as_mut_ptr(),
            size_of::<uint32_t>() as c_uint, &pcrsize as *const uint32_t as *mut c_uchar,
            size_of::<uint32_t>() as c_uint, &datsize as *const uint32_t as *mut c_uchar,
            datalen, data as *mut c_uchar, 0u32, 0usize);
    } else {
        /* pcr info specified */
        ret = TSS_authhmac((*td).pubauth.as_mut_ptr(), sess.secret.as_ptr(), SHA1_DIGEST_SIZE as c_uint,
            sess.enonce.as_mut_ptr(), (*td).nonceodd.as_mut_ptr(), cont as c_uint,
            size_of::<uint32_t>() as c_uint, &ordinal as *const uint32_t as *mut c_uchar,
            SHA1_DIGEST_SIZE as c_uint, (*td).encauth.as_mut_ptr(),
            size_of::<uint32_t>() as c_uint, &pcrsize as *const uint32_t as *mut c_uchar,
            pcrinfosize, pcrinfo as *mut c_uchar,
            size_of::<uint32_t>() as c_uint, &datsize as *const uint32_t as *mut c_uchar,
            datalen, data as *mut c_uchar, 0u32, 0usize);
    }
    if ret < 0 {
        kfree_sensitive(td as *mut c_void);
        return ret;
    }

    /* build and send the TPM request packet */
    tpm_buf_reset(tb, TPM_TAG_RQU_AUTH1_COMMAND, TPM_ORD_SEAL);
    tpm_buf_append_u32(tb, keyhandle);
    tpm_buf_append(tb, (*td).encauth.as_mut_ptr(), SHA1_DIGEST_SIZE as c_uint);
    tpm_buf_append_u32(tb, pcrinfosize);
    tpm_buf_append(tb, pcrinfo as *mut c_uchar, pcrinfosize);
    tpm_buf_append_u32(tb, datalen);
    tpm_buf_append(tb, data as *mut c_uchar, datalen);
    tpm_buf_append_u32(tb, sess.handle);
    tpm_buf_append(tb, (*td).nonceodd.as_mut_ptr(), TPM_NONCE_SIZE as c_uint);
    tpm_buf_append_u8(tb, cont);
    tpm_buf_append(tb, (*td).pubauth.as_mut_ptr(), SHA1_DIGEST_SIZE as c_uint);

    ret = trusted_tpm_send(tb);
    if ret < 0 {
        kfree_sensitive(td as *mut c_void);
        return ret;
    }

    /* calculate the size of the returned Blob */
    let sealinfosize = LOAD32((*tb).data, TPM_DATA_OFFSET + size_of::<uint32_t>()) as c_int;
    let encdatasize = LOAD32((*tb).data, TPM_DATA_OFFSET + size_of::<uint32_t>() + size_of::<uint32_t>() + sealinfosize as usize) as c_int;
    let storedsize = (size_of::<uint32_t>() + size_of::<uint32_t>() + sealinfosize as usize + size_of::<uint32_t>() + encdatasize as usize) as c_int;

    /* check the HMAC in the response */
    ret = TSS_checkhmac1((*tb).data, ordinal, (*td).nonceodd.as_mut_ptr(), sess.secret.as_ptr(),
        SHA1_DIGEST_SIZE as c_uint, storedsize as c_uint, TPM_DATA_OFFSET as c_uint, 0u32, 0usize);

    /* copy the returned blob to caller */
    if ret == 0 {
        memcpy(blob, (*tb).data.add(TPM_DATA_OFFSET), storedsize as usize);
        *bloblen = storedsize as uint32_t;
    }
    kfree_sensitive(td as *mut c_void);
    ret
}

/*
 * use the AUTH2_COMMAND form of unseal, to authorize both key and blob
 */
unsafe fn tpm_unseal(
    tb: *mut tpm_buf,
    keyhandle: uint32_t,
    keyauth: *const c_uchar,
    blob: *const c_uchar,
    bloblen: c_int,
    blobauth: *const c_uchar,
    data: *mut c_uchar,
    datalen: *mut c_uint,
) -> c_int {
    let mut nonceodd = [0u8; TPM_NONCE_SIZE];
    let mut enonce1 = [0u8; TPM_NONCE_SIZE];
    let mut enonce2 = [0u8; TPM_NONCE_SIZE];
    let mut authdata1 = [0u8; SHA1_DIGEST_SIZE];
    let mut authdata2 = [0u8; SHA1_DIGEST_SIZE];
    let mut authhandle1: uint32_t = 0;
    let mut authhandle2: uint32_t = 0;
    let cont: c_uchar = 0;

    /* sessions for unsealing key and data */
    let mut ret = oiap(tb, &mut authhandle1, enonce1.as_mut_ptr());
    if ret < 0 {
        pr_info(c"oiap failed (%d)\n".as_ptr(), ret);
        return ret;
    }
    ret = oiap(tb, &mut authhandle2, enonce2.as_mut_ptr());
    if ret < 0 {
        pr_info(c"oiap failed (%d)\n".as_ptr(), ret);
        return ret;
    }

    let ordinal = htonl(TPM_ORD_UNSEAL);
    ret = tpm_get_random(chip, nonceodd.as_mut_ptr(), TPM_NONCE_SIZE);
    if ret < 0 {
        return ret;
    }
    if ret != TPM_NONCE_SIZE as c_int {
        pr_info(c"tpm_get_random failed (%d)\n".as_ptr(), ret);
        return -EIO;
    }
    ret = TSS_authhmac(authdata1.as_mut_ptr(), keyauth, TPM_NONCE_SIZE as c_uint,
        enonce1.as_mut_ptr(), nonceodd.as_mut_ptr(), cont as c_uint,
        size_of::<uint32_t>() as c_uint, &ordinal as *const uint32_t as *mut c_uchar,
        bloblen as c_uint, blob as *mut c_uchar, 0u32, 0usize);
    if ret < 0 {
        return ret;
    }
    ret = TSS_authhmac(authdata2.as_mut_ptr(), blobauth, TPM_NONCE_SIZE as c_uint,
        enonce2.as_mut_ptr(), nonceodd.as_mut_ptr(), cont as c_uint,
        size_of::<uint32_t>() as c_uint, &ordinal as *const uint32_t as *mut c_uchar,
        bloblen as c_uint, blob as *mut c_uchar, 0u32, 0usize);
    if ret < 0 {
        return ret;
    }

    /* build and send TPM request packet */
    tpm_buf_reset(tb, TPM_TAG_RQU_AUTH2_COMMAND, TPM_ORD_UNSEAL);
    tpm_buf_append_u32(tb, keyhandle);
    tpm_buf_append(tb, blob as *mut c_uchar, bloblen as c_uint);
    tpm_buf_append_u32(tb, authhandle1);
    tpm_buf_append(tb, nonceodd.as_mut_ptr(), TPM_NONCE_SIZE as c_uint);
    tpm_buf_append_u8(tb, cont);
    tpm_buf_append(tb, authdata1.as_mut_ptr(), SHA1_DIGEST_SIZE as c_uint);
    tpm_buf_append_u32(tb, authhandle2);
    tpm_buf_append(tb, nonceodd.as_mut_ptr(), TPM_NONCE_SIZE as c_uint);
    tpm_buf_append_u8(tb, cont);
    tpm_buf_append(tb, authdata2.as_mut_ptr(), SHA1_DIGEST_SIZE as c_uint);

    ret = trusted_tpm_send(tb);
    if ret < 0 {
        pr_info(c"authhmac failed (%d)\n".as_ptr(), ret);
        return ret;
    }

    *datalen = LOAD32((*tb).data, TPM_DATA_OFFSET);
    ret = TSS_checkhmac2((*tb).data, ordinal, nonceodd.as_mut_ptr(),
        keyauth, SHA1_DIGEST_SIZE as c_uint, blobauth, SHA1_DIGEST_SIZE as c_uint,
        size_of::<uint32_t>() as c_uint, TPM_DATA_OFFSET as c_uint,
        *datalen, (TPM_DATA_OFFSET + size_of::<uint32_t>()) as c_uint, 0u32, 0usize);
    if ret < 0 {
        pr_info(c"TSS_checkhmac2 failed (%d)\n".as_ptr(), ret);
        return ret;
    }
    memcpy(data, (*tb).data.add(TPM_DATA_OFFSET + size_of::<uint32_t>()), *datalen as usize);
    0
}

/*
 * Have the TPM seal(encrypt) the symmetric key
 */
unsafe fn key_seal(p: *mut trusted_key_payload, o: *mut trusted_key_options) -> c_int {
    let tb = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if tb.is_null() {
        return -ENOMEM;
    }

    tpm_buf_init(tb, TPM_BUFSIZE);

    /* include migratable flag at end of sealed key */
    (*p).key[(*p).key_len as usize] = (*p).migratable;

    let ret = tpm_seal(tb, (*o).keytype, (*o).keyhandle, (*o).keyauth.as_ptr(),
        (*p).key.as_ptr(), (*p).key_len + 1, (*p).blob.as_mut_ptr(), &mut (*p).blob_len,
        (*o).blobauth.as_ptr(), (*o).pcrinfo.as_ptr(), (*o).pcrinfo_len);
    if ret < 0 {
        pr_info(c"srkseal failed (%d)\n".as_ptr(), ret);
    }
    kfree(tb as *mut c_void);
    ret
}

/*
 * Have the TPM unseal(decrypt) the symmetric key
 */
unsafe fn key_unseal(p: *mut trusted_key_payload, o: *mut trusted_key_options) -> c_int {
    let tb = kzalloc(TPM_BUFSIZE, GFP_KERNEL) as *mut tpm_buf;
    if tb.is_null() {
        return -ENOMEM;
    }

    tpm_buf_init(tb, TPM_BUFSIZE);

    let ret = tpm_unseal(tb, (*o).keyhandle, (*o).keyauth.as_ptr(), (*p).blob.as_ptr(), (*p).blob_len as c_int,
        (*o).blobauth.as_ptr(), (*p).key.as_mut_ptr(), &mut (*p).key_len);
    if ret < 0 {
        pr_info(c"srkunseal failed (%d)\n".as_ptr(), ret);
    } else {
        /* pull migratable flag out of sealed key */
        (*p).key_len -= 1;
        (*p).migratable = (*p).key[(*p).key_len as usize];
    }

    kfree(tb as *mut c_void);
    ret
}

const Opt_err: c_int = 0;
const Opt_keyhandle: c_int = 1;
const Opt_keyauth: c_int = 2;
const Opt_blobauth: c_int = 3;
const Opt_pcrinfo: c_int = 4;
const Opt_pcrlock: c_int = 5;
const Opt_migratable: c_int = 6;
const Opt_hash: c_int = 7;
const Opt_policydigest: c_int = 8;
const Opt_policyhandle: c_int = 9;

static key_tokens: match_table_t = [
    match_token { token: Opt_keyhandle, pattern: c"keyhandle=%s".as_ptr() },
    match_token { token: Opt_keyauth, pattern: c"keyauth=%s".as_ptr() },
    match_token { token: Opt_blobauth, pattern: c"blobauth=%s".as_ptr() },
    match_token { token: Opt_pcrinfo, pattern: c"pcrinfo=%s".as_ptr() },
    match_token { token: Opt_pcrlock, pattern: c"pcrlock=%s".as_ptr() },
    match_token { token: Opt_migratable, pattern: c"migratable=%s".as_ptr() },
    match_token { token: Opt_hash, pattern: c"hash=%s".as_ptr() },
    match_token { token: Opt_policydigest, pattern: c"policydigest=%s".as_ptr() },
    match_token { token: Opt_policyhandle, pattern: c"policyhandle=%s".as_ptr() },
    match_token { token: Opt_err, pattern: ptr::null() },
];

/* can have zero or more token= options */
unsafe fn getoptions(mut c: *mut c_char, pay: *mut trusted_key_payload, opt: *mut trusted_key_options) -> c_int {
    let mut args: [substring_t; MAX_OPT_ARGS] = core::mem::zeroed();
    let mut p = c;
    let mut handle: c_ulong = 0;
    let mut lock: c_ulong = 0;
    let mut token_mask: c_ulong = 0;

    let tpm2 = tpm_is_tpm2(chip);
    if tpm2 < 0 {
        return tpm2;
    }

    (*opt).hash = if tpm2 != 0 { HASH_ALGO_SHA256 } else { HASH_ALGO_SHA1 };

    if c.is_null() {
        return 0;
    }

    loop {
        p = strsep(&mut c, c" \t".as_ptr());
        if p.is_null() {
            break;
        }
        if *p == 0 || *p == b' ' as c_char || *p == b'\t' as c_char {
            continue;
        }
        let token = match_token_fn(p, key_tokens.as_ptr(), args.as_mut_ptr());
        if test_and_set_bit(token as c_int, &mut token_mask) != 0 {
            return -EINVAL;
        }

        match token {
            Opt_pcrinfo => {
                (*opt).pcrinfo_len = (strlen(args[0].from) / 2) as uint32_t;
                if (*opt).pcrinfo_len > MAX_PCRINFO_SIZE as uint32_t {
                    return -EINVAL;
                }
                if hex2bin((*opt).pcrinfo.as_mut_ptr(), args[0].from, (*opt).pcrinfo_len) < 0 {
                    return -EINVAL;
                }
            }
            Opt_keyhandle => {
                if kstrtoul(args[0].from, 16, &mut handle) < 0 {
                    return -EINVAL;
                }
                (*opt).keytype = SEAL_keytype as uint16_t;
                (*opt).keyhandle = handle as uint32_t;
            }
            Opt_keyauth => {
                if strlen(args[0].from) != 2 * SHA1_DIGEST_SIZE {
                    return -EINVAL;
                }
                if hex2bin((*opt).keyauth.as_mut_ptr(), args[0].from, SHA1_DIGEST_SIZE as uint32_t) < 0 {
                    return -EINVAL;
                }
            }
            Opt_blobauth => {
                /*
                 * TPM 1.2 authorizations are sha1 hashes passed in as
                 * hex strings.  TPM 2.0 authorizations are simple
                 * passwords (although it can take a hash as well)
                 */
                (*opt).blobauth_len = strlen(args[0].from) as uint32_t;

                if (*opt).blobauth_len == (2 * TPM_DIGEST_SIZE) as uint32_t {
                    if hex2bin((*opt).blobauth.as_mut_ptr(), args[0].from, TPM_DIGEST_SIZE as uint32_t) < 0 {
                        return -EINVAL;
                    }
                    (*opt).blobauth_len = TPM_DIGEST_SIZE as uint32_t;
                    continue;
                }

                if tpm2 != 0 && (*opt).blobauth_len as usize <= (*opt).blobauth.len() {
                    memcpy((*opt).blobauth.as_mut_ptr(), args[0].from as *const c_uchar, (*opt).blobauth_len as usize);
                    continue;
                }

                return -EINVAL;
            }
            Opt_migratable => {
                if *args[0].from == b'0' as c_char {
                    (*pay).migratable = 0;
                } else if *args[0].from != b'1' as c_char {
                    return -EINVAL;
                }
            }
            Opt_pcrlock => {
                if kstrtoul(args[0].from, 10, &mut lock) < 0 {
                    return -EINVAL;
                }
                (*opt).pcrlock = lock as c_int;
            }
            Opt_hash => {
                if test_bit(Opt_policydigest, &token_mask) != 0 {
                    return -EINVAL;
                }
                let mut i = 0;
                while i < HASH_ALGO__LAST {
                    if strcmp(args[0].from, hash_algo_name[i as usize]) == 0 {
                        (*opt).hash = i;
                        break;
                    }
                    i += 1;
                }
                if i == HASH_ALGO__LAST {
                    return -EINVAL;
                }
                if tpm2 == 0 && i != HASH_ALGO_SHA1 {
                    pr_info(c"TPM 1.x only supports SHA-1.\n".as_ptr());
                    return -EINVAL;
                }
            }
            Opt_policydigest => {
                let digest_len = hash_digest_size[(*opt).hash as usize] as c_uint;
                if tpm2 == 0 || strlen(args[0].from) != (2 * digest_len as usize) {
                    return -EINVAL;
                }
                if hex2bin((*opt).policydigest.as_mut_ptr(), args[0].from, digest_len) < 0 {
                    return -EINVAL;
                }
                (*opt).policydigest_len = digest_len;
            }
            Opt_policyhandle => {
                if tpm2 == 0 {
                    return -EINVAL;
                }
                if kstrtoul(args[0].from, 16, &mut handle) < 0 {
                    return -EINVAL;
                }
                (*opt).policyhandle = handle as uint32_t;
            }
            _ => return -EINVAL,
        }
    }
    0
}

unsafe fn trusted_options_alloc() -> *mut trusted_key_options {
    let tpm2 = tpm_is_tpm2(chip);
    if tpm2 < 0 {
        return ptr::null_mut();
    }

    let options = kzalloc(size_of::<trusted_key_options>(), GFP_KERNEL) as *mut trusted_key_options;
    if !options.is_null() {
        /* set any non-zero defaults */
        (*options).keytype = SRK_keytype as uint16_t;

        if tpm2 == 0 {
            (*options).keyhandle = SRKHANDLE;
        }
    }
    options
}

unsafe fn trusted_tpm_seal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    let tpm2 = tpm_is_tpm2(chip);
    if tpm2 < 0 {
        return tpm2;
    }

    let options = trusted_options_alloc();
    if options.is_null() {
        return -ENOMEM;
    }

    let mut ret = getoptions(datablob, p, options);
    if ret < 0 {
        kfree_sensitive(options as *mut c_void);
        return ret;
    }
    dump_options(options);

    if (*options).keyhandle == 0 && tpm2 == 0 {
        kfree_sensitive(options as *mut c_void);
        return -EINVAL;
    }

    if tpm2 != 0 {
        ret = tpm2_seal_trusted(chip, p, options);
    } else {
        ret = key_seal(p, options);
    }
    if ret < 0 {
        pr_info(c"key_seal failed (%d)\n".as_ptr(), ret);
        kfree_sensitive(options as *mut c_void);
        return ret;
    }

    if (*options).pcrlock != 0 {
        ret = pcrlock((*options).pcrlock);
        if ret < 0 {
            pr_info(c"pcrlock failed (%d)\n".as_ptr(), ret);
            kfree_sensitive(options as *mut c_void);
            return ret;
        }
    }
    kfree_sensitive(options as *mut c_void);
    ret
}

unsafe fn trusted_tpm_unseal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    let tpm2 = tpm_is_tpm2(chip);
    if tpm2 < 0 {
        return tpm2;
    }

    let options = trusted_options_alloc();
    if options.is_null() {
        return -ENOMEM;
    }

    let mut ret = getoptions(datablob, p, options);
    if ret < 0 {
        kfree_sensitive(options as *mut c_void);
        return ret;
    }
    dump_options(options);

    if (*options).keyhandle == 0 && tpm2 == 0 {
        kfree_sensitive(options as *mut c_void);
        return -EINVAL;
    }

    if tpm2 != 0 {
        ret = tpm2_unseal_trusted(chip, p, options);
    } else {
        ret = key_unseal(p, options);
    }
    if ret < 0 {
        pr_info(c"key_unseal failed (%d)\n".as_ptr(), ret);
    }

    if (*options).pcrlock != 0 {
        ret = pcrlock((*options).pcrlock);
        if ret < 0 {
            pr_info(c"pcrlock failed (%d)\n".as_ptr(), ret);
            kfree_sensitive(options as *mut c_void);
            return ret;
        }
    }
    kfree_sensitive(options as *mut c_void);
    ret
}

unsafe fn trusted_tpm_get_random(key: *mut c_uchar, key_len: size_t) -> c_int {
    tpm_get_random(chip, key, key_len)
}

unsafe fn init_digests() -> c_int {
    digests = kzalloc(size_of::<tpm_digest>() * (*chip).nr_allocated_banks as usize, GFP_KERNEL) as *mut tpm_digest;
    if digests.is_null() {
        return -ENOMEM;
    }

    let mut i = 0;
    while i < (*chip).nr_allocated_banks {
        (*digests.add(i as usize)).alg_id = (*(*chip).allocated_banks.add(i as usize)).alg_id;
        i += 1;
    }

    0
}

unsafe fn trusted_tpm_init() -> c_int {
    chip = tpm_default_chip();
    if chip.is_null() {
        return -ENODEV;
    }

    let mut ret = init_digests();
    if ret < 0 {
        put_device(&mut (*chip).dev);
        return ret;
    }
    ret = register_key_type(&mut key_type_trusted);
    if ret < 0 {
        kfree(digests as *mut c_void);
        put_device(&mut (*chip).dev);
        return ret;
    }
    0
}

unsafe fn trusted_tpm_exit() {
    if !chip.is_null() {
        unregister_key_type(&mut key_type_trusted);
        put_device(&mut (*chip).dev);
        kfree(digests as *mut c_void);
    }
}

#[no_mangle]
static mut trusted_key_tpm_ops: trusted_key_ops = trusted_key_ops {
    migratable: 1, /* migratable by default */
    init: Some(trusted_tpm_init),
    seal: Some(trusted_tpm_seal),
    unseal: Some(trusted_tpm_unseal),
    get_random: Some(trusted_tpm_get_random),
    exit: Some(trusted_tpm_exit),
};

#[repr(C)]
struct hmac_sha1_ctx {
    _private: [u8; 0],
}

#[repr(C)]
struct sha1_ctx {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct tpm_bank_info {
    alg_id: uint16_t,
}

#[repr(C)]
struct tpm_chip {
    dev: device,
    nr_allocated_banks: c_int,
    allocated_banks: *mut tpm_bank_info,
}

#[repr(C)]
struct tpm_digest {
    alg_id: uint16_t,
    digest: [c_uchar; TPM_DIGEST_SIZE],
}

#[repr(C)]
struct tpm_buf {
    data: *mut c_uchar,
}

#[repr(C)]
struct trusted_key_payload {
    key: [c_uchar; 128],
    key_len: uint32_t,
    blob: [c_uchar; 512],
    blob_len: uint32_t,
    migratable: c_uchar,
}

#[repr(C)]
struct trusted_key_options {
    keytype: uint16_t,
    keyhandle: uint32_t,
    keyauth: [c_uchar; SHA1_DIGEST_SIZE],
    blobauth: [c_uchar; 128],
    blobauth_len: uint32_t,
    pcrinfo: [c_uchar; MAX_PCRINFO_SIZE],
    pcrinfo_len: uint32_t,
    pcrlock: c_int,
    hash: c_int,
    policydigest: [c_uchar; 64],
    policydigest_len: c_uint,
    policyhandle: uint32_t,
}

#[repr(C)]
struct substring_t {
    from: *mut c_char,
    to: *mut c_char,
}

#[repr(C)]
struct match_token {
    token: c_int,
    pattern: *const c_char,
}

#[repr(C)]
struct key_type {
    _private: [u8; 0],
}

#[repr(C)]
struct trusted_key_ops {
    migratable: c_int,
    init: Option<unsafe fn() -> c_int>,
    seal: Option<unsafe fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    unseal: Option<unsafe fn(*mut trusted_key_payload, *mut c_char) -> c_int>,
    get_random: Option<unsafe fn(*mut c_uchar, size_t) -> c_int>,
    exit: Option<unsafe fn()>,
}

unsafe extern "C" {
    static mut key_type_trusted: key_type;
    static hash_algo_name: [*const c_char; HASH_ALGO__LAST as usize];
    static hash_digest_size: [c_uint; HASH_ALGO__LAST as usize];

    fn ntohl(x: uint32_t) -> uint32_t;
    fn ntohs(x: uint16_t) -> uint16_t;
    fn htonl(x: uint32_t) -> uint32_t;
    fn memcpy(dst: *mut c_uchar, src: *const c_uchar, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn hex2bin(dst: *mut c_uchar, src: *const c_char, count: uint32_t) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;

    fn hmac_sha1_init_usingrawkey(ctx: *mut hmac_sha1_ctx, key: *const c_uchar, keylen: c_uint);
    fn hmac_sha1_update(ctx: *mut hmac_sha1_ctx, data: *const c_uchar, dlen: c_uint);
    fn hmac_sha1_final(ctx: *mut hmac_sha1_ctx, digest: *mut c_uchar);
    fn sha1_init(ctx: *mut sha1_ctx);
    fn sha1_update(ctx: *mut sha1_ctx, data: *const u8, len: c_uint);
    fn sha1_final(ctx: *mut sha1_ctx, out: *mut c_uchar);
    fn sha1(data: *const c_uchar, len: c_uint, out: *mut c_uchar);
    fn crypto_memneq(a: *const c_uchar, b: *const c_uchar, size: size_t) -> c_int;

    fn tpm_try_get_ops(chip: *mut tpm_chip) -> c_int;
    fn tpm_put_ops(chip: *mut tpm_chip);
    fn tpm_transmit_cmd(chip: *mut tpm_chip, buf: *mut tpm_buf, min_rsp_body_length: c_int, desc: *const c_char) -> c_int;
    fn tpm_pcr_extend(chip: *mut tpm_chip, pcr_idx: c_int, digests: *mut tpm_digest) -> c_int;
    fn tpm_get_random(chip: *mut tpm_chip, out: *mut c_uchar, max: size_t) -> c_int;
    fn tpm_buf_reset(buf: *mut tpm_buf, tag: uint16_t, ordinal: uint32_t);
    fn tpm_buf_append_u16(buf: *mut tpm_buf, value: uint16_t);
    fn tpm_buf_append_u32(buf: *mut tpm_buf, value: uint32_t);
    fn tpm_buf_append_u8(buf: *mut tpm_buf, value: c_uchar);
    fn tpm_buf_append(buf: *mut tpm_buf, new_data: *mut c_uchar, new_length: c_uint);
    fn tpm_buf_init(buf: *mut tpm_buf, size: size_t);
    fn tpm_default_chip() -> *mut tpm_chip;
    fn tpm_is_tpm2(chip: *mut tpm_chip) -> c_int;
    fn tpm2_seal_trusted(chip: *mut tpm_chip, p: *mut trusted_key_payload, o: *mut trusted_key_options) -> c_int;
    fn tpm2_unseal_trusted(chip: *mut tpm_chip, p: *mut trusted_key_payload, o: *mut trusted_key_options) -> c_int;

    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kfree_sensitive(ptr: *mut c_void);
    fn capable(cap: c_int) -> c_int;
    fn pr_info(fmt: *const c_char, ...);
    fn put_device(dev: *mut device);
    fn register_key_type(ktype: *mut key_type) -> c_int;
    fn unregister_key_type(ktype: *mut key_type);
    #[link_name = "match_token"]
    fn match_token_fn(s: *mut c_char, table: *const match_token, args: *mut substring_t) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
