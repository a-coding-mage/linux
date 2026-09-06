// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 *
 * Author: Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_api.c -> ima_api.rs
 *	Implements must_appraise_or_measure, collect_measurement,
 *	appraise_measurement, store_measurement and store_template.
 */

// External dependencies from linux kernel and ima module
// #include <linux/slab.h>
// #include <linux/file.h>
// #include <linux/fs.h>
// #include <linux/hex.h>
// #include <linux/xattr.h>
// #include <linux/evm.h>
// #include <linux/fsverity.h>
// #include "ima.h"

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

// External types and functions from kernel/ima modules
extern "C" {
    // Types (declared externally)
    pub struct ima_template_entry;
    pub struct ima_iint_cache;
    pub struct ima_template_desc;
    pub struct ima_event_data;
    pub struct ima_digest_data;
    pub struct ima_max_digest_data;
    pub struct inode;
    pub struct file;
    pub struct cred;
    pub struct lsm_prop;
    pub struct mnt_idmap;
    pub struct evm_ima_xattr_data;
    pub struct modsig;
    pub struct name_snapshot;
    pub struct kstat;
    pub struct path;
    pub struct audit_buffer;
    pub struct dentry;

    // External functions
    fn kfree(ptr: *const std::ffi::c_void);
    fn kzalloc_flex(
        ptr: *const std::ffi::c_void,
        field: *const std::ffi::c_void,
        num_fields: usize,
        flags: c_uint,
    ) -> *const std::ffi::c_void;
    fn kzalloc_objs(
        ptr: *const std::ffi::c_void,
        num: usize,
        flags: c_uint,
    ) -> *const std::ffi::c_void;
    fn ima_template_desc_current() -> *const ima_template_desc;
    fn ima_calc_field_array_hash(
        template_data: *const std::ffi::c_void,
        entry: *const ima_template_entry,
    ) -> c_int;
    fn integrity_audit_msg(
        audit_type: c_int,
        inode: *const inode,
        template_name: *const c_char,
        op: *const c_char,
        cause: *const c_char,
        result: c_int,
        audit_flags: c_int,
    );
    fn ima_add_template_entry(
        entry: *const ima_template_entry,
        violation: c_int,
        op: *const c_char,
        inode: *const inode,
        filename: *const c_char,
    ) -> c_int;
    fn file_inode(file: *const file) -> *const inode;
    fn atomic_long_inc(ptr: *mut i64);
    fn ima_match_policy(
        idmap: *const mnt_idmap,
        inode: *const inode,
        cred: *const cred,
        prop: *const lsm_prop,
        func: c_uint,
        mask: c_int,
        flags: c_int,
        pcr: *mut c_int,
        template_desc: *mut *const ima_template_desc,
        func_data: *const c_char,
        allowed_algos: *mut c_uint,
    ) -> c_int;
    fn fsverity_get_digest(
        inode: *const inode,
        digest: *mut u8,
        digest_type: *const std::ffi::c_void,
        alg: *mut c_uint,
    ) -> c_int;
    fn ima_collect_modsig(
        modsig: *const modsig,
        buf: *const std::ffi::c_void,
        size: i64,
    );
    fn vfs_getattr_nosec(
        path: *const path,
        stat: *mut kstat,
        request_mask: c_uint,
        query_flags: c_uint,
    ) -> c_int;
    fn inode_peek_iversion(inode: *const inode) -> u64;
    fn ima_calc_buffer_hash(
        buf: *const std::ffi::c_void,
        size: i64,
        hash_hdr: *mut ima_digest_data,
    ) -> c_int;
    fn ima_calc_file_hash(file: *const file, hash_hdr: *mut ima_digest_data) -> c_int;
    fn krealloc(ptr: *const std::ffi::c_void, size: usize, flags: c_uint) -> *mut std::ffi::c_void;
    fn integrity_inode_attrs_store(
        iint: *mut std::ffi::c_void,
        i_version: u64,
        inode: *const inode,
    );
    fn take_dentry_name_snapshot(snapshot: *mut name_snapshot, dentry: *const dentry);
    fn release_dentry_name_snapshot(snapshot: *mut name_snapshot);
    fn audit_log_start(
        context: *const std::ffi::c_void,
        flags: c_uint,
        audit_type: c_int,
    ) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, filename: *const c_char);
    fn audit_log_task_info(ab: *mut audit_buffer);
    fn audit_log_end(ab: *mut audit_buffer);
    fn audit_context() -> *const std::ffi::c_void;
    fn __getname() -> *mut c_char;
    fn __putname(name: *mut c_char);
    fn d_absolute_path(
        path: *const path,
        buf: *mut c_char,
        buflen: c_int,
    ) -> *const c_char;
    fn d_real_inode(file: *const dentry) -> *const inode;
    fn file_dentry(file: *const file) -> *const dentry;
    fn memset(s: *mut std::ffi::c_void, c: c_int, n: usize) -> *mut std::ffi::c_void;
    fn memcpy(
        dest: *mut std::ffi::c_void,
        src: *const std::ffi::c_void,
        n: usize,
    ) -> *mut std::ffi::c_void;
    fn hex_byte_pack(buf: *mut c_char, byte: u8);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;

    // External constants
    pub static ima_policy_flag: c_int;
    pub static ima_num_violations: i64;
    pub static ima_tpm_chip: std::ffi::c_void;
    pub static ima_extra_slots: usize;
    pub static hash_digest_size: [usize; 256];
    pub static hash_algo_name: [*const c_char; 256];
}

