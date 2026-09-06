// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005-2010 IBM Corporation
 *
 * Author:
 * Mimi Zohar <zohar@us.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * File: evm_main.c
 *	implements evm_inode_setxattr, evm_inode_post_setxattr,
 *	evm_inode_removexattr, evm_verifyxattr, and evm_inode_set_acl.
 */

// External types from Linux kernel (dependencies to be provided)
// struct dentry, struct inode, struct file, struct iattr, struct posix_acl, etc.
// These would be imported from linux-generated bindings

use core::ffi::CStr;
use core::mem;

// pr_fmt(fmt) "EVM: "fmt macro equivalent
// Using static strings with EVM: prefix in pr_info/pr_err calls

pub static mut EVM_INITIALIZED: i32 = 0;

static INTEGRITY_STATUS_MSG: &[&str] = &[
    "pass",
    "pass_immutable",
    "fail",
    "fail_immutable",
    "no_label",
    "no_xattrs",
    "unknown",
];

pub static mut EVM_HMAC_ATTRS: i32 = 0;

// External type stubs for kernel structures
// In practice these would be defined in kernel bindings
#[repr(C)]
pub struct XattrList {
    pub name: *const i8,
    pub enabled: bool,
    pub list: ListHead,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct IntegrityStatus(i32);

#[repr(C)]
pub struct Dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EvmImaXattrData {
    pub typ: u8,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct SignatureV2Hdr {
    pub version: u8,
    pub hash_algo: u8,
    _reserved: [u8; 0],
}

#[repr(C)]
pub struct EvmDigest {
    pub hdr: DigestHdr,
    pub digest: [u8; 64],
}

#[repr(C)]
pub struct DigestHdr {
    pub algo: u8,
    pub length: u8,
    _reserved: [u16; 0],
}

#[repr(C)]
pub struct EvmIintCache {
    pub evm_status: i32,
    pub flags: usize,
    pub metadata_inode: Inode,
}

#[repr(C)]
pub struct EvmXattr {
    pub typ: u8,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct MntIdmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Iattr {
    pub ia_valid: u32,
    pub ia_mode: u32,
    pub ia_uid: u32,
    pub ia_gid: u32,
    _private: [u8; 0],
}

#[repr(C)]
pub struct QStr {
    pub name: *const u8,
    pub len: u32,
}

#[repr(C)]
pub struct Xattr {
    pub name: *const i8,
    pub value: *mut u8,
    pub value_len: usize,
}

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PosixAcl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SecurityHookList {
    pub hook: *mut u8,
    pub lsm: *const i8,
}

#[repr(C)]
pub struct LsmId {
    pub name: *const i8,
    pub id: i32,
}

#[repr(C)]
pub struct LsmBlobSizes {
    pub lbs_inode: usize,
    pub lbs_xattr_count: i32,
}

// Constants (from preprocessor)
const HASH_ALGO_SHA1: u8 = 2;
const SHA1_DIGEST_SIZE: usize = 20;
const EVM_KEY_MASK: i32 = 0x01;
const EVM_INIT_HMAC: i32 = 0x02;
const EVM_INIT_X509: i32 = 0x04;
const EVM_SETUP_COMPLETE: i32 = 0x08;
const EVM_SIGV3_REQUIRED: i32 = 0x10;
const EVM_ALLOW_METADATA_WRITES: i32 = 0x20;
const EVM_XATTR_HMAC: u8 = 1;
const EVM_XATTR_PORTABLE_DIGSIG: u8 = 3;
const EVM_IMA_XATTR_DIGSIG: u8 = 2;
const EVM_IMMUTABLE_DIGSIG: usize = 0x01;
const EVM_NEW_FILE: usize = 0x02;
const EOPNOTSUPP: i32 = -95;
const ENODATA: i32 = -61;
const EINVAL: i32 = -22;
const EPERM: i32 = -1;
const ENOMEM: i32 = -12;
const ECANCELED: i32 = -125;
const INTEGRITY_PASS: i32 = 0;
const INTEGRITY_PASS_IMMUTABLE: i32 = 1;
const INTEGRITY_FAIL: i32 = 2;
const INTEGRITY_FAIL_IMMUTABLE: i32 = 3;
const INTEGRITY_NOLABEL: i32 = 4;
const INTEGRITY_NOXATTRS: i32 = 5;
const INTEGRITY_UNKNOWN: i32 = 6;
const TMPFS_MAGIC: u32 = 0x01021994;
const SYSFS_MAGIC: u32 = 0x62656572;
const AUDIT_INTEGRITY_METADATA: i32 = 1500;
const INTEGRITY_KEYRING_EVM: i32 = 1;
const ATTR_MODE: u32 = 0x01;
const ATTR_UID: u32 = 0x02;
const ATTR_GID: u32 = 0x04;
const LSM_ID_EVM: i32 = 8;
const LSM_ORDER_LAST: i32 = 1000;
const FMODE_WRITE: u32 = 0x2;
const XATTR_SECURITY_PREFIX_LEN: usize = 9;

// Global list (external dependencies would provide LIST_HEAD macro implementation)
pub static mut EVM_CONFIG_XATTRNAMES: ListHead = ListHead {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

static mut EVM_CONFIG_DEFAULT_XATTRNAMES: [XattrList; 8] = unsafe {
    [
        XattrList {
            name: b"security.selinux\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_SECURITY_SELINUX)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.smack\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_SECURITY_SMACK)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.smackexec\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_EVM_EXTRA_SMACK_XATTRS)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.smacktransmute\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_EVM_EXTRA_SMACK_XATTRS)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.smackmmap\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_EVM_EXTRA_SMACK_XATTRS)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.apparmor\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_SECURITY_APPARMOR)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.ima\0".as_ptr() as *const i8,
            enabled: true, // IS_ENABLED(CONFIG_IMA_APPRAISE)
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
        XattrList {
            name: b"security.capability\0".as_ptr() as *const i8,
            enabled: true,
            list: ListHead {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        },
    ]
};

static mut EVM_CMDLINE: *mut i8 = core::ptr::null_mut();

static mut EVM_FIXMODE: i32 = 0;

unsafe fn evm_set_fixmode() {
    if EVM_CMDLINE.is_null() {
        return;
    }

    // strncmp comparison: "fix" with first 3 chars
    if libc::strncmp(EVM_CMDLINE, b"fix\0".as_ptr() as *const i8, 3) == 0 {
        if arch_get_secureboot() != 0 {
            pr_info("Secure boot enabled: ignoring evm=fix");
            return;
        }
        EVM_FIXMODE = 1;
    } else {
        pr_err("invalid \"%s\" mode", EVM_CMDLINE);
    }
}

unsafe fn evm_init_config() {
    let xattrs = 8; // ARRAY_SIZE(evm_config_default_xattrnames)

    pr_info("Initialising EVM extended attributes:\n");
    for i in 0..xattrs {
        let name = EVM_CONFIG_DEFAULT_XATTRNAMES[i].name;
        let enabled = EVM_CONFIG_DEFAULT_XATTRNAMES[i].enabled;
        let suffix = if !enabled { " (disabled)" } else { "" };
        pr_info("%s%s\n", name, suffix.as_ptr() as *const i8);
        // list_add_tail: would add to EVM_CONFIG_XATTRNAMES
        // Simplified; actual implementation needs list manipulation
    }

    // #ifdef CONFIG_EVM_ATTR_FSUUID
    // EVM_HMAC_ATTRS |= EVM_ATTR_FSUUID;
    // #endif

    pr_info("HMAC attrs: 0x%x\n", EVM_HMAC_ATTRS);
}

fn evm_key_loaded() -> bool {
    unsafe { (EVM_INITIALIZED & EVM_KEY_MASK) != 0 }
}

fn evm_hmac_disabled() -> bool {
    unsafe {
        if (EVM_INITIALIZED & EVM_INIT_HMAC) != 0 {
            return false;
        }

        if (EVM_INITIALIZED & EVM_SETUP_COMPLETE) == 0 {
            return false;
        }

        true
    }
}

fn evm_sigv3_required() -> bool {
    unsafe { (EVM_INITIALIZED & EVM_SIGV3_REQUIRED) != 0 }
}

unsafe fn evm_find_protected_xattrs(dentry: *const Dentry) -> i32 {
    let inode = d_backing_inode(dentry);
    let mut count = 0;

    // Check if inode supports xattr
    if !inode_supports_xattr(inode) {
        return -EOPNOTSUPP;
    }

    // list_for_each_entry_lockless - would iterate over xattrs
    // Simplified implementation
    for i in 0..8 {
        let xattr_name = EVM_CONFIG_DEFAULT_XATTRNAMES[i].name;
        let error = __vfs_getxattr(dentry, inode, xattr_name, core::ptr::null_mut(), 0);
        if error < 0 {
            if error == -ENODATA {
                continue;
            }
            return error;
        }
        count += 1;
    }

    count
}

unsafe fn is_unsupported_hmac_fs(dentry: *const Dentry) -> i32 {
    let inode = d_backing_inode(dentry);

    if sb_i_evm_hmac_unsupported(inode) != 0 {
        pr_info_once("%s not supported\n", inode_sb_name(inode));
        return 1;
    }
    0
}

unsafe fn evm_verify_hmac(
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *mut i8,
    xattr_value_len: usize,
) -> i32 {
    let mut xattr_data: *mut EvmImaXattrData = core::ptr::null_mut();
    let mut evm_status = INTEGRITY_PASS;
    let mut digest: EvmDigest = mem::zeroed();
    let inode = d_backing_inode(dentry);
    let iint = evm_iint_inode(inode);
    let mut rc: i32;
    let mut xattr_len: i32;
    let mut evm_immutable = 0;

    if !iint.is_null()
        && ((*iint).evm_status == INTEGRITY_PASS
            || (*iint).evm_status == INTEGRITY_PASS_IMMUTABLE)
    {
        return (*iint).evm_status;
    }

    // Check for unsupported filesystems
    if (EVM_INITIALIZED & EVM_INIT_X509) == 0 && is_unsupported_hmac_fs(dentry) != 0 {
        return INTEGRITY_UNKNOWN;
    }

    // Get EVM xattr
    rc = vfs_getxattr_alloc(&NOP_MNT_IDMAP, dentry, b"security.evm\0".as_ptr() as *const i8,
                           &mut xattr_data as *mut *mut EvmImaXattrData as *mut *mut i8,
                           0, 0); // GFP_NOFS
    if rc <= 0 {
        evm_status = INTEGRITY_FAIL;
        if rc == -ENODATA {
            rc = evm_find_protected_xattrs(dentry);
            if rc > 0 {
                evm_status = INTEGRITY_NOLABEL;
            } else if rc == 0 {
                evm_status = INTEGRITY_NOXATTRS;
            }
        } else if rc == -EOPNOTSUPP {
            evm_status = INTEGRITY_UNKNOWN;
        }
        return evm_status;
    }

    xattr_len = rc;

    // Check xattr type
    if !xattr_data.is_null() {
        match (*xattr_data).typ {
            EVM_XATTR_HMAC => {
                if xattr_len != mem::size_of::<EvmXattr>() as i32 {
                    evm_status = INTEGRITY_FAIL;
                } else {
                    digest.hdr.algo = HASH_ALGO_SHA1;
                    rc = evm_calc_hmac(dentry, xattr_name, xattr_value, xattr_value_len, &mut digest, iint);
                    if rc == 0 {
                        rc = crypto_memneq(
                            (*xattr_data).data.as_ptr() as *const u8,
                            digest.digest.as_ptr(),
                            SHA1_DIGEST_SIZE,
                        ) as i32;
                        if rc != 0 {
                            rc = -EINVAL;
                        }
                    }
                }
            }
            EVM_XATTR_PORTABLE_DIGSIG => {
                evm_immutable = 1;
                // fallthrough
                goto_portable_digsig(&mut evm_status, dentry, xattr_name, xattr_value, xattr_value_len,
                                    &mut digest, iint, xattr_len, xattr_data, &mut rc);
            }
            EVM_IMA_XATTR_DIGSIG => {
                goto_portable_digsig(&mut evm_status, dentry, xattr_name, xattr_value, xattr_value_len,
                                    &mut digest, iint, xattr_len, xattr_data, &mut rc);
            }
            _ => {
                rc = -EINVAL;
            }
        }
    }

    if rc != 0 {
        if rc == -ENODATA {
            evm_status = INTEGRITY_NOXATTRS;
        } else if evm_immutable != 0 {
            evm_status = INTEGRITY_FAIL_IMMUTABLE;
        } else {
            evm_status = INTEGRITY_FAIL;
        }
    }

    pr_debug("digest: (%d) [%*phN]\n", digest.hdr.length, digest.hdr.length, digest.digest.as_ptr());

    if !iint.is_null() {
        (*iint).evm_status = evm_status;
    }
    kfree(xattr_data as *mut u8);
    evm_status
}

unsafe fn goto_portable_digsig(
    evm_status: &mut i32,
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *mut i8,
    xattr_value_len: usize,
    digest: &mut EvmDigest,
    iint: *mut EvmIintCache,
    xattr_len: i32,
    xattr_data: *const EvmImaXattrData,
    rc: &mut i32,
) {
    if xattr_len <= mem::size_of::<SignatureV2Hdr>() as i32 {
        *evm_status = INTEGRITY_FAIL;
        return;
    }

    let hdr = xattr_data as *const SignatureV2Hdr;

    if evm_sigv3_required() && (*hdr).version != 3 {
        *evm_status = INTEGRITY_FAIL;
        return;
    }

    digest.hdr.algo = (*hdr).hash_algo;
    *rc = evm_calc_hash(dentry, xattr_name, xattr_value, xattr_value_len,
                       (*xattr_data).typ, digest, iint);
    if *rc == 0 {
        *rc = integrity_digsig_verify(
            INTEGRITY_KEYRING_EVM,
            xattr_data as *const i8,
            xattr_len as usize,
            digest.digest.as_ptr(),
            digest.hdr.length as usize,
            digest.hdr.algo,
        );
        if *rc == 0 {
            if (*xattr_data).typ == EVM_XATTR_PORTABLE_DIGSIG {
                if !iint.is_null() {
                    (*iint).flags |= EVM_IMMUTABLE_DIGSIG;
                }
                *evm_status = INTEGRITY_PASS_IMMUTABLE;
            } else if !inode_is_readonly(d_backing_inode(dentry))
                && !inode_sb_readonly(d_backing_inode(dentry))
                && !inode_is_immutable(d_backing_inode(dentry))
                && is_unsupported_hmac_fs(dentry) == 0
            {
                evm_update_evmxattr(dentry, xattr_name, xattr_value, xattr_value_len);
            }
        }
    }
}

unsafe fn evm_protected_xattr_common(req_xattr_name: *const i8, all_xattrs: bool) -> i32 {
    let mut found = 0;
    let namelen = libc::strlen(req_xattr_name);

    for i in 0..8 {
        let xattr = &EVM_CONFIG_DEFAULT_XATTRNAMES[i];
        if !all_xattrs && !xattr.enabled {
            continue;
        }

        let xattr_namelen = libc::strlen(xattr.name);
        if xattr_namelen == namelen
            && libc::strncmp(req_xattr_name, xattr.name, namelen) == 0
        {
            found = 1;
            break;
        }
        if libc::strncmp(
            req_xattr_name,
            xattr.name.add(XATTR_SECURITY_PREFIX_LEN),
            libc::strlen(req_xattr_name),
        ) == 0
        {
            found = 1;
            break;
        }
    }

    found
}

pub unsafe fn evm_protected_xattr(req_xattr_name: *const i8) -> i32 {
    evm_protected_xattr_common(req_xattr_name, false)
}

pub unsafe fn evm_protected_xattr_if_enabled(req_xattr_name: *const i8) -> i32 {
    evm_protected_xattr_common(req_xattr_name, true)
}

pub unsafe fn evm_read_protected_xattrs(
    dentry: *const Dentry,
    buffer: *mut u8,
    buffer_size: i32,
    typ: u8,
    canonical_fmt: bool,
) -> i32 {
    let mut size: i32;
    let mut total_size: i32 = 0;

    for i in 0..8 {
        let xattr = &EVM_CONFIG_DEFAULT_XATTRNAMES[i];
        let rc = __vfs_getxattr(dentry, d_backing_inode(dentry), xattr.name, core::ptr::null_mut(), 0);
        if rc < 0 && rc == -ENODATA {
            continue;
        } else if rc < 0 {
            return rc;
        }

        match typ {
            b'n' => {
                size = (libc::strlen(xattr.name) + 1) as i32;
                if !buffer.is_null() {
                    if total_size > 0 {
                        *buffer.add(total_size as usize) = b'|';
                    }
                    libc::memcpy(
                        buffer.add(total_size as usize) as *mut u8,
                        xattr.name as *const u8,
                        size as usize,
                    );
                }
            }
            b'l' => {
                size = 4; // sizeof(u32)
                if !buffer.is_null() {
                    let val = if canonical_fmt { u32::to_le(rc as u32) } else { rc as u32 };
                    *(buffer.add(total_size as usize) as *mut u32) = val;
                }
            }
            b'v' => {
                size = rc;
                if !buffer.is_null() {
                    rc = __vfs_getxattr(
                        dentry,
                        d_backing_inode(dentry),
                        xattr.name,
                        buffer.add(total_size as usize) as *mut i8,
                        (buffer_size - total_size) as usize,
                    );
                    if rc < 0 {
                        return rc;
                    }
                }
            }
            _ => {
                return -EINVAL;
            }
        }

        total_size += size;
    }

    total_size
}

pub unsafe fn evm_verifyxattr(
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *mut u8,
    xattr_value_len: usize,
) -> i32 {
    if !evm_key_loaded() || evm_protected_xattr(xattr_name) == 0 {
        return INTEGRITY_UNKNOWN;
    }

    evm_verify_hmac(dentry, xattr_name, xattr_value as *mut i8, xattr_value_len)
}

unsafe fn evm_verify_current_integrity(dentry: *const Dentry) -> i32 {
    let inode = d_backing_inode(dentry);

    if !evm_key_loaded() || !inode_is_regular_file(inode) || EVM_FIXMODE != 0 {
        return INTEGRITY_PASS;
    }
    evm_verify_hmac(dentry, core::ptr::null(), core::ptr::null_mut(), 0)
}

unsafe fn evm_xattr_change(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *const u8,
    xattr_value_len: usize,
) -> i32 {
    let mut xattr_data: *mut i8 = core::ptr::null_mut();
    let mut rc = 0;

    rc = vfs_getxattr_alloc(
        &NOP_MNT_IDMAP,
        dentry,
        xattr_name,
        &mut xattr_data,
        0,
        0, // GFP_NOFS
    );
    if rc < 0 {
        rc = 1;
    } else if rc == xattr_value_len as i32 {
        rc = (libc::memcmp(xattr_value as *const u8, xattr_data as *const u8, rc as usize) != 0) as i32;
    } else {
        rc = 1;
    }

    kfree(xattr_data as *mut u8);
    rc
}

unsafe fn evm_protect_xattr(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *const u8,
    xattr_value_len: usize,
) -> i32 {
    let mut evm_status: i32;

    if libc::strcmp(xattr_name, b"security.evm\0".as_ptr() as *const i8) == 0 {
        if !capable(1) {
            // CAP_SYS_ADMIN = 1
            return -EPERM;
        }
        if is_unsupported_hmac_fs(dentry) != 0 {
            return -EPERM;
        }
    } else if evm_protected_xattr(xattr_name) == 0 {
        if !posix_xattr_acl(xattr_name) {
            return 0;
        }
        if is_unsupported_hmac_fs(dentry) != 0 {
            return 0;
        }

        evm_status = evm_verify_current_integrity(dentry);
        if evm_status == INTEGRITY_PASS || evm_status == INTEGRITY_NOXATTRS {
            return 0;
        }
    } else if is_unsupported_hmac_fs(dentry) != 0 {
        return 0;
    }

    evm_status = evm_verify_current_integrity(dentry);
    if evm_status == INTEGRITY_NOXATTRS {
        if evm_hmac_disabled() {
            return 0;
        }

        let iint = evm_iint_inode(d_backing_inode(dentry));
        if !iint.is_null() && ((*iint).flags & EVM_NEW_FILE) != 0 {
            return 0;
        }

        // Check for pseudo filesystems
        let sb_magic = inode_sb_magic(d_backing_inode(dentry));
        if sb_magic == TMPFS_MAGIC || sb_magic == SYSFS_MAGIC {
            return 0;
        }

        integrity_audit_msg(
            AUDIT_INTEGRITY_METADATA,
            d_backing_inode(dentry),
            dentry_name(dentry),
            b"update_metadata\0".as_ptr() as *const i8,
            INTEGRITY_STATUS_MSG[evm_status as usize].as_ptr() as *const i8,
            -EPERM,
            0,
        );
    }

    if evm_hmac_disabled()
        && (evm_status == INTEGRITY_NOLABEL || evm_status == INTEGRITY_UNKNOWN)
    {
        return 0;
    }

    if evm_status == INTEGRITY_FAIL_IMMUTABLE {
        return 0;
    }

    if evm_status == INTEGRITY_PASS_IMMUTABLE
        && evm_xattr_change(idmap, dentry, xattr_name, xattr_value, xattr_value_len) == 0
    {
        return 0;
    }

    if evm_status != INTEGRITY_PASS && evm_status != INTEGRITY_PASS_IMMUTABLE {
        integrity_audit_msg(
            AUDIT_INTEGRITY_METADATA,
            d_backing_inode(dentry),
            dentry_name(dentry),
            b"appraise_metadata\0".as_ptr() as *const i8,
            INTEGRITY_STATUS_MSG[evm_status as usize].as_ptr() as *const i8,
            -EPERM,
            0,
        );
    }
    if evm_status == INTEGRITY_PASS { 0 } else { -EPERM }
}

pub unsafe fn evm_inode_setxattr(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *const u8,
    xattr_value_len: usize,
    flags: i32,
) -> i32 {
    let xattr_data = xattr_value as *const EvmImaXattrData;

    if (EVM_INITIALIZED & EVM_ALLOW_METADATA_WRITES) != 0 {
        return 0;
    }

    if libc::strcmp(xattr_name, b"security.evm\0".as_ptr() as *const i8) == 0 {
        if xattr_value_len == 0 {
            return -EINVAL;
        }
        if !xattr_data.is_null() {
            if (*xattr_data).typ != EVM_IMA_XATTR_DIGSIG
                && (*xattr_data).typ != EVM_XATTR_PORTABLE_DIGSIG
            {
                return -EPERM;
            }
        }
    }
    evm_protect_xattr(idmap, dentry, xattr_name, xattr_value, xattr_value_len)
}

pub unsafe fn evm_inode_removexattr(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    xattr_name: *const i8,
) -> i32 {
    if (EVM_INITIALIZED & EVM_ALLOW_METADATA_WRITES) != 0 {
        return 0;
    }

    evm_protect_xattr(idmap, dentry, xattr_name, core::ptr::null(), 0)
}

// CONFIG_FS_POSIX_ACL conditional
unsafe fn evm_inode_set_acl_change(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    name: *const i8,
    kacl: *mut PosixAcl,
) -> i32 {
    if kacl.is_null() {
        return 1;
    }

    let inode = d_backing_inode(dentry);
    let mut mode: u32 = 0;

    let rc = posix_acl_update_mode(idmap, inode, &mut mode, kacl);
    if rc != 0 || inode_i_mode(inode) != mode {
        return 1;
    }

    0
}

pub unsafe fn evm_inode_set_acl(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    acl_name: *const i8,
    kacl: *mut PosixAcl,
) -> i32 {
    if (EVM_INITIALIZED & EVM_ALLOW_METADATA_WRITES) != 0 {
        return 0;
    }

    let evm_status = evm_verify_current_integrity(dentry);
    if evm_status == INTEGRITY_PASS || evm_status == INTEGRITY_NOXATTRS {
        return 0;
    }

    if evm_hmac_disabled() && (evm_status == INTEGRITY_NOLABEL || evm_status == INTEGRITY_UNKNOWN) {
        return 0;
    }

    if evm_status == INTEGRITY_FAIL_IMMUTABLE {
        return 0;
    }

    if evm_status == INTEGRITY_PASS_IMMUTABLE
        && evm_inode_set_acl_change(idmap, dentry, acl_name, kacl) == 0
    {
        return 0;
    }

    if evm_status != INTEGRITY_PASS_IMMUTABLE {
        integrity_audit_msg(
            AUDIT_INTEGRITY_METADATA,
            d_backing_inode(dentry),
            dentry_name(dentry),
            b"appraise_metadata\0".as_ptr() as *const i8,
            INTEGRITY_STATUS_MSG[evm_status as usize].as_ptr() as *const i8,
            -EPERM,
            0,
        );
    }
    -EPERM
}

pub unsafe fn evm_inode_remove_acl(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    acl_name: *const i8,
) -> i32 {
    evm_inode_set_acl(idmap, dentry, acl_name, core::ptr::null_mut())
}

unsafe fn evm_reset_status(inode: *const Inode) {
    let iint = evm_iint_inode(inode);
    if !iint.is_null() {
        (*iint).evm_status = INTEGRITY_UNKNOWN;
    }
}

pub unsafe fn evm_metadata_changed(inode: *const Inode, metadata_inode: *const Inode) -> bool {
    let iint = evm_iint_inode(inode);
    let mut ret = false;

    if !iint.is_null() {
        ret = !inode_is_i_version(metadata_inode)
            || integrity_inode_attrs_changed(&(*iint).metadata_inode, metadata_inode) != 0;
        if ret {
            (*iint).evm_status = INTEGRITY_UNKNOWN;
        }
    }

    ret
}

pub unsafe fn evm_revalidate_status(xattr_name: *const i8) -> bool {
    if !evm_key_loaded() {
        return false;
    }

    // evm_inode_post_setattr() passes NULL
    if xattr_name.is_null() {
        return true;
    }

    if evm_protected_xattr(xattr_name) == 0
        && !posix_xattr_acl(xattr_name)
        && libc::strcmp(xattr_name, b"security.evm\0".as_ptr() as *const i8) != 0
    {
        return false;
    }

    true
}

pub unsafe fn evm_fix_hmac(
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *const i8,
    xattr_value_len: usize,
) -> i32 {
    if EVM_FIXMODE == 0 || !evm_revalidate_status(xattr_name) {
        return -EPERM;
    }

    if (EVM_INITIALIZED & EVM_INIT_HMAC) == 0 {
        return -EPERM;
    }

    if is_unsupported_hmac_fs(dentry) != 0 {
        return -EOPNOTSUPP;
    }

    evm_update_evmxattr(dentry, xattr_name, xattr_value, xattr_value_len)
}

unsafe fn evm_inode_post_setxattr(
    dentry: *const Dentry,
    xattr_name: *const i8,
    xattr_value: *const u8,
    xattr_value_len: usize,
    flags: i32,
) {
    if !evm_revalidate_status(xattr_name) {
        return;
    }

    evm_reset_status((*dentry).d_inode);

    if libc::strcmp(xattr_name, b"security.evm\0".as_ptr() as *const i8) == 0 {
        return;
    }

    if (EVM_INITIALIZED & EVM_INIT_HMAC) == 0 {
        return;
    }

    if is_unsupported_hmac_fs(dentry) != 0 {
        return;
    }

    evm_update_evmxattr(dentry, xattr_name, xattr_value, xattr_value_len);
}

unsafe fn evm_inode_post_set_acl(
    dentry: *const Dentry,
    acl_name: *const i8,
    kacl: *mut PosixAcl,
) {
    evm_inode_post_setxattr(dentry, acl_name, core::ptr::null(), 0, 0);
}

unsafe fn evm_inode_post_removexattr(dentry: *const Dentry, xattr_name: *const i8) {
    if !evm_revalidate_status(xattr_name) {
        return;
    }

    evm_reset_status((*dentry).d_inode);

    if libc::strcmp(xattr_name, b"security.evm\0".as_ptr() as *const i8) == 0 {
        return;
    }

    if (EVM_INITIALIZED & EVM_INIT_HMAC) == 0 {
        return;
    }

    evm_update_evmxattr(dentry, xattr_name, core::ptr::null(), 0);
}

unsafe fn evm_inode_post_remove_acl(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    acl_name: *const i8,
) {
    evm_inode_post_removexattr(dentry, acl_name);
}

unsafe fn evm_attr_change(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    attr: *const Iattr,
) -> i32 {
    let inode = d_backing_inode(dentry);
    let ia_valid = (*attr).ia_valid;

    if !i_uid_needs_update(idmap, attr, inode)
        && !i_gid_needs_update(idmap, attr, inode)
        && ((ia_valid & ATTR_MODE) == 0 || (*attr).ia_mode == inode_i_mode(inode))
    {
        return 0;
    }

    1
}

pub unsafe fn evm_inode_setattr(
    idmap: *const MntIdmap,
    dentry: *const Dentry,
    attr: *const Iattr,
) -> i32 {
    let ia_valid = (*attr).ia_valid;

    if (EVM_INITIALIZED & EVM_ALLOW_METADATA_WRITES) != 0 {
        return 0;
    }

    if is_unsupported_hmac_fs(dentry) != 0 {
        return 0;
    }

    if (ia_valid & (ATTR_MODE | ATTR_UID | ATTR_GID)) == 0 {
        return 0;
    }

    let evm_status = evm_verify_current_integrity(dentry);
    if evm_status == INTEGRITY_PASS
        || evm_status == INTEGRITY_NOXATTRS
        || evm_status == INTEGRITY_FAIL_IMMUTABLE
        || (evm_hmac_disabled() && (evm_status == INTEGRITY_NOLABEL || evm_status == INTEGRITY_UNKNOWN))
    {
        return 0;
    }

    if evm_status == INTEGRITY_PASS_IMMUTABLE && evm_attr_change(idmap, dentry, attr) == 0 {
        return 0;
    }

    integrity_audit_msg(
        AUDIT_INTEGRITY_METADATA,
        d_backing_inode(dentry),
        dentry_name(dentry),
        b"appraise_metadata\0".as_ptr() as *const i8,
        INTEGRITY_STATUS_MSG[evm_status as usize].as_ptr() as *const i8,
        -EPERM,
        0,
    );
    -EPERM
}

unsafe fn evm_inode_post_setattr(idmap: *const MntIdmap, dentry: *const Dentry, ia_valid: i32) {
    if !evm_revalidate_status(core::ptr::null()) {
        return;
    }

    evm_reset_status((*dentry).d_inode);

    if (EVM_INITIALIZED & EVM_INIT_HMAC) == 0 {
        return;
    }

    if is_unsupported_hmac_fs(dentry) != 0 {
        return;
    }

    if (ia_valid & (ATTR_MODE | ATTR_UID | ATTR_GID)) != 0 {
        evm_update_evmxattr(dentry, core::ptr::null(), core::ptr::null(), 0);
    }
}

unsafe fn evm_inode_copy_up_xattr(src: *const Dentry, name: *const i8) -> i32 {
    let mut xattr_data: *mut EvmImaXattrData = core::ptr::null_mut();
    let mut rc: i32;

    if libc::strcmp(name, b"security.evm\0".as_ptr() as *const i8) != 0 {
        return -EOPNOTSUPP;
    }

    rc = vfs_getxattr_alloc(
        &NOP_MNT_IDMAP,
        src,
        b"security.evm\0".as_ptr() as *const i8,
        &mut xattr_data as *mut *mut EvmImaXattrData as *mut *mut i8,
        0,
        0, // GFP_NOFS
    );
    if rc <= 0 {
        return -EPERM;
    }

    if rc < (mem::offset_of!(EvmImaXattrData, data) + mem::size_of::<u8>()) as i32 {
        return -EPERM;
    }

    match (*xattr_data).typ {
        EVM_XATTR_PORTABLE_DIGSIG => {
            rc = 0;
        }
        EVM_XATTR_HMAC | EVM_IMA_XATTR_DIGSIG => {
            rc = -ECANCELED;
        }
        _ => {
            rc = -ECANCELED;
        }
    }

    kfree(xattr_data as *mut u8);
    rc
}

pub unsafe fn evm_inode_init_security(
    inode: *const Inode,
    dir: *const Inode,
    qstr: *const QStr,
    xattrs: *mut Xattr,
    xattr_count: *mut i32,
) -> i32 {
    let mut xattr_data: *mut EvmXattr;
    let mut xattr: *mut Xattr;
    let mut evm_xattr: *mut Xattr;
    let mut evm_protected_xattrs = false;
    let mut rc: i32;

    if (EVM_INITIALIZED & EVM_INIT_HMAC) == 0 || xattrs.is_null() {
        return 0;
    }

    xattr = xattrs;
    while !(*xattr).name.is_null() {
        if evm_protected_xattr((*xattr).name) != 0 {
            evm_protected_xattrs = true;
        }
        xattr = xattr.add(1);
    }

    if !evm_protected_xattrs {
        return 0;
    }

    evm_xattr = lsm_get_xattr_slot(xattrs, xattr_count);

    xattr_data = kzalloc_obj(mem::size_of::<EvmXattr>(), 0) as *mut EvmXattr;
    if xattr_data.is_null() {
        return -ENOMEM;
    }

    (*xattr_data).typ = EVM_XATTR_HMAC;
    rc = evm_init_hmac(inode, xattrs, &mut (*xattr_data).data);
    if rc < 0 {
        kfree(xattr_data as *mut u8);
        return rc;
    }

    (*evm_xattr).value = xattr_data as *mut u8;
    (*evm_xattr).value_len = mem::size_of::<EvmXattr>();
    (*evm_xattr).name = b"security.evm\0".as_ptr() as *const i8;
    0
}

unsafe fn evm_inode_alloc_security(inode: *const Inode) -> i32 {
    let iint = evm_iint_inode(inode);

    if !iint.is_null() {
        (*iint).flags = 0;
        (*iint).evm_status = INTEGRITY_UNKNOWN;
    }

    0
}

unsafe fn evm_file_release(file: *const File) {
    let inode = file_inode(file);
    let iint = evm_iint_inode(inode);
    let mode = file_f_mode(file);

    if !inode_is_regular_file(inode) || (mode & FMODE_WRITE) == 0 {
        return;
    }

    if !iint.is_null()
        && ((*iint).flags & EVM_NEW_FILE) != 0
        && atomic_read_i_writecount(inode) == 1
    {
        (*iint).flags &= !EVM_NEW_FILE;
    }
}

unsafe fn evm_post_path_mknod(idmap: *const MntIdmap, dentry: *const Dentry) {
    let inode = d_backing_inode(dentry);
    let iint = evm_iint_inode(inode);

    if !inode_is_regular_file(inode) {
        return;
    }

    if !iint.is_null() {
        (*iint).flags |= EVM_NEW_FILE;
    }
}

// CONFIG_EVM_LOAD_X509 conditional
unsafe fn evm_load_x509() {
    let rc = integrity_load_x509(INTEGRITY_KEYRING_EVM, b"/lib/firmware/evm.x509.der\0".as_ptr() as *const i8);
    if rc == 0 {
        EVM_INITIALIZED |= EVM_INIT_X509;
    }
}

unsafe fn init_evm() -> i32 {
    evm_init_config();
    evm_set_fixmode();

    let error = integrity_init_keyring(INTEGRITY_KEYRING_EVM);
    if error != 0 {
        return error;
    }

    let error = evm_init_secfs();
    if error < 0 {
        pr_info("Error registering secfs\n");
        return error;
    }

    0
}

unsafe fn init_evm_lsm() -> i32 {
    // security_add_hooks(evm_hooks, ARRAY_SIZE(evm_hooks), &evm_lsmid)
    // This would register the hooks
    0
}

// External function declarations (dependencies)
extern "C" {
    // Kernel functions that need to be provided
    fn d_backing_inode(dentry: *const Dentry) -> *const Inode;
    fn d_backing_inode_mut(dentry: *const Dentry) -> *mut Inode;
    fn __vfs_getxattr(
        dentry: *const Dentry,
        inode: *const Inode,
        name: *const i8,
        value: *mut i8,
        size: usize,
    ) -> i32;
    fn vfs_getxattr_alloc(
        idmap: *const MntIdmap,
        dentry: *const Dentry,
        name: *const i8,
        xattr_value: *mut *mut i8,
        xattr_size: usize,
        gfp: i32,
    ) -> i32;
    fn kfree(ptr: *mut u8);
    fn capable(cap: i32) -> bool;
    fn posix_xattr_acl(name: *const i8) -> bool;
    fn evm_calc_hmac(
        dentry: *const Dentry,
        xattr_name: *const i8,
        xattr_value: *mut i8,
        xattr_value_len: usize,
        digest: *mut EvmDigest,
        iint: *mut EvmIintCache,
    ) -> i32;
    fn crypto_memneq(a: *const u8, b: *const u8, len: usize) -> i32;
    fn evm_calc_hash(
        dentry: *const Dentry,
        xattr_name: *const i8,
        xattr_value: *mut i8,
        xattr_value_len: usize,
        typ: u8,
        digest: *mut EvmDigest,
        iint: *mut EvmIintCache,
    ) -> i32;
    fn integrity_digsig_verify(
        keyring: i32,
        sig: *const i8,
        sig_len: usize,
        digest: *const u8,
        digest_len: usize,
        hash_algo: u8,
    ) -> i32;
    fn evm_update_evmxattr(
        dentry: *const Dentry,
        xattr_name: *const i8,
        xattr_value: *const u8,
        xattr_value_len: usize,
    ) -> i32;
    fn evm_iint_inode(inode: *const Inode) -> *mut EvmIintCache;
    fn integrity_audit_msg(
        audit_type: i32,
        inode: *const Inode,
        name: *const i8,
        op: *const i8,
        cause: *const i8,
        result: i32,
        info: i32,
    );
    fn pr_info(fmt: *const i8, args: ...);
    fn pr_info_once(fmt: *const i8, args: ...);
    fn pr_err(fmt: *const i8, args: ...);
    fn pr_debug(fmt: *const i8, args: ...);
    fn arch_get_secureboot() -> i32;
    fn inode_supports_xattr(inode: *const Inode) -> bool;
    fn sb_i_evm_hmac_unsupported(inode: *const Inode) -> i32;
    fn inode_sb_name(inode: *const Inode) -> *const i8;
    fn inode_is_readonly(inode: *const Inode) -> bool;
    fn inode_sb_readonly(inode: *const Inode) -> bool;
    fn inode_is_immutable(inode: *const Inode) -> bool;
    fn inode_is_regular_file(inode: *const Inode) -> bool;
    fn inode_i_mode(inode: *const Inode) -> u32;
    fn inode_sb_magic(inode: *const Inode) -> u32;
    fn dentry_name(dentry: *const Dentry) -> *const i8;
    fn posix_acl_update_mode(
        idmap: *const MntIdmap,
        inode: *const Inode,
        mode: *mut u32,
        acl: *mut *mut PosixAcl,
    ) -> i32;
    fn i_uid_needs_update(
        idmap: *const MntIdmap,
        attr: *const Iattr,
        inode: *const Inode,
    ) -> bool;
    fn i_gid_needs_update(
        idmap: *const MntIdmap,
        attr: *const Iattr,
        inode: *const Inode,
    ) -> bool;
    fn inode_is_i_version(inode: *const Inode) -> bool;
    fn integrity_inode_attrs_changed(
        iint_inode: *const Inode,
        metadata_inode: *const Inode,
    ) -> i32;
    fn lsm_get_xattr_slot(xattrs: *mut Xattr, xattr_count: *mut i32) -> *mut Xattr;
    fn kzalloc_obj(size: usize, gfp: i32) -> *mut u8;
    fn evm_init_hmac(inode: *const Inode, xattrs: *mut Xattr, digest: *mut u8) -> i32;
    fn file_inode(file: *const File) -> *const Inode;
    fn file_f_mode(file: *const File) -> u32;
    fn atomic_read_i_writecount(inode: *const Inode) -> i32;
    fn integrity_load_x509(keyring: i32, path: *const i8) -> i32;
    fn integrity_init_keyring(keyring: i32) -> i32;
    fn evm_init_secfs() -> i32;
}

static mut NOP_MNT_IDMAP: MntIdmap = MntIdmap {
    _private: [0; 0],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
