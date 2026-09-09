// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook */
// Linux kernel dependencies and bpf_lru_list.h are supplied externally.

const LOCAL_FREE_TARGET: u32 = 128;
const LOCAL_NR_SCANS: u32 = LOCAL_FREE_TARGET;
const PERCPU_FREE_TARGET: u32 = 4;
const PERCPU_NR_SCANS: u32 = PERCPU_FREE_TARGET;

#[inline]
unsafe fn is_local_list_type(t: u8) -> bool { t >= BPF_LOCAL_LIST_T_OFFSET }

unsafe fn bpf_lru_node_is_ref(node: *const bpf_lru_node) -> bool { (*node).ref_ != 0 }
unsafe fn bpf_lru_node_clear_ref(node: *mut bpf_lru_node) { (*node).ref_ = 0; }

unsafe fn bpf_lru_list_count_inc(l: *mut bpf_lru_list, ty: bpf_lru_list_type) {
    if ty < NR_BPF_LRU_LIST_COUNT { (*l).counts[ty as usize] += 1; }
}
unsafe fn bpf_lru_list_count_dec(l: *mut bpf_lru_list, ty: bpf_lru_list_type) {
    if ty < NR_BPF_LRU_LIST_COUNT { (*l).counts[ty as usize] -= 1; }
}

unsafe fn __bpf_lru_node_move_to_free(l: *mut bpf_lru_list, node: *mut bpf_lru_node,
    free_list: *mut list_head, tgt_free_type: bpf_lru_list_type) {
    if is_local_list_type((*node).type_) { return; }
    if &mut (*node).list == (*l).next_inactive_rotation { (*l).next_inactive_rotation = (*l).next_inactive_rotation.as_mut().unwrap().prev; }
    bpf_lru_list_count_dec(l, (*node).type_);
    (*node).type_ = tgt_free_type;
    (*node).pending_free = 0;
    list_move(&mut (*node).list, free_list);
}

unsafe fn __bpf_lru_node_move_in(l: *mut bpf_lru_list, node: *mut bpf_lru_node, tgt_type: bpf_lru_list_type) {
    if !is_local_list_type((*node).type_) || is_local_list_type(tgt_type) { return; }
    bpf_lru_list_count_inc(l, tgt_type); (*node).type_ = tgt_type; bpf_lru_node_clear_ref(node);
    if tgt_type == BPF_LRU_LIST_T_FREE { (*node).pending_free = 0; }
    list_move(&mut (*node).list, &mut (*l).lists[tgt_type as usize]);
}

unsafe fn __bpf_lru_node_move(l: *mut bpf_lru_list, node: *mut bpf_lru_node, tgt_type: bpf_lru_list_type) {
    if is_local_list_type((*node).type_) || is_local_list_type(tgt_type) { return; }
    if (*node).type_ != tgt_type { bpf_lru_list_count_dec(l, (*node).type_); bpf_lru_list_count_inc(l, tgt_type); (*node).type_ = tgt_type; }
    bpf_lru_node_clear_ref(node);
    if &mut (*node).list == (*l).next_inactive_rotation { (*l).next_inactive_rotation = (*l).next_inactive_rotation.as_mut().unwrap().prev; }
    list_move(&mut (*node).list, &mut (*l).lists[tgt_type as usize]);
}

unsafe fn bpf_lru_list_inactive_low(l: *const bpf_lru_list) -> bool { (*l).counts[BPF_LRU_LIST_T_INACTIVE as usize] < (*l).counts[BPF_LRU_LIST_T_ACTIVE as usize] }

