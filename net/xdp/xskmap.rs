// SPDX-License-Identifier: GPL-2.0
/* XSKMAP used for AF_XDP sockets
 * Copyright(c) 2018 Intel Corporation.
 */

// Kernel headers and xsk.h provide the referenced types, constants, and helpers.

unsafe fn xsk_map_node_alloc(
    map: *mut xsk_map,
    map_entry: *mut *mut xdp_sock,
) -> *mut xsk_map_node {
    let node = bpf_map_kzalloc(
        &mut (*map).map as *mut bpf_map,
        core::mem::size_of::<xsk_map_node>(),
        GFP_ATOMIC | __GFP_NOWARN,
    ) as *mut xsk_map_node;
    if node.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    bpf_map_inc(&mut (*map).map);
    atomic_inc(&mut (*map).count);
    (*node).map = map;
    (*node).map_entry = map_entry;
    node
}

unsafe fn xsk_map_node_free(node: *mut xsk_map_node) {
    let map = (*node).map;
    bpf_map_put(&mut (*map).map);
    kfree(node as *mut core::ffi::c_void);
    atomic_dec(&mut (*map).count);
}

unsafe fn xsk_map_sock_add(xs: *mut xdp_sock, node: *mut xsk_map_node) {
    spin_lock_bh(&mut (*xs).map_list_lock);
    list_add_tail(&mut (*node).node, &mut (*xs).map_list);
    spin_unlock_bh(&mut (*xs).map_list_lock);
}

unsafe fn xsk_map_sock_delete(xs: *mut xdp_sock, map_entry: *mut *mut xdp_sock) {
    spin_lock_bh(&mut (*xs).map_list_lock);
    let mut n = (*xs).map_list.next as *mut xsk_map_node;
    while n != &mut (*xs).map_list as *mut _ as *mut xsk_map_node {
        let tmp = (*n).node.next as *mut xsk_map_node;
        if (*n).map_entry == map_entry {
            list_del(&mut (*n).node);
            xsk_map_node_free(n);
        }
        n = tmp;
    }
    spin_unlock_bh(&mut (*xs).map_list_lock);
}

unsafe fn xsk_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    if (*attr).max_entries == 0 || (*attr).key_size != 4 || (*attr).value_size != 4
        || (*attr).map_flags & !(BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY) != 0
    {
        return ERR_PTR(-EINVAL);
    }
    let numa_node = bpf_map_attr_numa_node(attr);
    let size = struct_size_xsk_map((*attr).max_entries);
    let m = bpf_map_area_alloc(size, numa_node) as *mut xsk_map;
    if m.is_null() { return ERR_PTR(-ENOMEM); }
    bpf_map_init_from_attr(&mut (*m).map, attr);
    spin_lock_init(&mut (*m).lock);
    &mut (*m).map
}

unsafe fn xsk_map_mem_usage(map: *const bpf_map) -> u64 {
    let m = container_of_xsk_map(map);
    struct_size_xsk_map((*map).max_entries) as u64
        + (atomic_read(&(*m).count) as u64) * core::mem::size_of::<xsk_map_node>() as u64
}

unsafe fn xsk_map_free(map: *mut bpf_map) {
    synchronize_net();
    bpf_map_area_free(container_of_xsk_map(map));
}

unsafe fn xsk_map_get_next_key(map: *mut bpf_map, key: *const core::ffi::c_void, next_key: *mut core::ffi::c_void) -> i32 {
    let m = container_of_xsk_map(map);
    let index = if key.is_null() { u32::MAX } else { *(key as *const u32) };
    let next = next_key as *mut u32;
    if index >= (*m).map.max_entries { *next = 0; return 0; }
    if index == (*m).map.max_entries - 1 { return -ENOENT; }
    *next = index + 1; 0
}

unsafe fn xsk_map_gen_lookup(map: *mut bpf_map, insn_buf: *mut bpf_insn) -> i32 {
    let ret = BPF_REG_0; let mp = BPF_REG_1; let index = BPF_REG_2;
    let mut insn = insn_buf;
    *insn = BPF_LDX_MEM(BPF_W, ret, index, 0); insn = insn.add(1);
    *insn = BPF_JMP_IMM(BPF_JGE, ret, (*map).max_entries, 5); insn = insn.add(1);
    *insn = BPF_ALU64_IMM(BPF_LSH, ret, ilog2(core::mem::size_of::<*mut xsk_sock>())); insn = insn.add(1);
    *insn = BPF_ALU64_IMM(BPF_ADD, mp, offsetof_xsk_map()); insn = insn.add(1);
    *insn = BPF_ALU64_REG(BPF_ADD, ret, mp); insn = insn.add(1);
    *insn = BPF_LDX_MEM(core::mem::size_of::<*mut xsk_sock>(), ret, ret, 0); insn = insn.add(1);
    *insn = BPF_JMP_IMM(BPF_JA, 0, 0, 1); insn = insn.add(1);
    *insn = BPF_MOV64_IMM(ret, 0); insn.offset_from(insn_buf) as i32 + 1
}

