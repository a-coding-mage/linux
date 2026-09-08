// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005-2010 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * File: evm_crypto.c
 *	 Using root's kernel master key (kmk), calculate the HMAC
 */

// Linux kernel and crypto subsystem types and functions
// These are external dependencies from other modules/headers
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::sync::Mutex;

// External types from Linux kernel
#[repr(C)]
pub struct CryptoShash {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ShashDesc {
    pub tfm: *mut CryptoShash,
    // Variable-length field for descriptor data
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Dentry {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    pub i_opflags: c_ulong,
    pub i_ino: u64,
    pub i_generation: u32,
    pub i_uid: u32,
    pub i_gid: u32,
    pub i_mode: u32,
    pub i_sb: *mut Superblock,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Superblock {
    pub s_uuid: [u8; 16],
    pub s_user_ns: *mut UserNamespace,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct UserNamespace {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct XattrList {
    pub name: *const c_char,
    pub enabled: bool,
    // list node omitted - managed by kernel
}

#[repr(C)]
pub struct EvmDigestHdr {
    pub algo: u8,
    pub xattr: EvmXattrUnion,
    pub length: c_int,
}

#[repr(C)]
pub union EvmXattrUnion {
    pub sha1: EvmXattrSha1,
    pub data: [u8; 48],
}

#[repr(C)]
pub struct EvmXattrSha1 {
    pub xattr_type: c_char,
    pub digest: [u8; 20],
}

#[repr(C)]
pub struct EvmDigest {
    pub hdr: EvmDigestHdr,
    pub digest: [u8; 64],
}

#[repr(C)]
pub struct EvmImaXattrData {
    pub xattr_type: c_char,
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct EvmIintCache {
    pub flags: u32,
    pub metadata_inode: [u8; 32],
}

#[repr(C)]
pub struct Xattr {
    pub name: *const c_char,
    pub value: *const u8,
    pub value_len: usize,
}

// Constants
const EVMKEY: &str = "evm-key";
const MAX_KEY_SIZE: usize = 128;
const EVM_SET_KEY_BUSY: usize = 0;

// Sentinel value that indicates an array of structs ends
const XATTR_SECURITY_PREFIX_LEN: usize = 8; // "security."

// Constants from Linux
const EVM_XATTR_HMAC: c_char = 1;
const EVM_XATTR_PORTABLE_DIGSIG: c_char = 2;
const EVM_INIT_HMAC: u32 = 1;
const EVM_ATTR_FSUUID: u32 = 1;
const EVM_IMMUTABLE_DIGSIG: u32 = 1;
const HASH_ALGO_SHA1: u8 = 1;
const HASH_ALGO__LAST: u8 = 12;
const UUID_SIZE: usize = 16;
const SHA1_DIGEST_SIZE: usize = 20;
const IOP_XATTR: c_ulong = 0x0010;

const ENOKEY: c_int = -126;
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;
const EOPNOTSUPP: c_int = -95;
const ENODATA: c_int = -61;
const EPERM: c_int = -1;
const EBUSY: c_int = -16;
const ENOENT: c_int = -2;

// Static globals
static mut EVMKEY_BUF: [u8; MAX_KEY_SIZE] = [0; MAX_KEY_SIZE];
static EVMKEY_LEN: c_int = MAX_KEY_SIZE as c_int;

static mut HMAC_TFM: *mut CryptoShash = std::ptr::null_mut();
static mut EVM_TFM: [*mut CryptoShash; 12] = [std::ptr::null_mut(); 12];

static MUTEX: Mutex<()> = Mutex::new(());

static mut EVM_SET_KEY_FLAGS: c_ulong = 0;

static EVM_HMAC_STR: &[u8] = b"hmac(sha1)\0";

// External globals and functions from Linux kernel
extern "C" {
    static mut evm_initialized: u32;
    static mut evm_hmac_attrs: u32;
    static evm_config_xattrnames: XattrList;
    static init_user_ns: UserNamespace;
    static nop_mnt_idmap: c_void;
    static hash_algo_name: [*const c_char; 12];

    fn test_and_set_bit(bit: usize, word: *mut c_ulong) -> c_int;
    fn clear_bit(bit: usize, word: *mut c_ulong);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn bin2hex(dst: *mut c_char, src: *const c_void, count: usize) -> *mut c_char;

    fn crypto_alloc_shash(
        alg: *const c_char,
        type_: c_int,
        mask: c_int,
    ) -> *mut CryptoShash;
    fn crypto_free_shash(tfm: *mut CryptoShash);
    fn crypto_shash_setkey(
        desc: *mut CryptoShash,
        key: *const u8,
        keylen: usize,
    ) -> c_int;
    fn crypto_shash_init(desc: *mut ShashDesc) -> c_int;
    fn crypto_shash_update(
        desc: *mut ShashDesc,
        data: *const u8,
        len: usize,
    ) -> c_int;
    fn crypto_shash_final(desc: *mut ShashDesc, out: *mut u8) -> c_int;
    fn crypto_shash_descsize(tfm: *mut CryptoShash) -> usize;
    fn crypto_shash_digestsize(tfm: *mut CryptoShash) -> c_int;

    fn d_inode(dentry: *const Dentry) -> *mut Inode;
    fn d_real(dentry: *const Dentry, flags: c_int) -> *const Dentry;
    fn d_backing_inode(dentry: *const Dentry) -> *mut Inode;

    fn vfs_getxattr_alloc(
        idmap: *const c_void,
        dentry: *const Dentry,
        name: *const c_char,
        xattr_value: *mut *mut c_char,
        xattr_size: usize,
        flags: c_int,
    ) -> c_int;
    fn vfs_getxattr(
        idmap: *const c_void,
        dentry: *const Dentry,
        name: *const c_char,
        value: *mut c_void,
        size: usize,
    ) -> c_int;
    fn __vfs_setxattr_noperm(
        idmap: *const c_void,
        dentry: *const Dentry,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    fn __vfs_removexattr(
        idmap: *const c_void,
        dentry: *const Dentry,
        name: *const c_char,
    ) -> c_int;

    fn from_kuid(ns: *const UserNamespace, uid: u32) -> u32;
    fn from_kgid(ns: *const UserNamespace, gid: u32) -> u32;

    fn evm_iint_inode(inode: *mut Inode) -> *mut EvmIintCache;
    fn inode_query_iversion(inode: *mut Inode) -> u64;
    fn integrity_inode_attrs_store(
        data: *mut u8,
        version: u64,
        inode: *mut Inode,
    );
    fn IS_I_VERSION(inode: *mut Inode) -> bool;

    fn request_key(
        type_: *const c_void,
        description: *const c_char,
        callout_info: *const c_void,
    ) -> *mut c_void;
    fn key_put(key: *mut c_void);
    fn down_read(sem: *mut c_void);
    fn up_read(sem: *mut c_void);

    static key_type_encrypted: c_void;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_err_once(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn ERR_PTR(error: c_int) -> *const c_void;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_CAST(ptr: *const c_void) -> *mut c_void;
}

// Encryption key payload structure from kernel
#[repr(C)]
pub struct EncryptedKeyPayload {
    pub decrypted_data: *mut u8,
    pub decrypted_datalen: usize,
}

// Key structure with payload (simplified)
#[repr(C)]
pub struct Key {
    pub sem: c_void,
    pub payload: KeyPayload,
}

#[repr(C)]
pub struct KeyPayload {
    pub data: [*mut c_void; 4],
}

const GFP_KERNEL: c_int = 0xd0;
const GFP_NOFS: c_int = 0x50;
const CRYPTO_NOLOAD: c_int = 0x10000000;

const XATTR_NAME_IMA: &[u8] = b"security.ima\0";
const XATTR_NAME_EVM: &[u8] = b"security.evm\0";

// Export symbol equivalent
pub const EVM_EXPORT_SYMBOL_GPL: () = ();

/// evm_set_key() - set EVM HMAC key from the kernel
/// @key: pointer to a buffer with the key data
/// @keylen: length of the key data
///
/// This function allows setting the EVM HMAC key from the kernel
/// without using the "encrypted" key subsystem keys. It can be used
/// by the crypto HW kernel module which has its own way of managing
/// keys.
///
/// key length should be between 32 and 128 bytes long
pub unsafe extern "C" fn evm_set_key(key: *mut c_void, keylen: usize) -> c_int {
    let mut rc: c_int;

    rc = EBUSY;
    if test_and_set_bit(EVM_SET_KEY_BUSY, &mut EVM_SET_KEY_FLAGS) != 0 {
        pr_err(b"EVM: key initialization failed\n".as_ptr() as *const c_char);
        return rc;
    }
    rc = EINVAL;
    if keylen > MAX_KEY_SIZE {
        clear_bit(EVM_SET_KEY_BUSY, &mut EVM_SET_KEY_FLAGS);
        pr_err(b"EVM: key initialization failed\n".as_ptr() as *const c_char);
        return rc;
    }
    memcpy(EVMKEY_BUF.as_mut_ptr() as *mut c_void, key, keylen);
    evm_initialized |= EVM_INIT_HMAC;
    pr_info(b"EVM: key initialized\n".as_ptr() as *const c_char);
    0
}

unsafe fn init_desc(
    xattr_type: c_char,
    hash_algo: u8,
) -> *mut ShashDesc {
    let mut rc: c_long;
    let mut algo: *const c_char;
    let mut tfm: *mut *mut CryptoShash;
    let mut tmp_tfm: *mut CryptoShash;
    let mut desc: *mut ShashDesc;

    if xattr_type == EVM_XATTR_HMAC {
        if (evm_initialized & EVM_INIT_HMAC) == 0 {
            pr_err_once(b"EVM: HMAC key is not set\n".as_ptr() as *const c_char);
            return ERR_PTR(ENOKEY) as *mut ShashDesc;
        }
        tfm = &mut HMAC_TFM;
        algo = EVM_HMAC_STR.as_ptr() as *const c_char;
    } else {
        if (hash_algo as c_int) >= HASH_ALGO__LAST as c_int {
            return ERR_PTR(EINVAL) as *mut ShashDesc;
        }
        tfm = &mut EVM_TFM[hash_algo as usize];
        algo = *hash_algo_name.as_ptr().add(hash_algo as usize);
    }

    if !(*tfm).is_null() {
        // goto alloc
    } else {
        let _guard = MUTEX.lock().unwrap();
        if (*tfm).is_null() {
            tmp_tfm = crypto_alloc_shash(algo, 0, CRYPTO_NOLOAD);
            if IS_ERR(tmp_tfm as *const c_void) {
                pr_err(
                    b"EVM: Can not allocate %s (reason: %ld)\n".as_ptr() as *const c_char,
                    algo,
                    PTR_ERR(tmp_tfm as *const c_void) as c_long,
                );
                return ERR_CAST(tmp_tfm as *const c_void) as *mut ShashDesc;
            }
            if xattr_type == EVM_XATTR_HMAC {
                rc = crypto_shash_setkey(tmp_tfm, EVMKEY_BUF.as_ptr(), EVMKEY_LEN as usize) as c_long;
                if rc != 0 {
                    crypto_free_shash(tmp_tfm);
                    return ERR_PTR(rc as c_int) as *mut ShashDesc;
                }
            }
            *tfm = tmp_tfm;
        }
    }

    desc = kmalloc(
        std::mem::size_of::<ShashDesc>() + crypto_shash_descsize(*tfm),
        GFP_KERNEL,
    ) as *mut ShashDesc;
    if desc.is_null() {
        return ERR_PTR(ENOMEM) as *mut ShashDesc;
    }

    (*desc).tfm = *tfm;

    rc = crypto_shash_init(desc) as c_long;
    if rc != 0 {
        kfree(desc as *mut c_void);
        return ERR_PTR(rc as c_int) as *mut ShashDesc;
    }
    desc
}

/// Protect against 'cutting & pasting' security.evm xattr, include inode
/// specific info.
///
/// (Additional directory/file metadata needs to be added for more complete
/// protection.)
unsafe fn hmac_add_misc(
    desc: *mut ShashDesc,
    inode: *mut Inode,
    xattr_type: c_char,
    digest: *mut u8,
) {
    #[repr(C)]
    struct HMiscData {
        ino: c_ulong,
        generation: u32,
        uid: u32,
        gid: u32,
        mode: u32,
    }

    let mut hmac_misc: HMiscData = HMiscData {
        ino: 0,
        generation: 0,
        uid: 0,
        gid: 0,
        mode: 0,
    };

    // Don't include the inode or generation number in portable signatures
    if xattr_type != EVM_XATTR_PORTABLE_DIGSIG {
        hmac_misc.ino = (*inode).i_ino as c_ulong;
        hmac_misc.generation = (*inode).i_generation;
    }

    // The hmac uid and gid must be encoded in the initial user namespace
    hmac_misc.uid = from_kuid(&init_user_ns, (*inode).i_uid);
    hmac_misc.gid = from_kgid(&init_user_ns, (*inode).i_gid);
    hmac_misc.mode = (*inode).i_mode;

    crypto_shash_update(
        desc,
        &hmac_misc as *const HMiscData as *const u8,
        std::mem::size_of::<HMiscData>(),
    );

    if (evm_hmac_attrs & EVM_ATTR_FSUUID) != 0
        && xattr_type != EVM_XATTR_PORTABLE_DIGSIG
    {
        crypto_shash_update(
            desc,
            &(*(*inode).i_sb).s_uuid as *const [u8; 16] as *const u8,
            UUID_SIZE,
        );
    }
    crypto_shash_final(desc, digest);

    pr_debug(
        b"EVM: hmac_misc: (%zu) [%*phN]\n".as_ptr() as *const c_char,
        std::mem::size_of::<HMiscData>(),
        std::mem::size_of::<HMiscData>() as c_int,
        &hmac_misc as *const HMiscData as *const c_void,
    );
}

/// Dump large security xattr values as a continuous ascii hexadecimal string.
/// (pr_debug is limited to 64 bytes.)
unsafe fn dump_security_xattr_l(
    prefix: *const c_char,
    src: *const c_void,
    count: usize,
) {
    #[cfg(any(feature = "debug", feature = "dynamic_debug"))]
    {
        let mut asciihex: *mut c_char;
        let mut p: *mut c_char;

        p = kmalloc(count * 2 + 1, GFP_KERNEL) as *mut c_char;
        asciihex = p;
        if asciihex.is_null() {
            return;
        }

        p = bin2hex(p, src, count);
        *p = 0;
        pr_debug(
            b"EVM: %s: (%zu) %.*s\n".as_ptr() as *const c_char,
            prefix,
            count,
            (count * 2) as c_int,
            asciihex,
        );
        kfree(asciihex as *mut c_void);
    }
}

unsafe fn dump_security_xattr(
    name: *const c_char,
    value: *const c_char,
    value_len: usize,
) {
    if value_len < 64 {
        pr_debug(
            b"EVM: %s: (%zu) [%*phN]\n".as_ptr() as *const c_char,
            name,
            value_len,
            value_len as c_int,
            value as *const c_void,
        );
    } else {
        dump_security_xattr_l(name, value as *const c_void, value_len);
    }
}

/// Calculate the HMAC value across the set of protected security xattrs.
///
/// Instead of retrieving the requested xattr, for performance, calculate
/// the hmac using the requested xattr value. Don't alloc/free memory for
/// each xattr, but attempt to re-use the previously allocated memory.
unsafe fn evm_calc_hmac_or_hash(
    dentry: *const Dentry,
    req_xattr_name: *const c_char,
    req_xattr_value: *const c_char,
    req_xattr_value_len: usize,
    xattr_type: u8,
    data: *mut EvmDigest,
    iint: *mut EvmIintCache,
) -> c_int {
    let mut inode: *mut Inode = d_inode(d_real(dentry, 1)) as *mut Inode;
    let mut xattr: *const XattrList;
    let mut desc: *mut ShashDesc;
    let mut xattr_size: usize = 0;
    let mut xattr_value: *mut c_char = std::ptr::null_mut();
    let mut error: c_int;
    let mut size: c_int;
    let mut user_space_size: c_int;
    let mut ima_present: bool = false;
    let mut i_version: u64 = 0;

    if ((*inode).i_opflags & IOP_XATTR) == 0
        || (*(*inode).i_sb).s_user_ns != &init_user_ns as *const UserNamespace as *mut UserNamespace
    {
        return EOPNOTSUPP;
    }

    desc = init_desc(xattr_type as c_char, (*data).hdr.algo);
    if IS_ERR(desc as *const c_void) {
        return PTR_ERR(desc as *const c_void);
    }

    (*data).hdr.length = crypto_shash_digestsize((*desc).tfm);

    error = ENODATA;

    xattr = &evm_config_xattrnames;
    while !(*xattr).name.is_null() {
        let is_ima = strcmp((*xattr).name, XATTR_NAME_IMA.as_ptr() as *const c_char) == 0;
        if xattr_type as c_char != EVM_XATTR_PORTABLE_DIGSIG && !(*xattr).enabled {
            xattr = xattr.add(1); continue;
        }
        if !req_xattr_name.is_null() && !req_xattr_value.is_null()
            && strcmp((*xattr).name, req_xattr_name) == 0 {
            error = 0;
            crypto_shash_update(desc, req_xattr_value as *const u8, req_xattr_value_len);
            if is_ima { ima_present = true; }
            xattr = xattr.add(1); continue;
        }
        size = vfs_getxattr_alloc(&nop_mnt_idmap as *const c_void, dentry,
            (*xattr).name, &mut xattr_value, xattr_size, GFP_NOFS);
        if size == -ENOMEM { error = -ENOMEM; break; }
        if size < 0 { xattr = xattr.add(1); continue; }
        error = 0;
        xattr_size = size as usize;
        crypto_shash_update(desc, xattr_value as *const u8, xattr_size);
        if is_ima { ima_present = true; }
        xattr = xattr.add(1);
    }

    hmac_add_misc(desc, inode, xattr_type as c_char, (*data).digest.as_mut_ptr());

    if inode != d_backing_inode(dentry) && !iint.is_null() {
        if IS_I_VERSION(inode) {
            i_version = inode_query_iversion(inode);
        }
        integrity_inode_attrs_store(
            (*iint).metadata_inode.as_mut_ptr(),
            i_version,
            inode,
        );
    }

    // Portable EVM signatures must include an IMA hash
    if xattr_type as c_char == EVM_XATTR_PORTABLE_DIGSIG && !ima_present {
        error = EPERM;
    }

    kfree(xattr_value as *mut c_void);
    kfree(desc as *mut c_void);
    error
}

pub unsafe extern "C" fn evm_calc_hmac(
    dentry: *const Dentry,
    req_xattr_name: *const c_char,
    req_xattr_value: *const c_char,
    req_xattr_value_len: usize,
    data: *mut EvmDigest,
    iint: *mut EvmIintCache,
) -> c_int {
    evm_calc_hmac_or_hash(
        dentry,
        req_xattr_name,
        req_xattr_value,
        req_xattr_value_len,
        EVM_XATTR_HMAC as u8,
        data,
        iint,
    )
}

pub unsafe extern "C" fn evm_calc_hash(
    dentry: *const Dentry,
    req_xattr_name: *const c_char,
    req_xattr_value: *const c_char,
    req_xattr_value_len: usize,
    xattr_type: c_char,
    data: *mut EvmDigest,
    iint: *mut EvmIintCache,
) -> c_int {
    evm_calc_hmac_or_hash(
        dentry,
        req_xattr_name,
        req_xattr_value,
        req_xattr_value_len,
        xattr_type as u8,
        data,
        iint,
    )
}

unsafe fn evm_is_immutable(
    dentry: *const Dentry,
    inode: *mut Inode,
) -> c_int {
    let mut xattr_data: *const EvmImaXattrData = std::ptr::null();
    let mut iint: *mut EvmIintCache;
    let mut rc: c_int = 0;

    iint = evm_iint_inode(inode);
    if !iint.is_null() && ((*iint).flags & EVM_IMMUTABLE_DIGSIG) != 0 {
        return 1;
    }

    // Do this the hard way
    rc = vfs_getxattr_alloc(
        &nop_mnt_idmap as *const c_void,
        dentry,
        XATTR_NAME_EVM.as_ptr() as *const c_char,
        &mut (xattr_data as *mut c_char),
        0,
        GFP_NOFS,
    );
    if rc <= 0 {
        if rc == -ENODATA {
            rc = 0;
        }
    } else if (*xattr_data).xattr_type == EVM_XATTR_PORTABLE_DIGSIG {
        rc = 1;
    } else {
        rc = 0;
    }

    kfree(xattr_data as *mut c_void);
    rc
}

/// Calculate the hmac and update security.evm xattr
///
/// Expects to be called with i_mutex locked.
pub unsafe extern "C" fn evm_update_evmxattr(
    dentry: *const Dentry,
    xattr_name: *const c_char,
    xattr_value: *const c_char,
    xattr_value_len: usize,
) -> c_int {
    let mut inode: *mut Inode = d_backing_inode(dentry);
    let mut iint: *mut EvmIintCache = evm_iint_inode(inode);
    let mut data: EvmDigest = std::mem::zeroed();
    let mut rc: c_int = 0;

    // Don't permit any transformation of the EVM xattr if the signature
    // is of an immutable type
    rc = evm_is_immutable(dentry, inode);
    if rc < 0 {
        return rc;
    }
    if rc != 0 {
        return EPERM;
    }

    data.hdr.algo = HASH_ALGO_SHA1;
    rc = evm_calc_hmac(dentry, xattr_name, xattr_value, xattr_value_len, &mut data, iint);
    if rc == 0 {
        data.hdr.xattr.sha1.xattr_type = EVM_XATTR_HMAC;
        rc = __vfs_setxattr_noperm(
            &nop_mnt_idmap as *const c_void,
            dentry,
            XATTR_NAME_EVM.as_ptr() as *const c_char,
            &data.hdr.xattr.data[1] as *const u8 as *const c_void,
            SHA1_DIGEST_SIZE + 1,
            0,
        );
    } else if rc == -ENODATA && ((*inode).i_opflags & IOP_XATTR) != 0 {
        rc = __vfs_removexattr(
            &nop_mnt_idmap as *const c_void,
            dentry,
            XATTR_NAME_EVM.as_ptr() as *const c_char,
        );
    }
    rc
}

pub unsafe extern "C" fn evm_init_hmac(
    inode: *mut Inode,
    xattrs: *const Xattr,
    hmac_val: *mut c_char,
) -> c_int {
    let mut desc: *mut ShashDesc;
    let mut xattr: *const Xattr;
    let mut xattr_entry: *const XattrList;

    desc = init_desc(EVM_XATTR_HMAC, HASH_ALGO_SHA1);
    if IS_ERR(desc as *const c_void) {
        pr_info(b"EVM: init_desc failed\n".as_ptr() as *const c_char);
        return PTR_ERR(desc as *const c_void);
    }

    xattr_entry = &evm_config_xattrnames;
    while !(*xattr_entry).name.is_null() {
        xattr = xattrs;
        while !(*xattr).name.is_null() {
            if strcmp((*xattr_entry).name.add(XATTR_SECURITY_PREFIX_LEN), (*xattr).name) == 0 {
                crypto_shash_update(desc, (*xattr).value, (*xattr).value_len);
            }
            xattr = xattr.add(1);
        }
        xattr_entry = xattr_entry.add(1);
    }

    hmac_add_misc(desc, inode, EVM_XATTR_HMAC, hmac_val as *mut u8);
    kfree(desc as *mut c_void);
    0
}

/// Get the key from the TPM for the SHA1-HMAC
pub unsafe extern "C" fn evm_init_key() -> c_int {
    let mut evm_key: *mut Key;
    let mut ekp: *mut EncryptedKeyPayload;
    let mut rc: c_int;

    evm_key = request_key(
        &key_type_encrypted as *const c_void,
        EVMKEY.as_bytes().as_ptr() as *const c_char,
        std::ptr::null(),
    ) as *mut Key;
    if IS_ERR(evm_key as *const c_void) {
        return ENOENT;
    }

    down_read(&mut (*evm_key).sem as *mut c_void);
    ekp = (*evm_key).payload.data[0] as *mut EncryptedKeyPayload;

    rc = evm_set_key((*ekp).decrypted_data as *mut c_void, (*ekp).decrypted_datalen);

    // burn the original key contents
    memset((*ekp).decrypted_data as *mut c_void, 0, (*ekp).decrypted_datalen);
    up_read(&mut (*evm_key).sem as *mut c_void);
    key_put(evm_key as *mut c_void);
    rc
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