unsafe fn __bpf_lru_list_rotate_active(lru: *mut bpf_lru, l: *mut bpf_lru_list) {
    let active = &mut (*l).lists[BPF_LRU_LIST_T_ACTIVE as usize];
    let first_node = list_first_entry(active, bpf_lru_node, list);
    let mut i = 0u32;
    let mut node: *mut bpf_lru_node = core::ptr::null_mut();
    let mut tmp: *mut bpf_lru_node = core::ptr::null_mut();
    list_for_each_entry_safe_reverse!(node, tmp, active, list, bpf_lru_node);
    while !node.is_null() {
        if bpf_lru_node_is_ref(node) { __bpf_lru_node_move(l, node, BPF_LRU_LIST_T_ACTIVE); } else { __bpf_lru_node_move(l, node, BPF_LRU_LIST_T_INACTIVE); }
        i += 1; if i == (*lru).nr_scans || node == first_node { break; }
        node = tmp;
    }
}

unsafe fn __bpf_lru_list_rotate_inactive(lru: *mut bpf_lru, l: *mut bpf_lru_list) {
    let inactive = &mut (*l).lists[BPF_LRU_LIST_T_INACTIVE as usize];
    if list_empty(inactive) { return; }
    let mut cur = (*l).next_inactive_rotation;
    let mut last = cur.as_mut().unwrap().next;
    if last == inactive { last = last.as_mut().unwrap().next; }
    let mut next = inactive; let mut i = 0u32;
    while i < (*lru).nr_scans {
        if cur == inactive { cur = cur.as_mut().unwrap().prev; continue; }
        let node = list_entry(cur, bpf_lru_node, list); next = cur.as_mut().unwrap().prev;
        if bpf_lru_node_is_ref(node) { __bpf_lru_node_move(l, node, BPF_LRU_LIST_T_ACTIVE); }
        if cur == last { break; } cur = next; i += 1;
    }
    (*l).next_inactive_rotation = next;
}

unsafe fn __bpf_lru_list_rotate(lru: *mut bpf_lru, l: *mut bpf_lru_list) { if bpf_lru_list_inactive_low(l) { __bpf_lru_list_rotate_active(lru,l); } __bpf_lru_list_rotate_inactive(lru,l); }

unsafe fn __bpf_lru_list_shrink(lru: *mut bpf_lru, l: *mut bpf_lru_list, n: u32, free_list: *mut list_head, ty: bpf_lru_list_type) -> u32 {
    let _ = (lru, l, n, free_list, ty); 0
}
unsafe fn bpf_lru_list_push_free(l: *mut bpf_lru_list, node: *mut bpf_lru_node) {
    if is_local_list_type((*node).type_) { return; }
    __bpf_lru_node_move(l, node, BPF_LRU_LIST_T_FREE);
}
unsafe fn bpf_lru_populate(lru: *mut bpf_lru, buf: *mut core::ffi::c_void, node_offset: u32, elem_size: u32, nr_elems: u32) {
    let l = &mut (*lru).common_lru.lru_list;
    for i in 0..nr_elems { let node = (buf as *mut u8).add(node_offset as usize + i as usize * elem_size as usize) as *mut bpf_lru_node; (*node).type_ = BPF_LRU_LIST_T_FREE; (*node).pending_free = 0; bpf_lru_node_clear_ref(node); list_add(&mut (*node).list, &mut l.lists[BPF_LRU_LIST_T_FREE as usize]); }
}
unsafe fn bpf_lru_push_free(lru: *mut bpf_lru, node: *mut bpf_lru_node) { bpf_lru_list_push_free(&mut (*lru).common_lru.lru_list, node); }
unsafe fn bpf_lru_pop_free(_lru: *mut bpf_lru, _hash: u32) -> *mut bpf_lru_node { core::ptr::null_mut() }
unsafe fn bpf_lru_init(lru: *mut bpf_lru, percpu: bool, hash_offset: u32, del_from_htab: del_from_htab_func, del_arg: *mut core::ffi::c_void) -> i32 {
    (*lru).percpu = percpu; (*lru).hash_offset = hash_offset; (*lru).del_from_htab = del_from_htab; (*lru).del_arg = del_arg; 0
}
unsafe fn bpf_lru_destroy(_lru: *mut bpf_lru) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
