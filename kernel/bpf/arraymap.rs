// SPDX-License-Identifier: GPL-2.0-only
/* Translation of bpf/arraymap.c. Kernel-provided types, constants, macros,
 * and helper functions are intentionally referenced as external dependencies.
 */

const ARRAY_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_MMAPABLE |
    BPF_F_ACCESS_MASK | BPF_F_PRESERVE_ELEMS | BPF_F_INNER_MAP;

unsafe fn bpf_array_free_percpu(array: *mut bpf_array) {
    for i in 0..(*array).map.max_entries {
        free_percpu((*array).pptrs[i as usize]);
        cond_resched();
    }
}

unsafe fn bpf_array_alloc_percpu(array: *mut bpf_array) -> i32 {
    for i in 0..(*array).map.max_entries {
        let ptr = bpf_map_alloc_percpu(&mut (*array).map, (*array).elem_size, 8,
                                       GFP_USER | __GFP_NOWARN);
        if ptr.is_null() {
            bpf_array_free_percpu(array);
            return -ENOMEM;
        }
        (*array).pptrs[i as usize] = ptr;
        cond_resched();
    }
    0
}

pub unsafe fn array_map_alloc_check(attr: *mut bpf_attr) -> i32 {
    let percpu = (*attr).map_type == BPF_MAP_TYPE_PERCPU_ARRAY;
    let numa_node = bpf_map_attr_numa_node(attr);
    if (*attr).max_entries == 0 || (*attr).key_size != 4 || (*attr).value_size == 0 ||
       ((*attr).map_flags & !ARRAY_CREATE_FLAG_MASK) != 0 ||
       !bpf_map_flags_access_ok((*attr).map_flags) ||
       (percpu && numa_node != NUMA_NO_NODE) { return -EINVAL; }
    if (*attr).map_type != BPF_MAP_TYPE_ARRAY &&
       ((*attr).map_flags & (BPF_F_MMAPABLE | BPF_F_INNER_MAP)) != 0 { return -EINVAL; }
    if (*attr).map_type != BPF_MAP_TYPE_PERF_EVENT_ARRAY &&
       ((*attr).map_flags & BPF_F_PRESERVE_ELEMS) != 0 { return -EINVAL; }
    if (*attr).value_size > INT_MAX { return -E2BIG; }
    if percpu && round_up((*attr).value_size, 8) > PCPU_MIN_UNIT_SIZE { return -E2BIG; }
    0
}

unsafe fn array_map_elem_ptr(array: *mut bpf_array, index: u32) -> *mut u8 {
    (*array).value.add(((*array).elem_size as u64 * index as u64) as usize)
}

unsafe fn array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let array = container_of!(map, bpf_array, map);
    let index = *(key as *const u32);
    if unlikely(index >= (*array).map.max_entries) { return core::ptr::null_mut(); }
    array_map_elem_ptr(array, index & (*array).index_mask) as *mut c_void
}

unsafe fn percpu_array_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let array = container_of!(map, bpf_array, map);
    let index = *(key as *const u32);
    if unlikely(index >= (*array).map.max_entries) { return core::ptr::null_mut(); }
    this_cpu_ptr((*array).pptrs[(index & (*array).index_mask) as usize])
}

pub unsafe fn bpf_array_get_next_key(map: *mut bpf_map, key: *mut c_void,
                                     next_key: *mut c_void) -> i32 {
    let index = if key.is_null() { u32::MAX } else { *(key as *const u32) };
    let next = next_key as *mut u32;
    if index >= (*map).max_entries { *next = 0; return 0; }
    if index == (*map).max_entries - 1 { return -ENOENT; }
    *next = index + 1; 0
}

unsafe fn array_map_delete_elem(_map: *mut bpf_map, _key: *mut c_void) -> i64 { -EINVAL as i64 }

unsafe fn array_map_update_elem(map: *mut bpf_map, key: *mut c_void,
                                value: *mut c_void, map_flags: u64) -> i64 {
    let array = container_of!(map, bpf_array, map);
    let index = *(key as *const u32);
    if unlikely((map_flags & !BPF_F_LOCK) > BPF_EXIST as u64) { return -EINVAL as i64; }
    if unlikely(index >= (*array).map.max_entries) { return -E2BIG as i64; }
    if unlikely((map_flags & BPF_NOEXIST as u64) != 0) { return -EEXIST as i64; }
    if unlikely((map_flags & BPF_F_LOCK as u64) != 0 &&
                !btf_record_has_field((*map).record, BPF_SPIN_LOCK)) { return -EINVAL as i64; }
    let val = if (*array).map.map_type == BPF_MAP_TYPE_PERCPU_ARRAY {
        this_cpu_ptr((*array).pptrs[(index & (*array).index_mask) as usize])
    } else { array_map_elem_ptr(array, index & (*array).index_mask) as *mut c_void };
    if (*array).map.map_type != BPF_MAP_TYPE_PERCPU_ARRAY &&
       (map_flags & BPF_F_LOCK as u64) != 0 {
        copy_map_value_locked(map, val, value, false);
    } else { copy_map_value(map, val, value); }
    bpf_obj_cancel_fields(map, val); 0
}

unsafe fn array_map_mem_usage(map: *const bpf_map) -> u64 {
    let array = container_of!(map, bpf_array, map);
    let entries = (*map).max_entries as u64;
    let mut usage = core::mem::size_of::<bpf_array>() as u64;
    if (*map).map_type == BPF_MAP_TYPE_PERCPU_ARRAY {
        usage += entries * core::mem::size_of::<*mut c_void>() as u64;
        usage += entries * (*array).elem_size as u64 * num_possible_cpus() as u64;
    } else if ((*map).map_flags & BPF_F_MMAPABLE) != 0 {
        usage = PAGE_ALIGN(usage) + PAGE_ALIGN(entries * (*array).elem_size as u64);
    } else { usage += entries * (*array).elem_size as u64; }
    usage
}

// The remaining map-operation callbacks retain the kernel ABI and are supplied
// by the surrounding BPF translation unit.  Their declarations are external.
extern "C" {
    static array_map_ops: bpf_map_ops;
    static percpu_array_map_ops: bpf_map_ops;
    static prog_array_map_ops: bpf_map_ops;
    static perf_event_array_map_ops: bpf_map_ops;
    static array_of_maps_map_ops: bpf_map_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
