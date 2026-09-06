// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005,2006,2007,2008 IBM Corporation
 *
 * Authors:
 * Serge Hallyn <serue@us.ibm.com>
 * Reiner Sailer <sailer@watson.ibm.com>
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_queue.c
 *       Implements queues that store template measurements and
 *       maintains aggregate over the stored measurements
 *       in the pre-configured TPM PCR (if available).
 *       The measurement list is append-only. No entry is
 *       ever removed or changed during the boot-cycle.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const AUDIT_CAUSE_LEN_MAX: usize = 32;

extern "C" {
    static mut ima_tpm_chip: *mut tpm_chip;
    static mut ima_hash_algo_idx: usize;
    static mut ima_hash_algo: usize;
    static hash_digest_size: [usize; 0];

    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn lockdep_is_held(lock: *mut mutex) -> bool;
    fn synchronize_rcu();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn ima_hash_key(digest_value: *mut u8) -> c_uint;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn tpm_pcr_extend(chip: *mut tpm_chip, pcr: c_int, digests: *mut tpm_digest) -> c_int;
    fn integrity_audit_msg(
        audit_msgno: c_int,
        inode: *mut inode,
        fname: *const u8,
        op: *const c_char,
        cause: *const c_char,
        result: c_int,
        info: c_int,
    );
    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_replace(old: *mut list_head, new: *mut list_head);
    fn __list_cut_position(list: *mut list_head, head: *mut list_head, entry: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn hlist_add_head_rcu(n: *mut hlist_node, h: *mut hlist_head);
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn ima_measure_kexec_event(event: *const c_char);
    fn register_reboot_notifier(nb: *mut notifier_block) -> c_int;
}

type c_uint = u32;
type atomic_long_t = atomic_long;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_long {
    pub counter: c_long,
}

type c_long = isize;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_digest {
    pub alg_id: u16,
    pub digest: *mut u8,
}

#[repr(C)]
pub struct tpm_bank_info {
    pub alg_id: u16,
    pub digest_size: u16,
    pub crypto_id: u16,
}

#[repr(C)]
pub struct tpm_chip {
    pub nr_allocated_banks: c_int,
    pub allocated_banks: *mut tpm_bank_info,
}

#[repr(C)]
pub struct ima_template_desc {
    pub name: *const c_char,
    pub num_fields: c_uint,
}

#[repr(C)]
pub struct ima_field_data {
    pub data: *mut c_void,
    pub len: u32,
}

#[repr(C)]
pub struct ima_template_entry {
    pub digests: *mut tpm_digest,
    pub pcr: c_int,
    pub template_desc: *mut ima_template_desc,
    pub template_data_len: c_int,
    pub template_data: *mut ima_field_data,
}

#[repr(C)]
pub struct ima_queue_entry {
    pub later: list_head,
    pub hnext: hlist_node,
    pub entry: *mut ima_template_entry,
}

#[repr(C)]
pub struct ima_kexec_hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum binary_lists {
    BINARY = 0,
    BINARY_FULL = 1,
    BINARY_STAGED = 2,
    BINARY__LAST = 3,
}

const BINARY_LAST_USIZE: usize = binary_lists::BINARY__LAST as usize;
const IMA_MEASURE_HTABLE_SIZE: usize = 0;
const GFP_KERNEL: c_uint = 0;
const GFP_NOFS: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EEXIST: c_int = 17;
const ENOENT: c_int = 2;
const EACCES: c_int = 13;
const ESTALE: c_int = 116;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const TPM_DIGEST_SIZE: usize = 20;
const SHA1_DIGEST_SIZE: u16 = 20;
const HASH_ALGO__LAST: u16 = 0xffff;
const AUDIT_INTEGRITY_PCR: c_int = 0;
const SYS_RESTART: c_ulong = 0x0123_4567;
const NOTIFY_DONE: c_int = 0;

static mut ima_flush_htable: bool = false;

unsafe extern "C" fn ima_flush_htable_setup(_str: *mut c_char) -> c_int {
    if IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE() {
        pr_warn(c"Hash table not enabled, ignoring request to flush\n".as_ptr());
        return 1;
    }

    ima_flush_htable = true;
    1
}

/* __setup("ima_flush_htable", ima_flush_htable_setup); */

/* pre-allocated array of tpm_digest structures to extend a PCR */
static mut digests: *mut tpm_digest = ptr::null_mut();

static mut ima_measurements: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
}; /* list of all measurements */
static mut ima_measurements_staged: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
}; /* list of staged measurements */