// External flags
const IMA_MEASURE: c_int = 0x00000001;
const IMA_AUDIT: c_int = 0x00000010;
const IMA_APPRAISE: c_int = 0x00000004;
const IMA_HASH: c_int = 0x00000040;
const IMA_COLLECTED: u32 = 0x00000020;
const IMA_VERITY_REQUIRED: u32 = 0x01000000;
const IMA_MEASURED: u32 = 0x00000100;
const IMA_AUDITED: u32 = 0x00000200;

const GFP_NOFS: c_uint = 0x4000;
const GFP_KERNEL: c_uint = 0x0;
const O_DIRECT: c_int = 0o40000;
const STATX_CHANGE_COOKIE: c_uint = 0x8000;
const AT_STATX_SYNC_AS_STAT: c_uint = 0x0;
const PATH_MAX: c_int = 4096;
const NAME_MAX: usize = 255;

const AUDIT_INTEGRITY_PCR: c_int = 2401;
const AUDIT_INTEGRITY_DATA: c_int = 2402;
const AUDIT_INTEGRITY_RULE: c_int = 2403;

// Helper macro translations
fn NR_BANKS(_ima_tpm_chip: *const std::ffi::c_void) -> usize {
    // This would need to be provided by external kernel code
    // For now, returning a placeholder that would be determined at runtime
    0
}

fn IS_I_VERSION(inode: *const inode) -> bool {
    // This would check inode flags in actual kernel code
    // For now, returning false as a placeholder
    false
}

fn IS_ERR(ptr: *const std::ffi::c_void) -> bool {
    (ptr as usize) > (-4096isize as usize)
}

fn ERR_PTR(err: c_int) -> *const std::ffi::c_void {
    (err as usize) as *const std::ffi::c_void
}

fn PTR_ERR(ptr: *const std::ffi::c_void) -> c_int {
    (ptr as isize) as c_int
}

// container_of macro translation
fn container_of<T, U>(ptr: *const U, member_offset: usize) -> *const T {
    ((ptr as usize) - member_offset) as *const T
}

/*
 * ima_free_template_entry - free an existing template entry
 */
pub unsafe fn ima_free_template_entry(entry: *mut ima_template_entry) {
    if entry.is_null() {
        return;
    }

    let template_desc = (*entry).template_desc;
    if !template_desc.is_null() {
        let num_fields = (*template_desc).num_fields;
        let template_data = &mut (*entry).template_data as *mut std::ffi::c_void;

        for i in 0..num_fields {
            let data_ptr = template_data.add(i * std::mem::size_of::<std::ffi::c_void>())
                as *mut *mut std::ffi::c_void;
            if !(*data_ptr).is_null() {
                kfree(*data_ptr);
            }
        }
    }

    let digests = (*entry).digests;
    if !digests.is_null() {
        kfree(digests as *const std::ffi::c_void);
    }

    kfree(entry as *const std::ffi::c_void);
}

/*
 * ima_alloc_init_template - create and initialize a new template entry
 */
