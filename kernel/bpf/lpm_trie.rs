// SPDX-License-Identifier: GPL-2.0-only
/* Longest prefix match list implementation. */

/* Kernel dependencies supplied by the surrounding translation unit. */

const LPM_TREE_NODE_FLAG_IM: u32 = 1 << 0;

#[repr(C)]
pub struct lpm_trie_node {
    pub child: [*mut lpm_trie_node; 2],
    pub prefixlen: u32,
    pub flags: u32,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct lpm_trie {
    pub map: bpf_map,
    pub root: *mut lpm_trie_node,
    pub ma: bpf_mem_alloc,
    pub n_entries: usize,
    pub max_prefixlen: usize,
    pub data_size: usize,
    pub lock: rqspinlock_t,
}

#[inline]
unsafe fn extract_bit(data: *const u8, index: usize) -> i32 {
    ((*data.add(index / 8) & (1 << (7 - (index % 8)))) != 0) as i32
}

#[inline(always)]
unsafe fn __longest_prefix_match(trie: *const lpm_trie, node: *const lpm_trie_node,
    key: *const bpf_lpm_trie_key_u8) -> usize {
    let limit = (*node).prefixlen.min((*key).prefixlen);
    let mut prefixlen: u32 = 0;
    let mut i: usize = 0;
    while (*trie).data_size >= i + 4 {
        let a = u32::from_be_bytes(*(std::ptr::addr_of!((*node).data).cast::<u8>().add(i) as *const [u8; 4]));
        let b = u32::from_be_bytes(*(std::ptr::addr_of!((*key).data).cast::<u8>().add(i) as *const [u8; 4]));
        let diff = a ^ b;
        prefixlen = prefixlen.wrapping_add(if diff == 0 { 32 } else { diff.leading_zeros() });
        if prefixlen >= limit { return limit as usize; }
        if diff != 0 { return prefixlen as usize; }
        i += 4;
    }
    if (*trie).data_size >= i + 2 {
        let a = u16::from_be_bytes(*(std::ptr::addr_of!((*node).data).cast::<u8>().add(i) as *const [u8; 2]));
        let b = u16::from_be_bytes(*(std::ptr::addr_of!((*key).data).cast::<u8>().add(i) as *const [u8; 2]));
        let diff = a ^ b;
        prefixlen = prefixlen.wrapping_add(if diff == 0 { 16 } else { diff.leading_zeros() - 16 });
        if prefixlen >= limit { return limit as usize; }
        if diff != 0 { return prefixlen as usize; }
        i += 2;
    }
    if (*trie).data_size >= i + 1 {
        prefixlen = prefixlen.wrapping_add(((*node).data.as_ptr().add(i).read() ^ (*key).data.as_ptr().add(i).read()).leading_zeros() - 24);
        if prefixlen >= limit { return limit as usize; }
    }
    prefixlen as usize
}

unsafe fn longest_prefix_match(t: *const lpm_trie, n: *const lpm_trie_node, k: *const bpf_lpm_trie_key_u8) -> usize {
    __longest_prefix_match(t, n, k)
}

unsafe fn lpm_trie_node_alloc(trie: *mut lpm_trie, value: *const core::ffi::c_void) -> *mut lpm_trie_node {
    let node = bpf_mem_cache_alloc(&mut (*trie).ma);
    if node.is_null() { return core::ptr::null_mut(); }
    (*node).flags = 0;
    if !value.is_null() {
        core::ptr::copy_nonoverlapping(value.cast::<u8>(), (*node).data.as_mut_ptr().add((*trie).data_size), (*trie).map.value_size as usize);
    }
    node
}

unsafe fn trie_check_add_elem(trie: *mut lpm_trie, flags: u64) -> i32 {
    if flags == BPF_EXIST { return -ENOENT; }
    if (*trie).n_entries == (*trie).map.max_entries as usize { return -ENOSPC; }
    (*trie).n_entries += 1; 0
}

unsafe fn trie_lookup_elem(map: *mut bpf_map, keyp: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let trie = container_of!(map, lpm_trie, map);
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    if (*key).prefixlen > (*trie).max_prefixlen as u32 { return core::ptr::null_mut(); }
    let mut node = (*trie).root;
    let mut found = core::ptr::null_mut();
    while !node.is_null() {
        let m = __longest_prefix_match(trie, node, key);
        if m == (*trie).max_prefixlen { found = node; break; }
        if m < (*node).prefixlen as usize { break; }
        if (*node).flags & LPM_TREE_NODE_FLAG_IM == 0 { found = node; }
        node = (*node).child[extract_bit((*key).data.as_ptr(), (*node).prefixlen as usize) as usize];
    }
    if found.is_null() { core::ptr::null_mut() } else { (*found).data.as_mut_ptr().add((*trie).data_size).cast() }
}

unsafe fn trie_update_elem(map: *mut bpf_map, keyp: *mut core::ffi::c_void, value: *mut core::ffi::c_void, flags: u64) -> i64 {
    let trie = container_of!(map, lpm_trie, map);
    if flags > BPF_EXIST || (*keyp.cast::<bpf_lpm_trie_key_u8>()).prefixlen > (*trie).max_prefixlen as u32 { return -EINVAL as i64; }
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    let new_node = lpm_trie_node_alloc(trie, value);
    if new_node.is_null() { return -ENOMEM as i64; }
    (*new_node).prefixlen = (*key).prefixlen;
    (*new_node).child = [core::ptr::null_mut(); 2];
    core::ptr::copy_nonoverlapping((*key).data.as_ptr(), (*new_node).data.as_mut_ptr(), (*trie).data_size);
    let mut slot: *mut *mut lpm_trie_node = &mut (*trie).root;
    let mut node;
    loop {
        node = *slot;
        if node.is_null() { break; }
        let m = longest_prefix_match(trie, node, key);
        if (*node).prefixlen as usize != m || (*node).prefixlen == (*key).prefixlen { break; }
        slot = &mut (*node).child[extract_bit((*key).data.as_ptr(), (*node).prefixlen as usize) as usize];
    }
    if node.is_null() {
        let r = trie_check_add_elem(trie, flags); if r != 0 { bpf_mem_cache_free(&mut (*trie).ma, new_node); return r as i64; }
        *slot = new_node; return 0;
    }
    let m = longest_prefix_match(trie, node, key);
    if (*node).prefixlen as usize == m {
        if (*node).flags & LPM_TREE_NODE_FLAG_IM == 0 && flags == BPF_NOEXIST { bpf_mem_cache_free(&mut (*trie).ma, new_node); return -EEXIST as i64; }
        if (*node).flags & LPM_TREE_NODE_FLAG_IM != 0 { let r = trie_check_add_elem(trie, flags); if r != 0 { bpf_mem_cache_free(&mut (*trie).ma, new_node); return r as i64; } }
        (*new_node).child = (*node).child; *slot = new_node; bpf_mem_cache_free_rcu(&mut (*trie).ma, node); return 0;
    }
    let r = trie_check_add_elem(trie, flags); if r != 0 { bpf_mem_cache_free(&mut (*trie).ma, new_node); return r as i64; }
    if m == (*key).prefixlen as usize {
        (*new_node).child[extract_bit((*node).data.as_ptr(), m) as usize] = node; *slot = new_node; return 0;
    }
    let im = lpm_trie_node_alloc(trie, core::ptr::null());
    if im.is_null() { (*trie).n_entries -= 1; bpf_mem_cache_free(&mut (*trie).ma, new_node); return -ENOMEM as i64; }
    (*im).prefixlen = m as u32; (*im).flags = LPM_TREE_NODE_FLAG_IM;
    core::ptr::copy_nonoverlapping((*node).data.as_ptr(), (*im).data.as_mut_ptr(), (*trie).data_size);
    let bit = extract_bit((*key).data.as_ptr(), m) as usize;
    (*im).child[bit] = new_node; (*im).child[1 - bit] = node; *slot = im; 0
}

unsafe fn trie_delete_elem(map: *mut bpf_map, keyp: *mut core::ffi::c_void) -> i64 {
    let trie = container_of!(map, lpm_trie, map); let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    if (*key).prefixlen > (*trie).max_prefixlen as u32 { return -EINVAL as i64; }
    let mut slot: *mut *mut lpm_trie_node = &mut (*trie).root; let mut node = *slot;
    while !node.is_null() { let m = longest_prefix_match(trie, node, key); if (*node).prefixlen as usize != m || (*node).prefixlen == (*key).prefixlen { break; } slot = &mut (*node).child[extract_bit((*key).data.as_ptr(), (*node).prefixlen as usize) as usize]; node = *slot; }
    if node.is_null() || (*node).prefixlen != (*key).prefixlen || longest_prefix_match(trie, node, key) != (*node).prefixlen as usize || (*node).flags & LPM_TREE_NODE_FLAG_IM != 0 { return -ENOENT as i64; }
    (*trie).n_entries -= 1;
    if !(*node).child[0].is_null() && !(*node).child[1].is_null() { (*node).flags |= LPM_TREE_NODE_FLAG_IM; return 0; }
    *slot = if !(*node).child[0].is_null() { (*node).child[0] } else { (*node).child[1] };
    bpf_mem_cache_free_rcu(&mut (*trie).ma, node); 0
}

const LPM_DATA_SIZE_MAX: usize = 256;
const LPM_DATA_SIZE_MIN: usize = 1;
const LPM_VAL_SIZE_MIN: usize = 1;
const LPM_KEY_SIZE_MIN: usize = core::mem::size_of::<bpf_lpm_trie_key_u8>() + LPM_DATA_SIZE_MIN;
const LPM_KEY_SIZE_MAX: usize = core::mem::size_of::<bpf_lpm_trie_key_u8>() + LPM_DATA_SIZE_MAX;
const LPM_CREATE_FLAG_MASK: u64 = BPF_F_NO_PREALLOC | BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK;

/* The remaining map lifecycle/BTF operations are declaration-level kernel interfaces. */
unsafe fn trie_alloc(attr: *mut bpf_attr) -> *mut bpf_map { unimplemented!("kernel allocator and map initialization are external dependencies: {:?}", attr) }
unsafe fn trie_free(_map: *mut bpf_map) { unimplemented!("kernel allocator teardown is an external dependency") }
unsafe fn trie_get_next_key(_map: *mut bpf_map, _key: *mut core::ffi::c_void, _next: *mut core::ffi::c_void) -> i32 { unimplemented!("translated helper dependencies are external") }
unsafe fn trie_check_btf(_map: *mut bpf_map, _btf: *const btf, _key: *const btf_type, _value: *const btf_type) -> i32 { unimplemented!("BTF helpers are external") }
unsafe fn trie_mem_usage(_map: *const bpf_map) -> u64 { unimplemented!("map layout dependencies are external") }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
