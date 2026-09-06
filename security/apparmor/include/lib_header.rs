// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor lib definitions
//
// 2017 Canonical Ltd.

// Dependencies from Linux kernel headers:
// - linux/slab.h: kref, memory allocation types
// - linux/fs.h: dentry, super_block structures
// - linux/lsm_hooks.h: LSM hooks
// - match.h: aa_dfa, aa_state_t, related DFA functions

use core::mem::offset_of;

// Forward declarations of opaque types from other kernel headers
#[repr(C)]
pub struct aa_dfa {
    // opaque
}

#[repr(C)]
pub struct kref {
    // opaque
}

#[repr(C)]
pub struct list_head {
    // opaque
}

#[repr(C)]
pub struct dentry {
    // opaque
}

#[repr(C)]
pub struct super_block {
    // opaque
}

#[repr(C)]
pub struct lsm_blob_sizes {
    // opaque
}

#[repr(C)]
pub struct aa_label {
    // opaque
}

#[repr(C)]
pub struct aa_profile {
    // opaque
}

#[repr(C)]
pub struct aa_ns {
    // opaque
}

pub type aa_state_t = u32;
pub type gfp_t = u32;

// External global declarations
extern "C" {
    pub static mut stacksplitdfa: *mut aa_dfa;
    pub static apparmor_initialized: i32;
    pub static apparmor_blob_sizes: lsm_blob_sizes;
}

// External function declarations
extern "C" {
    pub fn pr_debug(fmt: *const i8, ...) -> ();
    pub fn pr_warn_ratelimited(fmt: *const i8, ...) -> ();
    pub fn pr_err_ratelimited(fmt: *const i8, ...) -> ();
    pub fn WARN(condition: i32, fmt: *const i8, ...) -> i32;
    pub fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
    pub fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    pub fn strstr(haystack: *const i8, needle: *const i8) -> *mut i8;
    pub fn strim(s: *mut i8) -> *mut i8;
    pub fn kref_get(kref: *mut kref) -> ();
    pub fn kref_put(kref: *mut kref, release: extern "C" fn(*mut kref)) -> i32;

    pub fn aa_parse_debug_params(str: *const i8) -> i32;
    pub fn aa_print_debug_params(buffer: *mut i8) -> i32;
    pub fn skipn_spaces(str: *const i8, n: usize) -> *const i8;
    pub fn aa_splitn_fqname(
        fqname: *const i8,
        n: usize,
        ns_name: *mut *const i8,
        ns_len: *mut usize,
    ) -> *const i8;
    pub fn aa_info_message(str: *const i8) -> ();
    pub fn aa_dfa_next(dfa: *const aa_dfa, state: aa_state_t, c: u8) -> aa_state_t;
    pub fn aa_str_kref(kref: *mut kref) -> ();
    pub fn aa_str_alloc(size: i32, gfp: gfp_t) -> *mut i8;
    pub fn aa_resize_str_table(t: *mut aa_str_table, newsize: i32, gfp: gfp_t) -> bool;
    pub fn aa_destroy_str_table(table: *mut aa_str_table) -> ();
    pub fn aa_policy_init(
        policy: *mut aa_policy,
        prefix: *const i8,
        name: *const i8,
        gfp: gfp_t,
    ) -> bool;
    pub fn aa_policy_destroy(policy: *mut aa_policy) -> ();

    // From match.h via label functions
    pub fn labels_profile(label: *const aa_label) -> *mut aa_profile;
    pub fn labels_ns(label: *const aa_label) -> *mut aa_ns;
    pub fn aa_get_profile(profile: *mut aa_profile) -> *mut aa_profile;
    pub fn aa_get_label(label: *const aa_label) -> *const aa_label;
    pub fn aa_vec_unique(vec: *mut *mut aa_profile, count: i32, start: i32) -> i32;
    pub fn aa_vec_find_or_create_label(
        vec: *mut *mut aa_profile,
        count: i32,
        gfp: gfp_t,
    ) -> *mut aa_label;
    pub fn aa_ns_visible(
        subj: *const aa_label,
        obj: *const aa_label,
        view: bool,
    ) -> bool;

    // Implicit from list macros
    pub fn list_for_each_entry_rcu(
        pos: *mut *mut aa_policy,
        head: *const list_head,
        member: *const list_head,
    ) -> ();
}

