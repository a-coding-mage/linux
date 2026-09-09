// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * heartbeat.c
 *
 * Register ourselves with the heartbeat service, keep our node maps
 * up to date, and fire off recovery when needed.
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux and OCFS2 headers provide the types, constants, bitmap operations,
// locking primitives, logging, tracing, and recovery functions referenced here.

unsafe extern "C" {
    fn bitmap_zero(map: *mut usize, nbits: usize);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn set_bit(bit: i32, map: *mut usize);
    fn clear_bit(bit: i32, map: *mut usize);
    fn test_bit(bit: i32, map: *const usize) -> i32;
    fn ocfs2_recovery_thread(osb: *mut ocfs2_super, node_num: i32);
    fn trace_ocfs2_do_node_down(node_num: i32);
    fn mlog(level: i32, format: *const u8, ...);
    fn BUG();
    fn BUG_ON(condition: bool);
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocfs2_node_map {
    pub num_nodes: i32,
    pub map: [usize; (OCFS2_NODE_MAP_MAX_NODES as usize + usize::BITS as usize - 1) / usize::BITS as usize],
}

#[repr(C)]
pub struct ocfs2_super {
    pub node_map_lock: spinlock_t,
    pub osb_recovering_orphan_dirs: ocfs2_node_map,
    pub node_num: i32,
    pub cconn: *mut core::ffi::c_void,
}

pub const OCFS2_NODE_MAP_MAX_NODES: i32 = 255;
pub const ML_ERROR: i32 = 0;

/* special case -1 for now
 * TODO: should *really* make sure the calling func never passes -1!!  */
unsafe fn ocfs2_node_map_init(map: *mut ocfs2_node_map) {
    (*map).num_nodes = OCFS2_NODE_MAP_MAX_NODES;
    bitmap_zero((*map).map.as_mut_ptr(), OCFS2_NODE_MAP_MAX_NODES as usize);
}

pub unsafe fn ocfs2_init_node_maps(osb: *mut ocfs2_super) {
    spin_lock_init(&mut (*osb).node_map_lock);
    ocfs2_node_map_init(&mut (*osb).osb_recovering_orphan_dirs);
}

pub unsafe fn ocfs2_do_node_down(node_num: i32, data: *mut core::ffi::c_void) {
    let osb = data as *mut ocfs2_super;

    BUG_ON((*osb).node_num == node_num);

    trace_ocfs2_do_node_down(node_num);

    if (*osb).cconn.is_null() {
        /*
         * No cluster connection means we're not even ready to
         * participate yet.  We check the slots after the cluster
         * comes up, so we will notice the node death then.  We
         * can safely ignore it here.
         */
        return;
    }

    ocfs2_recovery_thread(osb, node_num);
}

pub unsafe fn ocfs2_node_map_set_bit(
    osb: *mut ocfs2_super,
    map: *mut ocfs2_node_map,
    bit: i32,
) {
    if bit == -1 {
        return;
    }
    BUG_ON(bit >= (*map).num_nodes);
    spin_lock(&mut (*osb).node_map_lock);
    set_bit(bit, (*map).map.as_mut_ptr());
    spin_unlock(&mut (*osb).node_map_lock);
}

pub unsafe fn ocfs2_node_map_clear_bit(
    osb: *mut ocfs2_super,
    map: *mut ocfs2_node_map,
    bit: i32,
) {
    if bit == -1 {
        return;
    }
    BUG_ON(bit >= (*map).num_nodes);
    spin_lock(&mut (*osb).node_map_lock);
    clear_bit(bit, (*map).map.as_mut_ptr());
    spin_unlock(&mut (*osb).node_map_lock);
}

pub unsafe fn ocfs2_node_map_test_bit(
    osb: *mut ocfs2_super,
    map: *mut ocfs2_node_map,
    bit: i32,
) -> i32 {
    let ret: i32;
    if bit >= (*map).num_nodes {
        mlog(ML_ERROR, b"bit=%d map->num_nodes=%d\0".as_ptr(), bit, (*map).num_nodes);
        BUG();
    }
    spin_lock(&mut (*osb).node_map_lock);
    ret = test_bit(bit, (*map).map.as_ptr());
    spin_unlock(&mut (*osb).node_map_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
