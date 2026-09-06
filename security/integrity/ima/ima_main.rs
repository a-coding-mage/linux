// SPDX-License-Identifier: GPL-2.0-only
/*
 * Integrity Measurement Architecture
 *
 * Copyright (C) 2005,2006,2007,2008 IBM Corporation
 *
 * Authors:
 * Reiner Sailer <sailer@watson.ibm.com>
 * Serge Hallyn <serue@us.ibm.com>
 * Kylene Hall <kylene@us.ibm.com>
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_main.c
 *	implements the IMA hooks: ima_bprm_check, ima_file_mmap,
 *	and ima_file_check.
 */

// Linux kernel headers:
// #include <linux/module.h>
// #include <linux/file.h>
// #include <linux/binfmts.h>
// #include <linux/kernel_read_file.h>
// #include <linux/mount.h>
// #include <linux/mman.h>
// #include <linux/slab.h>
// #include <linux/xattr.h>
// #include <linux/ima.h>
// #include <linux/fs.h>
// #include <linux/iversion.h>
// #include <linux/evm.h>
// #include <linux/crash_dump.h>
// #include "ima.h"

use core::mem;
use core::ptr;
use core::ffi::c_char;

// External type declarations - from linux kernel headers and ima.h
#[repr(C)]
pub struct file {
    // opaque
}

#[repr(C)]
pub struct cred {
    // opaque
}

#[repr(C)]
pub struct lsm_prop {
    // opaque
}

#[repr(C)]
pub struct inode {
    // opaque
}

#[repr(C)]
pub struct ima_iint_cache {
    // opaque
}

#[repr(C)]
pub struct ima_template_desc {
    // opaque
}

#[repr(C)]
pub struct evm_ima_xattr_data {
    // opaque
}

#[repr(C)]
pub struct modsig {
    // opaque
}

#[repr(C)]
pub struct kstat {
    // opaque
}

#[repr(C)]
pub struct linux_binprm {
    // opaque
}

#[repr(C)]
pub struct vm_area_struct {
    // opaque
}

#[repr(C)]
pub struct path {
    // opaque
}

#[repr(C)]
pub struct dentry {
    // opaque
}

#[repr(C)]
pub struct mnt_idmap {
    // opaque
}

#[repr(C)]
pub struct ima_event_data {
    // opaque
}

#[repr(C)]
pub struct ima_template_entry {
    // opaque
}

#[repr(C)]
pub struct ima_max_digest_data {
    hdr: ima_digest_data,
}

#[repr(C)]
pub struct ima_digest_data {
    // opaque
}

#[repr(C)]
pub struct security_hook_list {
    // opaque
}

#[repr(C)]
pub struct lsm_id {
    name: *const c_char,
    id: i32,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    lbs_inode: usize,
}

// External constants and enums
pub const CONFIG_IMA_MEASURE_PCR_IDX: i32 = 0;
pub const CONFIG_IMA_DEFAULT_HASH: &str = "sha1";
pub const CONFIG_IMA_INIT_LATE_SYNC: bool = false;
pub const CONFIG_KEXEC_SIG: bool = false;
pub const CONFIG_IMA_APPRAISE_ENFORCE: i32 = 0;
pub const CONFIG_IMA_APPRAISE: bool = true;

pub const IMA_APPRAISE_ENFORCE: i32 = 0;
pub const IMA_TEMPLATE_IMA_NAME: &str = "ima";

pub const HASH_ALGO_SHA1: i32 = 0;
pub const HASH_ALGO_MD5: i32 = 1;
pub const HASH_ALGO__LAST: i32 = 10;

pub const FILE_CHECK: i32 = 0;
pub const MMAP_CHECK: i32 = 1;
pub const MMAP_CHECK_REQPROT: i32 = 2;
pub const BPRM_CHECK: i32 = 3;
pub const CREDS_CHECK: i32 = 4;
pub const KEXEC_CMDLINE: i32 = 5;
pub const POLICY_CHECK: i32 = 6;
pub const FIRMWARE_CHECK: i32 = 7;
pub const MODULE_CHECK: i32 = 8;
pub const KEXEC_KERNEL_CHECK: i32 = 9;
pub const KEXEC_INITRAMFS_CHECK: i32 = 10;
pub const CRITICAL_DATA: i32 = 11;

pub const IMA_MEASURE: i32 = 0x0001;
pub const IMA_MEASURED: i32 = 0x0002;
pub const IMA_APPRAISE: i32 = 0x0004;
pub const IMA_APPRAISED: i32 = 0x0008;
pub const IMA_AUDIT: i32 = 0x0010;
pub const IMA_AUDITED: i32 = 0x0020;
pub const IMA_HASH: i32 = 0x0040;
pub const IMA_HASHED: i32 = 0x0080;
pub const IMA_FILE_APPRAISE: i32 = 0x0100;
pub const IMA_APPRAISE_SUBMASK: i32 = 0x0200;
pub const IMA_APPRAISED_SUBMASK: i32 = 0x0400;
pub const IMA_NONACTION_RULE_FLAGS: i32 = 0x0800;
pub const IMA_DONE_MASK: i32 = 0x1000;
pub const IMA_DO_MASK: i32 = 0x2000;
pub const IMA_FAIL_UNVERIFIABLE_SIGS: i32 = 0x4000;
pub const IMA_DIGSIG: i32 = 0x8000;
pub const IMA_UPDATE_XATTR: i32 = 0x10000;
pub const IMA_NEW_FILE: i32 = 0x20000;
pub const IMA_PERMIT_DIRECTIO: i32 = 0x40000;
pub const IMA_MAY_EMIT_TOMTOU: i32 = 0x80000;
pub const IMA_EMITTED_OPENWRITERS: i32 = 0x100000;
pub const IMA_CHANGE_ATTR: i32 = 0x200000;
pub const IMA_CHANGE_XATTR: i32 = 0x400000;
pub const IMA_MODSIG_ALLOWED: i32 = 0x800000;

pub const MAY_EXEC: i32 = 0x01;
pub const MAY_WRITE: i32 = 0x02;
pub const MAY_READ: i32 = 0x04;
pub const MAY_APPEND: i32 = 0x08;
pub const MAY_ACCESS: i32 = 0x10;

pub const FMODE_WRITE: u32 = 0x02;
pub const FMODE_READ: u32 = 0x01;

pub const O_DIRECT: i32 = 0x04000;

pub const ETXTBSY: i32 = -26;
pub const ENOMEM: i32 = -12;
pub const EBADF: i32 = -9;
pub const EINVAL: i32 = -22;
pub const EOPNOTSUPP: i32 = -95;
pub const ENOENT: i32 = -2;
pub const EACCES: i32 = -13;
pub const EPERM: i32 = -1;
pub const ENOPARAM: i32 = -68;

pub const STATX_CHANGE_COOKIE: u32 = 0x00000040;
pub const AT_STATX_SYNC_AS_STAT: u32 = 0;

pub const EVM_IMA_XATTR_DIGSIG: u32 = 3;

