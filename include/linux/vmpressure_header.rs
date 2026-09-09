/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct vmpressure {
    pub scanned: ::core::ffi::c_ulong,
    pub reclaimed: ::core::ffi::c_ulong,
    /* The lock is used to keep the scanned/reclaimed in sync. */
    pub sr_lock: spinlock_t,

    #[cfg(CONFIG_MEMCG_V1)]
    /*
     * tree=true accumulators feed the v1 userspace eventfd interface
     * (memory.pressure_level). Drained by @work. v2 has no equivalent
     * interface, so this state is omitted on CONFIG_MEMCG_V1=n builds.
     */
    pub tree_scanned: ::core::ffi::c_ulong,
    #[cfg(CONFIG_MEMCG_V1)]
    pub tree_reclaimed: ::core::ffi::c_ulong,
    #[cfg(CONFIG_MEMCG_V1)]
    /* The list of vmpressure_event structs. */
    pub events: list_head,
    #[cfg(CONFIG_MEMCG_V1)]
    /* Have to grab the lock on events traversal or modifications. */
    pub events_lock: mutex,
    #[cfg(CONFIG_MEMCG_V1)]
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vmpressure_levels {
    VMPRESSURE_LOW = 0,
    VMPRESSURE_MEDIUM,
    VMPRESSURE_CRITICAL,
    VMPRESSURE_NUM_LEVELS,
}

pub struct mem_cgroup;

#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn vmpressure(
    gfp: gfp_t,
    order: ::core::ffi::c_int,
    memcg: *mut mem_cgroup,
    tree: bool,
    scanned: ::core::ffi::c_ulong,
    reclaimed: ::core::ffi::c_ulong,
);

#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn vmpressure_init(vmpr: *mut vmpressure);
#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn vmpressure_cleanup(vmpr: *mut vmpressure);
#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn memcg_to_vmpressure(memcg: *mut mem_cgroup) -> *mut vmpressure;
#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn vmpressure_to_memcg(vmpr: *mut vmpressure) -> *mut mem_cgroup;

/* Shared with the v1 vmpressure block in mm/memcontrol-v1.c. */
#[cfg(CONFIG_MEMCG)]
pub static mut vmpressure_win: ::core::ffi::c_ulong;
#[cfg(CONFIG_MEMCG)]
pub unsafe extern "C" fn vmpressure_calc_level(
    scanned: ::core::ffi::c_ulong,
    reclaimed: ::core::ffi::c_ulong,
) -> vmpressure_levels;

#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_prio(
    gfp: gfp_t,
    memcg: *mut mem_cgroup,
    prio: ::core::ffi::c_int,
);
#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_register_event(
    memcg: *mut mem_cgroup,
    eventfd: *mut eventfd_ctx,
    args: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_unregister_event(
    memcg: *mut mem_cgroup,
    eventfd: *mut eventfd_ctx,
);

/* v1 hooks called from mm/vmpressure.c; no-ops below when !MEMCG_V1. */
#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_v1_init(vmpr: *mut vmpressure);
#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_v1_cleanup(vmpr: *mut vmpressure);
#[cfg(all(CONFIG_MEMCG, CONFIG_MEMCG_V1))]
pub unsafe extern "C" fn vmpressure_v1_account_tree(
    vmpr: *mut vmpressure,
    scanned: ::core::ffi::c_ulong,
    reclaimed: ::core::ffi::c_ulong,
);

#[cfg(all(CONFIG_MEMCG, not(CONFIG_MEMCG_V1)))]
pub unsafe fn vmpressure_prio(_gfp: gfp_t, _memcg: *mut mem_cgroup, _prio: ::core::ffi::c_int) {}
#[cfg(all(CONFIG_MEMCG, not(CONFIG_MEMCG_V1)))]
pub unsafe fn vmpressure_v1_init(_vmpr: *mut vmpressure) {}
#[cfg(all(CONFIG_MEMCG, not(CONFIG_MEMCG_V1)))]
pub unsafe fn vmpressure_v1_cleanup(_vmpr: *mut vmpressure) {}
#[cfg(all(CONFIG_MEMCG, not(CONFIG_MEMCG_V1)))]
pub unsafe fn vmpressure_v1_account_tree(
    _vmpr: *mut vmpressure,
    _scanned: ::core::ffi::c_ulong,
    _reclaimed: ::core::ffi::c_ulong,
) {}

#[cfg(not(CONFIG_MEMCG))]
pub unsafe fn vmpressure(
    _gfp: gfp_t,
    _order: ::core::ffi::c_int,
    _memcg: *mut mem_cgroup,
    _tree: bool,
    _scanned: ::core::ffi::c_ulong,
    _reclaimed: ::core::ffi::c_ulong,
) {}
#[cfg(not(CONFIG_MEMCG))]
pub unsafe fn vmpressure_prio(
    _gfp: gfp_t,
    _memcg: *mut mem_cgroup,
    _prio: ::core::ffi::c_int,
) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
