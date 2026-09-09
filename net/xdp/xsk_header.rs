/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2019 Intel Corporation. */

#[repr(C)]
pub struct xdp_ring_offset_v1 {
    pub producer: u64,
    pub consumer: u64,
    pub desc: u64,
}

#[repr(C)]
pub struct xdp_mmap_offsets_v1 {
    pub rx: xdp_ring_offset_v1,
    pub tx: xdp_ring_offset_v1,
    pub fr: xdp_ring_offset_v1,
    pub cr: xdp_ring_offset_v1,
}

/* Nodes are linked in the struct xdp_sock map_list field, and used to
 * track which maps a certain socket reside in.
 */

#[repr(C)]
pub struct xsk_map_node {
    pub node: list_head,
    pub map: *mut xsk_map,
    pub map_entry: *mut *mut xdp_sock,
}

pub unsafe fn xdp_sk(sk: *mut sock) -> *mut xdp_sock {
    sk as *mut xdp_sock
}

unsafe extern "C" {
    pub fn xsk_map_try_sock_delete(
        map: *mut xsk_map,
        xs: *mut xdp_sock,
        map_entry: *mut *mut xdp_sock,
    );
    pub fn xsk_clear_pool_at_qid(dev: *mut net_device, queue_id: u16);
    pub fn xsk_reg_pool_at_qid(
        dev: *mut net_device,
        pool: *mut xsk_buff_pool,
        queue_id: u16,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