pub unsafe fn ima_alloc_init_template(
    event_data: *const ima_event_data,
    entry: *mut *mut ima_template_entry,
    desc: *const ima_template_desc,
) -> c_int {
    let template_desc = if !desc.is_null() {
        desc
    } else {
        ima_template_desc_current()
    };

    if template_desc.is_null() {
        return -12; // -ENOMEM
    }

    let num_fields = (*template_desc).num_fields;

    // Allocate entry with flexible array
    *entry = kzalloc_flex(
        entry as *const std::ffi::c_void,
        &(*template_desc).num_fields as *const _ as *const std::ffi::c_void,
        num_fields,
        GFP_NOFS,
    ) as *mut ima_template_entry;

    if (*entry).is_null() {
        return -12; // -ENOMEM
    }

    let digests = kzalloc_objs(
        &mut (*entry).digests as *mut _ as *const std::ffi::c_void,
        NR_BANKS(&ima_tpm_chip) + ima_extra_slots,
        GFP_NOFS,
    ) as *mut std::ffi::c_void;

    if digests.is_null() {
        kfree(*entry as *const std::ffi::c_void);
        *entry = ptr::null_mut();
        return -12; // -ENOMEM
    }

    (*(*entry)).digests = digests;
    (*(*entry)).template_desc = template_desc;

    let mut result = 0;
    for i in 0..num_fields {
        let field = (*template_desc).fields[i];
        let len: u32;

        if !field.is_null() {
            let field_init = (*field).field_init;
            let template_data_ptr = &mut (*(*entry)).template_data[i] as *mut std::ffi::c_void;

            result = field_init(event_data, template_data_ptr);
            if result != 0 {
                ima_free_template_entry(*entry);
                *entry = ptr::null_mut();
                return result;
            }

            len = (*(*entry)).template_data[i].len;
            (*(*entry)).template_data_len += std::mem::size_of::<u32>() as u32;
            (*(*entry)).template_data_len += len;
        }
    }

    0
}

/*
 * ima_store_template - store ima template measurements
 *
 * Calculate the hash of a template entry, add the template entry
 * to an ordered list of measurement entries maintained inside the kernel,
 * and also update the aggregate integrity value (maintained inside the
 * configured TPM PCR) over the hashes of the current list of measurement
 * entries.
 *
 * Applications retrieve the current kernel-held measurement list through
 * the securityfs entries in /sys/kernel/security/ima. The signed aggregate
 * TPM PCR (called quote) can be retrieved using a TPM user space library
 * and is used to validate the measurement list.
 *
 * Returns 0 on success, error code otherwise
 */
pub unsafe fn ima_store_template(
    entry: *mut ima_template_entry,
    violation: c_int,
    inode: *const inode,
    filename: *const c_char,
    pcr: c_int,
) -> c_int {
    const OP: &[u8] = b"add_template_measure\0";
    const AUDIT_CAUSE: &[u8] = b"hashing_error\0";

    let template_name = (*(*entry).template_desc).name;
    let mut result: c_int;

    if violation == 0 {
        result = ima_calc_field_array_hash(
            &(*entry).template_data[0] as *const _ as *const std::ffi::c_void,
            entry,
        );
        if result < 0 {
            integrity_audit_msg(
                AUDIT_INTEGRITY_PCR,
                inode,
                template_name,
                OP.as_ptr() as *const c_char,
                AUDIT_CAUSE.as_ptr() as *const c_char,
                result,
                0,
            );
            return result;
        }
    }

    (*entry).pcr = pcr;
    result = ima_add_template_entry(
        entry,
        violation,
        OP.as_ptr() as *const c_char,
        inode,
        filename,
    );
    result
}

/*
 * ima_add_violation - add violation to measurement list.
 *
 * Violations are flagged in the measurement list with zero hash values.
 * By extending the PCR with 0xFF's instead of with zeroes, the PCR
 * value is invalidated.
 */
pub unsafe fn ima_add_violation(
    file: *const file,
    filename: *const c_char,
    iint: *mut ima_iint_cache,
    op: *const c_char,
    cause: *const c_char,
) {
    let inode = file_inode(file);
    let mut entry: *mut ima_template_entry = ptr::null_mut();

    let mut event_data: ima_event_data = std::mem::zeroed();
    event_data.iint = iint;
    event_data.file = file as *mut file;
    event_data.filename = filename;
    event_data.violation = cause;

    atomic_long_inc(&mut ima_num_violations);

    let mut result = ima_alloc_init_template(&event_data, &mut entry, ptr::null());
    if result < 0 {
        result = -12; // -ENOMEM
    } else {
        const CONFIG_IMA_MEASURE_PCR_IDX: c_int = 10;
        result = ima_store_template(entry, 1, inode, filename, CONFIG_IMA_MEASURE_PCR_IDX);
        if result < 0 {
            ima_free_template_entry(entry);
        }
    }

    integrity_audit_msg(AUDIT_INTEGRITY_PCR, inode, filename, op, cause, result, 0);
}