/* CONFIG_IMA_KEXEC controls whether the array is initialized to zero or ULONG_MAX. */
static mut binary_runtime_size: [c_ulong; BINARY_LAST_USIZE] =
    [ULONG_MAX; BINARY_LAST_USIZE];

static mut ima_num_records: [atomic_long_t; BINARY_LAST_USIZE] =
    [atomic_long_t { counter: 0 }; BINARY_LAST_USIZE];
static mut ima_num_violations: atomic_long_t = atomic_long_t { counter: 0 };

/* key: inode (before secure-hashing a file) */
static mut ima_htable: *mut hlist_head = ptr::null_mut();

/* mutex protects atomicity of extending and staging measurement list
 * and extending the TPM PCR aggregate. Since tpm_extend can take
 * long (and the tpm driver uses a mutex), we can't use the spinlock.
 */
static mut ima_extend_list_mutex: mutex = mutex { _private: [] };

/*
 * Used internally by the kernel to suspend measurements.
 * Protected by ima_extend_list_mutex.
 */
static mut ima_measurements_suspended: bool = false;

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn ERR_PTR(err: c_long) -> *mut hlist_head {
    err as *mut hlist_head
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

#[inline]
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as usize) >= (usize::MAX - 4095)
}

#[inline]
unsafe fn rcu_replace_pointer<T>(p: *mut *mut T, v: *mut T, _c: bool) -> *mut T {
    let old = *p;
    *p = v;
    old
}

#[inline]
unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T {
    p
}

#[inline]
unsafe fn rcu_dereference_protected<T>(p: *mut T, _c: bool) -> *mut T {
    p
}

#[inline]
unsafe fn atomic_long_inc(v: *mut atomic_long_t) {
    (*v).counter += 1;
}

#[inline]
unsafe fn atomic_long_set(v: *mut atomic_long_t, i: c_long) {
    (*v).counter = i;
}

#[inline]
unsafe fn atomic_long_read(v: *const atomic_long_t) -> c_long {
    (*v).counter
}

#[inline]
unsafe fn atomic_long_sub(i: c_ulong, v: *mut atomic_long_t) {
    (*v).counter -= i as c_long;
}

#[inline]
fn IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE() -> bool {
    false
}

#[inline]
fn IS_ENABLED_CONFIG_IMA_KEXEC() -> bool {
    false
}

/* Callers must call synchronize_rcu() and free the hash table. */
unsafe fn ima_alloc_replace_htable() -> *mut hlist_head {
    let old_htable: *mut hlist_head;
    let new_htable: *mut hlist_head;

    /* Initializing to zeros is equivalent to call HLIST_HEAD_INIT. */
    new_htable = kcalloc(
        IMA_MEASURE_HTABLE_SIZE,
        size_of::<hlist_head>(),
        GFP_KERNEL,
    ) as *mut hlist_head;
    if new_htable.is_null() {
        return ERR_PTR(-(ENOMEM as c_long));
    }

    old_htable = rcu_replace_pointer(
        &mut ima_htable,
        new_htable,
        lockdep_is_held(&mut ima_extend_list_mutex),
    );

    old_htable
}

