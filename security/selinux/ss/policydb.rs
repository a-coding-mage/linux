// SPDX-License-Identifier: GPL-2.0-only
// Rust reconstruction of retained policydb.c. Default-off source proposal.
// Original implementation: Stephen Smalley; MLS: Trusted Computer Solutions;
// conditionals: Tresys Technology; capabilities: Hewlett-Packard;
// Infiniband: Mellanox Technologies. See retained C for copyright notices.
#![allow(missing_docs, unsafe_op_in_unsafe_fn, non_upper_case_globals)]
#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use kernel::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/selinux_policydb_generated.rs"));
}
use bindings::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::{mem::{self, size_of}, ptr};
use bindings::{
    lupos_policydb_kmalloc as kmalloc, lupos_policydb_kzalloc as kzalloc,
    lupos_policydb_kmemdup as kmemdup, lupos_policydb_kstrdup as kstrdup,
    lupos_policydb_next_entry as next_entry, lupos_policydb_put_entry as put_entry,
    lupos_policydb_size_check as size_check, lupos_policydb_sym_name as sym_name,
    lupos_policydb_context_destroy as context_destroy,
    lupos_policydb_ebitmap_init as ebitmap_init,
    lupos_policydb_ebitmap_start_positive as ebitmap_start_positive,
    lupos_policydb_ebitmap_next_positive as ebitmap_next_positive,
    lupos_policydb_hashtab_insert as hashtab_insert,
    lupos_policydb_hashtab_search as hashtab_search,
    lupos_policydb_hashtab_stat as hashtab_stat,
    lupos_policydb_avtab_hash_eval as avtab_hash_eval,
    lupos_policydb_jhash_3words as jhash_3words,
    lupos_policydb_cond_resched as cond_resched,
    lupos_policydb_mls_level_eq as mls_level_eq,
    lupos_policydb_error as pr_err, lupos_policydb_warn as pr_warn,
    lupos_policydb_debug as pr_debug,
};
const GFP_KERNEL: gfp_t = LUPOS_POLICYDB_GFP_KERNEL as gfp_t;
const __GFP_NOWARN: gfp_t = LUPOS_POLICYDB_GFP_NOWARN as gfp_t;
const ENOMEM: c_int = bindings::ENOMEM as c_int;
const EINVAL: c_int = bindings::EINVAL as c_int;
const ENOENT: c_int = bindings::ENOENT as c_int;
const EEXIST: c_int = bindings::EEXIST as c_int;

fn le32_to_cpu(x: u32) -> u32 { u32::from_le(x) }
fn cmp_int(a: u32, b: u32) -> c_int { if a < b { -1 } else if a > b { 1 } else { 0 } }
fn val_is_boolean(v: u32) -> bool { v == 0 || v == 1 }
unsafe fn zalloc_obj<T>() -> *mut T { kzalloc(size_of::<T>(), GFP_KERNEL).cast() }
unsafe fn malloc_obj<T>() -> *mut T { kmalloc(size_of::<T>(), GFP_KERNEL).cast() }
unsafe fn kzalloc_objs<T>(n: u32) -> *mut T {
    lupos_policydb_kcalloc(n as usize, size_of::<T>(), GFP_KERNEL).cast()
}
unsafe fn kvzalloc_objs<T>(n: u32) -> *mut T {
    lupos_policydb_kvcalloc(n as usize, size_of::<T>(), GFP_KERNEL).cast()
}
unsafe fn kvcalloc<T>(n: u32) -> *mut T { kvzalloc_objs::<T>(n) }

// All externally shared layouts come from the configured retained headers.
// The C compiler's values independently constrain bindgen's layout in Rust.
const _: () = {
    assert!(size_of::<policydb>() == LUPOS_POLICYDB_SIZE_POLICYDB as usize);
    assert!(core::mem::align_of::<policydb>() == LUPOS_POLICYDB_ALIGN_POLICYDB as usize);
    assert!(core::mem::offset_of!(policydb, symtab) == LUPOS_POLICYDB_OFFSET_SYMTAB as usize);
    assert!(core::mem::offset_of!(policydb, bool_val_to_struct) == LUPOS_POLICYDB_OFFSET_BOOLS as usize);
    assert!(core::mem::offset_of!(policydb, type_attr_map_array) == LUPOS_POLICYDB_OFFSET_TYPE_ATTR as usize);
    assert!(core::mem::offset_of!(policydb, len) == LUPOS_POLICYDB_OFFSET_LEN as usize);
    assert!(size_of::<hashtab>() == LUPOS_POLICYDB_SIZE_HASHTAB as usize);
    assert!(size_of::<ebitmap>() == LUPOS_POLICYDB_SIZE_EBITMAP as usize);
    assert!(size_of::<avtab>() == LUPOS_POLICYDB_SIZE_AVTAB as usize);
    assert!(size_of::<context>() == LUPOS_POLICYDB_SIZE_CONTEXT as usize);
    assert!(core::mem::offset_of!(context, range) == LUPOS_POLICYDB_OFFSET_CONTEXT_RANGE as usize);
    assert!(size_of::<class_datum>() == LUPOS_POLICYDB_SIZE_CLASS as usize);
    assert!(size_of::<type_datum>() == LUPOS_POLICYDB_SIZE_TYPE as usize);
    assert!(size_of::<level_datum>() == LUPOS_POLICYDB_SIZE_LEVEL as usize);
    assert!(size_of::<cat_datum>() == LUPOS_POLICYDB_SIZE_CAT as usize);
    assert!(size_of::<ocontext>() == LUPOS_POLICYDB_SIZE_OCONTEXT as usize);
    assert!(size_of::<policy_file>() == LUPOS_POLICYDB_SIZE_FILE as usize);
};
include!("policydb-core.rs");
include!("policydb-symbols.rs");
include!("policydb-wire.rs");