/**
 * ima_get_action - appraise & measure decision based on policy.
 * @idmap: idmap of the mount the inode was found from
 * @inode: pointer to the inode associated with the object being validated
 * @cred: pointer to credentials structure to validate
 * @prop: properties of the task being validated
 * @mask: contains the permission mask (MAY_READ, MAY_WRITE, MAY_EXEC,
 *        MAY_APPEND)
 * @func: caller identifier
 * @pcr: pointer filled in if matched measure policy sets pcr=
 * @template_desc: pointer filled in if matched measure policy sets template=
 * @func_data: func specific data, may be NULL
 * @allowed_algos: allowlist of hash algorithms for the IMA xattr
 *
 * The policy is defined in terms of keypairs:
 *		subj=, obj=, type=, func=, mask=, fsmagic=
 *	subj,obj, and type: are LSM specific.
 *	func: FILE_CHECK | BPRM_CHECK | CREDS_CHECK | MMAP_CHECK | MODULE_CHECK
 *	| KEXEC_CMDLINE | KEY_CHECK | CRITICAL_DATA | SETXATTR_CHECK
 *	| MMAP_CHECK_REQPROT
 *	mask: contains the permission mask
 *	fsmagic: hex value
 *
 * Returns IMA_MEASURE, IMA_APPRAISE mask.
 *
 */
pub unsafe fn ima_get_action(
    idmap: *const mnt_idmap,
    inode: *const inode,
    cred: *const cred,
    prop: *const lsm_prop,
    mask: c_int,
    func: c_uint,
    pcr: *mut c_int,
    template_desc: *mut *const ima_template_desc,
    func_data: *const c_char,
    allowed_algos: *mut c_uint,
) -> c_int {
    let mut flags = IMA_MEASURE | IMA_AUDIT | IMA_APPRAISE | IMA_HASH;

    flags &= ima_policy_flag;

    ima_match_policy(
        idmap,
        inode,
        cred,
        prop,
        func,
        mask,
        flags,
        pcr,
        template_desc,
        func_data,
        allowed_algos,
    )
}

unsafe fn ima_get_verity_digest(
    _iint: *const ima_iint_cache,
    inode: *const inode,
    hash: *mut ima_max_digest_data,
) -> bool {
    let mut alg: c_uint = 0;
    let mut digest_len: c_int;

    /*
     * On failure, 'measure' policy rules will result in a file data
     * hash containing 0's.
     */
    digest_len = fsverity_get_digest(
        inode,
        &mut (*hash).digest as *mut u8,
        ptr::null(),
        &mut alg,
    );

    if digest_len == 0 {
        return false;
    }

    /*
     * Unlike in the case of actually calculating the file hash, in
     * the fsverity case regardless of the hash algorithm, return
     * the verity digest to be included in the measurement list. A
     * mismatch between the verity algorithm and the xattr signature
     * algorithm, if one exists, will be detected later.
     */
    (*hash).hdr.algo = alg;
    (*hash).hdr.length = digest_len as u32;
    true
}

/*
 * ima_collect_measurement - collect file measurement
 *
 * Calculate the file hash, if it doesn't already exist,
 * storing the measurement and i_version in the iint.
 *
 * Must be called with iint->mutex held.
 *
 * Return 0 on success, error code otherwise
 */