pub unsafe extern "C" fn ima_init_htable() -> c_int {
    let old_htable: *mut hlist_head;

    mutex_lock(&mut ima_extend_list_mutex);
    old_htable = ima_alloc_replace_htable();
    mutex_unlock(&mut ima_extend_list_mutex);

    if IS_ERR(old_htable) {
        return PTR_ERR(old_htable);
    }

    /* Synchronize_rcu() and kfree() not necessary, only for robustness. */
    synchronize_rcu();
    kfree(old_htable as *const c_void);
    0
}

/* lookup up the digest value in the hash table, and return the entry */
unsafe fn ima_lookup_digest_entry(digest_value: *mut u8, pcr: c_int) -> *mut ima_queue_entry {
    let mut qe: *mut ima_queue_entry;
    let mut ret: *mut ima_queue_entry = ptr::null_mut();
    let htable: *mut hlist_head;
    let key: c_uint;
    let mut rc: c_int;

    key = ima_hash_key(digest_value);
    rcu_read_lock();
    htable = rcu_dereference(ima_htable);
    qe = (*htable.add(key as usize)).first as *mut ima_queue_entry;
    while !qe.is_null() {
        rc = memcmp(
            (*(*(*qe).entry).digests.add(ima_hash_algo_idx)).digest as *const c_void,
            digest_value as *const c_void,
            hash_digest_size[ima_hash_algo],
        );
        if rc == 0 && (*(*qe).entry).pcr == pcr {
            ret = qe;
            break;
        }
        qe = (*qe).hnext.next as *mut ima_queue_entry;
    }
    rcu_read_unlock();
    ret
}

/*
 * Calculate the memory required for serializing a single
 * binary_runtime_measurement list entry, which contains a
 * couple of variable length fields (e.g template name and data).
 */
unsafe fn get_binary_runtime_size(entry: *mut ima_template_entry) -> c_int {
    let mut size: c_int = 0;

    size += size_of::<u32>() as c_int; /* pcr */
    size += TPM_DIGEST_SIZE as c_int;
    size += size_of::<c_int>() as c_int; /* template name size field */
    size += strlen((*(*entry).template_desc).name) as c_int;
    size += size_of_val(&(*entry).template_data_len) as c_int;
    size += (*entry).template_data_len;
    size
}

#[inline]
fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

unsafe fn ima_update_binary_runtime_size(entry: *mut ima_template_entry, binary_list: binary_lists) {
    let size: c_int;
    let idx = binary_list as usize;

    if binary_runtime_size[idx] == ULONG_MAX {
        return;
    }

    size = get_binary_runtime_size(entry);
    binary_runtime_size[idx] = if binary_runtime_size[idx] < ULONG_MAX - size as c_ulong {
        binary_runtime_size[idx] + size as c_ulong
    } else {
        ULONG_MAX
    };
}

/* ima_add_template_entry helper function:
 * - Add template entry to the measurement list and hash table, for
 *   all entries except those carried across kexec.
 *
 * (Called with ima_extend_list_mutex held.)
 */
unsafe fn ima_add_digest_entry(entry: *mut ima_template_entry, update_htable: bool) -> c_int {
    let qe: *mut ima_queue_entry;
    let htable: *mut hlist_head;
    let key: c_uint;

    qe = kzalloc(size_of::<ima_queue_entry>(), GFP_KERNEL) as *mut ima_queue_entry;
    if qe.is_null() {
        pr_err(c"OUT OF MEMORY ERROR creating queue entry\n".as_ptr());
        return -ENOMEM;
    }
    (*qe).entry = entry;

    INIT_LIST_HEAD(&mut (*qe).later);
    list_add_tail_rcu(&mut (*qe).later, &mut ima_measurements);

    htable = rcu_dereference_protected(
        ima_htable,
        lockdep_is_held(&mut ima_extend_list_mutex),
    );

    atomic_long_inc(&mut ima_num_records[binary_lists::BINARY as usize]);
    atomic_long_inc(&mut ima_num_records[binary_lists::BINARY_FULL as usize]);

    if update_htable {
        key = ima_hash_key((*(*entry).digests.add(ima_hash_algo_idx)).digest);
        hlist_add_head_rcu(&mut (*qe).hnext, htable.add(key as usize));
    }

    ima_update_binary_runtime_size(entry, binary_lists::BINARY);
    ima_update_binary_runtime_size(entry, binary_lists::BINARY_FULL);

    0
}