pub const READING_FIRMWARE: i32 = 0;
pub const READING_MODULE: i32 = 1;
pub const READING_MODULE_COMPRESSED: i32 = 2;
pub const READING_KEXEC_IMAGE: i32 = 3;
pub const READING_KEXEC_INITRAMFS: i32 = 4;
pub const READING_POLICY: i32 = 5;
pub const READING_X509_CERTIFICATE: i32 = 6;
pub const READING_MAX_ID: i32 = 7;

pub const LOADING_KEXEC_IMAGE: i32 = 0;
pub const LOADING_FIRMWARE: i32 = 1;
pub const LOADING_MODULE: i32 = 2;

pub const SB_I_IMA_UNVERIFIABLE_SIGNATURE: u32 = 0x01;
pub const SB_I_UNTRUSTED_MOUNTER: u32 = 0x02;

pub const NAME_MAX: usize = 255;
pub const IMA_MAX_DIGEST_SIZE: usize = 64;

pub const AUDIT_INTEGRITY_DATA: i32 = 5401;
pub const AUDIT_INTEGRITY_PCR: i32 = 5402;

pub const LSM_ID_IMA: i32 = 0;
pub const LSM_ORDER_LAST: i32 = -1;

// Global variables
#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
pub static mut ima_appraise: i32 = 0;

#[cfg(feature = "CONFIG_IMA_APPRAISE")]
pub static mut ima_appraise: i32 = IMA_APPRAISE_ENFORCE;

pub static mut ima_hash_algo: i32 = HASH_ALGO_SHA1;
static mut hash_setup_done: i32 = 0;
static mut ima_disabled: i32 = 0;

static mut ima_lsm_policy_notifier: notifier_block = notifier_block {
    notifier_call: ima_lsm_policy_change,
};

#[repr(C)]
pub struct notifier_block {
    notifier_call: extern "C" fn() -> i32,
}

