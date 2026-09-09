/* SPDX-License-Identifier: GPL-2.0 */

// Declarations and helpers translated from linux/ns_common.h.
// Types, constants, and operations supplied by the included kernel headers are
// intentionally referenced here rather than reimplemented.

extern "C" {
    pub fn is_current_namespace(ns: *mut ns_common) -> bool;
    pub fn __ns_common_init(
        ns: *mut ns_common,
        ns_type: u32,
        ops: *const proc_ns_operations,
        inum: i32,
    ) -> i32;
    pub fn __ns_common_free(ns: *mut ns_common);
    pub fn ns_owner(ns: *mut ns_common) -> *mut ns_common;
    pub fn may_see_all_namespaces() -> bool;
    pub fn __ns_ref_active_put(ns: *mut ns_common);
    pub fn __ns_ref_active_get(ns: *mut ns_common);
}

#[inline(always)]
pub unsafe fn is_ns_init_inum(ns: *const ns_common) -> bool {
    VFS_WARN_ON_ONCE((*ns).inum == 0);
    unlikely(in_range(
        (*ns).inum,
        MNT_NS_INIT_INO,
        IPC_NS_INIT_INO - MNT_NS_INIT_INO + 1,
    ))
}

#[inline(always)]
pub unsafe fn is_ns_init_id(ns: *const ns_common) -> bool {
    VFS_WARN_ON_ONCE((*ns).ns_id == 0);
    (*ns).ns_id <= NS_LAST_INIT_ID
}

// NS_COMMON_INIT initializes an ns_common embedded in the supplied namespace.
#[macro_export]
macro_rules! NS_COMMON_INIT {
    ($nsname:expr) => {{
        .ns_type = ns_common_type(&$nsname),
        .ns_id = ns_init_id(&$nsname),
        .inum = ns_init_inum(&$nsname),
        .ops = to_ns_operations(&$nsname),
        .stashed = core::ptr::null_mut(),
        .__ns_ref = REFCOUNT_INIT!(1),
        .__ns_ref_active = ATOMIC_INIT!(1),
        .ns_unified_node.ns_list_entry = LIST_HEAD_INIT!($nsname.ns.ns_unified_node.ns_list_entry),
        .ns_tree_node.ns_list_entry = LIST_HEAD_INIT!($nsname.ns.ns_tree_node.ns_list_entry),
        .ns_owner_node.ns_list_entry = LIST_HEAD_INIT!($nsname.ns.ns_owner_node.ns_list_entry),
        .ns_owner_root.ns_list_head = LIST_HEAD_INIT!($nsname.ns.ns_owner_root.ns_list_head),
    }};
}

#[inline(always)]
pub unsafe fn ns_common_init(__ns: *mut core::ffi::c_void) -> i32 {
    __ns_common_init(
        to_ns_common(__ns),
        ns_common_type(__ns),
        to_ns_operations(__ns),
        if __ns == ns_init_ns(__ns) { ns_init_inum(__ns) } else { 0 },
    )
}

#[inline(always)]
pub unsafe fn ns_common_init_inum(__ns: *mut core::ffi::c_void, __inum: i32) -> i32 {
    __ns_common_init(
        to_ns_common(__ns),
        ns_common_type(__ns),
        to_ns_operations(__ns),
        __inum,
    )
}

#[inline(always)]
pub unsafe fn ns_common_free(__ns: *mut core::ffi::c_void) {
    __ns_common_free(to_ns_common(__ns));
}

#[inline(always)]
pub unsafe fn __ns_ref_active_read(ns: *const ns_common) -> i32 {
    atomic_read(&(*ns).__ns_ref_active)
}

#[inline(always)]
pub unsafe fn __ns_ref_read(ns: *const ns_common) -> i32 {
    refcount_read(&(*ns).__ns_ref)
}

#[inline(always)]
pub unsafe fn __ns_ref_put(ns: *mut ns_common) -> bool {
    if is_ns_init_id(ns) {
        VFS_WARN_ON_ONCE(__ns_ref_read(ns) != 1);
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 1);
        return false;
    }
    if refcount_dec_and_test(&mut (*ns).__ns_ref) {
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 0);
        return true;
    }
    false
}

#[inline(always)]
pub unsafe fn __ns_ref_get(ns: *mut ns_common) -> bool {
    if is_ns_init_id(ns) {
        VFS_WARN_ON_ONCE(__ns_ref_read(ns) != 1);
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 1);
        return true;
    }
    if refcount_inc_not_zero(&mut (*ns).__ns_ref) { return true; }
    VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 0);
    false
}

#[inline(always)]
pub unsafe fn __ns_ref_inc(ns: *mut ns_common) {
    if is_ns_init_id(ns) {
        VFS_WARN_ON_ONCE(__ns_ref_read(ns) != 1);
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 1);
        return;
    }
    refcount_inc(&mut (*ns).__ns_ref);
}

#[inline(always)]
pub unsafe fn __ns_ref_dec_and_lock(ns: *mut ns_common, ns_lock: *mut spinlock_t) -> bool {
    if is_ns_init_id(ns) {
        VFS_WARN_ON_ONCE(__ns_ref_read(ns) != 1);
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) != 1);
        return false;
    }
    refcount_dec_and_lock(&mut (*ns).__ns_ref, ns_lock)
}

#[inline(always)]
pub unsafe fn ns_get_unless_inactive(ns: *mut ns_common) -> *mut ns_common {
    if __ns_ref_active_read(ns) == 0 {
        VFS_WARN_ON_ONCE(is_ns_init_id(ns));
        return core::ptr::null_mut();
    }
    if !__ns_ref_get(ns) { return core::ptr::null_mut(); }
    ns
}

#[macro_export]
macro_rules! ns_ref_read { ($ns:expr) => { unsafe { __ns_ref_read(to_ns_common($ns)) } }; }
#[macro_export]
macro_rules! ns_ref_inc {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_inc(to_ns_common($ns)); } } }};
}
#[macro_export]
macro_rules! ns_ref_get {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_get(to_ns_common($ns)) } } else { false } }};
}
#[macro_export]
macro_rules! ns_ref_put {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_put(to_ns_common($ns)) } } else { false } }};
}
#[macro_export]
macro_rules! ns_ref_put_and_lock {
    ($ns:expr, $lock:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_dec_and_lock(to_ns_common($ns), $lock) } } else { false } }};
}
#[macro_export]
macro_rules! ns_ref_active_read {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_active_read(to_ns_common($ns)) } } else { 0 } }};
}
#[macro_export]
macro_rules! ns_ref_active_put {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_active_put(to_ns_common($ns)); } } }};
}
#[macro_export]
macro_rules! ns_ref_active_get {
    ($ns:expr) => {{ if !$ns.is_null() { unsafe { __ns_ref_active_get(to_ns_common($ns)); } } }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