/*
 * Return the amount of memory required for serializing the
 * entire binary_runtime_measurement list, including the ima_kexec_hdr
 * structure.
 */
pub unsafe extern "C" fn ima_get_binary_runtime_size(binary_list: binary_lists) -> c_ulong {
    let val: c_ulong;

    mutex_lock(&mut ima_extend_list_mutex);
    val = binary_runtime_size[binary_list as usize];
    mutex_unlock(&mut ima_extend_list_mutex);

    if val >= ULONG_MAX - size_of::<ima_kexec_hdr>() as c_ulong {
        ULONG_MAX
    } else {
        val + size_of::<ima_kexec_hdr>() as c_ulong
    }
}

unsafe fn ima_pcr_extend(digests_arg: *mut tpm_digest, pcr: c_int) -> c_int {
    let mut result: c_int = 0;

    if ima_tpm_chip.is_null() {
        return result;
    }

    result = tpm_pcr_extend(ima_tpm_chip, pcr, digests_arg);
    if result != 0 {
        pr_err(c"Error Communicating to TPM chip, result: %d\n".as_ptr(), result);
    }
    result
}

/*
 * Add template entry to the measurement list and hash table, and
 * extend the pcr.
 *
 * On systems which support carrying the IMA measurement list across
 * kexec, maintain the total memory size required for serializing the
 * binary_runtime_measurements.
 */
pub unsafe extern "C" fn ima_add_template_entry(
    entry: *mut ima_template_entry,
    violation: c_int,
    op: *const c_char,
    inode: *mut inode,
    filename: *const u8,
) -> c_int {
    let digest: *mut u8 = (*(*entry).digests.add(ima_hash_algo_idx)).digest;
    let mut digests_arg: *mut tpm_digest = (*entry).digests;
    let mut audit_cause: *const c_char = c"hash_added".as_ptr();
    let mut tpm_audit_cause: [c_char; AUDIT_CAUSE_LEN_MAX] = [0; AUDIT_CAUSE_LEN_MAX];
    let mut audit_info: c_int = 1;
    let mut result: c_int = 0;
    let mut tpmresult: c_int = 0;

    mutex_lock(&mut ima_extend_list_mutex);

    /*
     * Avoid appending to the measurement log when the TPM subsystem has
     * been shut down while preparing for system reboot.
     */
    if ima_measurements_suspended {
        audit_cause = c"measurements_suspended".as_ptr();
        audit_info = 0;
        result = -ENODEV;
    } else if violation == 0 && !IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE() {
        if !ima_lookup_digest_entry(digest, (*entry).pcr).is_null() {
            audit_cause = c"hash_exists".as_ptr();
            result = -EEXIST;
        } else {
            result = ima_add_digest_entry(entry, !IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE());
            if result < 0 {
                audit_cause = c"ENOMEM".as_ptr();
                audit_info = 0;
            } else {
                if violation != 0 {
                    /* invalidate pcr */
                    digests_arg = digests;
                }

                tpmresult = ima_pcr_extend(digests_arg, (*entry).pcr);
                if tpmresult != 0 {
                    snprintf(
                        tpm_audit_cause.as_mut_ptr(),
                        AUDIT_CAUSE_LEN_MAX,
                        c"TPM_error(%d)".as_ptr(),
                        tpmresult,
                    );
                    audit_cause = tpm_audit_cause.as_ptr();
                    audit_info = 0;
                }
            }
        }
    } else {
        result = ima_add_digest_entry(entry, !IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE());
        if result < 0 {
            audit_cause = c"ENOMEM".as_ptr();
            audit_info = 0;
        } else {
            if violation != 0 {
                /* invalidate pcr */
                digests_arg = digests;
            }

            tpmresult = ima_pcr_extend(digests_arg, (*entry).pcr);
            if tpmresult != 0 {
                snprintf(
                    tpm_audit_cause.as_mut_ptr(),
                    AUDIT_CAUSE_LEN_MAX,
                    c"TPM_error(%d)".as_ptr(),
                    tpmresult,
                );
                audit_cause = tpm_audit_cause.as_ptr();
                audit_info = 0;
            }
        }
    }

    mutex_unlock(&mut ima_extend_list_mutex);
    integrity_audit_msg(
        AUDIT_INTEGRITY_PCR,
        inode,
        filename,
        op,
        audit_cause,
        result,
        audit_info,
    );
    result
}