// External function declarations
extern "C" {
    pub fn is_kdump_kernel() -> bool;
    pub fn pr_info(fmt: *const c_char, ...);
    pub fn pr_err(fmt: *const c_char, ...);
    pub fn pr_warn(fmt: *const c_char, ...);
    pub fn is_module_sig_enforced() -> bool;
    pub fn arch_get_secureboot() -> bool;

    pub fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn kfree(ptr: *mut u8);
    pub fn __putname(name: *mut c_char);

    pub fn file_inode(file: *const file) -> *mut inode;
    pub fn file_mnt_idmap(file: *const file) -> *const mnt_idmap;
    pub fn file_dentry(file: *const file) -> *mut dentry;
    pub fn current_cred() -> *const cred;
    pub fn d_inode(dentry: *const dentry) -> *mut inode;
    pub fn d_real(dentry: *const dentry, want_fallback: *mut dentry) -> *mut dentry;
    pub fn d_real_inode(dentry: *const dentry) -> *mut inode;

    pub fn mapping_writably_mapped(mapping: *const libc::c_void) -> bool;
    pub fn inode_lock(inode: *mut inode);
    pub fn inode_unlock(inode: *mut inode);
    pub fn atomic_read(v: *const i32) -> i32;
    pub fn inode_is_open_for_write(inode: *const inode) -> bool;
    pub fn test_and_clear_bit(nr: i32, addr: *mut i32) -> bool;
    pub fn test_and_set_bit(nr: i32, addr: *mut i32) -> bool;
    pub fn set_bit(nr: i32, addr: *mut i32);
    pub fn clear_bit(nr: i32, addr: *mut i32);
    pub fn test_bit(nr: i32, addr: *const i32) -> bool;
    pub fn inode_eq_iversion(inode: *const inode, version: i32) -> bool;

    pub fn ima_iint_find(inode: *const inode) -> *mut ima_iint_cache;
    pub fn ima_inode_get(inode: *mut inode) -> *mut ima_iint_cache;
    pub fn ima_template_desc_current() -> *mut ima_template_desc;
    pub fn ima_template_desc_buf() -> *mut ima_template_desc;
    pub fn ima_get_action(
        idmap: *const mnt_idmap,
        inode: *mut inode,
        cred: *const cred,
        prop: *const lsm_prop,
        mask: i32,
        func: i32,
        pcr: *mut i32,
        template_desc: *mut *mut ima_template_desc,
        func_data: *const c_char,
        allowed_algos: *mut u32,
    ) -> i32;
    pub fn ima_d_path(
        path: *const path,
        pathbuf: *mut *mut c_char,
        filename: *mut c_char,
    ) -> *const c_char;
    pub fn ima_rdwr_violation_check(
        file: *const file,
        iint: *mut ima_iint_cache,
        must_measure: i32,
        pathbuf: *mut *mut c_char,
        pathname: *mut *const c_char,
        filename: *mut c_char,
    );
    pub fn ima_add_violation(
        file: *const file,
        pathname: *const c_char,
        iint: *mut ima_iint_cache,
        op: *const c_char,
        cause: *const c_char,
    );
    pub fn ima_detect_file_change(
        iint: *mut ima_iint_cache,
        inode: *const inode,
        file: *const file,
    ) -> bool;
    pub fn ima_update_xattr(iint: *mut ima_iint_cache, file: *const file);
    pub fn ima_collect_measurement(
        iint: *mut ima_iint_cache,
        file: *const file,
        buf: *const u8,
        size: i32,
        hash_algo: i32,
        modsig: *mut modsig,
    ) -> i32;
    pub fn ima_store_measurement(
        iint: *mut ima_iint_cache,
        file: *const file,
        pathname: *const c_char,
        xattr_value: *const evm_ima_xattr_data,
        xattr_len: i32,
        modsig: *mut modsig,
        pcr: i32,
        template_desc: *mut ima_template_desc,
    );
    pub fn ima_read_xattr(
        dentry: *mut dentry,
        xattr_value: *mut *mut evm_ima_xattr_data,
        xattr_len: i32,
    ) -> i32;
    pub fn ima_read_modsig(
        func: i32,
        buf: *const u8,
        size: i32,
        modsig: *mut *mut modsig,
    ) -> i32;
    pub fn ima_free_modsig(modsig: *mut modsig);
    pub fn ima_template_has_modsig(template_desc: *mut ima_template_desc) -> bool;
    pub fn ima_get_hash_algo(
        xattr_value: *const evm_ima_xattr_data,
        xattr_len: i32,
    ) -> i32;
    pub fn ima_check_blacklist(iint: *mut ima_iint_cache, modsig: *mut modsig, pcr: i32) -> i32;
    pub fn ima_appraise_measurement(
        func: i32,
        iint: *mut ima_iint_cache,
        file: *const file,
        pathname: *const c_char,
        xattr_value: *const evm_ima_xattr_data,
        xattr_len: i32,
        modsig: *mut modsig,
        bprm_is_check: bool,
    ) -> i32;
    pub fn ima_audit_measurement(iint: *mut ima_iint_cache, pathname: *const c_char);
    pub fn ima_get_cache_status(iint: *mut ima_iint_cache, func: i32) -> i32;
    pub fn integrity_audit_msg(
        audit_type: i32,
        inode: *const inode,
        pathname: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        rc: i32,
        unused: i32,
    );
    pub fn integrity_audit_message(
        audit_type: i32,
        inode: *const inode,
        pathname: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        rc: i32,
        unused: i32,
        ret: i32,
    );
    pub fn integrity_inode_attrs_changed(real_inode: *const inode, inode: *const inode) -> bool;
    pub fn evm_metadata_changed(inode: *const inode, metadata_inode: *const inode) -> bool;
    pub fn vfs_getattr_nosec(
        path: *const path,
        stat: *mut kstat,
        request_mask: u32,
        query_flags: u32,
    ) -> i32;
    pub fn ima_appraise_parse_cmdline();
    pub fn ima_init_template_list();
    pub fn ima_init() -> i32;
    pub fn register_blocking_lsm_notifier(nb: *mut notifier_block) -> i32;
    pub fn ima_update_policy_flags();
    pub fn ima_iintcache_init();
    pub fn init_ima_appraise_lsm(lsmid: *const lsm_id);
    pub fn security_add_hooks(
        hooks: *const security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    pub fn security_current_getlsmprop_subj(prop: *mut lsm_prop);
    pub fn match_string(
        array: *const *const c_char,
        n: i32,
        string: *const c_char,
    ) -> i32;
    pub fn ima_inode_free_rcu(inode: *mut inode);
    pub fn ima_post_key_create_or_update();
    pub fn ima_lsm_policy_change() -> i32;
    pub fn ima_calc_buffer_hash(
        buf: *const u8,
        size: usize,
        hash: *mut ima_digest_data,
    ) -> i32;
    pub fn ima_alloc_init_template(
        event_data: *const ima_event_data,
        entry: *mut *mut ima_template_entry,
        template: *mut ima_template_desc,
    ) -> i32;
    pub fn ima_store_template(
        entry: *mut ima_template_entry,
        violation: i32,
        inode: *const inode,
        buf: *const u8,
        pcr: i32,
    ) -> i32;
    pub fn ima_free_template_entry(entry: *mut ima_template_entry);
    pub fn ima_measure_critical_data(
        event_label: *const c_char,
        event_name: *const c_char,
        buf: *const u8,
        size: usize,
        hash: bool,
        digest: *mut u8,
        digest_len: usize,
    ) -> i32;
    pub fn func_measure_str(func: i32) -> *const c_char;

    pub static hash_algo_name: *const *const c_char;
    pub static hash_digest_size: *const usize;
    pub static nop_mnt_idmap: mnt_idmap;

    pub fn IS_ENABLED(config: i32) -> bool;
    pub fn IS_I_VERSION(inode: *const inode) -> bool;
    pub fn IS_IMA(inode: *const inode) -> bool;
    pub fn S_ISREG(mode: u32) -> bool;

    pub fn container_of(ptr: *const u8, type_: usize, member_: usize) -> *const u8;
    pub fn min_t(type_: usize, a: usize, b: usize) -> usize;
    pub fn fd_empty(f: *const libc::c_void) -> bool;
    pub fn fd_file(f: *const libc::c_void) -> *const file;
    pub fn CLASS(typ: i32, name: i32)(value: i32);

    pub fn mutex_init(mutex: *mut libc::c_void);
    pub fn mutex_lock(mutex: *mut libc::c_void);
    pub fn mutex_unlock(mutex: *mut libc::c_void);
}

pub const fn LSM_HOOK_INIT(hook: i32, function: extern "C" fn() -> i32) -> security_hook_list {
    security_hook_list {
        // opaque initialization
    }
}

pub const fn DEFINE_LSM(name: &str) {
    // macro-level initialization
}

pub const fn ARRAY_SIZE<T>(arr: &[T]) -> usize {
    arr.len()
}

pub const fn EXPORT_SYMBOL_GPL(sym: &str) {
    // macro-level export
}

pub const fn __setup(str: &str, func: extern "C" fn(*const c_char) -> i32) {
    // macro-level setup hook
}

// Setup functions
extern "C" fn ima_setup(str: *const c_char) -> i32 {
    unsafe {
        if !is_kdump_kernel() {
            pr_info(b"Warning: ima setup option only permitted in kdump\0" as *const u8 as *const c_char);
            return 1;
        }

        if strncmp(str, b"off\0" as *const u8 as *const c_char, 3) == 0 {
            ima_disabled = 1;
        } else if strncmp(str, b"on\0" as *const u8 as *const c_char, 2) == 0 {
            ima_disabled = 0;
        } else {
            pr_err(b"Invalid ima setup option: \"%s\" , please specify ima=on|off.\0" as *const u8 as *const c_char, str);
        }

        1
    }
}

extern "C" fn hash_setup(str: *const c_char) -> i32 {
    unsafe {
        let template_desc = ima_template_desc_current();

        if hash_setup_done != 0 {
            return 1;
        }

        if strcmp((*template_desc).name, b"ima\0" as *const u8 as *const c_char) == 0 {
            if strncmp(str, b"sha1\0" as *const u8 as *const c_char, 4) == 0 {
                ima_hash_algo = HASH_ALGO_SHA1;
            } else if strncmp(str, b"md5\0" as *const u8 as *const c_char, 3) == 0 {
                ima_hash_algo = HASH_ALGO_MD5;
            } else {
                pr_err(b"invalid hash algorithm \"%s\" for template \"%s\"\0" as *const u8 as *const c_char, str, b"ima\0" as *const u8 as *const c_char);
                return 1;
            }
            goto_out();
        }

        let i = match_string(hash_algo_name, HASH_ALGO__LAST, str);
        if i < 0 {
            pr_err(b"invalid hash algorithm \"%s\"\0" as *const u8 as *const c_char, str);
            return 1;
        }

        ima_hash_algo = i;
        goto_out();

        hash_setup_done = 1;
        1
    }
}

fn goto_out() {}

pub extern "C" fn ima_get_current_hash_algo() -> i32 {
    unsafe { ima_hash_algo }
}

extern "C" fn mmap_violation_check(
    func: i32,
    file: *const file,
    pathbuf: *mut *mut c_char,
    pathname: *mut *const c_char,
    filename: *mut c_char,
) -> i32 {
    unsafe {
        let mut rc = 0;

        if (func == MMAP_CHECK || func == MMAP_CHECK_REQPROT) &&
            mapping_writably_mapped((*file).f_mapping as *const libc::c_void) {
            rc = ETXTBSY;
            let inode = file_inode(file);

            if *pathbuf.is_null() {
                *pathname = ima_d_path(&(*file).f_path, pathbuf, filename);
            }
            integrity_audit_msg(AUDIT_INTEGRITY_DATA, inode, *pathname,
                    b"mmap_file\0" as *const u8 as *const c_char,
                    b"mmapped_writers\0" as *const u8 as *const c_char, rc, 0);
        }
        rc
    }
}

extern "C" fn ima_rdwr_violation_check(
    file: *const file,
    iint: *mut ima_iint_cache,
    must_measure: i32,
    pathbuf: *mut *mut c_char,
    pathname: *mut *const c_char,
    filename: *mut c_char,
) {
    unsafe {
        let inode = file_inode(file);
        let mode = (*file).f_mode;
        let mut send_tomtou = false;
        let mut send_writers = false;

        if mode & FMODE_WRITE != 0 {
            if atomic_read(&(*inode).i_readcount) != 0 && IS_IMA(inode) {
                let mut iint_local = iint;
                if iint_local.is_null() {
                    iint_local = ima_iint_find(inode);
                }

                if !iint_local.is_null() && test_and_clear_bit(
                    IMA_MAY_EMIT_TOMTOU as i32,
                    &mut (*iint_local).atomic_flags,
                ) {
                    send_tomtou = true;
                }
            }
        } else {
            if must_measure != 0 {
                set_bit(IMA_MAY_EMIT_TOMTOU as i32, &mut (*iint).atomic_flags);
            }

            if inode_is_open_for_write(inode) && must_measure != 0 {
                if !test_and_set_bit(
                    IMA_EMITTED_OPENWRITERS as i32,
                    &mut (*iint).atomic_flags,
                ) {
                    send_writers = true;
                }
            }
        }

        if !send_tomtou && !send_writers {
            return;
        }

        *pathname = ima_d_path(&(*file).f_path, pathbuf, filename);

        if send_tomtou {
            ima_add_violation(
                file,
                *pathname,
                iint,
                b"invalid_pcr\0" as *const u8 as *const c_char,
                b"ToMToU\0" as *const u8 as *const c_char,
            );
        }
        if send_writers {
            ima_add_violation(
                file,
                *pathname,
                iint,
                b"invalid_pcr\0" as *const u8 as *const c_char,
                b"open_writers\0" as *const u8 as *const c_char,
            );
        }
    }
}

extern "C" fn ima_detect_file_change_wrapper(
    iint: *mut ima_iint_cache,
    inode: *const inode,
    file: *const file,
) -> bool {
    unsafe {
        let mut stat: kstat = mem::zeroed();
        let result = vfs_getattr_nosec(
            &(*file).f_path,
            &mut stat,
            STATX_CHANGE_COOKIE,
            AT_STATX_SYNC_AS_STAT,
        );

        if result == 0 && (stat.result_mask & STATX_CHANGE_COOKIE) != 0 {
            return stat.change_cookie != (*iint).real_inode.version;
        }

        if IS_I_VERSION(inode) {
            return !inode_eq_iversion(inode, (*iint).real_inode.version);
        }

        true
    }
}

extern "C" fn ima_check_last_writer(
    iint: *mut ima_iint_cache,
    inode: *const inode,
    file: *const file,
) {
    unsafe {
        let mode = (*file).f_mode;

        if (mode & FMODE_WRITE) == 0 {
            return;
        }

        mutex_lock(&mut (*iint).mutex as *mut libc::c_void);
        if atomic_read(&(*inode).i_writecount) == 1 {
            clear_bit(IMA_EMITTED_OPENWRITERS as i32, &mut (*iint).atomic_flags);

            let update = test_and_clear_bit(IMA_UPDATE_XATTR as i32, &mut (*iint).atomic_flags);

            if ((*iint).flags & IMA_NEW_FILE) != 0 || ima_detect_file_change(iint, inode, file) {
                (*iint).flags &= !(IMA_DONE_MASK | IMA_NEW_FILE);
                (*iint).measured_pcrs = 0;
                if update {
                    ima_update_xattr(iint, file);
                }
            }
        }
        mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);
    }
}