pub unsafe fn ima_collect_measurement(
    iint: *mut ima_iint_cache,
    file: *const file,
    buf: *const std::ffi::c_void,
    size: i64,
    algo: c_uint,
    modsig: *const modsig,
) -> c_int {
    let mut audit_cause = "failed\0".as_ptr() as *const c_char;
    let inode = file_inode(file);
    let real_inode = d_real_inode(file_dentry(file));
    let mut hash: ima_max_digest_data = std::mem::zeroed();
    let hash_hdr = &mut hash.hdr as *mut _ as *mut ima_digest_data;
    let mut filename: name_snapshot = std::mem::zeroed();
    let mut stat: kstat = std::mem::zeroed();
    let mut result: c_int = 0;
    let mut length: usize;
    let mut tmpbuf: *mut std::ffi::c_void;
    let mut i_version: u64 = 0;

    /*
     * Always collect the modsig, because IMA might have already collected
     * the file digest without collecting the modsig in a previous
     * measurement rule.
     */
    if !modsig.is_null() {
        ima_collect_modsig(modsig, buf, size);
    }

    if ((*iint).flags & IMA_COLLECTED) != 0 {
        return 0;
    }

    /*
     * Detect file change based on STATX_CHANGE_COOKIE, when supported,
     * and fallback to detecting file change based on i_version.
     *
     * On filesystems which did not support i_version, support was
     * originally limited to an initial measurement/appraisal/audit,
     * but was later modified to assume the file changed.
     */
    result = vfs_getattr_nosec(
        &(*file).f_path,
        &mut stat,
        STATX_CHANGE_COOKIE,
        AT_STATX_SYNC_AS_STAT,
    );

    if result == 0 && (stat.result_mask & STATX_CHANGE_COOKIE) != 0 {
        i_version = stat.change_cookie;
    } else if IS_I_VERSION(real_inode) {
        i_version = inode_peek_iversion(real_inode);
    }

    (*hash).hdr.algo = algo;
    (*hash).hdr.length = hash_digest_size[algo as usize] as u32;

    // Initialize hash digest to 0's in case of failure
    memset(
        &mut (*hash).digest as *mut _ as *mut std::ffi::c_void,
        0,
        std::mem::size_of_val(&(*hash).digest),
    );

    if ((*iint).flags & IMA_VERITY_REQUIRED) != 0 {
        if !ima_get_verity_digest(iint, inode, &mut hash) {
            audit_cause = "no-verity-digest\0".as_ptr() as *const c_char;
            result = -61; // -ENODATA
        }
    } else if !buf.is_null() {
        result = ima_calc_buffer_hash(buf, size, hash_hdr);
    } else {
        result = ima_calc_file_hash(file, hash_hdr);
    }

    if result != 0 && result != -74 && result != -22 {
        // -74 = EBADF, -22 = EINVAL
        goto_out(&filename);
        return result;
    }

    length = std::mem::size_of::<std::ffi::c_void>() + (*hash).hdr.length as usize;
    tmpbuf = krealloc((*iint).ima_hash, length, GFP_NOFS);
    if tmpbuf.is_null() {
        result = -12; // -ENOMEM
        goto_out(&filename);
        return result;
    }

    (*iint).ima_hash = tmpbuf;
    memcpy(
        tmpbuf,
        &hash as *const _ as *const std::ffi::c_void,
        length,
    );

    if real_inode == inode {
        (*iint).real_inode.version = i_version;
    } else {
        integrity_inode_attrs_store(
            &mut (*iint).real_inode as *mut _ as *mut std::ffi::c_void,
            i_version,
            real_inode,
        );
    }

    // Possibly temporary failure due to type of read (eg. O_DIRECT)
    if result == 0 {
        (*iint).flags |= IMA_COLLECTED;
    }

    if result != 0 {
        if ((*file).f_flags & O_DIRECT) != 0 {
            audit_cause = "failed(directio)\0".as_ptr() as *const c_char;
        }

        take_dentry_name_snapshot(&mut filename, (*file).f_path.dentry);

        integrity_audit_msg(
            AUDIT_INTEGRITY_DATA,
            inode,
            filename.name.name,
            "collect_data\0".as_ptr() as *const c_char,
            audit_cause,
            result,
            0,
        );

        release_dentry_name_snapshot(&mut filename);
    }

    result
}

fn goto_out(_filename: *const name_snapshot) {
    // This is a label placeholder; in Rust we handle this with early returns
}

/*
 * ima_store_measurement - store file measurement
 *
 * Create an "ima" template and then store the template by calling
 * ima_store_template.
 *
 * We only get here if the inode has not already been measured,
 * but the measurement could already exist:
 *	- multiple copies of the same file on either the same or
 *	  different filesystems.
 *	- the inode was previously flushed as well as the iint info,
 *	  containing the hashing info.
 *
 * Must be called with iint->mutex held.
 */