/**
 * ima_queue_stage - Stage all measurements
 *
 * If the staged measurements list is empty, the current measurements list is
 * not empty, and measurement is not suspended, move the measurements from the
 * current list to the staged one, and update the number of records and binary
 * run-time size accordingly.
 *
 * Do not allow staging after measurement is suspended, so that dumping
 * measurements can be done in a lockless way.
 *
 * Return: Zero on success, a negative value otherwise.
 */
pub unsafe extern "C" fn ima_queue_stage() -> c_int {
    let mut ret: c_int = 0;

    mutex_lock(&mut ima_extend_list_mutex);
    if !list_empty(&ima_measurements_staged) {
        ret = -EEXIST;
    } else if list_empty(&ima_measurements) {
        ret = -ENOENT;
    } else if ima_measurements_suspended {
        ret = -EACCES;
    } else {
        list_replace(&mut ima_measurements, &mut ima_measurements_staged);
        INIT_LIST_HEAD(&mut ima_measurements);

        atomic_long_set(
            &mut ima_num_records[binary_lists::BINARY_STAGED as usize],
            atomic_long_read(&ima_num_records[binary_lists::BINARY as usize]),
        );
        atomic_long_set(&mut ima_num_records[binary_lists::BINARY as usize], 0);

        if IS_ENABLED_CONFIG_IMA_KEXEC() {
            binary_runtime_size[binary_lists::BINARY_STAGED as usize] =
                binary_runtime_size[binary_lists::BINARY as usize];
            binary_runtime_size[binary_lists::BINARY as usize] = 0;
        }
    }
    mutex_unlock(&mut ima_extend_list_mutex);
    ret
}

/**
 * ima_queue_staged_delete_all - Delete staged measurements
 *
 * Move staged measurements to a temporary list, ima_measurements_trim, update
 * the number of records and the binary run-time size accordingly. Finally,
 * delete measurements in the temporary list.
 *
 * Refuse to delete staged measurements if measurement is suspended, so that
 * dump can be done in a lockless way and user space is notified about staged
 * measurements being carried over to the secondary kernel, so that it does not
 * save them twice.
 *
 * Return: Zero on success, a negative value otherwise.
 */
