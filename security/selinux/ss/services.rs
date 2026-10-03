// SPDX-License-Identifier: GPL-2.0-only
// Faithful source reconstruction of retained services.c; default-off proposal.
// No locally reconstructed ABI layouts or synchronization primitives.
#![allow(missing_docs, unsafe_op_in_unsafe_fn, non_upper_case_globals)]
#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use kernel::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/selinux_services_generated.rs"));
}
use bindings::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::{mem::{size_of, zeroed}, ptr::{addr_of, addr_of_mut, null, null_mut}};
const GFP_ATOMIC: gfp_t = LUPOS_SERVICES_GFP_ATOMIC as gfp_t;
const GFP_KERNEL: gfp_t = LUPOS_SERVICES_GFP_KERNEL as gfp_t;

struct RcuGuard;
impl RcuGuard {
    unsafe fn new() -> Self { lupos_services_rcu_read_lock(); Self }
}
impl Drop for RcuGuard {
    fn drop(&mut self) { unsafe { lupos_services_rcu_read_unlock(); } }
}


// Layout checks are source obligations until an admitted configured build executes them.
const _: () = {
    assert!(size_of::<context>() == LUPOS_SERVICES_SIZE_CONTEXT as usize);
    assert!(core::mem::align_of::<context>() == LUPOS_SERVICES_ALIGN_CONTEXT as usize);
    assert!(core::mem::offset_of!(context, user) == LUPOS_SERVICES_OFFSET_CONTEXT_USER as usize);
    assert!(core::mem::offset_of!(context, role) == LUPOS_SERVICES_OFFSET_CONTEXT_ROLE as usize);
    assert!(core::mem::offset_of!(context, type_) == LUPOS_SERVICES_OFFSET_CONTEXT_TYPE as usize);
    assert!(core::mem::offset_of!(context, len) == LUPOS_SERVICES_OFFSET_CONTEXT_LEN as usize);
    assert!(core::mem::offset_of!(context, range) == LUPOS_SERVICES_OFFSET_CONTEXT_RANGE as usize);
    assert!(core::mem::offset_of!(context, str_) == LUPOS_SERVICES_OFFSET_CONTEXT_STR as usize);
    assert!(size_of::<policydb>() == LUPOS_SERVICES_SIZE_POLICYDB as usize);
    assert!(core::mem::align_of::<policydb>() == LUPOS_SERVICES_ALIGN_POLICYDB as usize);
    assert!(core::mem::offset_of!(policydb, mls_enabled) == LUPOS_SERVICES_OFFSET_POLICYDB_MLS_ENABLED as usize);
    assert!(core::mem::offset_of!(policydb, symtab) == LUPOS_SERVICES_OFFSET_POLICYDB_SYMTAB as usize);
    assert!(core::mem::offset_of!(policydb, class_val_to_struct) == LUPOS_SERVICES_OFFSET_POLICYDB_CLASS_VAL_TO_STRUCT as usize);
    assert!(core::mem::offset_of!(policydb, type_val_to_struct) == LUPOS_SERVICES_OFFSET_POLICYDB_TYPE_VAL_TO_STRUCT as usize);
    assert!(core::mem::offset_of!(policydb, te_avtab) == LUPOS_SERVICES_OFFSET_POLICYDB_TE_AVTAB as usize);
    assert!(core::mem::offset_of!(policydb, te_cond_avtab) == LUPOS_SERVICES_OFFSET_POLICYDB_TE_COND_AVTAB as usize);
    assert!(core::mem::offset_of!(policydb, type_attr_map_array) == LUPOS_SERVICES_OFFSET_POLICYDB_TYPE_ATTR_MAP_ARRAY as usize);
    assert!(core::mem::offset_of!(policydb, policycaps) == LUPOS_SERVICES_OFFSET_POLICYDB_POLICYCAPS as usize);
    assert!(core::mem::offset_of!(policydb, len) == LUPOS_SERVICES_OFFSET_POLICYDB_LEN as usize);
    assert!(core::mem::offset_of!(policydb, process_class) == LUPOS_SERVICES_OFFSET_POLICYDB_PROCESS_CLASS as usize);
    assert!(size_of::<selinux_policy>() == LUPOS_SERVICES_SIZE_SELINUX_POLICY as usize);
    assert!(core::mem::align_of::<selinux_policy>() == LUPOS_SERVICES_ALIGN_SELINUX_POLICY as usize);
    assert!(core::mem::offset_of!(selinux_policy, sidtab) == LUPOS_SERVICES_OFFSET_SELINUX_POLICY_SIDTAB as usize);
    assert!(core::mem::offset_of!(selinux_policy, policydb) == LUPOS_SERVICES_OFFSET_SELINUX_POLICY_POLICYDB as usize);
    assert!(core::mem::offset_of!(selinux_policy, map) == LUPOS_SERVICES_OFFSET_SELINUX_POLICY_MAP as usize);
    assert!(core::mem::offset_of!(selinux_policy, latest_granting) == LUPOS_SERVICES_OFFSET_SELINUX_POLICY_LATEST_GRANTING as usize);
    assert!(size_of::<selinux_map>() == LUPOS_SERVICES_SIZE_SELINUX_MAP as usize);
    assert!(core::mem::align_of::<selinux_map>() == LUPOS_SERVICES_ALIGN_SELINUX_MAP as usize);
    assert!(size_of::<sidtab>() == LUPOS_SERVICES_SIZE_SIDTAB as usize);
    assert!(core::mem::align_of::<sidtab>() == LUPOS_SERVICES_ALIGN_SIDTAB as usize);
    assert!(size_of::<sidtab_entry>() == LUPOS_SERVICES_SIZE_SIDTAB_ENTRY as usize);
    assert!(core::mem::align_of::<sidtab_entry>() == LUPOS_SERVICES_ALIGN_SIDTAB_ENTRY as usize);
    assert!(core::mem::offset_of!(sidtab_entry, context) == LUPOS_SERVICES_OFFSET_SIDTAB_ENTRY_CONTEXT as usize);
    assert!(size_of::<av_decision>() == LUPOS_SERVICES_SIZE_AV_DECISION as usize);
    assert!(core::mem::align_of::<av_decision>() == LUPOS_SERVICES_ALIGN_AV_DECISION as usize);
    assert!(size_of::<avtab_key>() == LUPOS_SERVICES_SIZE_AVTAB_KEY as usize);
    assert!(core::mem::align_of::<avtab_key>() == LUPOS_SERVICES_ALIGN_AVTAB_KEY as usize);
    assert!(core::mem::offset_of!(avtab_key, source_type) == LUPOS_SERVICES_OFFSET_AVTAB_KEY_SOURCE_TYPE as usize);
    assert!(core::mem::offset_of!(avtab_key, target_type) == LUPOS_SERVICES_OFFSET_AVTAB_KEY_TARGET_TYPE as usize);
    assert!(core::mem::offset_of!(avtab_key, target_class) == LUPOS_SERVICES_OFFSET_AVTAB_KEY_TARGET_CLASS as usize);
    assert!(core::mem::offset_of!(avtab_key, specified) == LUPOS_SERVICES_OFFSET_AVTAB_KEY_SPECIFIED as usize);
    assert!(size_of::<extended_perms>() == LUPOS_SERVICES_SIZE_EXTENDED_PERMS as usize);
    assert!(core::mem::align_of::<extended_perms>() == LUPOS_SERVICES_ALIGN_EXTENDED_PERMS as usize);
    assert!(core::mem::offset_of!(extended_perms, len) == LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_LEN as usize);
    assert!(core::mem::offset_of!(extended_perms, base_perms) == LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_BASE_PERMS as usize);
    assert!(core::mem::offset_of!(extended_perms, drivers) == LUPOS_SERVICES_OFFSET_EXTENDED_PERMS_DRIVERS as usize);
    assert!(size_of::<extended_perms_decision>() == LUPOS_SERVICES_SIZE_EXTENDED_PERMS_DECISION as usize);
    assert!(core::mem::align_of::<extended_perms_decision>() == LUPOS_SERVICES_ALIGN_EXTENDED_PERMS_DECISION as usize);
    assert!(size_of::<selinux_load_state>() == LUPOS_SERVICES_SIZE_SELINUX_LOAD_STATE as usize);
    assert!(core::mem::align_of::<selinux_load_state>() == LUPOS_SERVICES_ALIGN_SELINUX_LOAD_STATE as usize);
    assert!(core::mem::offset_of!(selinux_load_state, policy) == LUPOS_SERVICES_OFFSET_SELINUX_LOAD_STATE_POLICY as usize);
    assert!(core::mem::offset_of!(selinux_load_state, convert_data) == LUPOS_SERVICES_OFFSET_SELINUX_LOAD_STATE_CONVERT_DATA as usize);
};

include!("services-context.rs");
include!("services-access.rs");
include!("services-policy.rs");
include!("services-query.rs");
