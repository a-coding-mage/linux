/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 Red Hat, Inc.
 */

#[repr(C)]
pub struct xfs_group {
    pub xg_mount: *mut xfs_mount,
    pub xg_gno: u32,
    pub xg_type: xfs_group_type,
    pub xg_ref: atomic_t,        /* passive reference count */
    pub xg_active_ref: atomic_t, /* active reference count */

    /* Precalculated geometry info */
    pub xg_block_count: u32, /* max usable gbno */
    pub xg_min_gbno: u32,    /* min usable gbno */

    /* -- kernel only structures below this line -- */
    #[cfg(feature = "kernel")]
    pub xg_kernel: xfs_group_kernel,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub union xfs_group_kernel_union {
    /* For perags and non-zoned RT groups: track freed but not yet committed extents. */
    pub xg_busy_extents: *mut xfs_extent_busy_tree,
    /* For zoned RT groups: list of groups that need a zone reset. */
    pub xg_next_reset: *mut xfs_group,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct xfs_group_kernel {
    pub union_: xfs_group_kernel_union,
    /* Bitsets of per-ag metadata that have been checked and/or are sick. */
    pub xg_checked: u16,
    pub xg_sick: u16,
    pub xg_state_lock: spinlock_t,
    /* Track deferred log intent items queued but not yet processed. */
    pub xg_intents_drain: xfs_defer_drain,
    /* Hook to feed rmapbt updates to an active online repair. */
    pub xg_rmap_update_hooks: xfs_hooks,
}

extern "C" {
    pub fn xfs_group_get(mp: *mut xfs_mount, index: u32, type_: xfs_group_type) -> *mut xfs_group;
    pub fn xfs_group_get_by_fsb(mp: *mut xfs_mount, fsbno: xfs_fsblock_t, type_: xfs_group_type) -> *mut xfs_group;
    pub fn xfs_group_hold(xg: *mut xfs_group) -> *mut xfs_group;
    pub fn xfs_group_put(xg: *mut xfs_group);
    pub fn xfs_group_grab(mp: *mut xfs_mount, index: u32, type_: xfs_group_type) -> *mut xfs_group;
    pub fn xfs_group_next_range(mp: *mut xfs_mount, xg: *mut xfs_group, start_index: u32, end_index: u32, type_: xfs_group_type) -> *mut xfs_group;
    pub fn xfs_group_grab_next_mark(mp: *mut xfs_mount, xg: *mut xfs_group, mark: xa_mark_t, type_: xfs_group_type) -> *mut xfs_group;
    pub fn xfs_group_rele(xg: *mut xfs_group);
    pub fn xfs_group_free(mp: *mut xfs_mount, index: u32, type_: xfs_group_type, uninit: Option<unsafe extern "C" fn(*mut xfs_group)>);
    pub fn xfs_group_insert(mp: *mut xfs_mount, xg: *mut xfs_group, index: u32, type_: xfs_group_type) -> ::std::os::raw::c_int;
}

#[macro_export]
macro_rules! xfs_group_set_mark {
    ($xg:expr, $mark:expr) => { xa_set_mark(unsafe { &mut (*$xg).xg_mount.as_mut().unwrap() }.m_groups[unsafe { (*$xg).xg_type as usize }].xa.as_mut(), unsafe { (*$xg).xg_gno }, $mark) };
}
#[macro_export]
macro_rules! xfs_group_clear_mark {
    ($xg:expr, $mark:expr) => { xa_clear_mark(unsafe { &mut (*$xg).xg_mount.as_mut().unwrap() }.m_groups[unsafe { (*$xg).xg_type as usize }].xa.as_mut(), unsafe { (*$xg).xg_gno }, $mark) };
}
#[macro_export]
macro_rules! xfs_group_marked {
    ($mp:expr, $type_:expr, $mark:expr) => { xa_marked(unsafe { &mut (*$mp).m_groups[$type_ as usize].xa }.as_mut(), $mark) };
}

#[inline]
pub unsafe fn xfs_group_max_blocks(xg: *mut xfs_group) -> xfs_agblock_t {
    (*xg).xg_mount.as_ref().unwrap().m_groups[(*xg).xg_type as usize].blocks
}

#[inline]
pub unsafe fn xfs_groups_to_rfsbs(mp: *mut xfs_mount, nr_groups: u32, type_: xfs_group_type) -> xfs_rfsblock_t {
    ((*mp).m_groups[type_ as usize].blocks as xfs_rfsblock_t).wrapping_mul(nr_groups as xfs_rfsblock_t)
}

#[inline]
pub unsafe fn xfs_group_start_fsb(xg: *mut xfs_group) -> xfs_fsblock_t {
    (( (*xg).xg_gno as xfs_fsblock_t) << (*xg).xg_mount.as_ref().unwrap().m_groups[(*xg).xg_type as usize].blklog)
}

#[inline]
pub unsafe fn xfs_gbno_to_fsb(xg: *mut xfs_group, gbno: xfs_agblock_t) -> xfs_fsblock_t {
    xfs_group_start_fsb(xg) | gbno
}

#[inline]
pub unsafe fn xfs_gbno_to_daddr(xg: *mut xfs_group, gbno: xfs_agblock_t) -> xfs_daddr_t {
    let mp = (*xg).xg_mount;
    let g = &(*mp).m_groups[(*xg).xg_type as usize];
    let fsbno = if g.has_daddr_gaps { xfs_gbno_to_fsb(xg, gbno) } else { ((*xg).xg_gno as xfs_fsblock_t).wrapping_mul(g.blocks) + gbno };
    XFS_FSB_TO_BB(mp, g.start_fsb + fsbno)
}

#[inline]
pub unsafe fn xfs_fsb_to_gno(mp: *mut xfs_mount, fsbno: xfs_fsblock_t, type_: xfs_group_type) -> u32 {
    if (*mp).m_groups[type_ as usize].blklog == 0 { 0 } else { fsbno >> (*mp).m_groups[type_ as usize].blklog }
}

#[inline]
pub unsafe fn xfs_fsb_to_gbno(mp: *mut xfs_mount, fsbno: xfs_fsblock_t, type_: xfs_group_type) -> xfs_agblock_t {
    fsbno & (*mp).m_groups[type_ as usize].blkmask
}

#[inline]
pub unsafe fn xfs_verify_gbno(xg: *mut xfs_group, gbno: u32) -> bool {
    gbno >= (*xg).xg_min_gbno && gbno < (*xg).xg_block_count
}

#[inline]
pub unsafe fn xfs_verify_gbext(xg: *mut xfs_group, gbno: u32, glen: u32) -> bool {
    if !xfs_verify_gbno(xg, gbno) || glen == 0 { return false; }
    let end = match gbno.checked_add(glen - 1) { Some(v) => v, None => return false };
    xfs_verify_gbno(xg, end)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