extern "C" fn ima_file_free(file: *const file) {
    unsafe {
        let inode = file_inode(file);

        if ima_policy_flag == 0 || !S_ISREG((*inode).i_mode) {
            return;
        }

        let iint = ima_iint_find(inode);
        if iint.is_null() {
            return;
        }

        ima_check_last_writer(iint, inode, file);
    }
}

extern "C" fn process_measurement(
    file: *const file,
    cred: *const cred,
    prop: *const lsm_prop,
    buf: *const u8,
    size: i32,
    mask: i32,
    func: i32,
    read_id: i32,
    bprm_is_check: bool,
) -> i32 {
    unsafe {
        let inode = file_inode(file);
        let mut iint: *mut ima_iint_cache = ptr::null_mut();
        let mut template_desc: *mut ima_template_desc = ptr::null_mut();
        let mut pathbuf: *mut c_char = ptr::null_mut();
        let mut filename: [c_char; NAME_MAX] = [0; NAME_MAX];
        let mut pathname: *const c_char = ptr::null();
        let mut rc = 0;
        let mut action = 0;
        let mut must_appraise = 0;
        let mut pcr = CONFIG_IMA_MEASURE_PCR_IDX;
        let mut xattr_value: *mut evm_ima_xattr_data = ptr::null_mut();
        let mut modsig: *mut modsig = ptr::null_mut();
        let mut xattr_len = 0;
        let mut allowed_algos: u32 = 0;

        if ima_policy_flag == 0 || !S_ISREG((*inode).i_mode) {
            return 0;
        }

        action = ima_get_action(
            file_mnt_idmap(file),
            inode as *mut inode,
            cred,
            prop,
            mask,
            func,
            &mut pcr,
            &mut template_desc,
            ptr::null(),
            &mut allowed_algos,
        );

        let violation_check = ((func == FILE_CHECK || func == MMAP_CHECK || func == MMAP_CHECK_REQPROT)
            && (ima_policy_flag & IMA_MEASURE) != 0
            && ((action & IMA_MEASURE) != 0 || ((*file).f_mode & FMODE_WRITE) != 0));

        if action == 0 && !violation_check {
            return 0;
        }

        must_appraise = action & IMA_APPRAISE;

        if (action & IMA_FILE_APPRAISE) != 0 {
            func_local = FILE_CHECK;
        }

        inode_lock(inode as *mut inode);

        if action != 0 {
            iint = ima_inode_get(inode as *mut inode);
            if iint.is_null() {
                rc = ENOMEM;
            }
        }

        if rc == 0 && violation_check {
            ima_rdwr_violation_check(
                file,
                iint,
                action & IMA_MEASURE,
                &mut pathbuf,
                &mut pathname,
                filename.as_mut_ptr(),
            );
        }

        inode_unlock(inode as *mut inode);

        if rc != 0 {
            goto out_label;
        }
        if action == 0 {
            goto out_label;
        }

        mutex_lock(&mut (*iint).mutex as *mut libc::c_void);

        if test_and_clear_bit(IMA_CHANGE_ATTR as i32, &mut (*iint).atomic_flags) {
            (*iint).flags &= !(IMA_APPRAISE | IMA_APPRAISED | IMA_APPRAISE_SUBMASK
                | IMA_APPRAISED_SUBMASK | IMA_NONACTION_RULE_FLAGS);
        }

        if test_and_clear_bit(IMA_CHANGE_XATTR as i32, &mut (*iint).atomic_flags)
            || (((*(*inode).i_sb).s_iflags & SB_I_IMA_UNVERIFIABLE_SIGNATURE) != 0
                && ((*(*inode).i_sb).s_iflags & SB_I_UNTRUSTED_MOUNTER) == 0
                && (action & IMA_FAIL_UNVERIFIABLE_SIGS) == 0) {
            (*iint).flags &= !IMA_DONE_MASK;
            (*iint).measured_pcrs = 0;
        }

        let real_inode = d_real_inode(file_dentry(file));
        if real_inode != inode as *mut inode && (action & IMA_DO_MASK) != 0
            && ((*iint).flags & IMA_DONE_MASK) != 0 {
            if !IS_I_VERSION(real_inode)
                || integrity_inode_attrs_changed(&(*iint).real_inode, real_inode) {
                (*iint).flags &= !IMA_DONE_MASK;
                (*iint).measured_pcrs = 0;
            }

            let metadata_inode = d_inode(d_real(file_dentry(file),
                ptr::null_mut::<dentry>() as *mut dentry));
            if evm_metadata_changed(inode, metadata_inode) {
                (*iint).flags &= !(IMA_APPRAISED | IMA_APPRAISED_SUBMASK);
            }
        }

        (*iint).flags |= action;
        action &= IMA_DO_MASK;
        action &= !(((*iint).flags & (IMA_DONE_MASK ^ IMA_MEASURED)) >> 1);

        if (action & IMA_MEASURE) != 0 && ((*iint).measured_pcrs & (0x1 << pcr)) != 0 {
            action ^= IMA_MEASURE;
        }

        if (action & IMA_HASH) != 0 && !test_bit(IMA_DIGSIG as i32, &(*iint).atomic_flags) {
            xattr_len =
                ima_read_xattr(file_dentry(file), &mut xattr_value, xattr_len);
            if !xattr_value.is_null() && xattr_len > 2 && (*xattr_value).type_ == EVM_IMA_XATTR_DIGSIG {
                set_bit(IMA_DIGSIG as i32, &mut (*iint).atomic_flags);
            }
            (*iint).flags |= IMA_HASHED;
            action ^= IMA_HASH;
            set_bit(IMA_UPDATE_XATTR as i32, &mut (*iint).atomic_flags);
        }

        if action == 0 {
            if must_appraise != 0 {
                rc = mmap_violation_check(
                    func_local,
                    file,
                    &mut pathbuf,
                    &mut pathname,
                    filename.as_mut_ptr(),
                );
                if rc == 0 {
                    rc = ima_get_cache_status(iint, func_local);
                }
            }
            goto out_locked_label;
        }

        if (action & IMA_APPRAISE_SUBMASK) != 0
            || strcmp((*template_desc).name, b"ima\0" as *const u8 as *const c_char) != 0 {
            xattr_len = ima_read_xattr(file_dentry(file), &mut xattr_value, xattr_len);

            if ((*iint).flags & IMA_MODSIG_ALLOWED) != 0 {
                rc = ima_read_modsig(func_local, buf, size, &mut modsig);

                if rc == 0 && ima_template_has_modsig(template_desc)
                    && ((*iint).flags & IMA_MEASURED) != 0 {
                    action |= IMA_MEASURE;
                }
            }
        }

        let hash_algo = ima_get_hash_algo(xattr_value, xattr_len);

        rc = ima_collect_measurement(iint, file, buf, size, hash_algo, modsig);
        if rc != 0 && rc != EBADF && rc != EINVAL {
            goto out_locked_label;
        }

        if read_id == READING_MODULE_COMPRESSED {
            must_appraise = 0;
            goto out_locked_label;
        }

        if pathbuf.is_null() {
            pathname = ima_d_path(&(*file).f_path, &mut pathbuf, filename.as_mut_ptr());
        }

        if (action & IMA_MEASURE) != 0 {
            ima_store_measurement(
                iint,
                file,
                pathname,
                xattr_value,
                xattr_len,
                modsig,
                pcr,
                template_desc,
            );
        }
        if rc == 0 && (action & IMA_APPRAISE_SUBMASK) != 0 {
            rc = ima_check_blacklist(iint, modsig, pcr);
            if rc != EPERM {
                inode_lock(inode as *mut inode);
                rc = ima_appraise_measurement(
                    func_local,
                    iint,
                    file,
                    pathname,
                    xattr_value,
                    xattr_len,
                    modsig,
                    bprm_is_check,
                );
                inode_unlock(inode as *mut inode);
            }
            if rc == 0 {
                rc = mmap_violation_check(
                    func_local,
                    file,
                    &mut pathbuf,
                    &mut pathname,
                    filename.as_mut_ptr(),
                );
            }
        }
        if (action & IMA_AUDIT) != 0 {
            ima_audit_measurement(iint, pathname);
        }

        if ((*file).f_flags & O_DIRECT) != 0 && ((*iint).flags & IMA_PERMIT_DIRECTIO) != 0 {
            rc = 0;
        }

        if rc == 0 && must_appraise != 0 && allowed_algos != 0
            && (allowed_algos & (1u32 << hash_algo)) == 0 {
            rc = EACCES;

            integrity_audit_msg(
                AUDIT_INTEGRITY_DATA,
                file_inode(file),
                pathname,
                b"collect_data\0" as *const u8 as *const c_char,
                b"denied-hash-algorithm\0" as *const u8 as *const c_char,
                rc,
                0,
            );
        }

        out_locked_label: if (mask & MAY_WRITE) != 0 && test_bit(IMA_DIGSIG as i32, &(*iint).atomic_flags)
            && ((*iint).flags & IMA_NEW_FILE) == 0 {
            rc = EACCES;
        }
        mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);
        kfree(xattr_value as *mut u8);
        ima_free_modsig(modsig);

        out_label: if !pathbuf.is_null() {
            __putname(pathbuf);
        }
        if must_appraise != 0 {
            if rc != 0 && (ima_appraise & IMA_APPRAISE_ENFORCE) != 0 {
                return EACCES;
            }
            if ((*file).f_mode & FMODE_WRITE) != 0 {
                set_bit(IMA_UPDATE_XATTR as i32, &mut (*iint).atomic_flags);
            }
        }
        0
    }
}

