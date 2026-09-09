/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from memcontrol-v1.h. */

/* Cgroup v1 and v2 common declarations */

/*
 * Iteration constructs for visiting all cgroups (under a tree).  If
 * loops are exited prematurely (break), mem_cgroup_iter_break() must
 * be used for reference counting.
 */
#[macro_export]
macro_rules! for_each_mem_cgroup_tree {
    ($iter:ident, $root:expr) => {
        for $iter in unsafe { mem_cgroup_iter($root, core::ptr::null_mut(), core::ptr::null_mut()) }
            .into_iter()
        {
            if $iter.is_null() {
                break;
            }
            /* The C macro updates the iterator after each loop body. */
        }
    };
}

extern "C" {
    pub fn drain_all_stock(root_memcg: *mut mem_cgroup);
    pub fn memory_stat_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn mem_cgroup_private_id_get_online(
        memcg: *mut mem_cgroup,
        n: core::ffi::c_uint,
    ) -> *mut mem_cgroup;
    pub fn mem_cgroup_iter(
        root: *mut mem_cgroup,
        prev: *mut mem_cgroup,
        css: *mut core::ffi::c_void,
    ) -> *mut mem_cgroup;

    pub fn cgroup_subsys_on_dfl(subsys: *mut core::ffi::c_void) -> bool;
    pub fn page_counter_uncharge(counter: *mut page_counter, nr_pages: core::ffi::c_uint);

    pub fn memcg_events_local(memcg: *mut mem_cgroup, event: core::ffi::c_int) -> core::ffi::c_ulong;
    pub fn memcg_page_state_local(memcg: *mut mem_cgroup, idx: core::ffi::c_int) -> core::ffi::c_ulong;
    pub fn memcg_page_state_local_output(memcg: *mut mem_cgroup, item: core::ffi::c_int) -> core::ffi::c_ulong;
    pub fn memcg1_alloc_events(memcg: *mut mem_cgroup) -> bool;
    pub fn memcg1_free_events(memcg: *mut mem_cgroup);
    pub fn memcg1_memcg_init(memcg: *mut mem_cgroup);
    pub fn memcg1_remove_from_trees(memcg: *mut mem_cgroup);
    pub fn memcg1_css_offline(memcg: *mut mem_cgroup);
    pub fn memcg1_oom_prepare(memcg: *mut mem_cgroup, locked: *mut bool) -> bool;
    pub fn memcg1_oom_finish(memcg: *mut mem_cgroup, locked: bool);
    pub fn memcg1_oom_recover(memcg: *mut mem_cgroup);
    pub fn memcg1_commit_charge(folio: *mut folio, memcg: *mut mem_cgroup);
    pub fn memcg1_uncharge_batch(memcg: *mut mem_cgroup, pgpgout: core::ffi::c_ulong, nr_memory: core::ffi::c_ulong, nid: core::ffi::c_int);
    pub fn memcg1_stat_format(memcg: *mut mem_cgroup, s: *mut seq_buf);
    pub fn reparent_memcg1_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup);
    pub fn reparent_memcg1_lruvec_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup);
    pub fn reparent_memcg_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, idx: core::ffi::c_int);
    pub fn reparent_memcg_lruvec_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, idx: core::ffi::c_int);
    pub fn memcg1_account_kmem(memcg: *mut mem_cgroup, nr_pages: core::ffi::c_int);
    pub fn memcg1_tcpmem_active(memcg: *mut mem_cgroup) -> bool;
    pub fn memcg1_charge_skmem(memcg: *mut mem_cgroup, nr_pages: core::ffi::c_uint, gfp_mask: gfp_t) -> bool;
    pub fn memcg1_uncharge_skmem(memcg: *mut mem_cgroup, nr_pages: core::ffi::c_uint);
}

/* C declarations supplied by the included kernel headers. */
#[allow(non_camel_case_types)]
pub enum mem_cgroup {}
pub enum folio {}
pub enum seq_file {}
pub enum seq_buf {}
pub enum page_counter {}
pub type gfp_t = core::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum res_type { _MEM, _MEMSWAP, _KMEM, _TCP }

/* CONFIG_MEMCG_V1 selects the real implementations; otherwise these are stubs. */
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline]
pub const fn do_memsw_account() -> bool { false }

#[cfg(feature = "CONFIG_MEMCG_V1")]
#[inline]
pub unsafe fn do_memsw_account() -> bool {
    !cgroup_subsys_on_dfl(core::ptr::addr_of_mut!(memory_cgrp_subsys) as *mut core::ffi::c_void)
}

/* The following inline declarations preserve the header's local interfaces.
 * Their referenced fields and constants are provided by the kernel headers. */
extern "C" {
    static mut memory_cgrp_subsys: core::ffi::c_void;
    static mut PAGE_COUNTER_MAX: core::ffi::c_ulong;
    static mut memsw_files: cftype;
    static mut mem_cgroup_legacy_files: cftype;
}

pub enum cftype {}

#[cfg(feature = "CONFIG_MEMCG_V1")]
#[inline]
pub unsafe fn memcg1_soft_limit_reset(memcg: *mut mem_cgroup) {
    /* WRITE_ONCE(memcg->soft_limit, PAGE_COUNTER_MAX); */
    let _ = (memcg, PAGE_COUNTER_MAX);
}

#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_alloc_events(_: *mut mem_cgroup) -> bool { true }
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_free_events(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_memcg_init(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_remove_from_trees(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_soft_limit_reset(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_css_offline(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub unsafe fn memcg1_oom_prepare(_: *mut mem_cgroup, locked: *mut bool) -> bool { *locked = false; true }
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_oom_finish(_: *mut mem_cgroup, _: bool) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_oom_recover(_: *mut mem_cgroup) {}
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_tcpmem_active(_: *mut mem_cgroup) -> bool { false }
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_charge_skmem(_: *mut mem_cgroup, _: core::ffi::c_uint, _: gfp_t) -> bool { true }
#[cfg(not(feature = "CONFIG_MEMCG_V1"))]
#[inline] pub fn memcg1_uncharge_skmem(_: *mut mem_cgroup, _: core::ffi::c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
