// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2005,2006,2007,2008 IBM Corporation
//
// Authors:
// Reiner Sailer <sailer@watson.ibm.com>
// Mimi Zohar <zohar@us.ibm.com>
//
// File: ima.h
//	internal Integrity Measurement Architecture (IMA) definitions

// Dependencies from <linux/types.h>, <linux/crypto.h>, <linux/fs.h>,
// <linux/security.h>, <linux/hash.h>, <linux/tpm.h>, <linux/audit.h>,
// <crypto/hash_info.h>, "../integrity.h"

use core::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImaShowType {
    ImaBinary,
    ImaBinaryNoFieldLen,
    ImaBinaryOldStringFmt,
    ImaAscii,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum TpmPcrs {
    TpmPcr0 = 0,
    TpmPcr8 = 8,
    TpmPcr10 = 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BinaryLists {
    BINARY,
    BINARY_STAGED,
    BINARY_FULL,
    BINARY__LAST,
}

pub const IMA_DIGEST_SIZE: usize = 20; // SHA1_DIGEST_SIZE
pub const IMA_EVENT_NAME_LEN_MAX: usize = 255;

pub const IMA_HASH_BITS: usize = 10;
pub const IMA_MEASURE_HTABLE_SIZE: usize = 1 << IMA_HASH_BITS;

pub const IMA_TEMPLATE_FIELD_ID_MAX_LEN: usize = 16;
pub const IMA_TEMPLATE_NUM_FIELDS_MAX: usize = 15;

pub const IMA_TEMPLATE_IMA_NAME: &[u8] = b"ima";
pub const IMA_TEMPLATE_IMA_FMT: &[u8] = b"d|n";

// NR_BANKS macro: ((chip != NULL) ? chip->nr_allocated_banks : 0)
// This is a macro that depends on external types, so it's preserved as a comment

// External declarations
extern "C" {
    pub static mut ima_policy_flag: c_int;
    pub static ima_write_mutex: core::ffi::c_void; // struct mutex
}

extern "C" {
    pub static ima_setxattr_allowed_hash_algorithms: core::ffi::c_void; // atomic_t
}

#[repr(C)]
pub struct ImaAlgoDesc {
    pub tfm: *mut core::ffi::c_void,         // struct crypto_shash
    pub algo: c_int,                          // enum hash_algo
    pub digest_size: u32,
}

extern "C" {
    pub static mut ima_hash_algo: c_int;
    pub static mut ima_sha1_idx: c_int;
    pub static mut ima_hash_algo_idx: c_int;
    pub static mut ima_extra_slots: c_int;
    pub static mut ima_algo_array: *mut ImaAlgoDesc;
}

extern "C" {
    pub static mut ima_appraise: c_int;
    pub static ima_tpm_chip: *mut core::ffi::c_void; // struct tpm_chip
    pub static boot_aggregate_name: [c_int; 0];
    pub static boot_aggregate_late_name: [c_int; 0];
}

#[repr(C)]
pub struct ImaEventData {
    pub iint: *mut core::ffi::c_void,         // struct ima_iint_cache
    pub file: *mut core::ffi::c_void,         // struct file
    pub filename: *const u8,                  // const unsigned char
    pub xattr_value: *mut core::ffi::c_void, // struct evm_ima_xattr_data
    pub xattr_len: c_int,
    pub modsig: *const core::ffi::c_void,     // struct modsig
    pub violation: *const c_int,              // const char
    pub buf: *const core::ffi::c_void,
    pub buf_len: c_int,
}

#[repr(C)]
pub struct ImaFieldData {
    pub data: *mut u8,
    pub len: u32,
}

#[repr(C)]
pub struct ImaTemplateField {
    pub field_id: [c_int; IMA_TEMPLATE_FIELD_ID_MAX_LEN],
    pub field_init: Option<unsafe extern "C" fn(
        *mut ImaEventData,
        *mut ImaFieldData,
    ) -> c_int>,
    pub field_show: Option<unsafe extern "C" fn(
        *mut core::ffi::c_void,  // struct seq_file
        ImaShowType,
        *mut ImaFieldData,
    )>,
}

#[repr(C)]
pub struct ImaTemplateDesc {
    pub list: core::ffi::c_void,                   // struct list_head
    pub name: *mut c_int,                         // char
    pub fmt: *mut c_int,                          // char
    pub num_fields: c_int,
    pub fields: *mut *const ImaTemplateField,
}

#[repr(C)]
pub struct ImaTemplateEntry {
    pub pcr: c_int,
    pub digests: *mut core::ffi::c_void,          // struct tpm_digest
    pub template_desc: *mut ImaTemplateDesc,
    pub template_data_len: u32,
    pub template_data: [ImaFieldData; 0],         // Flexible array member
}

#[repr(C)]
pub struct ImaQueueEntry {
    pub hnext: core::ffi::c_void,                 // struct hlist_node
    pub later: core::ffi::c_void,                 // struct list_head
    pub entry: *mut ImaTemplateEntry,
}

extern "C" {
    pub static ima_measurements: core::ffi::c_void;         // struct list_head
    pub static ima_measurements_staged: core::ffi::c_void;  // struct list_head
}

#[repr(C)]
pub struct ImaKexecHdr {
    pub version: u16,
    pub _reserved0: u16,
    pub _reserved1: u32,
    pub buffer_size: u64,
    pub count: u64,
}

// IMA iint action cache flags
pub const IMA_MEASURE: u32 = 0x00000001;
pub const IMA_MEASURED: u32 = 0x00000002;
pub const IMA_APPRAISE: u32 = 0x00000004;
pub const IMA_APPRAISED: u32 = 0x00000008;
pub const IMA_COLLECTED: u32 = 0x00000020;
pub const IMA_AUDIT: u32 = 0x00000040;
pub const IMA_AUDITED: u32 = 0x00000080;
pub const IMA_HASH: u32 = 0x00000100;
pub const IMA_HASHED: u32 = 0x00000200;

// IMA iint policy rule cache flags
pub const IMA_NONACTION_FLAGS: u32 = 0xff000000;
pub const IMA_DIGSIG_REQUIRED: u32 = 0x01000000;
pub const IMA_PERMIT_DIRECTIO: u32 = 0x02000000;
pub const IMA_NEW_FILE: u32 = 0x04000000;
pub const IMA_SIGV3_REQUIRED: u32 = 0x08000000;
pub const IMA_FAIL_UNVERIFIABLE_SIGS: u32 = 0x10000000;
pub const IMA_MODSIG_ALLOWED: u32 = 0x20000000;
pub const IMA_CHECK_BLACKLIST: u32 = 0x40000000;
pub const IMA_VERITY_REQUIRED: u32 = 0x80000000;

pub const IMA_NONACTION_RULE_FLAGS: u32 = IMA_NONACTION_FLAGS & !IMA_NEW_FILE;

pub const IMA_DO_MASK: u32 = IMA_MEASURE | IMA_APPRAISE | IMA_AUDIT |
                             IMA_HASH | IMA_APPRAISE_SUBMASK;
pub const IMA_DONE_MASK: u32 = IMA_MEASURED | IMA_APPRAISED | IMA_AUDITED |
                               IMA_HASHED | IMA_COLLECTED |
                               IMA_APPRAISED_SUBMASK;

// IMA iint subaction appraise cache flags
pub const IMA_FILE_APPRAISE: u32 = 0x00001000;
pub const IMA_FILE_APPRAISED: u32 = 0x00002000;
pub const IMA_MMAP_APPRAISE: u32 = 0x00004000;
pub const IMA_MMAP_APPRAISED: u32 = 0x00008000;
pub const IMA_BPRM_APPRAISE: u32 = 0x00010000;
pub const IMA_BPRM_APPRAISED: u32 = 0x00020000;
pub const IMA_READ_APPRAISE: u32 = 0x00040000;
pub const IMA_READ_APPRAISED: u32 = 0x00080000;
pub const IMA_CREDS_APPRAISE: u32 = 0x00100000;
pub const IMA_CREDS_APPRAISED: u32 = 0x00200000;

pub const IMA_APPRAISE_SUBMASK: u32 = IMA_FILE_APPRAISE | IMA_MMAP_APPRAISE |
                                      IMA_BPRM_APPRAISE | IMA_READ_APPRAISE |
                                      IMA_CREDS_APPRAISE;
pub const IMA_APPRAISED_SUBMASK: u32 = IMA_FILE_APPRAISED | IMA_MMAP_APPRAISED |
                                       IMA_BPRM_APPRAISED | IMA_READ_APPRAISED |
                                       IMA_CREDS_APPRAISED;

// IMA iint cache atomic_flags (bit indices)
pub const IMA_CHANGE_XATTR: usize = 0;
pub const IMA_UPDATE_XATTR: usize = 1;
pub const IMA_CHANGE_ATTR: usize = 2;
pub const IMA_DIGSIG: usize = 3;
pub const IMA_MAY_EMIT_TOMTOU: usize = 4;
pub const IMA_EMITTED_OPENWRITERS: usize = 5;

#[repr(C)]
pub struct ImaIintCache {
    pub mutex: core::ffi::c_void,                     // struct mutex
    pub real_inode: core::ffi::c_void,                // struct integrity_inode_attributes
    pub flags: usize,
    pub measured_pcrs: usize,
    pub atomic_flags: usize,
    pub ima_file_status: u8,                          // enum integrity_status (4 bits)
    pub ima_mmap_status: u8,                          // enum integrity_status (4 bits)
    pub ima_bprm_status: u8,                          // enum integrity_status (4 bits)
    pub ima_read_status: u8,                          // enum integrity_status (4 bits)
    pub ima_creds_status: u8,                         // enum integrity_status (4 bits)
    pub ima_hash: *mut core::ffi::c_void,             // struct ima_digest_data
}

extern "C" {
    pub static ima_blob_sizes: core::ffi::c_void;  // struct lsm_blob_sizes
}

#[inline]
pub unsafe fn ima_inode_get_iint(inode: *const core::ffi::c_void) -> *mut ImaIintCache {
    let inode_security = *(inode as *const *const core::ffi::c_void).offset(1);
    if unlikely(inode_security.is_null()) {
        return core::ptr::null_mut();
    }

    let iint_sec = (inode_security as *mut *mut ImaIintCache)
        .offset(ima_blob_sizes as isize);
    *iint_sec
}

#[inline]
pub unsafe fn ima_inode_set_iint(inode: *const core::ffi::c_void, iint: *mut ImaIintCache) {
    let inode_security = *(inode as *const *const core::ffi::c_void).offset(1);
    if unlikely(inode_security.is_null()) {
        return;
    }

    let iint_sec = (inode_security as *mut *mut ImaIintCache)
        .offset(ima_blob_sizes as isize);
    *iint_sec = iint;
}

#[cold]
#[inline]
fn unlikely(b: bool) -> bool {
    b
}

extern "C" {
    pub fn ima_iint_find(inode: *mut core::ffi::c_void) -> *mut ImaIintCache;
    pub fn ima_inode_get(inode: *mut core::ffi::c_void) -> *mut ImaIintCache;
    pub fn ima_inode_free_rcu(inode_security: *mut core::ffi::c_void);
    pub fn ima_iintcache_init();
}

extern "C" {
    pub static read_idmap: [c_int; 0];
}

#[cfg(feature = "CONFIG_HAVE_IMA_KEXEC")]
extern "C" {
    pub fn ima_load_kexec_buffer();
}

#[cfg(not(feature = "CONFIG_HAVE_IMA_KEXEC"))]
#[inline]
pub fn ima_load_kexec_buffer() {}

#[cfg(feature = "CONFIG_IMA_MEASURE_ASYMMETRIC_KEYS")]
extern "C" {
    pub fn ima_post_key_create_or_update(
        keyring: *mut core::ffi::c_void,  // struct key
        key: *mut core::ffi::c_void,      // struct key
        payload: *const core::ffi::c_void,
        plen: usize,
        flags: usize,
        create: bool,
    );
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
extern "C" {
    pub fn ima_measure_kexec_event(event_name: *const c_int);  // const char
}

#[cfg(not(feature = "CONFIG_IMA_KEXEC"))]
#[inline]
pub fn ima_measure_kexec_event(_event_name: *const c_int) {}

extern "C" {
    pub static ima_canonical_fmt: bool;
}

extern "C" {
    pub fn ima_init() -> c_int;
    pub fn ima_fs_init() -> c_int;
    pub fn ima_add_template_entry(
        entry: *mut ImaTemplateEntry,
        violation: c_int,
        op: *const c_int,                      // const char
        inode: *mut core::ffi::c_void,         // struct inode
        filename: *const u8,                   // const unsigned char
    ) -> c_int;
    pub fn ima_calc_file_hash(
        file: *mut core::ffi::c_void,          // struct file
        hash: *mut core::ffi::c_void,          // struct ima_digest_data
    ) -> c_int;
    pub fn ima_calc_buffer_hash(
        buf: *const core::ffi::c_void,
        len: i64,                              // loff_t
        hash: *mut core::ffi::c_void,          // struct ima_digest_data
    ) -> c_int;
    pub fn ima_calc_field_array_hash(
        field_data: *mut ImaFieldData,
        entry: *mut ImaTemplateEntry,
    ) -> c_int;
    pub fn ima_calc_boot_aggregate(hash: *mut core::ffi::c_void) -> c_int;
    pub fn ima_add_violation(
        file: *mut core::ffi::c_void,          // struct file
        filename: *const u8,                   // const unsigned char
        iint: *mut ImaIintCache,
        op: *const c_int,                      // const char
        cause: *const c_int,                   // const char
    );
    pub fn ima_init_crypto() -> c_int;
    pub fn ima_putc(m: *mut core::ffi::c_void, data: *mut core::ffi::c_void, datalen: c_int);
    pub fn ima_print_digest(m: *mut core::ffi::c_void, digest: *mut u8, size: u32);
    pub fn template_desc_init_fields(
        template_fmt: *const c_int,            // const char
        fields: *mut *const *const ImaTemplateField,
        num_fields: *mut c_int,
    ) -> c_int;
    pub fn ima_template_desc_current() -> *mut ImaTemplateDesc;
    pub fn ima_template_desc_buf() -> *mut ImaTemplateDesc;
    pub fn lookup_template_desc(name: *const c_int) -> *mut ImaTemplateDesc;
    pub fn ima_template_has_modsig(ima_template: *const ImaTemplateDesc) -> bool;
    pub fn ima_queue_stage() -> c_int;
    pub fn ima_queue_staged_delete_all() -> c_int;
    pub fn ima_queue_delete_partial(req_value: usize) -> c_int;
    pub fn ima_restore_measurement_entry(entry: *mut ImaTemplateEntry) -> c_int;
    pub fn ima_restore_measurement_list(bufsize: i64, buf: *mut core::ffi::c_void) -> c_int;
    pub fn ima_measurements_show(m: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> c_int;
    pub fn ima_init_htable() -> c_int;
    pub fn ima_get_binary_runtime_size(binary_list: BinaryLists) -> usize;
    pub fn ima_init_template() -> c_int;
    pub fn ima_init_template_list();
    pub fn ima_init_digests() -> c_int;
    pub fn ima_init_reboot_notifier();
    pub fn ima_lsm_policy_change(
        nb: *mut core::ffi::c_void,             // struct notifier_block
        event: usize,
        lsm_data: *mut core::ffi::c_void,
    ) -> c_int;
}

extern "C" {
    pub static ima_queue_lock: core::ffi::c_void;  // spinlock_t
}

extern "C" {
    pub static ima_num_records: [core::ffi::c_void; 4];  // atomic_long_t[BINARY__LAST]
    pub static ima_num_violations: core::ffi::c_void;    // atomic_long_t
    pub static ima_htable: *mut core::ffi::c_void;       // struct hlist_head
    pub static ima_flush_htable: bool;
}

#[inline]
pub fn ima_hash_key(digest: *mut u8) -> u32 {
    unsafe {
        let d0 = *digest as u32;
        let d1 = (*digest.offset(1) as u32) << 8;
        ((d0 | d1) % IMA_MEASURE_HTABLE_SIZE as u32)
    }
}

// __ima_hooks macro expansion
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ImaHooks {
    NONE = 0,
    FILE_CHECK = 1,
    MMAP_CHECK = 2,
    MMAP_CHECK_REQPROT = 3,
    BPRM_CHECK = 4,
    CREDS_CHECK = 5,
    POST_SETATTR = 6,
    MODULE_CHECK = 7,
    FIRMWARE_CHECK = 8,
    KEXEC_KERNEL_CHECK = 9,
    KEXEC_INITRAMFS_CHECK = 10,
    POLICY_CHECK = 11,
    KEXEC_CMDLINE = 12,
    KEY_CHECK = 13,
    CRITICAL_DATA = 14,
    SETXATTR_CHECK = 15,
    MAX_CHECK = 16,
}

pub static IMA_HOOKS_MEASURE_STR: &[&str] = &[
    "measuring_none",
    "measuring_file",
    "measuring_mmap",
    "measuring_mmap_reqprot",
    "measuring_bprm",
    "measuring_creds",
    "measuring_post_setattr",
    "measuring_module",
    "measuring_firmware",
    "measuring_kexec_kernel",
    "measuring_kexec_initramfs",
    "measuring_policy",
    "measuring_kexec_cmdline",
    "measuring_key",
    "measuring_critical_data",
    "measuring_setxattr_check",
    "measuring_none",
];

#[inline]
pub fn func_measure_str(func: ImaHooks) -> &'static str {
    let idx = func as usize;
    if idx >= ImaHooks::MAX_CHECK as usize {
        return IMA_HOOKS_MEASURE_STR[ImaHooks::NONE as usize];
    }
    IMA_HOOKS_MEASURE_STR[idx]
}

extern "C" {
    pub static func_tokens: *const *const c_int;  // const char * const
}

// struct modsig is opaque
pub enum Modsig {}

#[cfg(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS")]
#[repr(C)]
pub struct ImaKeyEntry {
    pub list: core::ffi::c_void,               // struct list_head
    pub payload: *mut core::ffi::c_void,
    pub payload_len: usize,
    pub keyring_name: *mut c_int,              // char
}

#[cfg(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS")]
extern "C" {
    pub fn ima_init_key_queue();
    pub fn ima_should_queue_key() -> bool;
    pub fn ima_queue_key(
        keyring: *mut core::ffi::c_void,  // struct key
        payload: *const core::ffi::c_void,
        payload_len: usize,
    ) -> bool;
    pub fn ima_process_queued_keys();
}

#[cfg(not(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS"))]
#[inline]
pub fn ima_init_key_queue() {}

#[cfg(not(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS"))]
#[inline]
pub fn ima_should_queue_key() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS"))]
#[inline]
pub fn ima_queue_key(
    _keyring: *mut core::ffi::c_void,
    _payload: *const core::ffi::c_void,
    _payload_len: usize,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_IMA_QUEUE_EARLY_BOOT_KEYS"))]
#[inline]
pub fn ima_process_queued_keys() {}

extern "C" {
    pub fn ima_get_action(
        idmap: *mut core::ffi::c_void,                  // struct mnt_idmap
        inode: *mut core::ffi::c_void,                  // struct inode
        cred: *const core::ffi::c_void,                 // struct cred
        prop: *mut core::ffi::c_void,                   // struct lsm_prop
        mask: c_int,
        func: ImaHooks,
        pcr: *mut c_int,
        template_desc: *mut *mut ImaTemplateDesc,
        func_data: *const c_int,                        // const char
        allowed_algos: *mut u32,
    ) -> c_int;
    pub fn ima_must_measure(
        inode: *mut core::ffi::c_void,  // struct inode
        mask: c_int,
        func: ImaHooks,
    ) -> c_int;
    pub fn ima_collect_measurement(
        iint: *mut ImaIintCache,
        file: *mut core::ffi::c_void,   // struct file
        buf: *mut core::ffi::c_void,
        size: i64,                      // loff_t
        algo: c_int,                    // enum hash_algo
        modsig: *mut Modsig,
    ) -> c_int;
    pub fn ima_store_measurement(
        iint: *mut ImaIintCache,
        file: *mut core::ffi::c_void,           // struct file
        filename: *const u8,                    // const unsigned char
        xattr_value: *mut core::ffi::c_void,    // struct evm_ima_xattr_data
        xattr_len: c_int,
        modsig: *const Modsig,
        pcr: c_int,
        template_desc: *mut ImaTemplateDesc,
    );
    pub fn process_buffer_measurement(
        idmap: *mut core::ffi::c_void,          // struct mnt_idmap
        inode: *mut core::ffi::c_void,          // struct inode
        buf: *const core::ffi::c_void,
        size: c_int,
        eventname: *const c_int,                // const char
        func: ImaHooks,
        pcr: c_int,
        func_data: *const c_int,                // const char
        buf_hash: bool,
        digest: *mut u8,
        digest_len: usize,
    ) -> c_int;
    pub fn ima_audit_measurement(
        iint: *mut ImaIintCache,
        filename: *const u8,                    // const unsigned char
    );
    pub fn ima_alloc_init_template(
        event_data: *mut ImaEventData,
        entry: *mut *mut ImaTemplateEntry,
        template_desc: *mut ImaTemplateDesc,
    ) -> c_int;
    pub fn ima_store_template(
        entry: *mut ImaTemplateEntry,
        violation: c_int,
        inode: *mut core::ffi::c_void,          // struct inode
        filename: *const u8,                    // const unsigned char
        pcr: c_int,
    ) -> c_int;
    pub fn ima_free_template_entry(entry: *mut ImaTemplateEntry);
    pub fn ima_d_path(
        path: *const core::ffi::c_void,         // struct path
        pathbuf: *mut *mut c_int,               // char **
        filename: *mut c_int,                   // char
    ) -> *const c_int;  // const char
}

// IMA policy related functions
extern "C" {
    pub fn ima_match_policy(
        idmap: *mut core::ffi::c_void,                  // struct mnt_idmap
        inode: *mut core::ffi::c_void,                  // struct inode
        cred: *const core::ffi::c_void,                 // struct cred
        prop: *mut core::ffi::c_void,                   // struct lsm_prop
        func: ImaHooks,
        mask: c_int,
        flags: c_int,
        pcr: *mut c_int,
        template_desc: *mut *mut ImaTemplateDesc,
        func_data: *const c_int,                        // const char
        allowed_algos: *mut u32,
    ) -> c_int;
    pub fn ima_init_policy();
    pub fn ima_update_policy();
    pub fn ima_update_policy_flags();
    pub fn ima_parse_add_rule(rule: *mut c_int) -> i64;  // ssize_t, char
    pub fn ima_delete_rules();
    pub fn ima_check_policy() -> c_int;
    pub fn ima_policy_start(m: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void;
    pub fn ima_policy_next(m: *mut core::ffi::c_void, v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void;
    pub fn ima_policy_stop(m: *mut core::ffi::c_void, v: *mut core::ffi::c_void);
    pub fn ima_policy_show(m: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> c_int;
    pub fn ima_measure_loaded_policy();
    pub fn ima_measure_raw_policy(buf: *const c_int, buf_len: usize) -> c_int;  // const char
}

// Appraise integrity measurements
pub const IMA_APPRAISE_ENFORCE: u32 = 0x01;
pub const IMA_APPRAISE_FIX: u32 = 0x02;
pub const IMA_APPRAISE_LOG: u32 = 0x04;
pub const IMA_APPRAISE_MODULES: u32 = 0x08;
pub const IMA_APPRAISE_FIRMWARE: u32 = 0x10;
pub const IMA_APPRAISE_POLICY: u32 = 0x20;
pub const IMA_APPRAISE_KEXEC: u32 = 0x40;

#[cfg(feature = "CONFIG_IMA_APPRAISE")]
extern "C" {
    pub fn ima_check_blacklist(
        iint: *mut ImaIintCache,
        modsig: *const Modsig,
        pcr: c_int,
    ) -> c_int;
    pub fn ima_appraise_measurement(
        func: ImaHooks,
        iint: *mut ImaIintCache,
        file: *mut core::ffi::c_void,               // struct file
        filename: *const u8,                        // const unsigned char
        xattr_value: *mut core::ffi::c_void,        // struct evm_ima_xattr_data
        xattr_len: c_int,
        modsig: *const Modsig,
        bprm_is_check: bool,
    ) -> c_int;
    pub fn ima_must_appraise(
        idmap: *mut core::ffi::c_void,              // struct mnt_idmap
        inode: *mut core::ffi::c_void,              // struct inode
        mask: c_int,
        func: ImaHooks,
    ) -> c_int;
    pub fn ima_update_xattr(iint: *mut ImaIintCache, file: *mut core::ffi::c_void);
    pub fn ima_get_cache_status(
        iint: *mut ImaIintCache,
        func: ImaHooks,
    ) -> c_int;  // enum integrity_status
    pub fn ima_get_hash_algo(
        xattr_value: *const core::ffi::c_void,     // struct evm_ima_xattr_data
        xattr_len: c_int,
    ) -> c_int;  // enum hash_algo
    pub fn ima_read_xattr(
        dentry: *mut core::ffi::c_void,             // struct dentry
        xattr_value: *mut *mut core::ffi::c_void,   // struct evm_ima_xattr_data **
        xattr_len: c_int,
    ) -> c_int;
    pub fn init_ima_appraise_lsm(lsmid: *const core::ffi::c_void);  // struct lsm_id
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_check_blacklist(
    _iint: *mut ImaIintCache,
    _modsig: *const Modsig,
    _pcr: c_int,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_appraise_measurement(
    _func: ImaHooks,
    _iint: *mut ImaIintCache,
    _file: *mut core::ffi::c_void,
    _filename: *const u8,
    _xattr_value: *mut core::ffi::c_void,
    _xattr_len: c_int,
    _modsig: *const Modsig,
    _bprm_is_check: bool,
) -> c_int {
    // INTEGRITY_UNKNOWN
    -1
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_must_appraise(
    _idmap: *mut core::ffi::c_void,
    _inode: *mut core::ffi::c_void,
    _mask: c_int,
    _func: ImaHooks,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_update_xattr(_iint: *mut ImaIintCache, _file: *mut core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_get_cache_status(
    _iint: *mut ImaIintCache,
    _func: ImaHooks,
) -> c_int {
    // INTEGRITY_UNKNOWN
    -1
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_get_hash_algo(
    _xattr_value: *const core::ffi::c_void,
    _xattr_len: c_int,
) -> c_int {
    unsafe { ima_hash_algo }
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn ima_read_xattr(
    _dentry: *mut core::ffi::c_void,
    _xattr_value: *mut *mut core::ffi::c_void,
    _xattr_len: c_int,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE"))]
#[inline]
pub fn init_ima_appraise_lsm(_lsmid: *const core::ffi::c_void) {}

#[cfg(feature = "CONFIG_IMA_APPRAISE_MODSIG")]
extern "C" {
    pub fn ima_read_modsig(
        func: ImaHooks,
        buf: *const core::ffi::c_void,
        buf_len: i64,  // loff_t
        modsig: *mut *mut Modsig,
    ) -> c_int;
    pub fn ima_collect_modsig(
        modsig: *mut Modsig,
        buf: *const core::ffi::c_void,
        size: i64,  // loff_t
    );
    pub fn ima_get_modsig_digest(
        modsig: *const Modsig,
        algo: *mut c_int,  // enum hash_algo
        digest: *mut *const u8,
        digest_size: *mut u32,
    ) -> c_int;
    pub fn ima_get_raw_modsig(
        modsig: *const Modsig,
        data: *mut *const core::ffi::c_void,
        data_len: *mut u32,
    ) -> c_int;
    pub fn ima_free_modsig(modsig: *mut Modsig);
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE_MODSIG"))]
#[inline]
pub fn ima_read_modsig(
    _func: ImaHooks,
    _buf: *const core::ffi::c_void,
    _buf_len: i64,
    _modsig: *mut *mut Modsig,
) -> c_int {
    -95  // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE_MODSIG"))]
#[inline]
pub fn ima_collect_modsig(_modsig: *mut Modsig, _buf: *const core::ffi::c_void, _size: i64) {}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE_MODSIG"))]
#[inline]
pub fn ima_get_modsig_digest(
    _modsig: *const Modsig,
    _algo: *mut c_int,
    _digest: *mut *const u8,
    _digest_size: *mut u32,
) -> c_int {
    -95  // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE_MODSIG"))]
#[inline]
pub fn ima_get_raw_modsig(
    _modsig: *const Modsig,
    _data: *mut *const core::ffi::c_void,
    _data_len: *mut u32,
) -> c_int {
    -95  // -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_IMA_APPRAISE_MODSIG"))]
#[inline]
pub fn ima_free_modsig(_modsig: *mut Modsig) {}

// LSM based policy rules
#[cfg(feature = "CONFIG_IMA_LSM_RULES")]
pub use security_audit_rule_init as ima_filter_rule_init;
pub use security_audit_rule_free as ima_filter_rule_free;
pub use security_audit_rule_match as ima_filter_rule_match;

#[cfg(not(feature = "CONFIG_IMA_LSM_RULES"))]
#[inline]
pub fn ima_filter_rule_init(
    _field: u32,
    _op: u32,
    _rulestr: *mut c_int,  // char
    _lsmrule: *mut *mut core::ffi::c_void,
    _gfp: u32,
) -> c_int {
    -22  // -EINVAL
}

#[cfg(not(feature = "CONFIG_IMA_LSM_RULES"))]
#[inline]
pub fn ima_filter_rule_free(_lsmrule: *mut core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_IMA_LSM_RULES"))]
#[inline]
pub fn ima_filter_rule_match(
    _prop: *mut core::ffi::c_void,  // struct lsm_prop
    _field: u32,
    _op: u32,
    _lsmrule: *mut core::ffi::c_void,
) -> c_int {
    -22  // -EINVAL
}

// These functions are referenced but not defined in this header
#[cfg(feature = "CONFIG_IMA_LSM_RULES")]
extern "C" {
    pub fn security_audit_rule_init(
        field: u32,
        op: u32,
        rulestr: *mut c_int,
        lsmrule: *mut *mut core::ffi::c_void,
        gfp: u32,
    ) -> c_int;
    pub fn security_audit_rule_free(lsmrule: *mut core::ffi::c_void);
    pub fn security_audit_rule_match(
        prop: *mut core::ffi::c_void,
        field: u32,
        op: u32,
        lsmrule: *mut core::ffi::c_void,
    ) -> c_int;
}

#[cfg(feature = "CONFIG_IMA_READ_POLICY")]
pub const POLICY_FILE_FLAGS: u32 = 0o600;  // S_IWUSR | S_IRUSR

#[cfg(not(feature = "CONFIG_IMA_READ_POLICY"))]
pub const POLICY_FILE_FLAGS: u32 = 0o200;  // S_IWUSR


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