static mut ima_policy_flag: i32 = 0;
static mut func_local: i32 = FILE_CHECK;

extern "C" fn ima_file_mmap(
    file: *const file,
    reqprot: u32,
    prot: u32,
    flags: u32,
) -> i32 {
    unsafe {
        if file.is_null() {
            return 0;
        }

        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);

        if (reqprot & 0x04) != 0 {
            let ret = process_measurement(
                file,
                current_cred(),
                &prop,
                ptr::null(),
                0,
                MAY_EXEC,
                MMAP_CHECK_REQPROT,
                0,
                false,
            );
            if ret != 0 {
                return ret;
            }
        }

        if (prot & 0x04) != 0 {
            return process_measurement(
                file,
                current_cred(),
                &prop,
                ptr::null(),
                0,
                MAY_EXEC,
                MMAP_CHECK,
                0,
                false,
            );
        }

        0
    }
}

extern "C" fn ima_file_mprotect(
    vma: *const vm_area_struct,
    reqprot: u32,
    prot: u32,
) -> i32 {
    unsafe {
        let mut template: *mut ima_template_desc = ptr::null_mut();
        let mut filename: [c_char; NAME_MAX] = [0; NAME_MAX];
        let mut pathbuf: *mut c_char = ptr::null_mut();
        let mut pathname: *const c_char = ptr::null();
        let mut pcr = 0;

        if (ima_policy_flag & IMA_APPRAISE) == 0 || (*vma).vm_file.is_null()
            || (prot & 0x04) == 0 || ((*vma).vm_flags & 0x08) != 0 {
            return 0;
        }

        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        let inode = file_inode((*vma).vm_file);
        let mut action = ima_get_action(
            file_mnt_idmap((*vma).vm_file),
            inode as *mut inode,
            current_cred(),
            &prop,
            MAY_EXEC,
            MMAP_CHECK,
            &mut pcr,
            &mut template,
            ptr::null(),
            ptr::null_mut(),
        );
        action |= ima_get_action(
            file_mnt_idmap((*vma).vm_file),
            inode as *mut inode,
            current_cred(),
            &prop,
            MAY_EXEC,
            MMAP_CHECK_REQPROT,
            &mut pcr,
            &mut template,
            ptr::null(),
            ptr::null_mut(),
        );

        if (action & (IMA_MEASURE | IMA_APPRAISE_SUBMASK)) == 0 {
            return 0;
        }

        let mut result = 0;
        if (action & IMA_APPRAISE_SUBMASK) != 0 {
            result = EPERM;
        }

        let file = (*vma).vm_file;
        pathname = ima_d_path(&(*file).f_path, &mut pathbuf, filename.as_mut_ptr());
        integrity_audit_msg(
            AUDIT_INTEGRITY_DATA,
            inode,
            pathname,
            b"collect_data\0" as *const u8 as *const c_char,
            b"failed-mprotect\0" as *const u8 as *const c_char,
            result,
            0,
        );
        if !pathbuf.is_null() {
            __putname(pathbuf);
        }

        result
    }
}