pub unsafe fn ima_store_measurement(
    iint: *mut ima_iint_cache,
    file: *const file,
    filename: *const c_char,
    xattr_value: *const evm_ima_xattr_data,
    xattr_len: c_int,
    modsig: *const modsig,
    pcr: c_int,
    template_desc: *const ima_template_desc,
) {
    const OP: &[u8] = b"add_template_measure\0";
    const AUDIT_CAUSE: &[u8] = b"ENOMEM\0";

    let mut result: c_int = -12; // -ENOMEM
    let inode = file_inode(file);
    let mut entry: *mut ima_template_entry = ptr::null_mut();

    let mut event_data: ima_event_data = std::mem::zeroed();
    event_data.iint = iint;
    event_data.file = file as *mut file;
    event_data.filename = filename;
    event_data.xattr_value = xattr_value as *mut evm_ima_xattr_data;
    event_data.xattr_len = xattr_len;
    event_data.modsig = modsig as *mut modsig;

    /*
     * We still need to store the measurement in the case of MODSIG because
     * we only have its contents to put in the list at the time of
     * appraisal, but a file measurement from earlier might already exist in
     * the measurement list.
     */
    if ((*iint).measured_pcrs & (0x1 << pcr)) != 0 && modsig.is_null() {
        return;
    }

    result = ima_alloc_init_template(&event_data, &mut entry, template_desc);
    if result < 0 {
        integrity_audit_msg(
            AUDIT_INTEGRITY_PCR,
            inode,
            filename,
            OP.as_ptr() as *const c_char,
            AUDIT_CAUSE.as_ptr() as *const c_char,
            result,
            0,
        );
        return;
    }

    result = ima_store_template(entry, 0, inode, filename, pcr);
    if (result == 0 || result == -17) && ((*file).f_flags & O_DIRECT) == 0 {
        // -17 = EEXIST
        (*iint).flags |= IMA_MEASURED;
        (*iint).measured_pcrs |= 0x1 << pcr;
    }
    if result < 0 {
        ima_free_template_entry(entry);
    }
}

pub unsafe fn ima_audit_measurement(iint: *const ima_iint_cache, filename: *const c_char) {
    let mut ab: *mut audit_buffer;
    let mut hash: *mut c_char;
    let algo_name = hash_algo_name[(*(*iint).ima_hash).algo as usize];
    let mut i: c_int;

    if ((*iint).flags & IMA_AUDITED) != 0 {
        return;
    }

    hash = kzalloc(
        (((*(*iint).ima_hash).length as c_int * 2) + 1) as c_uint,
        GFP_KERNEL,
    ) as *mut c_char;
    if hash.is_null() {
        return;
    }

    i = 0;
    while i < (*(*iint).ima_hash).length as c_int {
        hex_byte_pack(
            hash.add((i * 2) as usize),
            (*(*iint).ima_hash).digest[i as usize],
        );
        i += 1;
    }
    *hash.add((i * 2) as usize) = 0;

    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_INTEGRITY_RULE);
    if !ab.is_null() {
        audit_log_format(ab, "file=\0".as_ptr() as *const c_char);
        audit_log_untrustedstring(ab, filename);
        audit_log_format(ab, " hash=\"%s:%s\"\0".as_ptr() as *const c_char, algo_name, hash);

        audit_log_task_info(ab);
        audit_log_end(ab);

        (*iint).flags |= IMA_AUDITED;
    }

    kfree(hash as *const std::ffi::c_void);
}

fn kzalloc(size: c_uint, flags: c_uint) -> *mut std::ffi::c_void {
    unsafe {
        let ptr = libc::malloc(size as usize);
        if !ptr.is_null() {
            std::ptr::write_bytes(ptr, 0, size as usize);
        }
        ptr
    }
}

/*
 * ima_d_path - return a pointer to the full pathname
 *
 * Attempt to return a pointer to the full pathname for use in the
 * IMA measurement list, IMA audit records, and auditing logs.
 *
 * On failure, return a pointer to a copy of the filename, not dname.
 * Returning a pointer to dname, could result in using the pointer
 * after the memory has been freed.
 */
pub unsafe fn ima_d_path(
    path: *const path,
    pathbuf: *mut *mut c_char,
    namebuf: *mut c_char,
) -> *const c_char {
    let mut filename: name_snapshot = std::mem::zeroed();
    let mut pathname: *const c_char = ptr::null();

    *pathbuf = __getname();
    if !(*pathbuf).is_null() {
        pathname = d_absolute_path(path, *pathbuf, PATH_MAX);
        if IS_ERR(pathname as *const std::ffi::c_void) {
            __putname(*pathbuf);
            *pathbuf = ptr::null_mut();
            pathname = ptr::null();
        }
    }

    if pathname.is_null() {
        take_dentry_name_snapshot(&mut filename, (*path).dentry);
        strscpy(namebuf, filename.name.name, NAME_MAX);
        release_dentry_name_snapshot(&mut filename);

        pathname = namebuf;
    }

    pathname
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
