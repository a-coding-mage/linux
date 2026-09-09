// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Isovalent */

#[repr(C)]
pub struct bpf_insn_array {
    pub map: bpf_map,
    pub used: atomic_t,
    pub ips: *mut c_long,
    pub values: [bpf_insn_array_value; 0],
}

pub const INSN_DELETED: u32 = u32::MAX;

#[inline]
unsafe fn insn_array_alloc_size(max_entries: u32) -> u64 {
    (core::mem::size_of::<bpf_insn_array>() as u64)
        .wrapping_add((max_entries as u64).wrapping_mul(
            (core::mem::size_of::<bpf_insn_array_value>() + core::mem::size_of::<c_long>()) as u64,
        ))
}

#[inline]
unsafe fn cast_insn_array<'a>(map: *mut bpf_map) -> *mut bpf_insn_array {
    (map as *mut u8).sub(core::mem::offset_of!(bpf_insn_array, map)) as *mut bpf_insn_array
}

unsafe fn insn_array_alloc_check(attr: *mut bpf_attr) -> c_int {
    let value_size = core::mem::size_of::<bpf_insn_array_value>() as u32;
    if (*attr).max_entries == 0 || (*attr).key_size != 4
        || (*attr).value_size != value_size || (*attr).map_flags != 0
    { return -EINVAL; }
    0
}

unsafe fn insn_array_free(map: *mut bpf_map) {
    bpf_map_area_free(cast_insn_array(map) as *mut c_void);
}

unsafe fn insn_array_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let size = insn_array_alloc_size((*attr).max_entries);
    let insn_array = bpf_map_area_alloc(size, NUMA_NO_NODE) as *mut bpf_insn_array;
    if insn_array.is_null() { return ERR_PTR(-ENOMEM) as *mut bpf_map; }
    (*insn_array).ips = (*insn_array).values.add((*attr).max_entries as usize) as *mut c_long;
    bpf_map_init_from_attr(&mut (*insn_array).map, attr);
    (*insn_array).map.map_flags |= BPF_F_RDONLY_PROG;
    &mut (*insn_array).map
}

unsafe fn insn_array_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
    let insn_array = cast_insn_array(map);
    let index = *(key as *mut u32);
    if index >= (*insn_array).map.max_entries { return core::ptr::null_mut(); }
    (*insn_array).values.add(index as usize) as *mut c_void
}

unsafe fn insn_array_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
    let insn_array = cast_insn_array(map);
    let index = *(key as *mut u32);
    let mut val: bpf_insn_array_value = core::mem::zeroed();
    if index >= (*insn_array).map.max_entries { return -E2BIG as c_long; }
    if map_flags & BPF_NOEXIST as u64 != 0 { return -EEXIST as c_long; }
    copy_map_value(map, &mut val as *mut _ as *mut c_void, value);
    if val.jitted_off != 0 || val.xlated_off != 0 { return -EINVAL as c_long; }
    (*insn_array).values[index as usize].orig_off = val.orig_off;
    0
}

unsafe fn insn_array_delete_elem(_map: *mut bpf_map, _key: *mut c_void) -> c_long { -EINVAL as c_long }

unsafe fn insn_array_check_btf(_map: *mut bpf_map, _btf: *const btf, key_type: *const btf_type, value_type: *const btf_type) -> c_int {
    if !btf_type_is_i32(key_type) || !btf_type_is_i64(value_type) { return -EINVAL; }
    0
}

unsafe fn insn_array_mem_usage(map: *const bpf_map) -> u64 { insn_array_alloc_size((*map).max_entries) }

unsafe fn insn_array_map_direct_value_addr(map: *const bpf_map, imm: *mut u64, off: u32) -> c_int {
    let insn_array = cast_insn_array(map as *mut bpf_map);
    if off % core::mem::size_of::<c_long>() as u32 != 0 || off / core::mem::size_of::<c_long>() as u32 >= (*map).max_entries { return -EACCES; }
    *imm = (*insn_array).ips as usize as u64;
    0
}