pub unsafe extern "C" fn ima_queue_staged_delete_all() -> c_int {
    let mut old_queue: *mut hlist_head = ptr::null_mut();
    let mut ima_measurements_trim = list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    };
    INIT_LIST_HEAD(&mut ima_measurements_trim);

    mutex_lock(&mut ima_extend_list_mutex);
    if list_empty(&ima_measurements_staged) {
        mutex_unlock(&mut ima_extend_list_mutex);
        return -ENOENT;
    }

    if ima_measurements_suspended {
        mutex_unlock(&mut ima_extend_list_mutex);
        return -ESTALE;
    }

    list_replace(&mut ima_measurements_staged, &mut ima_measurements_trim);
    INIT_LIST_HEAD(&mut ima_measurements_staged);

    atomic_long_set(&mut ima_num_records[binary_lists::BINARY_STAGED as usize], 0);

    if IS_ENABLED_CONFIG_IMA_KEXEC() {
        binary_runtime_size[binary_lists::BINARY_STAGED as usize] = 0;
    }

    if ima_flush_htable {
        old_queue = ima_alloc_replace_htable();
        if IS_ERR(old_queue) {
            mutex_unlock(&mut ima_extend_list_mutex);
            return PTR_ERR(old_queue);
        }
    }

    mutex_unlock(&mut ima_extend_list_mutex);

    if ima_flush_htable {
        synchronize_rcu();
        kfree(old_queue as *const c_void);
    }

    ima_queue_delete(&mut ima_measurements_trim, ima_flush_htable);
    0
}

/**
 * ima_queue_delete_partial - Delete current measurements
 * @req_value: Number of measurements to delete
 *
 * Delete the requested number of measurements from the current measurements
 * list, and update the number of records and the binary run-time size
 * accordingly.
 *
 * Refuse to delete current measurements if measurement is suspended, so that
 * dump can be done in a lockless way and user space is notified about current
 * measurements being carried over to the secondary kernel, so that it does not
 * save them twice.
 *
 * Return: Zero on success, a negative value otherwise.
 */
pub unsafe extern "C" fn ima_queue_delete_partial(req_value: c_ulong) -> c_int {
    let mut req_value_copy: c_ulong = req_value;
    let mut size_to_remove: c_ulong = 0;
    let mut num_to_remove: c_ulong = 0;
    let mut ima_measurements_trim = list_head {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
    };
    let mut qe: *mut ima_queue_entry;
    let ret: c_int = 0;
    INIT_LIST_HEAD(&mut ima_measurements_trim);

    /*
     * list_for_each_entry_rcu() without rcu_read_lock() is fine because
     * only list append can happen concurrently. No list replace due to the
     * staging/delete writers mutual exclusion.
     */
    qe = ima_measurements.next as *mut ima_queue_entry;
    while !qe.is_null() && &mut (*qe).later as *mut list_head != &mut ima_measurements {
        size_to_remove += get_binary_runtime_size((*qe).entry) as c_ulong;
        num_to_remove += 1;

        req_value_copy = req_value_copy.wrapping_sub(1);
        if req_value_copy == 0 {
            break;
        }
        qe = (*qe).later.next as *mut ima_queue_entry;
    }

    /* Not enough records to delete. */
    if req_value_copy > 0 {
        return -ENOENT;
    }

    mutex_lock(&mut ima_extend_list_mutex);
    if ima_measurements_suspended {
        mutex_unlock(&mut ima_extend_list_mutex);
        return -ESTALE;
    }

    /*
     * qe remains valid because ima_fs.c enforces single-writer exclusion.
     */
    __list_cut_position(
        &mut ima_measurements_trim,
        &mut ima_measurements,
        &mut (*qe).later,
    );

    atomic_long_sub(num_to_remove, &mut ima_num_records[binary_lists::BINARY as usize]);

    if IS_ENABLED_CONFIG_IMA_KEXEC() {
        binary_runtime_size[binary_lists::BINARY as usize] -= size_to_remove;
    }

    mutex_unlock(&mut ima_extend_list_mutex);

    ima_queue_delete(&mut ima_measurements_trim, false);
    ret
}

/**
 * ima_queue_delete - Delete measurements
 * @head: List head measurements are deleted from
 * @flush_htable: Whether or not the hash table is being flushed
 *
 * Delete the measurements from the passed list head completely if the
 * hash table is not enabled or is being flushed, or partially (only the
 * template data), if the hash table is used.
 */