extern "C" fn ima_bprm_check(bprm: *const linux_binprm) -> i32 {
    unsafe {
        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        process_measurement(
            (*bprm).file,
            current_cred(),
            &prop,
            ptr::null(),
            0,
            MAY_EXEC,
            BPRM_CHECK,
            0,
            (*bprm).is_check,
        )
    }
}

extern "C" fn ima_creds_check(bprm: *const linux_binprm, file: *const file) -> i32 {
    unsafe {
        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        process_measurement(
            file,
            (*bprm).cred,
            &prop,
            ptr::null(),
            0,
            MAY_EXEC,
            CREDS_CHECK,
            0,
            false,
        )
    }
}

extern "C" fn ima_bprm_creds_for_exec(bprm: *const linux_binprm) -> i32 {
    unsafe {
        if !(*bprm).is_check {
            return 0;
        }

        ima_bprm_check(bprm)
    }
}

extern "C" fn ima_file_check(file: *const file, mask: i32) -> i32 {
    unsafe {
        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        process_measurement(
            file,
            current_cred(),
            &prop,
            ptr::null(),
            0,
            mask & (MAY_READ | MAY_WRITE | MAY_EXEC | MAY_APPEND),
            FILE_CHECK,
            0,
            false,
        )
    }
}

extern "C" fn ima_reset_action_flags(inode: *const inode) {
    unsafe {
        if ima_policy_flag == 0 || !S_ISREG((*inode).i_mode) {
            return;
        }

        let iint = ima_iint_find(inode);
        if iint.is_null() {
            return;
        }

        mutex_lock(&mut (*iint).mutex as *mut libc::c_void);
        (*iint).flags &= !IMA_DONE_MASK;
        (*iint).measured_pcrs = 0;
        mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);
    }
}

extern "C" fn ima_path_truncate(path: *const path) -> i32 {
    unsafe {
        ima_reset_action_flags((*(*path).dentry).d_inode);
        0
    }
}

extern "C" fn ima_file_truncate(file: *const file) -> i32 {
    unsafe {
        ima_reset_action_flags(file_inode(file));
        0
    }
}

extern "C" fn __ima_inode_hash(
    inode: *const inode,
    file: *const file,
    buf: *mut c_char,
    buf_size: usize,
) -> i32 {
    unsafe {
        let mut iint: *mut ima_iint_cache = ptr::null_mut();
        let mut tmp_iint: ima_iint_cache = mem::zeroed();
        let mut rc: i32;

        if ima_policy_flag != 0 {
            iint = ima_iint_find(inode);
            if !iint.is_null() {
                mutex_lock(&mut (*iint).mutex as *mut libc::c_void);
            }
        }

        if (iint.is_null() || ((*iint).flags & IMA_COLLECTED) == 0) && !file.is_null() {
            if !iint.is_null() {
                mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);
            }

            memset(&mut tmp_iint as *mut ima_iint_cache as *mut u8, 0, mem::size_of::<ima_iint_cache>());
            mutex_init(&mut tmp_iint.mutex as *mut libc::c_void);

            rc = ima_collect_measurement(&mut tmp_iint, file, ptr::null(), 0, ima_hash_algo, ptr::null_mut());
            if rc < 0 {
                if rc != ENOMEM {
                    kfree(tmp_iint.ima_hash as *mut u8);
                }

                return EOPNOTSUPP;
            }

            iint = &mut tmp_iint;
            mutex_lock(&mut (*iint).mutex as *mut libc::c_void);
        }

        if iint.is_null() {
            return EOPNOTSUPP;
        }

        if iint.ima_hash.is_null() || ((*iint).flags & IMA_COLLECTED) == 0 {
            mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);
            return EOPNOTSUPP;
        }

        if !buf.is_null() {
            let copied_size =
                min_t(usize, (*iint).ima_hash.length as usize, buf_size);
            memcpy(buf as *mut u8, (*iint).ima_hash.digest as *const u8, copied_size);
        }
        let hash_algo = (*iint).ima_hash.algo;
        mutex_unlock(&mut (*iint).mutex as *mut libc::c_void);

        if iint as *const ima_iint_cache == &tmp_iint as *const ima_iint_cache {
            kfree((*iint).ima_hash as *mut u8);
        }

        hash_algo
    }
}

pub extern "C" fn ima_file_hash(file: *const file, buf: *mut c_char, buf_size: usize) -> i32 {
    unsafe {
        if file.is_null() {
            return EINVAL;
        }

        __ima_inode_hash(file_inode(file), file, buf, buf_size)
    }
}

pub extern "C" fn ima_inode_hash(inode: *const inode, buf: *mut c_char, buf_size: usize) -> i32 {
    unsafe {
        if inode.is_null() {
            return EINVAL;
        }

        __ima_inode_hash(inode, ptr::null(), buf, buf_size)
    }
}