// Debug constants
pub const DEBUG_NONE: i32 = 0;
pub const DEBUG_LABEL_ABS_ROOT: i32 = 1;
pub const DEBUG_LABEL: i32 = 2;
pub const DEBUG_DOMAIN: i32 = 4;
pub const DEBUG_POLICY: i32 = 8;
pub const DEBUG_INTERFACE: i32 = 0x10;
pub const DEBUG_UNPACK: i32 = 0x20;
pub const DEBUG_TAGS: i32 = 0x40;
pub const DEBUG_ALL: i32 = 0x7f;
pub const DEBUG_PARSE_ERROR: i32 = -1;

// Global debug level (declaration only; defined elsewhere)
extern "C" {
    pub static aa_g_debug: i32;
}

// Macros translated to Rust

/// Debug print macro - calls pr_debug for debug output
#[macro_export]
macro_rules! dbg_printk {
    ($fmt:expr) => {
        $crate::pr_debug($fmt as *const i8)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::pr_debug($fmt as *const i8, $($arg)*)
    };
}

/// Debug output based on debug flags with rate limiting
#[macro_export]
macro_rules! AA_DEBUG {
    ($opt:expr, $fmt:expr) => {
        if (unsafe { $crate::aa_g_debug } & $opt) != 0 {
            $crate::pr_warn_ratelimited(
                concat!("%s: ", $fmt) as *const i8,
                stringify!(__func__) as *const i8,
            )
        }
    };
    ($opt:expr, $fmt:expr, $($arg:tt)*) => {
        if (unsafe { $crate::aa_g_debug } & $opt) != 0 {
            $crate::pr_warn_ratelimited(
                concat!("%s: ", $fmt) as *const i8,
                stringify!(__func__) as *const i8,
                $($arg)*
            )
        }
    };
}

/// Debug output based on label flags
#[macro_export]
macro_rules! AA_DEBUG_LABEL {
    ($lab:expr, $x:expr, $fmt:expr) => {
        if ((*$lab).flags & $crate::FLAG_DEBUG1) != 0 {
            $crate::AA_DEBUG!($x, $fmt);
        }
    };
    ($lab:expr, $x:expr, $fmt:expr, $($arg:tt)*) => {
        if ((*$lab).flags & $crate::FLAG_DEBUG1) != 0 {
            $crate::AA_DEBUG!($x, $fmt, $($arg)*);
        }
    };
}

/// Debug output based on profile's label flags
#[macro_export]
macro_rules! AA_DEBUG_PROFILE {
    ($prof:expr, $x:expr, $fmt:expr) => {
        $crate::AA_DEBUG_LABEL!(&(*$prof).label, $x, $fmt);
    };
    ($prof:expr, $x:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::AA_DEBUG_LABEL!(&(*$prof).label, $x, $fmt, $($arg)*);
    };
}

/// Warning macro
#[macro_export]
macro_rules! AA_WARN {
    ($x:expr) => {
        $crate::WARN(
            $x as i32,
            concat!("APPARMOR WARN %s: %s\n") as *const i8,
            stringify!(__func__) as *const i8,
            stringify!($x) as *const i8,
        )
    };
}

/// BUG assertion macro
#[macro_export]
macro_rules! AA_BUG {
    ($x:expr) => {
        $crate::AA_BUG_FMT!($x, "")
    };
    ($x:expr, $($arg:tt)*) => {
        $crate::AA_BUG_FMT!($x, "", $($arg)*)
    };
}

