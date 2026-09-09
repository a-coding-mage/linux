// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Linux kernel dependencies are supplied by the surrounding translation.

const BLOOM_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_ZERO_SEED | BPF_F_ACCESS_MASK;

#[repr(C)]
pub struct bpf_bloom_filter {
    pub map: bpf_map,
    pub bitset_mask: u32,
    pub hash_seed: u32,
    pub nr_hash_funcs: u32,
    pub bitset: [c_ulong; 0],
}

unsafe fn hash(bloom: *mut bpf_bloom_filter, value: *mut c_void,
               value_size: u32, index: u32) -> u32 {
    let h: u32;
    if value_size % 4 == 0 {
        h = jhash2(value, value_size / 4, (*bloom).hash_seed.wrapping_add(index));
    } else {
        h = jhash(value, value_size, (*bloom).hash_seed.wrapping_add(index));
    }
    h & (*bloom).bitset_mask
}

unsafe fn bloom_map_peek_elem(map: *mut bpf_map, value: *mut c_void) -> c_long {
    let bloom = container_of!(map, bpf_bloom_filter, map);
    let mut i: u32 = 0;
    while i < (*bloom).nr_hash_funcs {
        let h = hash(bloom, value, (*map).value_size, i);
        if !test_bit(h % BITS_PER_LONG, (*bloom).bitset.as_mut_ptr().add(BIT_WORD(h))) {
            return -ENOENT;
        }
        i += 1;
    }
    0
}

unsafe fn bloom_map_push_elem(map: *mut bpf_map, value: *mut c_void, flags: u64) -> c_long {
    let bloom = container_of!(map, bpf_bloom_filter, map);
    if flags != BPF_ANY { return -EINVAL; }
    let mut i: u32 = 0;
    while i < (*bloom).nr_hash_funcs {
        let h = hash(bloom, value, (*map).value_size, i);
        set_bit(h % BITS_PER_LONG, (*bloom).bitset.as_mut_ptr().add(BIT_WORD(h)));
        i += 1;
    }
    0
}

unsafe fn bloom_map_pop_elem(_map: *mut bpf_map, _value: *mut c_void) -> c_long { -EOPNOTSUPP }
unsafe fn bloom_map_delete_elem(_map: *mut bpf_map, _value: *mut c_void) -> c_long { -EOPNOTSUPP }
unsafe fn bloom_map_get_next_key(_map: *mut bpf_map, _key: *mut c_void, _next_key: *mut c_void) -> c_int { -EOPNOTSUPP }

unsafe fn bloom_map_alloc_check(attr: *mut bpf_attr) -> c_int {
    if (*attr).value_size > KMALLOC_MAX_SIZE { return -E2BIG; }
    0
}

unsafe fn bloom_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let mut bitset_mask: u32;
    let mut nr_hash_funcs: u32;
    let mut nr_bits: u32;
    let numa_node = bpf_map_attr_numa_node(attr);
    let mut bitset_bytes: u64;

    if (*attr).key_size != 0 || (*attr).value_size == 0 || (*attr).max_entries == 0 ||
       ((*attr).map_flags & !BLOOM_CREATE_FLAG_MASK) != 0 ||
       !bpf_map_flags_access_ok((*attr).map_flags) || ((*attr).map_extra & !0xF) != 0 {
        return ERR_PTR(-EINVAL);
    }
    nr_hash_funcs = (*attr).map_extra as u32;
    if nr_hash_funcs == 0 { nr_hash_funcs = 5; }

    if check_mul_overflow((*attr).max_entries, nr_hash_funcs, &mut nr_bits) ||
       check_mul_overflow(nr_bits / 5, 7u32, &mut nr_bits) || nr_bits > (1u64 << 31) as u32 {
        bitset_mask = U32_MAX;
    } else {
        if nr_bits <= BITS_PER_LONG as u32 { nr_bits = BITS_PER_LONG as u32; }
        else { nr_bits = roundup_pow_of_two(nr_bits); }
        bitset_mask = nr_bits - 1;
    }
    bitset_bytes = BITS_TO_LONGS((bitset_mask as u64) + 1) as u64 * core::mem::size_of::<c_ulong>() as u64;
    let bloom = bpf_map_area_alloc(core::mem::size_of::<bpf_bloom_filter>() as u64 + bitset_bytes, numa_node) as *mut bpf_bloom_filter;
    if bloom.is_null() { return ERR_PTR(-ENOMEM); }
    bpf_map_init_from_attr(&mut (*bloom).map, attr);
    (*bloom).nr_hash_funcs = nr_hash_funcs;
    (*bloom).bitset_mask = bitset_mask;
    if (*attr).map_flags & BPF_F_ZERO_SEED == 0 { (*bloom).hash_seed = get_random_u32(); }
    &mut (*bloom).map
}

unsafe fn bloom_map_free(map: *mut bpf_map) {
    let bloom = container_of!(map, bpf_bloom_filter, map);
    bpf_map_area_free(bloom as *mut c_void);
}

unsafe fn bloom_map_lookup_elem(_map: *mut bpf_map, _key: *mut c_void) -> *mut c_void { ERR_PTR(-EINVAL) }
unsafe fn bloom_map_update_elem(_map: *mut bpf_map, _key: *mut c_void, _value: *mut c_void, _flags: u64) -> c_long { -EINVAL }

unsafe fn bloom_map_check_btf(_map: *mut bpf_map, _btf: *const btf, key_type: *const btf_type, _value_type: *const btf_type) -> c_int {
    if btf_type_is_void(key_type) { 0 } else { -EINVAL }
}

unsafe fn bloom_map_mem_usage(map: *const bpf_map) -> u64 {
    let bloom = container_of!(map as *mut bpf_map, bpf_bloom_filter, map);
    let mut bitset_bytes = BITS_TO_BYTES((*bloom).bitset_mask as u64 + 1) as u64;
    bitset_bytes = roundup(bitset_bytes, core::mem::size_of::<c_ulong>() as u64);
    core::mem::size_of::<bpf_bloom_filter>() as u64 + bitset_bytes
}

// BTF_ID_LIST_SINGLE(bpf_bloom_map_btf_ids, struct, bpf_bloom_filter)
pub static mut bpf_bloom_map_btf_ids: [u32; 1] = [0];

pub static bloom_filter_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal),
    map_alloc_check: Some(bloom_map_alloc_check),
    map_alloc: Some(bloom_map_alloc),
    map_free: Some(bloom_map_free),
    map_get_next_key: Some(bloom_map_get_next_key),
    map_push_elem: Some(bloom_map_push_elem),
    map_peek_elem: Some(bloom_map_peek_elem),
    map_pop_elem: Some(bloom_map_pop_elem),
    map_lookup_elem: Some(bloom_map_lookup_elem),
    map_update_elem: Some(bloom_map_update_elem),
    map_delete_elem: Some(bloom_map_delete_elem),
    map_check_btf: Some(bloom_map_check_btf),
    map_mem_usage: Some(bloom_map_mem_usage),
    map_btf_id: unsafe { &bpf_bloom_map_btf_ids[0] },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
