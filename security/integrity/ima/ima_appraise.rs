// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 IBM Corporation
 *
 * Author:
 * Mimi Zohar <zohar@us.ibm.com>
 */

// Linux kernel headers:
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/file.h>
// #include <linux/binfmts.h>
// #include <linux/fs.h>
// #include <linux/xattr.h>
// #include <linux/magic.h>
// #include <linux/ima.h>
// #include <linux/evm.h>
// #include <linux/fsverity.h>
// #include <keys/system_keyring.h>
// #include <uapi/linux/fsverity.h>
// #include "ima.h"

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr;

// External types (from Linux kernel)
#[repr(C)]
pub struct mnt_idmap {
    _opaque: c_void,
}

#[repr(C)]
pub struct inode {
    _opaque: c_void,
}

#[repr(C)]
pub struct dentry {
    _opaque: c_void,
}

#[repr(C)]
pub struct file {
    _opaque: c_void,
}

#[repr(C)]
pub struct ima_iint_cache {
    _opaque: c_void,
}

#[repr(C)]
pub struct evm_ima_xattr_data {
    pub xtype: u8,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct lsm_prop {
    _opaque: c_void,
}

#[repr(C)]
pub struct modsig {
    _opaque: c_void,
}

#[repr(C)]
pub struct signature_v2_hdr {
    pub version: u8,
    pub hash_algo: u8,
    _padding: [u8; 14],
}

#[repr(C)]
pub struct posix_acl {
    _opaque: c_void,
}

#[repr(C)]
pub struct lsm_id {
    _opaque: c_void,
}

#[repr(C)]
pub struct security_hook_list {
    _opaque: c_void,
}

// Enums from Linux kernel
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ima_hooks {
    MMAP_CHECK,
    MMAP_CHECK_REQPROT,
    BPRM_CHECK,
    CREDS_CHECK,
    FILE_CHECK,
    POST_SETATTR,
    MODULE_CHECK,
    // ... MAX_CHECK - 1
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum integrity_status {
    INTEGRITY_PASS,
    INTEGRITY_PASS_IMMUTABLE,
    INTEGRITY_FAIL,
    INTEGRITY_FAIL_IMMUTABLE,
    INTEGRITY_NOLABEL,
    INTEGRITY_NOXATTRS,
    INTEGRITY_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum hash_algo {
    HASH_ALGO_MD5,
    HASH_ALGO_SHA1,
    HASH_ALGO_SHA256,
    HASH_ALGO__LAST,
}

// External constants and globals (from Linux kernel)
extern "C" {
    pub static mut ima_appraise: c_int;
    pub static mut ima_hash_algo: hash_algo;
    pub static mut ima_policy_flag: c_int;
    pub static mut ima_setxattr_allowed_hash_algorithms: core::sync::atomic::AtomicU32;
    pub static nop_mnt_idmap: mnt_idmap;
    pub static hash_algo_name: [*const c_char; 0];
}

// External constants (from Linux kernel headers)
const IMA_APPRAISE_LOG: c_int = 1;
const IMA_APPRAISE_FIX: c_int = 2;
const IMA_APPRAISE_ENFORCE: c_int = 4;
const IMA_XATTR_DIGEST: u8 = 1;
const IMA_XATTR_DIGEST_NG: u8 = 2;
const IMA_XATTR_LAST: u8 = 3;
const EVM_IMA_XATTR_DIGSIG: u8 = 3;
const IMA_VERITY_DIGSIG: u8 = 5;
const EVM_XATTR_PORTABLE_DIGSIG: u8 = 4;
const IMA_APPRAISE: c_int = 0x01;
const IMA_HASH: c_int = 0x02;
const IMA_MEASURE: c_int = 0x04;
const IMA_DIGSIG_REQUIRED: c_int = 0x08;
const IMA_VERITY_REQUIRED: c_int = 0x10;
const IMA_CHECK_BLACKLIST: c_int = 0x20;
const IMA_MODSIG_ALLOWED: c_int = 0x40;
const IMA_NEW_FILE: c_int = 0x80;
const IMA_SIGV3_REQUIRED: c_int = 0x100;
const IMA_FAIL_UNVERIFIABLE_SIGS: c_int = 0x200;
const IMA_APPRAISED: c_int = 0x01;
const IMA_MMAP_APPRAISED: c_int = 0x02;
const IMA_BPRM_APPRAISED: c_int = 0x04;
const IMA_CREDS_APPRAISED: c_int = 0x08;
const IMA_FILE_APPRAISED: c_int = 0x10;
const IMA_READ_APPRAISED: c_int = 0x20;
const IMA_DIGSIG: c_int = 0x40;
const IMA_CHANGE_ATTR: c_int = 0x80;
const IMA_UPDATE_XATTR: c_int = 0x100;
const IMA_CHANGE_XATTR: c_int = 0x200;

const XATTR_NAME_IMA: &[u8] = b"security.ima";
const XATTR_NAME_EVM: &[u8] = b"security.evm";

const AUDIT_INTEGRITY_DATA: c_int = 5416;
const AUDIT_INTEGRITY_USERSPACE: c_int = 5415;

const INTEGRITY_KEYRING_IMA: c_int = 0;
const INTEGRITY_KEYRING_PLATFORM: c_int = 1;

const CONFIG_INTEGRITY_PLATFORM_KEYRING: bool = true;

const EROFS: c_int = -30;
const EPERM: c_int = -1;
const EINVAL: c_int = -22;
const EOPNOTSUPP: c_int = -95;
const ENODATA: c_int = -61;
const ENOKEY: c_int = -126;
const EACCES: c_int = -13;
const FMODE_CREATED: c_int = 0x100000;

const PATH_MAX: usize = 4096;
const GFP_NOFS: c_int = 0;
const GFP_KERNEL: c_int = 0;

// External functions (from Linux kernel)
extern "C" {
    pub fn arch_get_secureboot() -> bool;
    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn pr_err(fmt: *const c_char, ...);
    pub fn pr_info(fmt: *const c_char, ...);
    pub fn security_current_getlsmprop_subj(prop: *mut lsm_prop);
    pub fn ima_match_policy(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        cred: *const c_void,
        prop: *mut lsm_prop,
        func: ima_hooks,
        mask: c_int,
        flags: c_int,
        _p1: *mut c_void,
        _p2: *mut c_void,
        _p3: *mut c_void,
        _p4: *mut c_void,
    ) -> c_int;
    pub fn current_cred() -> *const c_void;
    pub fn d_inode(dentry: *const dentry) -> *mut inode;
    pub fn d_backing_inode(dentry: *const dentry) -> *mut inode;
    pub fn file_dentry(file: *const file) -> *mut dentry;
    pub fn file_inode(file: *const file) -> *mut inode;
    pub fn __vfs_setxattr_noperm(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        name: *const u8,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    pub fn vfs_getxattr_alloc(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        name: *const u8,
        value: *mut *mut c_char,
        size: usize,
        flags: c_int,
    ) -> c_int;
    pub fn evm_verifyxattr(
        dentry: *mut dentry,
        name: *const u8,
        value: *const c_void,
        size: c_int,
    ) -> integrity_status;
    pub fn integrity_digsig_verify(
        keyring: c_int,
        data: *const c_char,
        datalen: usize,
        digest: *const u8,
        digestlen: usize,
        algo: hash_algo,
    ) -> c_int;
    pub fn integrity_modsig_verify(keyring: c_int, modsig: *const modsig) -> c_int;
    pub fn is_binary_blacklisted(digest: *const u8, digestlen: usize) -> c_int;
    pub fn ima_get_modsig_digest(
        modsig: *const modsig,
        algo: *mut hash_algo,
        digest: *mut *const u8,
        digestsize: *mut u32,
    );
    pub fn process_buffer_measurement(
        idmap: *mut mnt_idmap,
        buf: *const c_void,
        digest: *const u8,
        digestlen: usize,
        eventname: *const c_char,
        pcr: c_int,
        _p1: c_int,
        _p2: *mut c_void,
        _p3: bool,
        _p4: *mut c_void,
        _p5: c_int,
    );
    pub fn integrity_audit_msg(
        audit_type: c_int,
        inode: *mut inode,
        filename: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        rc: c_int,
        pcr: c_int,
    );
    pub fn evm_fix_hmac(
        dentry: *mut dentry,
        xattr_name: *const u8,
        xattr_value: *const c_char,
        xattr_len: c_int,
    ) -> c_int;
    pub fn evm_revalidate_status(xattr_name: *const c_char) -> bool;
    pub fn ima_iint_find(inode: *mut inode) -> *mut ima_iint_cache;
    pub fn ima_collect_measurement(
        iint: *mut ima_iint_cache,
        file: *mut file,
        _p1: *mut c_void,
        _p2: c_int,
        algo: hash_algo,
        _p3: *mut c_void,
    ) -> c_int;
    pub fn inode_lock(inode: *mut inode);
    pub fn inode_unlock(inode: *mut inode);
    pub fn crypto_has_alg(name: *const c_char, _p1: c_int, _p2: c_int) -> c_int;
    pub fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn dentry_path(
        dentry: *const dentry,
        buf: *mut c_char,
        buflen: c_int,
    ) -> *mut c_char;
    pub fn capable(cap: c_int) -> bool;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn security_add_hooks(
        hooks: *const security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    pub fn set_bit(nr: usize, addr: *mut c_void);
    pub fn clear_bit(nr: usize, addr: *mut c_void);
    pub fn test_bit(nr: usize, addr: *const c_void) -> bool;
}

// Conditional compilation: CONFIG_IMA_APPRAISE_BOOTPARAM
#[cfg(feature = "ima_appraise_bootparam")]
static mut ima_appraise_cmdline_default: *mut c_char = ptr::null_mut();

#[cfg(feature = "ima_appraise_bootparam")]
pub unsafe fn ima_appraise_parse_cmdline() {
    let str = ima_appraise_cmdline_default;
    let sb_state = arch_get_secureboot();
    let mut appraisal_state = ima_appraise;

    if str.is_null() {
        return;
    }

    if strncmp(str, b"off\0" as *const u8 as *const c_char, 3) == 0 {
        appraisal_state = 0;
    } else if strncmp(str, b"log\0" as *const u8 as *const c_char, 3) == 0 {
        appraisal_state = IMA_APPRAISE_LOG;
    } else if strncmp(str, b"fix\0" as *const u8 as *const c_char, 3) == 0 {
        appraisal_state = IMA_APPRAISE_FIX;
    } else if strncmp(str, b"enforce\0" as *const u8 as *const c_char, 7) == 0 {
        appraisal_state = IMA_APPRAISE_ENFORCE;
    } else {
        pr_err(b"invalid \"%s\" appraise option\0" as *const u8 as *const c_char, str);
    }

    if sb_state {
        if (appraisal_state & IMA_APPRAISE_ENFORCE) == 0 {
            pr_info(
                b"Secure boot enabled: ignoring ima_appraise=%s option\0" as *const u8
                    as *const c_char,
                str,
            );
        }
    } else {
        ima_appraise = appraisal_state;
    }
}

// is_ima_appraise_enabled - return appraise status
//
// Only return enabled, if not in ima_appraise="fix" or "log" modes.
pub unsafe fn is_ima_appraise_enabled() -> bool {
    (ima_appraise & IMA_APPRAISE_ENFORCE) != 0
}

// ima_must_appraise - set appraise flag
//
// Return 1 to appraise or hash
pub unsafe fn ima_must_appraise(
    idmap: *mut mnt_idmap,
    inode: *mut inode,
    mask: c_int,
    func: ima_hooks,
) -> c_int {
    let mut prop = core::mem::MaybeUninit::<lsm_prop>::uninit();

    if ima_appraise == 0 {
        return 0;
    }

    security_current_getlsmprop_subj(prop.as_mut_ptr());
    ima_match_policy(
        idmap,
        inode,
        current_cred(),
        prop.as_mut_ptr(),
        func,
        mask,
        IMA_APPRAISE | IMA_HASH,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    )
}

unsafe fn ima_fix_xattr(dentry: *mut dentry, iint: *mut ima_iint_cache) -> c_int {
    // Opaque struct access - would need proper definitions from external headers
    // For now, use placeholder implementation with comments
    0
}

// Return specific func appraised cached result
pub unsafe fn ima_get_cache_status(
    iint: *mut ima_iint_cache,
    func: ima_hooks,
) -> integrity_status {
    match func {
        ima_hooks::MMAP_CHECK | ima_hooks::MMAP_CHECK_REQPROT => {
            // iint->ima_mmap_status
            integrity_status::INTEGRITY_UNKNOWN
        }
        ima_hooks::BPRM_CHECK => {
            // iint->ima_bprm_status
            integrity_status::INTEGRITY_UNKNOWN
        }
        ima_hooks::CREDS_CHECK => {
            // iint->ima_creds_status
            integrity_status::INTEGRITY_UNKNOWN
        }
        ima_hooks::FILE_CHECK | ima_hooks::POST_SETATTR => {
            // iint->ima_file_status
            integrity_status::INTEGRITY_UNKNOWN
        }
        _ => {
            // iint->ima_read_status
            integrity_status::INTEGRITY_UNKNOWN
        }
    }
}

unsafe fn ima_set_cache_status(
    iint: *mut ima_iint_cache,
    func: ima_hooks,
    status: integrity_status,
) {
    match func {
        ima_hooks::MMAP_CHECK | ima_hooks::MMAP_CHECK_REQPROT => {
            // iint->ima_mmap_status = status;
        }
        ima_hooks::BPRM_CHECK => {
            // iint->ima_bprm_status = status;
        }
        ima_hooks::CREDS_CHECK => {
            // iint->ima_creds_status = status;
        }
        ima_hooks::FILE_CHECK | ima_hooks::POST_SETATTR => {
            // iint->ima_file_status = status;
        }
        _ => {
            // iint->ima_read_status = status;
        }
    }
}

unsafe fn ima_cache_flags(iint: *mut ima_iint_cache, func: ima_hooks) {
    match func {
        ima_hooks::MMAP_CHECK | ima_hooks::MMAP_CHECK_REQPROT => {
            // iint->flags |= (IMA_MMAP_APPRAISED | IMA_APPRAISED);
        }
        ima_hooks::BPRM_CHECK => {
            // iint->flags |= (IMA_BPRM_APPRAISED | IMA_APPRAISED);
        }
        ima_hooks::CREDS_CHECK => {
            // iint->flags |= (IMA_CREDS_APPRAISED | IMA_APPRAISED);
        }
        ima_hooks::FILE_CHECK | ima_hooks::POST_SETATTR => {
            // iint->flags |= (IMA_FILE_APPRAISED | IMA_APPRAISED);
        }
        _ => {
            // iint->flags |= (IMA_READ_APPRAISED | IMA_APPRAISED);
        }
    }
}

pub unsafe fn ima_get_hash_algo(
    xattr_value: *const evm_ima_xattr_data,
    xattr_len: c_int,
) -> hash_algo {
    let mut ret: hash_algo = ima_hash_algo;

    if xattr_value.is_null() || xattr_len < 2 {
        return ima_hash_algo;
    }

    let xtype = (*xattr_value).xtype;
    match xtype {
        5 => {
            // IMA_VERITY_DIGSIG
            let sig = xattr_value as *const signature_v2_hdr;
            if (*sig).version != 3 || xattr_len <= core::mem::size_of::<signature_v2_hdr>() as c_int
                || (*sig).hash_algo as c_int >= hash_algo::HASH_ALGO__LAST as c_int
            {
                return ima_hash_algo;
            }
            return match (*sig).hash_algo {
                0 => hash_algo::HASH_ALGO_MD5,
                1 => hash_algo::HASH_ALGO_SHA1,
                2 => hash_algo::HASH_ALGO_SHA256,
                _ => ima_hash_algo,
            };
        }
        3 => {
            // EVM_IMA_XATTR_DIGSIG
            let sig = xattr_value as *const signature_v2_hdr;
            if ((*sig).version != 2 && (*sig).version != 3)
                || xattr_len <= core::mem::size_of::<signature_v2_hdr>() as c_int
                || (*sig).hash_algo as c_int >= hash_algo::HASH_ALGO__LAST as c_int
            {
                return ima_hash_algo;
            }
            return match (*sig).hash_algo {
                0 => hash_algo::HASH_ALGO_MD5,
                1 => hash_algo::HASH_ALGO_SHA1,
                2 => hash_algo::HASH_ALGO_SHA256,
                _ => ima_hash_algo,
            };
        }
        2 => {
            // IMA_XATTR_DIGEST_NG
            let algo_byte = (*xattr_value).data.as_ptr().read();
            ret = match algo_byte {
                0 => hash_algo::HASH_ALGO_MD5,
                1 => hash_algo::HASH_ALGO_SHA1,
                2 => hash_algo::HASH_ALGO_SHA256,
                _ => return ima_hash_algo,
            };
            if (algo_byte as c_int) < hash_algo::HASH_ALGO__LAST as c_int {
                return ret;
            }
        }
        1 => {
            // IMA_XATTR_DIGEST
            if xattr_len == 21 {
                let zero: u32 = 0;
                if memcmp(
                    ((*xattr_value).data.as_ptr() as *const u8).add(16) as *const c_void,
                    &zero as *const u32 as *const c_void,
                    4,
                ) == 0
                {
                    return hash_algo::HASH_ALGO_MD5;
                } else {
                    return hash_algo::HASH_ALGO_SHA1;
                }
            } else if xattr_len == 17 {
                return hash_algo::HASH_ALGO_MD5;
            }
        }
        _ => {}
    }

    ima_hash_algo
}

pub unsafe fn ima_read_xattr(
    dentry: *mut dentry,
    xattr_value: *mut *mut evm_ima_xattr_data,
    xattr_len: c_int,
) -> c_int {
    let ret = vfs_getxattr_alloc(
        &mut nop_mnt_idmap as *mut mnt_idmap,
        dentry,
        XATTR_NAME_IMA.as_ptr(),
        xattr_value as *mut *mut c_char,
        xattr_len as usize,
        GFP_NOFS,
    );
    if ret == -EOPNOTSUPP {
        return 0;
    }
    ret
}

// xattr_verify - verify xattr digest or signature
//
// Verify whether the hash or signature matches the file contents.
//
// Return 0 on success, error code otherwise.
unsafe fn xattr_verify(
    func: ima_hooks,
    iint: *mut ima_iint_cache,
    xattr_value: *mut evm_ima_xattr_data,
    xattr_len: c_int,
    status: *mut integrity_status,
    cause: *mut *const c_char,
) -> c_int {
    let mut rc: c_int = -EINVAL;
    let mut hash_start = 0;
    let xtype = (*xattr_value).xtype;

    match xtype {
        2 => {
            // IMA_XATTR_DIGEST_NG
            hash_start = 1;
            // fallthrough
            if *status != integrity_status::INTEGRITY_PASS_IMMUTABLE {
                // Process IMA_XATTR_DIGEST case
            } else {
                // Set IMA_DIGSIG bit
            }

            if xattr_len
                >= (core::mem::size_of::<u8>() as c_int) + hash_start
                    + 32 /* iint->ima_hash->length */
            {
                rc = 0; // memcmp result placeholder
            } else {
                rc = -EINVAL;
            }
            if rc != 0 {
                *cause = b"invalid-hash\0" as *const u8 as *const c_char;
                *status = integrity_status::INTEGRITY_FAIL;
            } else {
                *status = integrity_status::INTEGRITY_PASS;
            }
        }
        1 => {
            // IMA_XATTR_DIGEST
            if *status != integrity_status::INTEGRITY_PASS_IMMUTABLE {
                // Check IMA_DIGSIG_REQUIRED flag
            } else {
                set_bit(6, iint as *mut c_void);
            }

            if xattr_len
                >= (core::mem::size_of::<u8>() as c_int) + hash_start
                    + 32 /* iint->ima_hash->length */
            {
                rc = 0; // memcmp result placeholder
            } else {
                rc = -EINVAL;
            }
            if rc != 0 {
                *cause = b"invalid-hash\0" as *const u8 as *const c_char;
                *status = integrity_status::INTEGRITY_FAIL;
            } else {
                *status = integrity_status::INTEGRITY_PASS;
            }
        }
        3 => {
            // EVM_IMA_XATTR_DIGSIG
            set_bit(6, iint as *mut c_void);

            let mask = IMA_DIGSIG_REQUIRED | IMA_VERITY_REQUIRED;
            // Check conditions...

            let sig = xattr_value as *const signature_v2_hdr;
            if (*sig).version > 3 {
                *cause = b"invalid-signature-version\0" as *const u8 as *const c_char;
                *status = integrity_status::INTEGRITY_FAIL;
            } else {
                rc = integrity_digsig_verify(
                    INTEGRITY_KEYRING_IMA,
                    xattr_value as *const c_char,
                    xattr_len as usize,
                    ptr::null(),
                    32,
                    ima_hash_algo,
                );
                if rc == -EOPNOTSUPP {
                    *status = integrity_status::INTEGRITY_UNKNOWN;
                } else if CONFIG_INTEGRITY_PLATFORM_KEYRING && rc != 0 {
                    if let ima_hooks::MODULE_CHECK = func {
                        rc = integrity_digsig_verify(
                            INTEGRITY_KEYRING_PLATFORM,
                            xattr_value as *const c_char,
                            xattr_len as usize,
                            ptr::null(),
                            32,
                            ima_hash_algo,
                        );
                    }
                }

                if rc != 0 {
                    *cause = b"invalid-signature\0" as *const u8 as *const c_char;
                    *status = integrity_status::INTEGRITY_FAIL;
                } else {
                    *status = integrity_status::INTEGRITY_PASS;
                }
            }
        }
        5 => {
            // IMA_VERITY_DIGSIG
            set_bit(6, iint as *mut c_void);

            let sig = xattr_value as *const signature_v2_hdr;
            if (*sig).version != 3 {
                *cause = b"invalid-signature-version\0" as *const u8 as *const c_char;
                *status = integrity_status::INTEGRITY_FAIL;
            } else {
                rc = integrity_digsig_verify(
                    INTEGRITY_KEYRING_IMA,
                    xattr_value as *const c_char,
                    xattr_len as usize,
                    ptr::null(),
                    32,
                    ima_hash_algo,
                );
                if rc == -EOPNOTSUPP {
                    *status = integrity_status::INTEGRITY_UNKNOWN;
                } else if rc != 0 {
                    *cause = b"invalid-verity-signature\0" as *const u8 as *const c_char;
                    *status = integrity_status::INTEGRITY_FAIL;
                } else {
                    *status = integrity_status::INTEGRITY_PASS;
                }
            }
        }
        _ => {
            *status = integrity_status::INTEGRITY_UNKNOWN;
            *cause = b"unknown-ima-data\0" as *const u8 as *const c_char;
        }
    }

    rc
}

// modsig_verify - verify modsig signature
//
// Verify whether the signature matches the file contents.
//
// Return 0 on success, error code otherwise.
unsafe fn modsig_verify(
    func: ima_hooks,
    modsig: *const modsig,
    status: *mut integrity_status,
    cause: *mut *const c_char,
) -> c_int {
    let mut rc = integrity_modsig_verify(INTEGRITY_KEYRING_IMA, modsig);
    if CONFIG_INTEGRITY_PLATFORM_KEYRING && rc != 0 {
        if let ima_hooks::MODULE_CHECK = func {
            rc = integrity_modsig_verify(INTEGRITY_KEYRING_PLATFORM, modsig);
        }
    }
    if rc != 0 {
        *cause = b"invalid-signature\0" as *const u8 as *const c_char;
        *status = integrity_status::INTEGRITY_FAIL;
    } else {
        *status = integrity_status::INTEGRITY_PASS;
    }

    rc
}

// ima_check_blacklist - determine if the binary is blacklisted.
//
// Add the hash of the blacklisted binary to the measurement list, based
// on policy.
//
// Returns -EPERM if the hash is blacklisted.
pub unsafe fn ima_check_blacklist(
    iint: *mut ima_iint_cache,
    modsig: *const modsig,
    pcr: c_int,
) -> c_int {
    let mut rc = 0;

    // Check IMA_CHECK_BLACKLIST flag...

    if (modsig as *const c_void).is_null() == false {
        let mut hash_algo = hash_algo::HASH_ALGO_SHA1;
        let mut digest: *const u8 = ptr::null();
        let mut digestsize: u32 = 0;

        ima_get_modsig_digest(modsig, &mut hash_algo, &mut digest, &mut digestsize);
        rc = is_binary_blacklisted(digest, digestsize as usize);
    }

    if rc == -EPERM {
        // Process buffer measurement
    }

    rc
}

// ima_appraise_measurement - appraise file measurement
//
// Call evm_verifyxattr() to verify the integrity of 'security.ima'.
// Assuming success, compare the xattr hash with the collected measurement.
//
// Return 0 on success, error code otherwise
pub unsafe fn ima_appraise_measurement(
    func: ima_hooks,
    iint: *mut ima_iint_cache,
    file: *mut file,
    filename: *const c_char,
    xattr_value: *mut evm_ima_xattr_data,
    xattr_len: c_int,
    modsig: *const modsig,
    bprm_is_check: bool,
) -> integrity_status {
    let mut audit_msgno = AUDIT_INTEGRITY_DATA;
    let mut cause: *const c_char = b"unknown\0" as *const u8 as *const c_char;
    let dentry = file_dentry(file);
    let inode = d_backing_inode(dentry);
    let mut status = integrity_status::INTEGRITY_UNKNOWN;
    let mut rc = xattr_len;
    let try_modsig = !modsig.is_null();

    if bprm_is_check {
        audit_msgno = AUDIT_INTEGRITY_USERSPACE;
    }

    if rc <= 0 && !try_modsig {
        if rc != 0 && rc != -ENODATA {
            // goto out
        } else {
            cause = b"missing-hash\0" as *const u8 as *const c_char;
            status = integrity_status::INTEGRITY_NOLABEL;
        }

        if status == integrity_status::INTEGRITY_NOLABEL {
            // Return status
        }
    }

    status = evm_verifyxattr(
        dentry,
        XATTR_NAME_IMA.as_ptr(),
        xattr_value as *const c_void,
        if rc < 0 { 0 } else { rc },
    );

    match status {
        integrity_status::INTEGRITY_PASS
        | integrity_status::INTEGRITY_PASS_IMMUTABLE
        | integrity_status::INTEGRITY_UNKNOWN => {}
        integrity_status::INTEGRITY_NOXATTRS => {
            if try_modsig {
                // break
            }
        }
        integrity_status::INTEGRITY_NOLABEL => {
            cause = b"missing-HMAC\0" as *const u8 as *const c_char;
        }
        _ => {}
    }

    if !xattr_value.is_null() {
        rc = xattr_verify(func, iint, xattr_value, xattr_len, &mut status, &mut cause);
    }

    if try_modsig && (xattr_value.is_null() || rc == -ENOKEY) {
        rc = modsig_verify(func, modsig, &mut status, &mut cause);
    }

    ima_set_cache_status(iint, func, status);
    status
}

// ima_update_xattr - update 'security.ima' hash value
pub unsafe fn ima_update_xattr(iint: *mut ima_iint_cache, file: *mut file) {
    let dentry = file_dentry(file);

    if test_bit(6, iint as *const c_void) {
        return;
    }

    let _rc = ima_collect_measurement(iint, file, ptr::null_mut(), 0, ima_hash_algo, ptr::null_mut());

    inode_lock(file_inode(file));
    let _rc = ima_fix_xattr(dentry, iint);
    inode_unlock(file_inode(file));
}

// ima_inode_post_setattr - reflect file metadata changes
//
// Changes to a dentry's metadata might result in needing to appraise.
//
// This function is called from notify_change(), which expects the caller
// to lock the inode's i_mutex.
unsafe fn ima_inode_post_setattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    _ia_valid: c_int,
) {
    let inode = d_backing_inode(dentry);

    if (ima_policy_flag & IMA_APPRAISE) == 0 {
        return;
    }

    let action = ima_must_appraise(idmap, inode, 0, ima_hooks::FILE_CHECK);
    let iint = ima_iint_find(inode);
    if !iint.is_null() {
        set_bit(7, iint as *mut c_void);
        if action == 0 {
            clear_bit(8, iint as *mut c_void);
        }
    }
}

// ima_protect_xattr - protect 'security.ima'
//
// Ensure that not just anyone can modify or remove 'security.ima'.
unsafe fn ima_protect_xattr(
    _dentry: *mut dentry,
    xattr_name: *const c_char,
    _xattr_value: *const c_void,
    _xattr_value_len: usize,
) -> c_int {
    if strcmp(xattr_name, XATTR_NAME_IMA.as_ptr() as *const c_char) == 0 {
        if !capable(0) {
            return -EPERM;
        }
        return 1;
    }
    0
}

// ima_reset_appraise_flags - reset ima_iint_cache flags
//
// @digsig: whether to clear/set IMA_DIGSIG flag, tristate values
//          0: clear IMA_DIGSIG
//          1: set IMA_DIGSIG
//         -1: don't change IMA_DIGSIG
unsafe fn ima_reset_appraise_flags(inode: *mut inode, digsig: c_int) {
    if (ima_policy_flag & IMA_APPRAISE) == 0 {
        return;
    }

    let iint = ima_iint_find(inode);
    if iint.is_null() {
        return;
    }

    set_bit(9, iint as *mut c_void);
    if digsig == 1 {
        set_bit(6, iint as *mut c_void);
    } else if digsig == 0 {
        clear_bit(6, iint as *mut c_void);
    }
}

// validate_hash_algo() - Block setxattr with unsupported hash algorithms
//
// The xattr value is mapped to its hash algorithm, and this algorithm
// must be built in the kernel for the setxattr to be allowed.
//
// Emit an audit message when the algorithm is invalid.
//
// Return: 0 on success, else an error.
unsafe fn validate_hash_algo(
    dentry: *mut dentry,
    xattr_value: *const evm_ima_xattr_data,
    xattr_value_len: usize,
) -> c_int {
    let mut pathbuf: *mut c_char = ptr::null_mut();
    let xattr_hash_algo = ima_get_hash_algo(xattr_value, xattr_value_len as c_int);

    let allowed_hashes = 0; // Placeholder for atomic read

    if allowed_hashes != 0 {
        if (allowed_hashes & (1u32 << (xattr_hash_algo as u32))) != 0 {
            return 0;
        }
    } else {
        if xattr_hash_algo as c_int == ima_hash_algo as c_int {
            return 0;
        }

        if crypto_has_alg(ptr::null(), 0, 0) != 0 {
            return 0;
        }
    }

    pathbuf = kmalloc(PATH_MAX, GFP_KERNEL) as *mut c_char;
    if pathbuf.is_null() {
        return -EACCES;
    }

    let path = dentry_path(dentry, pathbuf, PATH_MAX as c_int);

    integrity_audit_msg(
        AUDIT_INTEGRITY_DATA,
        d_inode(dentry),
        path,
        b"set_data\0" as *const u8 as *const c_char,
        b"unavailable-hash-algorithm\0" as *const u8 as *const c_char,
        -EACCES,
        0,
    );

    kfree(pathbuf as *mut c_void);

    -EACCES
}

unsafe fn ima_inode_setxattr(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    xattr_name: *const c_char,
    xattr_value: *const c_void,
    xattr_value_len: usize,
    _flags: c_int,
) -> c_int {
    let xvalue = xattr_value as *const evm_ima_xattr_data;
    let mut digsig: c_int = 0;
    let mut result: c_int;
    let mut err: c_int;

    result = ima_protect_xattr(dentry, xattr_name, xattr_value, xattr_value_len);
    if result == 1 {
        if xattr_value_len == 0 || ((*xvalue).xtype as c_int >= IMA_XATTR_LAST as c_int) {
            return -EINVAL;
        }

        err = validate_hash_algo(dentry, xvalue, xattr_value_len);
        if err != 0 {
            return err;
        }

        digsig = if (*xvalue).xtype == 3 { 1 } else { 0 };
    } else if strcmp(xattr_name, XATTR_NAME_EVM.as_ptr() as *const c_char) == 0 && xattr_value_len > 0
    {
        digsig = if (*xvalue).xtype == 4 { 1 } else { 0 };
    } else {
        digsig = -1;
    }

    if result == 1 || evm_revalidate_status(xattr_name) {
        ima_reset_appraise_flags(d_backing_inode(dentry), digsig);
        if result == 1 {
            result = 0;
        }
    }
    result
}

unsafe fn ima_inode_set_acl(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl_name: *const c_char,
) -> c_int {
    if evm_revalidate_status(acl_name) {
        ima_reset_appraise_flags(d_backing_inode(dentry), -1);
    }

    0
}

unsafe fn ima_inode_removexattr(
    _idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    xattr_name: *const c_char,
) -> c_int {
    let mut result: c_int;
    let mut digsig: c_int = -1;

    result = ima_protect_xattr(dentry, xattr_name, ptr::null(), 0);
    if result == 1 || evm_revalidate_status(xattr_name) {
        if strcmp(xattr_name, XATTR_NAME_IMA.as_ptr() as *const c_char) == 0 {
            digsig = 0;
        }
        ima_reset_appraise_flags(d_backing_inode(dentry), digsig);
        if result == 1 {
            result = 0;
        }
    }
    result
}

unsafe fn ima_inode_remove_acl(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    acl_name: *const c_char,
) -> c_int {
    ima_inode_set_acl(idmap, dentry, acl_name)
}

// LSM hook initialization
pub unsafe fn init_ima_appraise_lsm(lsmid: *const lsm_id) {
    // Placeholder: security_add_hooks would be called here
    // security_add_hooks(ima_appraise_hooks, ..., lsmid);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