// Elements are kept alive by RCU; rcu_read_lock() or local_bh_disable() protects them.
unsafe fn __xsk_map_lookup_elem(map: *mut bpf_map, key: u32) -> *mut core::ffi::c_void {
    let m = container_of_xsk_map(map);
    if key >= (*map).max_entries { return core::ptr::null_mut(); }
    rcu_dereference_check((*m).xsk_map[key as usize], rcu_read_lock_bh_held())
}

unsafe fn xsk_map_lookup_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    __xsk_map_lookup_elem(map, *(key as *const u32))
}

unsafe fn xsk_map_lookup_elem_sys_only(_: *mut bpf_map, _: *mut core::ffi::c_void) -> *mut core::ffi::c_void { ERR_PTR(-EOPNOTSUPP) }

unsafe fn xsk_map_update_elem(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, map_flags: u64) -> i64 {
    let m = container_of_xsk_map(map);
    let i = *(key as *const u32); let fd = *(value as *const u32);
    if unlikely(map_flags > BPF_EXIST) { return -EINVAL; }
    if unlikely(i >= (*m).map.max_entries) { return -E2BIG; }
    let mut err = 0;
    let sock = sockfd_lookup(fd, &mut err);
    if sock.is_null() { return err as i64; }
    if (*(*sock).sk).sk_family != PF_XDP { sockfd_put(sock); return -EOPNOTSUPP; }
    let xs = (*sock).sk as *mut xdp_sock;
    if !READ_ONCE((*xs).rx) { sockfd_put(sock); return -ENOBUFS; }
    let map_entry = (*m).xsk_map.as_mut_ptr().add(i as usize);
    let node = xsk_map_node_alloc(m, map_entry);
    if IS_ERR(node) { sockfd_put(sock); return PTR_ERR(node) as i64; }
    spin_lock_bh(&mut (*m).lock);
    let old_xs = rcu_dereference_protected(*map_entry, lockdep_is_held(&(*m).lock));
    if old_xs == xs { err = 0; }
    else if !old_xs.is_null() && map_flags == BPF_NOEXIST { err = -EEXIST; }
    else if old_xs.is_null() && map_flags == BPF_EXIST { err = -ENOENT; }
    else {
        xsk_map_sock_add(xs, node); rcu_assign_pointer(map_entry, xs);
        if !old_xs.is_null() { xsk_map_sock_delete(old_xs, map_entry); }
        spin_unlock_bh(&mut (*m).lock); sockfd_put(sock); return 0;
    }
    spin_unlock_bh(&mut (*m).lock); sockfd_put(sock); xsk_map_node_free(node); err as i64
}

unsafe fn xsk_map_delete_elem(map: *mut bpf_map, key: *mut core::ffi::c_void) -> i64 {
    let m = container_of_xsk_map(map); let k = *(key as *const u32);
    if k >= (*map).max_entries { return -EINVAL; }
    spin_lock_bh(&mut (*m).lock);
    let entry = (*m).xsk_map.as_mut_ptr().add(k as usize);
    let old_xs = unrcu_pointer(xchg(entry, core::ptr::null_mut()));
    if !old_xs.is_null() { xsk_map_sock_delete(old_xs, entry); }
    spin_unlock_bh(&mut (*m).lock); 0
}

unsafe fn xsk_map_redirect(map: *mut bpf_map, index: u64, flags: u64) -> i64 {
    __bpf_xdp_redirect_map(map, index, flags, 0, __xsk_map_lookup_elem)
}

pub unsafe fn xsk_map_try_sock_delete(map: *mut xsk_map, xs: *mut xdp_sock, map_entry: *mut *mut xdp_sock) {
    spin_lock_bh(&mut (*map).lock);
    if rcu_access_pointer(*map_entry) == xs { rcu_assign_pointer(map_entry, core::ptr::null_mut()); xsk_map_sock_delete(xs, map_entry); }
    spin_unlock_bh(&mut (*map).lock);
}

unsafe fn xsk_map_meta_equal(meta0: *const bpf_map, meta1: *const bpf_map) -> bool {
    (*meta0).max_entries == (*meta1).max_entries && bpf_map_meta_equal(meta0, meta1)
}

// BTF_ID_LIST_SINGLE(xsk_map_btf_ids, struct, xsk_map)
#[no_mangle]
pub static xsk_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(xsk_map_meta_equal), map_alloc: Some(xsk_map_alloc), map_free: Some(xsk_map_free),
    map_get_next_key: Some(xsk_map_get_next_key), map_lookup_elem: Some(xsk_map_lookup_elem),
    map_gen_lookup: Some(xsk_map_gen_lookup), map_lookup_elem_sys_only: Some(xsk_map_lookup_elem_sys_only),
    map_update_elem: Some(xsk_map_update_elem), map_delete_elem: Some(xsk_map_delete_elem),
    map_check_btf: Some(map_check_no_btf), map_mem_usage: Some(xsk_map_mem_usage),
    map_btf_id: core::ptr::null(), map_redirect: Some(xsk_map_redirect),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
