// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007 Casey Schaufler <casey@schaufler-ca.com>
 *
 * Author:
 *      Casey Schaufler <casey@schaufler-ca.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub type u32 = core::ffi::c_uint;
pub type gfp_t = core::ffi::c_uint;
pub type bool_t = bool;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_cache {
    pub free: *mut c_void,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_secattr_mls {
    pub lvl: c_int,
    pub cat: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_secattr_attr {
    pub secid: u32,
    pub mls: netlbl_lsm_secattr_mls,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_secattr {
    pub flags: u32,
    pub domain: *mut c_char,
    pub cache: *mut netlbl_lsm_cache,
    pub attr: netlbl_lsm_secattr_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smack_known {
    pub list: list_head,
    pub smk_hashed: hlist_node,
    pub smk_known: *mut c_char,
    pub smk_secid: u32,
    pub smk_rules: list_head,
    pub smk_rules_lock: mutex,
    pub smk_netlabel: netlbl_lsm_secattr,
}

#[repr(C)]
pub struct smack_rule {
    pub list: list_head,
    pub smk_subject: *mut smack_known,
    pub smk_object: *mut smack_known,
    pub smk_access: c_int,
}

#[repr(C)]
pub struct task_smack {
    pub smk_task: *mut smack_known,
    pub smk_rules: list_head,
}

#[repr(C)]
pub struct smk_audit_info {
    pub a: common_audit_data,
}

#[repr(C)]
pub struct common_audit_data {
    pub smack_audit_data: *mut smack_audit_data,
}

#[repr(C)]
pub struct smack_audit_data {
    pub function: *const c_char,
    pub subject: *mut c_char,
    pub object: *mut c_char,
    pub request: *mut c_char,
    pub result: c_int,
    pub subj_tsk: *mut task_struct,
}

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub flags: c_uint,
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smack_known_list_elem {
    pub list: list_head,
    pub smk_label: *mut smack_known,
}

pub const MAY_READ: c_int = 0x00000004;
pub const MAY_WRITE: c_int = 0x00000002;
pub const MAY_EXEC: c_int = 0x00000001;
pub const MAY_APPEND: c_int = 0x00000008;
pub const MAY_LOCK: c_int = 0x00000020;
pub const MAY_NOT: c_int = 0;
pub const MAY_ANYREAD: c_int = MAY_READ;
pub const MAY_TRANSMUTE: c_int = 0x00000100;
pub const MAY_BRINGUP: c_int = 0x00000200;

pub const EACCES: c_int = 13;
pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
pub const ENOMEM: c_int = 12;

pub const GFP_NOFS: gfp_t = 0;
pub const GFP_ATOMIC: gfp_t = 0;
pub const CAP_MAC_OVERRIDE: c_int = 32;
pub const CAP_OPT_NONE: c_int = 0;
pub const PF_KTHREAD: c_uint = 0x00200000;

pub const SMACK_AUDIT_DENIED: c_int = 1;
pub const SMACK_AUDIT_ACCEPT: c_int = 2;
pub const SMACK_BRINGUP_ALLOW: c_int = 1;
pub const SMACK_UNCONFINED_OBJECT: c_int = 2;
pub const SMACK_UNCONFINED_SUBJECT: c_int = 3;
pub const SMK_NUM_ACCESS_TYPE: usize = 6;
pub const SMK_LONGLABEL: c_int = 256;
pub const SMK_CIPSOLEN: c_int = 24;
pub const SMACK_HASH_SLOTS: usize = 16;

pub const NETLBL_SECATTR_MLS_CAT: u32 = 0x0001;
pub const NETLBL_SECATTR_CACHE: u32 = 0x0002;
pub const NETLBL_SECATTR_SECID: u32 = 0x0004;
pub const NETLBL_SECATTR_MLS_LVL: u32 = 0x0008;
pub const NETLBL_SECATTR_DOMAIN: u32 = 0x0010;

const QUESTION: &[u8] = b"?\0";
const HAT: &[u8] = b"^\0";
const STAR: &[u8] = b"*\0";
const FLOOR: &[u8] = b"_\0";
const WEB: &[u8] = b"@\0";
const UNKNOWN: &[u8] = b"unknown\0";
const DENIED: &[u8] = b"denied\0";
const GRANTED: &[u8] = b"granted\0";
const AUDIT_FMT1: &[u8] = b"lsm=SMACK fn=%s action=%s\0";
const AUDIT_SUBJECT: &[u8] = b" subject=\0";
const AUDIT_OBJECT: &[u8] = b" object=\0";
const AUDIT_LABELS_DIFFER: &[u8] = b" labels_differ\0";
const AUDIT_REQUESTED: &[u8] = b" requested=%s\0";
const AUDIT_SUBJ_PID: &[u8] = b" subj_pid=%d subj_comm=\0";
const US_SUFFIX: &[u8] = b"(US)\0";
const UO_SUFFIX: &[u8] = b"(UO)\0";

const fn smack_known_init(name: *mut c_char, secid: u32) -> smack_known {
    smack_known {
        list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
        smk_hashed: hlist_node { next: ptr::null_mut(), pprev: ptr::null_mut() },
        smk_known: name,
        smk_secid: secid,
        smk_rules: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
        smk_rules_lock: mutex { _private: [] },
        smk_netlabel: netlbl_lsm_secattr {
            flags: 0,
            domain: ptr::null_mut(),
            cache: ptr::null_mut(),
            attr: netlbl_lsm_secattr_attr {
                secid: 0,
                mls: netlbl_lsm_secattr_mls { lvl: 0, cat: ptr::null_mut() },
            },
        },
    }
}

#[no_mangle]
pub static mut smack_known_huh: smack_known = smack_known_init(QUESTION.as_ptr() as *mut c_char, 2);

#[no_mangle]
pub static mut smack_known_hat: smack_known = smack_known_init(HAT.as_ptr() as *mut c_char, 3);
#[no_mangle]
pub static mut smack_known_star: smack_known = smack_known_init(STAR.as_ptr() as *mut c_char, 4);
#[no_mangle]
pub static mut smack_known_floor: smack_known = smack_known_init(FLOOR.as_ptr() as *mut c_char, 5);
#[no_mangle]
pub static mut smack_known_web: smack_known = smack_known_init(WEB.as_ptr() as *mut c_char, 7);

#[no_mangle]
pub static mut smack_known_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

/*
 * The initial value needs to be bigger than any of the
 * known values above.
 */
static mut smack_next_secid: u32 = 10;

/* CONFIG_AUDIT: what events do we log; can be overwritten at run-time by /smack/logging */
#[no_mangle]
pub static mut log_policy: c_int = SMACK_AUDIT_DENIED;

#[no_mangle]
pub static mut smack_known_lock: mutex = mutex { _private: [] };

#[no_mangle]
pub static mut smack_known_hash: [hlist_head; SMACK_HASH_SLOTS] =
    [hlist_head { first: ptr::null_mut() }; SMACK_HASH_SLOTS];

#[no_mangle]
pub static mut smack_onlycap_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

#[no_mangle]
pub static mut smack_onlycap_lock: mutex = mutex { _private: [] };

unsafe extern "C" {
    static mut smack_unconfined: *mut smack_known;
    static mut smack_cipso_direct: c_int;
    static mut smack_cipso_mapped: c_int;
    static mut init_user_ns: user_namespace;
    static mut current: *mut task_struct;

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn smack_cred(cred: *const cred) -> *mut task_smack;
    fn current_cred() -> *const cred;
    fn full_name_hash(salt: *const c_void, name: *const c_char, len: usize) -> c_uint;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kstrndup(s: *const c_char, max: usize, gfp: gfp_t) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn kmalloc(size: usize, gfp: gfp_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn hlist_add_head_rcu(n: *mut hlist_node, h: *mut hlist_head);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn netlbl_catmap_setbit(catmap: *mut *mut c_void, bit: c_int, flags: gfp_t) -> c_int;
    fn netlbl_catmap_free(catmap: *mut c_void);
    fn netlbl_secattr_cache_alloc(flags: gfp_t) -> *mut netlbl_lsm_cache;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn common_lsm_audit(a: *mut common_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>, data: *mut c_void);
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, string: *const c_char);
    fn task_tgid_nr(tsk: *mut task_struct) -> c_int;
    fn get_task_comm(buf: *mut c_char, tsk: *mut task_struct) -> *mut c_char;
    fn cap_capable(cred: *const cred, ns: *mut user_namespace, cap: c_int, opts: c_int) -> c_int;
}

#[inline]
unsafe fn init_list_head(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

#[inline]
unsafe fn err_ptr(err: c_int) -> *mut c_char {
    err as isize as *mut c_char
}

#[inline]
unsafe fn err_ptr_smack(err: c_int) -> *mut smack_known {
    err as isize as *mut smack_known
}

#[inline]
unsafe fn is_err(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn err_cast(ptr: *mut c_char) -> *mut smack_known {
    ptr as *mut smack_known
}

#[inline]
unsafe fn kzalloc_smack_known(gfp: gfp_t) -> *mut smack_known {
    let p = kmalloc(size_of::<smack_known>(), gfp) as *mut smack_known;
    if !p.is_null() {
        memset(p as *mut c_void, 0, size_of::<smack_known>());
    }
    p
}

/**
 * smk_access_entry - look up matching access rule
 * @subject_label: a pointer to the subject's Smack label
 * @object_label: a pointer to the object's Smack label
 * @rule_list: the list of rules to search
 *
 * This function looks up the subject/object pair in the
 * access rule list and returns the access mode. If no
 * entry is found returns -ENOENT.
 *
 * NOTE:
 *
 * Earlier versions of this function allowed for labels that
 * were not on the label list. This was done to allow for
 * labels to come over the network that had never been seen
 * before on this host. Unless the receiving socket has the
 * star label this will always result in a failure check. The
 * star labeled socket case is now handled in the networking
 * hooks so there is no case where the label is not on the
 * label list. Checking to see if the address of two labels
 * is the same is now a reliable test.
 *
 * Do the object check first because that is more
 * likely to differ.
 *
 * Allowing write access implies allowing locking.
 */
#[no_mangle]
pub unsafe extern "C" fn smk_access_entry(
    subject_label: *mut c_char,
    object_label: *mut c_char,
    rule_list: *mut list_head,
) -> c_int {
    let mut pos = (*rule_list).next;
    while pos != rule_list {
        let srp = pos as *mut smack_rule;
        if (*(*srp).smk_object).smk_known == object_label
            && (*(*srp).smk_subject).smk_known == subject_label
        {
            let mut may = (*srp).smk_access;
            /*
             * MAY_WRITE implies MAY_LOCK.
             */
            if (may & MAY_WRITE) == MAY_WRITE {
                may |= MAY_LOCK;
            }
            return may;
        }
        pos = (*pos).next;
    }

    -ENOENT
}

/**
 * smk_access - determine if a subject has a specific access to an object
 */
#[no_mangle]
pub unsafe extern "C" fn smk_access(
    subject: *mut smack_known,
    object: *mut smack_known,
    request: c_int,
    a: *mut smk_audit_info,
) -> c_int {
    let mut may = MAY_NOT;
    let mut rc = 0;

    if subject == &raw mut smack_known_star {
        rc = -EACCES;
        goto_out_audit(subject, object, request, rc, a);
        return rc;
    }
    if object == &raw mut smack_known_web || subject == &raw mut smack_known_web {
        goto_out_audit(subject, object, request, rc, a);
        return rc;
    }
    if object == &raw mut smack_known_star {
        goto_out_audit(subject, object, request, rc, a);
        return rc;
    }
    if (*subject).smk_known == (*object).smk_known {
        goto_out_audit(subject, object, request, rc, a);
        return rc;
    }
    if (request & MAY_ANYREAD) == request || (request & MAY_LOCK) == request {
        if object == &raw mut smack_known_floor {
            goto_out_audit(subject, object, request, rc, a);
            return rc;
        }
        if subject == &raw mut smack_known_hat {
            goto_out_audit(subject, object, request, rc, a);
            return rc;
        }
    }

    rcu_read_lock();
    may = smk_access_entry((*subject).smk_known, (*object).smk_known, &mut (*subject).smk_rules);
    rcu_read_unlock();

    if may <= 0 || (request & may) != request {
        rc = -EACCES;
    } else if (may & MAY_BRINGUP) != 0 {
        /* CONFIG_SECURITY_SMACK_BRINGUP */
        rc = SMACK_BRINGUP_ALLOW;
    }

    if rc < 0 {
        /* CONFIG_SECURITY_SMACK_BRINGUP */
        if object == smack_unconfined {
            rc = SMACK_UNCONFINED_OBJECT;
        }
        if subject == smack_unconfined {
            rc = SMACK_UNCONFINED_SUBJECT;
        }
    }

    goto_out_audit(subject, object, request, rc, a);
    rc
}

unsafe fn goto_out_audit(
    subject: *mut smack_known,
    object: *mut smack_known,
    request: c_int,
    rc: c_int,
    a: *mut smk_audit_info,
) {
    /* CONFIG_AUDIT */
    if !a.is_null() {
        smack_log((*subject).smk_known, (*object).smk_known, request, rc, a);
    }
}

/**
 * smk_tskacc - determine if a task has a specific access to an object
 */
#[no_mangle]
pub unsafe extern "C" fn smk_tskacc(
    tsp: *mut task_smack,
    obj_known: *mut smack_known,
    mode: u32,
    a: *mut smk_audit_info,
) -> c_int {
    let sbj_known = smk_of_task(tsp);
    let may: c_int;
    let mut rc: c_int;

    rc = smk_access(sbj_known, obj_known, mode as c_int, ptr::null_mut());
    if rc >= 0 {
        may = smk_access_entry((*sbj_known).smk_known, (*obj_known).smk_known, &mut (*tsp).smk_rules);
        if may >= 0 && (mode as c_int & may) != mode as c_int {
            rc = -EACCES;
        }
    }

    if rc != 0 && smack_privileged(CAP_MAC_OVERRIDE) {
        rc = 0;
    }

    if !a.is_null() {
        smack_log((*sbj_known).smk_known, (*obj_known).smk_known, mode as c_int, rc, a);
    }
    rc
}

#[inline]
unsafe fn smk_of_task(tsp: *mut task_smack) -> *mut smack_known {
    (*tsp).smk_task
}

/**
 * smk_curacc - determine if current has a specific access to an object
 */
#[no_mangle]
pub unsafe extern "C" fn smk_curacc(
    obj_known: *mut smack_known,
    mode: u32,
    a: *mut smk_audit_info,
) -> c_int {
    let tsp = smack_cred(current_cred());

    smk_tskacc(tsp, obj_known, mode, a)
}

/**
 * smack_str_from_perm : helper to translate an int to a
 * readable string
 */
#[no_mangle]
pub unsafe extern "C" fn smack_str_from_perm(string: *mut c_char, access: c_int) -> c_int {
    let mut i: c_int = 0;

    if (access & MAY_READ) != 0 {
        *string.add(i as usize) = b'r' as c_char;
        i += 1;
    }
    if (access & MAY_WRITE) != 0 {
        *string.add(i as usize) = b'w' as c_char;
        i += 1;
    }
    if (access & MAY_EXEC) != 0 {
        *string.add(i as usize) = b'x' as c_char;
        i += 1;
    }
    if (access & MAY_APPEND) != 0 {
        *string.add(i as usize) = b'a' as c_char;
        i += 1;
    }
    if (access & MAY_TRANSMUTE) != 0 {
        *string.add(i as usize) = b't' as c_char;
        i += 1;
    }
    if (access & MAY_LOCK) != 0 {
        *string.add(i as usize) = b'l' as c_char;
        i += 1;
    }
    if (access & MAY_BRINGUP) != 0 {
        *string.add(i as usize) = b'b' as c_char;
        i += 1;
    }
    if i == 0 {
        *string.add(i as usize) = b'-' as c_char;
        i += 1;
    }
    *string.add(i as usize) = 0;
    i
}

/**
 * smack_log_callback - SMACK specific information
 * will be called by generic audit code
 */
unsafe extern "C" fn smack_log_callback(ab: *mut audit_buffer, a: *mut c_void) {
    let ad = a as *mut common_audit_data;
    let sad = (*ad).smack_audit_data;
    audit_log_format(
        ab,
        AUDIT_FMT1.as_ptr() as *const c_char,
        (*(*ad).smack_audit_data).function,
        if (*sad).result != 0 { DENIED.as_ptr() } else { GRANTED.as_ptr() },
    );
    audit_log_format(ab, AUDIT_SUBJECT.as_ptr() as *const c_char);
    audit_log_untrustedstring(ab, (*sad).subject);
    audit_log_format(ab, AUDIT_OBJECT.as_ptr() as *const c_char);
    audit_log_untrustedstring(ab, (*sad).object);
    if *(*sad).request == 0 {
        audit_log_format(ab, AUDIT_LABELS_DIFFER.as_ptr() as *const c_char);
    } else {
        audit_log_format(ab, AUDIT_REQUESTED.as_ptr() as *const c_char, (*sad).request);
    }

    if !(*sad).subj_tsk.is_null() {
        let mut comm = [0 as c_char; 16];

        audit_log_format(
            ab,
            AUDIT_SUBJ_PID.as_ptr() as *const c_char,
            task_tgid_nr((*sad).subj_tsk),
        );
        audit_log_untrustedstring(ab, get_task_comm(comm.as_mut_ptr(), (*sad).subj_tsk));
    }
}

/**
 *  smack_log - Audit the granting or denial of permissions.
 */
#[no_mangle]
pub unsafe extern "C" fn smack_log(
    subject_label: *mut c_char,
    object_label: *mut c_char,
    request: c_int,
    mut result: c_int,
    ad: *mut smk_audit_info,
) {
    let mut request_buffer = [0 as c_char; SMK_NUM_ACCESS_TYPE + 5];
    let sad: *mut smack_audit_data;
    let a = &mut (*ad).a as *mut common_audit_data;

    if result < 0 && (log_policy & SMACK_AUDIT_DENIED) == 0 {
        return;
    }
    if result == 0 && (log_policy & SMACK_AUDIT_ACCEPT) == 0 {
        return;
    }

    sad = (*a).smack_audit_data;

    if (*sad).function.is_null() {
        (*sad).function = UNKNOWN.as_ptr() as *const c_char;
    }

    smack_str_from_perm(request_buffer.as_mut_ptr(), request);
    (*sad).subject = subject_label;
    (*sad).object = object_label;
    /* CONFIG_SECURITY_SMACK_BRINGUP */
    if result == SMACK_UNCONFINED_SUBJECT {
        strcat(request_buffer.as_mut_ptr(), US_SUFFIX.as_ptr() as *const c_char);
    } else if result == SMACK_UNCONFINED_OBJECT {
        strcat(request_buffer.as_mut_ptr(), UO_SUFFIX.as_ptr() as *const c_char);
    }

    if result > 0 {
        result = 0;
    }
    (*sad).request = request_buffer.as_mut_ptr();
    (*sad).result = result;

    common_lsm_audit(a, Some(smack_log_callback), ptr::null_mut());
}

/**
 * smk_insert_entry - insert a smack label into a hash map,
 */
#[no_mangle]
pub unsafe extern "C" fn smk_insert_entry(skp: *mut smack_known) {
    let hash: c_uint;
    let head: *mut hlist_head;

    hash = full_name_hash(ptr::null(), (*skp).smk_known, strlen((*skp).smk_known));
    head = &mut smack_known_hash[(hash as usize) & (SMACK_HASH_SLOTS - 1)];

    hlist_add_head_rcu(&mut (*skp).smk_hashed, head);
    list_add_rcu(&mut (*skp).list, &mut smack_known_list);
}

/**
 * smk_find_entry - find a label on the list, return the list entry
 */
#[no_mangle]
pub unsafe extern "C" fn smk_find_entry(string: *const c_char) -> *mut smack_known {
    let hash: c_uint;
    let head: *mut hlist_head;
    let mut node: *mut hlist_node;

    hash = full_name_hash(ptr::null(), string, strlen(string));
    head = &mut smack_known_hash[(hash as usize) & (SMACK_HASH_SLOTS - 1)];

    node = (*head).first;
    while !node.is_null() {
        let skp = node as *mut smack_known;
        if strcmp((*skp).smk_known, string) == 0 {
            return skp;
        }
        node = (*node).next;
    }

    ptr::null_mut()
}

/**
 * smk_parse_label_len - calculate the length of the starting segment
 */
#[no_mangle]
pub unsafe extern "C" fn smk_parse_label_len(string: *const c_char, mut len: c_int) -> c_int {
    let mut i: c_int;

    if len <= 0 || len > SMK_LONGLABEL {
        len = SMK_LONGLABEL;
    }

    if *string == b'-' as c_char {
        return -EINVAL;
    }

    i = 0;
    while i < len {
        let ch = *string.add(i as usize);
        if ch > b'~' as c_char
            || ch <= b' ' as c_char
            || ch == b'/' as c_char
            || ch == b'"' as c_char
            || ch == b'\\' as c_char
            || ch == b'\'' as c_char
        {
            break;
        }
        i += 1;
    }

    if i == 0 || i >= SMK_LONGLABEL {
        return -EINVAL;
    }

    i
}

/**
 * smk_parse_smack - copy the starting segment in the string
 */
#[no_mangle]
pub unsafe extern "C" fn smk_parse_smack(string: *const c_char, len: c_int) -> *mut c_char {
    let i = smk_parse_label_len(string, len);

    if i < 0 {
        return err_ptr(-EINVAL);
    }

    let smack = kstrndup(string, i as usize, GFP_NOFS);
    if smack.is_null() {
        return err_ptr(-ENOMEM);
    }
    smack
}

/**
 * smk_netlbl_mls - convert a catset to netlabel mls categories
 */
#[no_mangle]
pub unsafe extern "C" fn smk_netlbl_mls(
    level: c_int,
    catset: *mut c_char,
    sap: *mut netlbl_lsm_secattr,
    len: c_int,
) -> c_int {
    let mut cp: *mut c_uchar = catset as *mut c_uchar;
    let mut cat: c_int;
    let mut byte: c_int;

    (*sap).flags |= NETLBL_SECATTR_MLS_CAT;
    (*sap).attr.mls.lvl = level;
    (*sap).attr.mls.cat = ptr::null_mut();

    cat = 1;
    byte = 0;
    while byte < len {
        let mut m: c_uchar = 0x80;
        while m != 0 {
            if (m & *cp) != 0 {
                let rc = netlbl_catmap_setbit(&mut (*sap).attr.mls.cat, cat, GFP_NOFS);
                if rc < 0 {
                    netlbl_catmap_free((*sap).attr.mls.cat);
                    return rc;
                }
            }
            m >>= 1;
            cat += 1;
        }
        cp = cp.add(1);
        byte += 1;
    }

    0
}

type c_uchar = u8;

/**
 * smack_populate_secattr - fill in the smack_known netlabel information
 */
#[no_mangle]
pub unsafe extern "C" fn smack_populate_secattr(skp: *mut smack_known) -> c_int {
    let slen: c_int;

    (*skp).smk_netlabel.attr.secid = (*skp).smk_secid;
    (*skp).smk_netlabel.domain = (*skp).smk_known;
    (*skp).smk_netlabel.cache = netlbl_secattr_cache_alloc(GFP_ATOMIC);
    if !(*skp).smk_netlabel.cache.is_null() {
        (*skp).smk_netlabel.flags |= NETLBL_SECATTR_CACHE;
        (*(*skp).smk_netlabel.cache).free = ptr::null_mut();
        (*(*skp).smk_netlabel.cache).data = skp as *mut c_void;
    }
    (*skp).smk_netlabel.flags |= NETLBL_SECATTR_SECID | NETLBL_SECATTR_MLS_LVL | NETLBL_SECATTR_DOMAIN;

    slen = strlen((*skp).smk_known) as c_int;
    if slen < SMK_CIPSOLEN {
        return smk_netlbl_mls(smack_cipso_direct, (*skp).smk_known, &mut (*skp).smk_netlabel, slen);
    }

    smk_netlbl_mls(
        smack_cipso_mapped,
        &mut (*skp).smk_secid as *mut u32 as *mut c_char,
        &mut (*skp).smk_netlabel,
        size_of::<u32>() as c_int,
    )
}

/**
 * smk_import_valid_allocated_label - import a label, return the list entry
 */
unsafe fn smk_import_allocated_label(smack: *mut c_char, gfp: gfp_t) -> *mut smack_known {
    let mut skp: *mut smack_known;
    let rc: c_int;

    mutex_lock(&mut smack_known_lock);

    skp = smk_find_entry(smack);
    if !skp.is_null() {
        kfree(smack as *mut c_void);
        mutex_unlock(&mut smack_known_lock);
        return skp;
    }

    skp = kzalloc_smack_known(gfp);
    if skp.is_null() {
        skp = err_ptr_smack(-ENOMEM);
        kfree(smack as *mut c_void);
        mutex_unlock(&mut smack_known_lock);
        return skp;
    }

    (*skp).smk_known = smack;
    (*skp).smk_secid = smack_next_secid;
    smack_next_secid = smack_next_secid.wrapping_add(1);

    rc = smack_populate_secattr(skp);
    if rc >= 0 {
        init_list_head(&mut (*skp).smk_rules);
        mutex_init(&mut (*skp).smk_rules_lock);
        smk_insert_entry(skp);
        mutex_unlock(&mut smack_known_lock);
        return skp;
    }
    kfree(skp as *mut c_void);
    skp = err_ptr_smack(rc);
    kfree(smack as *mut c_void);
    mutex_unlock(&mut smack_known_lock);

    skp
}

/**
 * smk_import_entry - import a label, return the list entry
 */
#[no_mangle]
pub unsafe extern "C" fn smk_import_entry(string: *const c_char, len: c_int) -> *mut smack_known {
    let smack = smk_parse_smack(string, len);

    if is_err(smack as *const c_void) {
        return err_cast(smack);
    }

    smk_import_allocated_label(smack, GFP_NOFS)
}

/**
 * smk_import_valid_label - import a label, return the list entry
 */
#[no_mangle]
pub unsafe extern "C" fn smk_import_valid_label(
    label: *const c_char,
    label_len: c_int,
    gfp: gfp_t,
) -> *mut smack_known {
    let smack = kstrndup(label, label_len as usize, gfp);

    if smack.is_null() {
        return err_ptr_smack(-ENOMEM);
    }

    smk_import_allocated_label(smack, gfp)
}

/**
 * smack_from_secid - find the Smack label associated with a secid
 */
#[no_mangle]
pub unsafe extern "C" fn smack_from_secid(secid: u32) -> *mut smack_known {
    rcu_read_lock();
    let mut pos = smack_known_list.next;
    while pos != &raw mut smack_known_list {
        let skp = pos as *mut smack_known;
        if (*skp).smk_secid == secid {
            rcu_read_unlock();
            return skp;
        }
        pos = (*pos).next;
    }

    /*
     * If we got this far someone asked for the translation
     * of a secid that is not on the list.
     */
    rcu_read_unlock();
    &raw mut smack_known_huh
}

/**
 * smack_privileged_cred - are all privilege requirements met by cred
 */
#[no_mangle]
pub unsafe extern "C" fn smack_privileged_cred(cap: c_int, cred: *const cred) -> bool_t {
    let tsp = smack_cred(cred);
    let skp = (*tsp).smk_task;
    let mut rc: c_int;

    rc = cap_capable(cred, &mut init_user_ns, cap, CAP_OPT_NONE);
    if rc != 0 {
        return false;
    }

    rcu_read_lock();
    if list_empty(&raw const smack_onlycap_list) {
        rcu_read_unlock();
        return true;
    }

    let mut pos = smack_onlycap_list.next;
    while pos != &raw mut smack_onlycap_list {
        let sklep = pos as *mut smack_known_list_elem;
        if (*sklep).smk_label == skp {
            rcu_read_unlock();
            return true;
        }
        pos = (*pos).next;
    }
    rcu_read_unlock();

    false
}

/**
 * smack_privileged - are all privilege requirements met
 */
#[no_mangle]
pub unsafe extern "C" fn smack_privileged(cap: c_int) -> bool_t {
    /*
     * All kernel tasks are privileged
     */
    if ((*current).flags & PF_KTHREAD) != 0 {
        return true;
    }

    smack_privileged_cred(cap, current_cred())
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