extern "C" fn ima_post_create_tmpfile(idmap: *const mnt_idmap, inode: *const inode) {
    unsafe {
        if ima_policy_flag == 0 || !S_ISREG((*inode).i_mode) {
            return;
        }

        let must_appraise = ima_must_appraise(idmap, inode, MAY_ACCESS, FILE_CHECK);
        if must_appraise == 0 {
            return;
        }

        let iint = ima_inode_get(inode as *mut inode);
        if iint.is_null() {
            return;
        }

        set_bit(IMA_UPDATE_XATTR as i32, &mut (*iint).atomic_flags);
        (*iint).ima_file_status = INTEGRITY_PASS;
    }
}

extern "C" fn ima_post_path_mknod(idmap: *const mnt_idmap, dentry: *const dentry) {
    unsafe {
        let inode = (*dentry).d_inode;

        if ima_policy_flag == 0 || !S_ISREG((*inode).i_mode) {
            return;
        }

        let must_appraise = ima_must_appraise(idmap, inode, MAY_ACCESS, FILE_CHECK);
        if must_appraise == 0 {
            return;
        }

        let iint = ima_inode_get(inode);
        if iint.is_null() {
            return;
        }

        (*iint).flags |= IMA_NEW_FILE;
    }
}

extern "C" fn ima_read_file(file: *const file, read_id: i32, contents: bool) -> i32 {
    unsafe {
        if contents {
            return 0;
        }

        let func = read_idmap[read_id as usize];
        let func = if func == 0 { FILE_CHECK } else { func };
        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        process_measurement(file, current_cred(), &prop, ptr::null(), 0, MAY_READ, func, 0, false)
    }
}

pub static mut read_idmap: [i32; 7] = [
    FIRMWARE_CHECK,
    MODULE_CHECK,
    MODULE_CHECK,
    KEXEC_KERNEL_CHECK,
    KEXEC_INITRAMFS_CHECK,
    POLICY_CHECK,
    0,
];

extern "C" fn ima_post_read_file(
    file: *const file,
    buf: *const c_char,
    size: i32,
    read_id: i32,
) -> i32 {
    unsafe {
        if file.is_null() && read_id == READING_X509_CERTIFICATE {
            return 0;
        }

        if file.is_null() || buf.is_null() || size == 0 {
            if (ima_appraise & IMA_APPRAISE_ENFORCE) != 0 {
                return EACCES;
            }
            return 0;
        }

        let func = read_idmap[read_id as usize];
        let func = if func == 0 { FILE_CHECK } else { func };
        let mut prop: lsm_prop = mem::zeroed();
        security_current_getlsmprop_subj(&mut prop);
        process_measurement(
            file,
            current_cred(),
            &prop,
            buf as *const u8,
            size,
            MAY_READ,
            func,
            read_id,
            false,
        )
    }
}

extern "C" fn ima_load_data(id: i32, contents: bool) -> i32 {
    unsafe {
        let ima_enforce = (ima_appraise & IMA_APPRAISE_ENFORCE) == IMA_APPRAISE_ENFORCE;

        match id {
            LOADING_KEXEC_IMAGE => {
                if IS_ENABLED(CONFIG_KEXEC_SIG as i32) && arch_get_secureboot() {
                    pr_err(b"impossible to appraise a kernel image without a file descriptor; try using kexec_file_load syscall.\n\0" as *const u8 as *const c_char);
                    return EACCES;
                }

                if ima_enforce && (ima_appraise & IMA_APPRAISE_KEXEC) != 0 {
                    pr_err(b"impossible to appraise a kernel image without a file descriptor; try using kexec_file_load syscall.\n\0" as *const u8 as *const c_char);
                    return EACCES;
                }
            }
            LOADING_FIRMWARE => {
                if ima_enforce && (ima_appraise & IMA_APPRAISE_FIRMWARE) != 0 && !contents {
                    pr_err(b"Prevent firmware sysfs fallback loading.\n\0" as *const u8 as *const c_char);
                    return EACCES;
                }
            }
            LOADING_MODULE => {
                let sig_enforce = is_module_sig_enforced();

                if ima_enforce && !sig_enforce && (ima_appraise & IMA_APPRAISE_MODULES) != 0 {
                    pr_err(b"impossible to appraise a module without a file descriptor. sig_enforce kernel parameter might help\n\0" as *const u8 as *const c_char);
                    return EACCES;
                }
            }
            _ => {}
        }
        0
    }
}

extern "C" fn ima_post_load_data(
    buf: *const c_char,
    size: i32,
    load_id: i32,
    description: *const c_char,
) -> i32 {
    unsafe {
        if load_id == LOADING_FIRMWARE {
            if (ima_appraise & IMA_APPRAISE_FIRMWARE) != 0
                && (ima_appraise & IMA_APPRAISE_ENFORCE) != 0 {
                pr_err(b"Prevent firmware loading_store.\n\0" as *const u8 as *const c_char);
                return EACCES;
            }
            return 0;
        }

        if load_id == LOADING_MODULE {
            ima_measure_critical_data(
                b"modules\0" as *const u8 as *const c_char,
                b"init_module\0" as *const u8 as *const c_char,
                buf as *const u8,
                size as usize,
                true,
                ptr::null_mut(),
                0,
            );
        }

        0
    }
}