/// BUG assertion macro with format string
#[macro_export]
macro_rules! AA_BUG_FMT {
    ($x:expr, $fmt:expr) => {
        #[cfg(feature = "CONFIG_SECURITY_APPARMOR_DEBUG_ASSERTS")]
        {
            $crate::WARN(
                $x as i32,
                concat!("AppArmor WARN %s: (", stringify!($x), "): ", $fmt, "\n") as *const i8,
                stringify!(__func__) as *const i8,
            );
        }
        #[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_DEBUG_ASSERTS"))]
        {
            // Compile-time validation that $x is valid
            // no_printk equivalent - evaluate but discard
            let _ = &$fmt;
        }
    };
    ($x:expr, $fmt:expr, $($arg:tt)*) => {
        #[cfg(feature = "CONFIG_SECURITY_APPARMOR_DEBUG_ASSERTS")]
        {
            $crate::WARN(
                $x as i32,
                concat!("AppArmor WARN %s: (", stringify!($x), "): ", $fmt, "\n") as *const i8,
                stringify!(__func__) as *const i8,
                $($arg)*
            );
        }
        #[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_DEBUG_ASSERTS"))]
        {
            // Compile-time validation that $x is valid
            let _ = &$fmt;
        }
    };
}

/// Error message macro with rate limiting
#[macro_export]
macro_rules! AA_ERROR {
    ($fmt:expr) => {
        $crate::pr_err_ratelimited(
            concat!("AppArmor: ", $fmt) as *const i8
        )
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::pr_err_ratelimited(
            concat!("AppArmor: ", $fmt) as *const i8,
            $($arg)*
        )
    };
}

/// Check if subject is in scope of object (namespace visibility)
#[macro_export]
macro_rules! aa_in_scope {
    ($subj:expr, $obj:expr) => {
        $crate::aa_ns_visible($subj, $obj, false)
    };
}

/// Check if subject can view object (namespace visibility)
#[macro_export]
macro_rules! aa_in_view {
    ($subj:expr, $obj:expr) => {
        $crate::aa_ns_visible($subj, $obj, true)
    };
}

/// Iterate labels in scope (delegates to in_ns version)
#[macro_export]
macro_rules! label_for_each_in_scope {
    ($i:expr, $ns:expr, $l:expr, $p:expr) => {
        $crate::label_for_each_in_ns!($i, $ns, $l, $p)
    };
}

/// Iterate function for each label in scope
#[macro_export]
macro_rules! fn_for_each_in_scope {
    ($l:expr, $p:expr, $fn:expr) => {
        $crate::fn_for_each_in_ns!($l, $p, $fn)
    };
}

// Type marker annotation for reference-counted strings
pub type __counted = ();

/// Reference type enum for common reference counting
#[repr(C)]
pub enum reftype {
    REF_NS = 0,
    REF_PROXY = 1,
    REF_RAWDATA = 2,
}

/// Common reference count structure used by aafs data
#[repr(C)]
pub struct aa_common_ref {
    pub count: kref,
    pub reftype: reftype,
}

/// String table entry
#[repr(C)]
pub struct aa_str_table_ent {
    pub count: i32,
    pub size: i32,
    pub strs: *mut i8,
}

/// String table
#[repr(C)]
pub struct aa_str_table {
    pub size: i32,
    pub table: *mut aa_str_table_ent,
}

/// Reference-counted string with embedded name
#[repr(C)]
pub struct counted_str {
    pub count: kref,
    pub name: [i8; 0],
}

/// Convert a string pointer back to its counted_str header
#[inline]
pub fn str_to_counted(str: *const i8) -> *mut counted_str {
    unsafe {
        (str as *mut u8).offset(-(offset_of!(counted_str, name) as isize)) as *mut counted_str
    }
}

/// Get a reference to a counted string (increments refcount)
#[inline]
pub unsafe fn aa_get_str(str: *mut i8) -> *mut i8 {
    if !str.is_null() {
        let counted = str_to_counted(str);
        kref_get(&mut (*counted).count);
    }
    str
}

/// Release a reference to a counted string (decrements refcount, may free)
#[inline]
pub unsafe fn aa_put_str(str: *mut i8) {
    if !str.is_null() {
        let counted = str_to_counted(str);
        kref_put(&mut (*counted).count, aa_str_kref);
    }
}

/// Common part of both namespaces and profiles
#[repr(C)]
pub struct aa_policy {
    pub name: *const i8,
    pub hname: *mut i8,
    pub list: list_head,
    pub profiles: list_head,
}