unsafe fn ima_queue_delete(head: *mut list_head, flush_htable: bool) {
    let mut qe: *mut ima_queue_entry;
    let mut qe_tmp: *mut ima_queue_entry;
    let mut i: c_uint;

    qe = (*head).next as *mut ima_queue_entry;
    while !qe.is_null() && &mut (*qe).later as *mut list_head != head {
        qe_tmp = (*qe).later.next as *mut ima_queue_entry;
        /*
         * Safe to free template_data here without synchronize_rcu()
         * because the only htable reader, ima_lookup_digest_entry(),
         * accesses only entry->digests, not template_data. If new
         * htable readers are added that access template_data, a
         * synchronize_rcu() is required here.
         */
        i = 0;
        while i < (*(*(*qe).entry).template_desc).num_fields {
            kfree((*(*(*qe).entry).template_data.add(i as usize)).data);
            (*(*(*qe).entry).template_data.add(i as usize)).data = ptr::null_mut();
            (*(*(*qe).entry).template_data.add(i as usize)).len = 0;
            i += 1;
        }

        list_del(&mut (*qe).later);

        /* No leak if condition is false, referenced by ima_htable. */
        if IS_ENABLED_CONFIG_IMA_DISABLE_HTABLE() || flush_htable {
            kfree((*(*qe).entry).digests as *const c_void);
            kfree((*qe).entry as *const c_void);
            kfree(qe as *const c_void);
        }
        qe = qe_tmp;
    }
}

pub unsafe extern "C" fn ima_restore_measurement_entry(entry: *mut ima_template_entry) -> c_int {
    let result: c_int;

    mutex_lock(&mut ima_extend_list_mutex);
    result = ima_add_digest_entry(entry, false);
    mutex_unlock(&mut ima_extend_list_mutex);
    result
}

unsafe fn ima_measurements_suspend() {
    mutex_lock(&mut ima_extend_list_mutex);
    ima_measurements_suspended = true;
    mutex_unlock(&mut ima_extend_list_mutex);
}

unsafe extern "C" fn ima_reboot_notifier(
    _nb: *mut notifier_block,
    action: c_ulong,
    data: *mut c_void,
) -> c_int {
    /* CONFIG_IMA_KEXEC: measure the kexec reboot event before suspending. */
    if IS_ENABLED_CONFIG_IMA_KEXEC()
        && action == SYS_RESTART
        && !data.is_null()
        && strcmp(data as *const c_char, c"kexec reboot".as_ptr()) == 0
    {
        ima_measure_kexec_event(c"kexec_execute".as_ptr());
    }

    ima_measurements_suspend();

    NOTIFY_DONE
}

static mut ima_reboot_nb: notifier_block = notifier_block {
    notifier_call: Some(ima_reboot_notifier),
};

pub unsafe extern "C" fn ima_init_reboot_notifier() {
    register_reboot_notifier(&mut ima_reboot_nb);
}

pub unsafe extern "C" fn ima_init_digests() -> c_int {
    let mut digest_size: u16;
    let crypto_id: u16;
    let mut i: c_int;

    if ima_tpm_chip.is_null() {
        return 0;
    }

    digests = kzalloc(
        size_of::<tpm_digest>() * (*ima_tpm_chip).nr_allocated_banks as usize,
        GFP_NOFS,
    ) as *mut tpm_digest;
    if digests.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*ima_tpm_chip).nr_allocated_banks {
        (*digests.add(i as usize)).alg_id =
            (*(*ima_tpm_chip).allocated_banks.add(i as usize)).alg_id;
        digest_size = (*(*ima_tpm_chip).allocated_banks.add(i as usize)).digest_size;
        crypto_id = (*(*ima_tpm_chip).allocated_banks.add(i as usize)).crypto_id;

        /* for unmapped TPM algorithms digest is still a padded SHA1 */
        if crypto_id == HASH_ALGO__LAST {
            digest_size = SHA1_DIGEST_SIZE;
        }

        ptr::write_bytes((*digests.add(i as usize)).digest, 0xff, digest_size as usize);
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
