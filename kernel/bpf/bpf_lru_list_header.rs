/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016 Facebook
 */

// C dependencies supplied by other translation units:
// linux/cache.h, linux/list.h, linux/llist.h, and asm/rqspinlock.h.

pub const NR_BPF_LRU_LIST_T: usize = 3;
pub const NR_BPF_LRU_LIST_COUNT: usize = 2;
pub const BPF_LOCAL_LIST_T_OFFSET: usize = NR_BPF_LRU_LIST_T;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_lru_list_type {
    BPF_LRU_LIST_T_ACTIVE,
    BPF_LRU_LIST_T_INACTIVE,
    BPF_LRU_LIST_T_FREE,
    BPF_LRU_LOCAL_LIST_T_FREE,
    BPF_LRU_LOCAL_LIST_T_PENDING,
}

#[repr(C)]
pub union bpf_lru_node_storage {
    pub list: list_head,
    pub llist: llist_node,
}

#[repr(C)]
pub struct bpf_lru_node {
    /* A node is in at most one list at a time. The free path on the
     * per-CPU locallist uses an llist, so share storage via a union. */
    pub storage: bpf_lru_node_storage,
    pub cpu: u16,
    pub type_: u8,
    pub ref_: u8,
    /* Marks nodes whose *_push_free() lock acquire failed; reclaimed
     * by flush/shrink which honor the flag instead of del_from_htab(). */
    pub pending_free: u8,
}

#[repr(C)]
pub struct bpf_lru_list {
    pub lists: [list_head; NR_BPF_LRU_LIST_T],
    pub counts: [core::ffi::c_uint; NR_BPF_LRU_LIST_COUNT],
    /* The next inactive list rotation starts from here */
    pub next_inactive_rotation: *mut list_head,
    // ____cacheline_aligned_in_smp
    pub lock: rqspinlock_t,
}

#[repr(C)]
pub struct bpf_lru_locallist {
    pub pending_list: list_head,
    pub free_llist: llist_head,
    pub next_steal: u16,
    pub lock: rqspinlock_t,
}

#[repr(C)]
pub struct bpf_common_lru {
    pub lru_list: bpf_lru_list,
    // __percpu
    pub local_list: *mut bpf_lru_locallist,
}

pub type del_from_htab_func = unsafe extern "C" fn(
    arg: *mut core::ffi::c_void,
    node: *mut bpf_lru_node,
) -> bool;

#[repr(C)]
pub union bpf_lru_storage {
    pub common_lru: bpf_common_lru,
    // __percpu
    pub percpu_lru: *mut bpf_lru_list,
}

#[repr(C)]
pub struct bpf_lru {
    pub storage: bpf_lru_storage,
    pub del_from_htab: Option<del_from_htab_func>,
    pub del_arg: *mut core::ffi::c_void,
    pub hash_offset: core::ffi::c_uint,
    pub target_free: core::ffi::c_uint,
    pub nr_scans: core::ffi::c_uint,
    pub percpu: bool,
}

#[inline]
pub unsafe fn bpf_lru_node_set_ref(node: *mut bpf_lru_node) {
    if core::ptr::read_volatile(core::ptr::addr_of!((*node).ref_)) == 0 {
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*node).ref_), 1);
    }
}

extern "C" {
    pub fn bpf_lru_init(
        lru: *mut bpf_lru,
        percpu: bool,
        hash_offset: u32,
        del_from_htab: Option<del_from_htab_func>,
        delete_arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn bpf_lru_populate(
        lru: *mut bpf_lru,
        buf: *mut core::ffi::c_void,
        node_offset: u32,
        elem_size: u32,
        nr_elems: u32,
    );
    pub fn bpf_lru_destroy(lru: *mut bpf_lru);
    pub fn bpf_lru_pop_free(lru: *mut bpf_lru, hash: u32) -> *mut bpf_lru_node;
    pub fn bpf_lru_push_free(lru: *mut bpf_lru, node: *mut bpf_lru_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