pub static mut insn_array_map_ops: bpf_map_ops = bpf_map_ops {
    map_alloc_check: Some(insn_array_alloc_check), map_alloc: Some(insn_array_alloc), map_free: Some(insn_array_free),
    map_get_next_key: Some(bpf_array_get_next_key), map_lookup_elem: Some(insn_array_lookup_elem),
    map_update_elem: Some(insn_array_update_elem), map_delete_elem: Some(insn_array_delete_elem),
    map_check_btf: Some(insn_array_check_btf), map_mem_usage: Some(insn_array_mem_usage),
    map_direct_value_addr: Some(insn_array_map_direct_value_addr), map_btf_id: core::ptr::null_mut(),
};

unsafe fn is_frozen(map: *mut bpf_map) -> bool { (*map).frozen }
unsafe fn is_insn_array(map: *const bpf_map) -> bool { (*map).map_type == BPF_MAP_TYPE_INSN_ARRAY }

unsafe fn valid_offsets(insn_array: *const bpf_insn_array, prog: *const bpf_prog) -> bool {
    for i in 0..(*insn_array).map.max_entries as usize {
        let off = (*insn_array).values[i].orig_off;
        if off >= (*prog).len { return false; }
        if off > 0 && (*prog).insnsi.add((off - 1) as usize).read().code == (BPF_LD | BPF_DW | BPF_IMM) { return false; }
    }
    true
}

pub unsafe fn bpf_insn_array_init(map: *mut bpf_map, prog: *const bpf_prog) -> c_int {
    let a = cast_insn_array(map);
    if !is_frozen(map) || !valid_offsets(a, prog) { return -EINVAL; }
    if atomic_xchg(&mut (*a).used, 1) != 0 { return -EBUSY; }
    for i in 0..(*map).max_entries as usize { (*a).values[i].xlated_off = (*a).values[i].orig_off; }
    0
}

pub unsafe fn bpf_insn_array_ready(map: *mut bpf_map) -> c_int {
    let a = cast_insn_array(map);
    for i in 0..(*map).max_entries as usize { if (*a).values[i].xlated_off != INSN_DELETED && (*a).ips.add(i).read() == 0 { return -EFAULT; } }
    0
}

pub unsafe fn bpf_insn_array_release(map: *mut bpf_map) { atomic_set(&mut (*cast_insn_array(map)).used, 0); }

pub unsafe fn bpf_insn_array_adjust(map: *mut bpf_map, off: u32, len: u32) {
    if len <= 1 { return; } let a = cast_insn_array(map);
    for i in 0..(*map).max_entries as usize { let v = &mut (*a).values[i]; if v.xlated_off > off && v.xlated_off != INSN_DELETED { v.xlated_off += len - 1; } }
}

pub unsafe fn bpf_insn_array_adjust_after_remove(map: *mut bpf_map, off: u32, len: u32) {
    let a = cast_insn_array(map);
    for i in 0..(*map).max_entries as usize { let v = &mut (*a).values[i]; if v.xlated_off >= off && v.xlated_off != INSN_DELETED { if v.xlated_off < off + len { v.xlated_off = INSN_DELETED; } else { v.xlated_off -= len; } } }
}

pub unsafe fn bpf_prog_update_insn_ptrs(prog: *mut bpf_prog, offsets: *mut u32, image: *mut c_void) {
    if offsets.is_null() || image.is_null() { return; }
    for i in 0..(*(*prog).aux).used_map_cnt as usize { let map = *(*prog).aux.used_maps.add(i); if !is_insn_array(map) { continue; } let a = cast_insn_array(map); for j in 0..(*map).max_entries as usize { let mut x = (*a).values[j].xlated_off; if x == INSN_DELETED || x < (*(*prog).aux).subprog_start { continue; } x -= (*(*prog).aux).subprog_start; if x >= (*prog).len { continue; } let o = *offsets.add(x as usize); (*a).values[j].jitted_off = o; *(*a).ips.add(j) = image.add(o as usize) as isize as c_long; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