pub extern "C" fn process_buffer_measurement(
    idmap: *const mnt_idmap,
    inode: *const inode,
    buf: *const u8,
    size: i32,
    eventname: *const c_char,
    func: i32,
    mut pcr: i32,
    func_data: *const c_char,
    buf_hash: bool,
    digest: *mut u8,
    digest_len: usize,
) -> i32 {
    unsafe {
        let mut ret = 0;
        let mut audit_cause = b"ENOMEM\0" as *const u8 as *const c_char;
        let mut entry: *mut ima_template_entry = ptr::null_mut();
        let mut iint: ima_iint_cache = mem::zeroed();
        let mut event_data: ima_event_data = mem::zeroed();
        let mut template: *mut ima_template_desc;
        let mut hash: ima_max_digest_data = mem::zeroed();
        let hash_hdr = container_of(&hash.hdr as *const ima_digest_data as *const u8, 0, 0)
            as *mut ima_digest_data;
        let mut digest_hash: [u8; IMA_MAX_DIGEST_SIZE] = [0; IMA_MAX_DIGEST_SIZE];
        let digest_hash_len = hash_digest_size[ima_hash_algo as usize];
        let mut violation = 0;
        let mut action = 0;
        let mut prop: lsm_prop = mem::zeroed();

        if digest != ptr::null_mut() && digest_len < digest_hash_len {
            return EINVAL;
        }

        if ima_policy_flag == 0 && digest == ptr::null_mut() {
            return ENOENT;
        }

        template = ima_template_desc_buf();
        if template.is_null() {
            ret = EINVAL;
            audit_cause = b"ima_template_desc_buf\0" as *const u8 as *const c_char;
            goto out_label2;
        }

        if func != 0 {
            security_current_getlsmprop_subj(&mut prop);
            action = ima_get_action(
                idmap,
                inode as *mut inode,
                current_cred(),
                &prop,
                0,
                func,
                &mut pcr,
                &mut template,
                func_data,
                ptr::null_mut(),
            );
            if (action & IMA_MEASURE) == 0 && digest == ptr::null_mut() {
                return ENOENT;
            }
        }

        if pcr == 0 {
            pcr = CONFIG_IMA_MEASURE_PCR_IDX;
        }

        iint.ima_hash = hash_hdr;
        (*iint.ima_hash).algo = ima_hash_algo;
        (*iint.ima_hash).length = hash_digest_size[ima_hash_algo as usize];

        ret = ima_calc_buffer_hash(buf, size as usize, iint.ima_hash);
        if ret < 0 {
            audit_cause = b"hashing_error\0" as *const u8 as *const c_char;
            goto out_label2;
        }

        if buf_hash {
            memcpy(
                digest_hash.as_mut_ptr(),
                (*hash_hdr).digest as *const u8,
                digest_hash_len,
            );

            ret = ima_calc_buffer_hash(
                digest_hash.as_ptr(),
                digest_hash_len,
                iint.ima_hash,
            );
            if ret < 0 {
                audit_cause = b"hashing_error\0" as *const u8 as *const c_char;
                goto out_label2;
            }

            event_data.buf = digest_hash.as_ptr();
            event_data.buf_len = digest_hash_len;
        }

        if digest != ptr::null_mut() {
            memcpy(
                digest,
                (*iint.ima_hash).digest as *const u8,
                digest_hash_len,
            );
        }

        if ima_policy_flag == 0 || (func != 0 && (action & IMA_MEASURE) == 0) {
            return 1;
        }

        ret = ima_alloc_init_template(&event_data, &mut entry, template);
        if ret < 0 {
            audit_cause = b"alloc_entry\0" as *const u8 as *const c_char;
            goto out_label2;
        }

        ret = ima_store_template(entry, violation, ptr::null(), buf, pcr);
        if ret < 0 {
            audit_cause = b"store_entry\0" as *const u8 as *const c_char;
            ima_free_template_entry(entry);
        }

        out_label2: if ret < 0 {
            integrity_audit_message(
                AUDIT_INTEGRITY_PCR,
                ptr::null(),
                eventname,
                func_measure_str(func),
                audit_cause,
                ret,
                0,
                ret,
            );
        }

        ret
    }
}

pub extern "C" fn ima_kexec_cmdline(kernel_fd: i32, buf: *const u8, size: i32) {
    unsafe {
        if buf.is_null() || size == 0 {
            return;
        }

        let f = kernel_fd;
        if fd_empty(&f as *const i32 as *const libc::c_void) {
            return;
        }

        let fd_file_ptr = fd_file(&f as *const i32 as *const libc::c_void);
        process_buffer_measurement(
            file_mnt_idmap(fd_file_ptr),
            file_inode(fd_file_ptr),
            buf,
            size,
            b"kexec-cmdline\0" as *const u8 as *const c_char,
            KEXEC_CMDLINE,
            0,
            ptr::null(),
            false,
            ptr::null_mut(),
            0,
        );
    }
}

pub extern "C" fn ima_measure_critical_data(
    event_label: *const c_char,
    event_name: *const c_char,
    buf: *const u8,
    buf_len: usize,
    hash: bool,
    digest: *mut u8,
    digest_len: usize,
) -> i32 {
    unsafe {
        if event_name.is_null() || event_label.is_null() || buf.is_null() || buf_len == 0 {
            return ENOPARAM;
        }

        process_buffer_measurement(
            &nop_mnt_idmap,
            ptr::null(),
            buf,
            buf_len as i32,
            event_name,
            CRITICAL_DATA,
            0,
            event_label,
            hash,
            digest,
            digest_len,
        )
    }
}

pub extern "C" fn ima_measure_raw_policy(buf: *const c_char, buf_len: usize) -> i32 {
    unsafe {
        if buf.is_null() || buf_len == 0 {
            return EINVAL;
        }

        process_buffer_measurement(
            &nop_mnt_idmap,
            ptr::null(),
            buf as *const u8,
            buf_len as i32,
            b"ima_policy_written\0" as *const u8 as *const c_char,
            POLICY_CHECK,
            0,
            ptr::null(),
            false,
            ptr::null_mut(),
            0,
        )
    }
}

extern "C" fn ima_kernel_module_request(kmod_name: *const c_char) -> i32 {
    unsafe {
        if strncmp(kmod_name, b"crypto-pkcs1(rsa,\0" as *const u8 as *const c_char, 17) == 0 {
            return EINVAL;
        }

        0
    }
}

extern "C" fn init_ima() -> i32 {
    unsafe {
        if ima_disabled != 0 && is_kdump_kernel() {
            pr_info(b"IMA functionality is disabled\0" as *const u8 as *const c_char);
            return 0;
        }

        ima_appraise_parse_cmdline();
        ima_init_template_list();
        hash_setup(b"sha1\0" as *const u8 as *const c_char);
        let mut error = ima_init();

        if error != 0
            && strcmp(
                hash_algo_name[ima_hash_algo as usize],
                b"sha1\0" as *const u8 as *const c_char,
            ) != 0 {
            pr_info(
                b"Allocating %s failed, going to use default hash algorithm %s\n\0" as *const u8 as *const c_char,
                hash_algo_name[ima_hash_algo as usize],
                b"sha1\0" as *const u8 as *const c_char,
            );
            hash_setup_done = 0;
            hash_setup(b"sha1\0" as *const u8 as *const c_char);
            error = ima_init();
        }

        if error != 0 {
            return error;
        }

        error = register_blocking_lsm_notifier(&mut ima_lsm_policy_notifier);
        if error != 0 {
            pr_warn(b"Couldn't register LSM notifier, error %d\n\0" as *const u8 as *const c_char, error);
        }

        if error == 0 {
            ima_update_policy_flags();
        }

        error
    }
}

static mut ima_hooks: [security_hook_list; 15] = unsafe {
    [
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
        security_hook_list {},
    ]
};

static ima_lsmid: lsm_id = lsm_id {
    name: b"ima\0" as *const u8 as *const c_char,
    id: LSM_ID_IMA,
};

extern "C" fn init_ima_lsm() -> i32 {
    unsafe {
        ima_iintcache_init();
        security_add_hooks(ima_hooks.as_ptr(), ima_hooks.len(), &ima_lsmid);
        init_ima_appraise_lsm(&ima_lsmid);
        0
    }
}

pub static ima_blob_sizes: lsm_blob_sizes = lsm_blob_sizes {
    lbs_inode: mem::size_of::<*const ima_iint_cache>(),
};

// External stubs for missing declarations
extern "C" {
    pub fn ima_must_appraise(
        idmap: *const mnt_idmap,
        inode: *const inode,
        mask: i32,
        func: i32,
    ) -> i32;
}

pub const INTEGRITY_PASS: i32 = 0;
pub const IMA_APPRAISE_KEXEC: i32 = 0x10;
pub const IMA_APPRAISE_FIRMWARE: i32 = 0x20;
pub const IMA_APPRAISE_MODULES: i32 = 0x40;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