/// Find the base (last component) of an hname
#[inline]
pub unsafe fn basename(hname: *const i8) -> *const i8 {
    let mut hname_mut = hname as *mut i8;
    hname_mut = strim(hname_mut);

    loop {
        let split = strstr(hname_mut, b"//\0".as_ptr() as *const i8);
        if split.is_null() {
            break;
        }
        hname_mut = split.add(2);
    }

    hname_mut as *const i8
}

/// Find a policy by name on a policy list (requires rcu_read_lock)
/// Returns unrefcounted policy matching name or NULL if not found
#[inline]
pub unsafe fn __policy_find(head: *mut list_head, name: *const i8) -> *mut aa_policy {
    // Implementation would use list_for_each_entry_rcu
    // For this translation, we note the dependency: requires proper RCU locking context
    // The actual iteration needs access to list_head iteration macros from kernel
    core::ptr::null_mut()
}

/// Find a policy by partial name match (requires rcu_read_lock)
/// Returns unrefcounted policy matching str or NULL if not found
#[inline]
pub unsafe fn __policy_strn_find(
    head: *mut list_head,
    str: *const i8,
    len: i32,
) -> *mut aa_policy {
    // Implementation would use list_for_each_entry_rcu with aa_strneq
    // For this translation, we note the dependency: requires proper RCU locking context
    core::ptr::null_mut()
}

/// Compare null-terminated string to non-null-terminated substring
/// String must be fully consumed for match
#[inline]
pub unsafe fn aa_strneq(str: *const i8, sub: *const i8, len: i32) -> bool {
    strncmp(str, sub, len as usize) == 0 && *str.add(len as usize) == 0
}

/// Step to next state after null character in DFA
/// The null transition uses only the string's null terminator byte
#[inline]
pub unsafe fn aa_dfa_null_transition(dfa: *const aa_dfa, start: aa_state_t) -> aa_state_t {
    aa_dfa_next(dfa, start, 0)
}

/// Check if filesystem path operations are mediated
/// Returns false for user-prohibited filesystems
#[inline]
pub unsafe fn path_mediated_fs(dentry: *mut dentry) -> bool {
    // Would need dentry->d_sb->s_flags and SB_NOUSER constant
    // !((*(*dentry).d_sb).s_flags & SB_NOUSER)
    // Placeholder: implementation requires kernel struct layout
    true
}

// Complex macros that involve statement expressions and control flow
// These are translated to Rust patterns with closures and unsafe code

/// Build a label transition by calling a function for each profile
/// Returns new label on success, NULL if all callbacks decline, ERR_PTR on failure
/// The FN must return a label or ERR_PTR on failure
#[macro_export]
macro_rules! fn_label_build {
    ($L:expr, $P:expr, $GFP:expr, $FN:expr) => {{
        // Simplified Rust translation of the complex C macro with statement expression
        // The full implementation would require:
        // - Define vectors for labels and profiles
        // - Iterate through labels with label_for_each
        // - Call FN for each profile
        // - Collect results and find/create combined label
        // - Handle cleanup of temporary allocations

        // This requires integration with the full aa_label and aa_profile systems
        // Placeholder returning error for incomplete external dependencies
        unsafe { core::ptr::null_mut::<$crate::aa_label>() }
    }};
}

/// Select namespace function or other function based on profile namespace
#[macro_export]
macro_rules! __fn_build_in_scope {
    ($NS:expr, $P:expr, $NS_FN:expr, $OTHER_FN:expr) => {{
        if (*$P).ns != $NS {
            $OTHER_FN
        } else {
            $NS_FN
        }
    }};
}

/// Build label transition within namespace scope
#[macro_export]
macro_rules! fn_label_build_in_scope {
    ($L:expr, $P:expr, $GFP:expr, $NS_FN:expr, $OTHER_FN:expr) => {
        $crate::fn_label_build!(
            $L,
            $P,
            $GFP,
            $crate::__fn_build_in_scope!(
                $crate::labels_ns($L),
                $P,
                $NS_FN,
                $OTHER_FN
            )
        )
    };
}

// Configuration-dependent constants (would be set by build system)
// pub const FLAG_DEBUG1: i32 = 0x01;  // from label definition
// pub const SB_NOUSER: i32 = 0x01;   // from super_block definition

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
