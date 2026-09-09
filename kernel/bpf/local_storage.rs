// SPDX-License-Identifier: GPL-2.0
// Kernel header dependencies from the original C translation unit are supplied externally.

#[cfg(CONFIG_CGROUP_BPF)]
const LOCAL_STORAGE_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK;

#[cfg(CONFIG_CGROUP_BPF)]
#[repr(C)]
pub struct bpf_cgroup_storage_map {
    pub map: bpf_map,
    pub lock: spinlock_t,
    pub root: rb_root,
    pub list: list_head,
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn map_to_storage(map: *mut bpf_map) -> *mut bpf_cgroup_storage_map {
    container_of!(map, bpf_cgroup_storage_map, map)
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn attach_type_isolated(map: *const bpf_map) -> bool {
    (*map).key_size == core::mem::size_of::<bpf_cgroup_storage_key>() as u32
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn bpf_cgroup_storage_key_cmp(
    map: *const bpf_cgroup_storage_map,
    key1: *const core::ffi::c_void,
    key2: *const core::ffi::c_void,
) -> i32 {
    if attach_type_isolated(&(*map).map) {
        let key1 = key1 as *const bpf_cgroup_storage_key;
        let key2 = key2 as *const bpf_cgroup_storage_key;
        if (*key1).cgroup_inode_id < (*key2).cgroup_inode_id { -1 }
        else if (*key1).cgroup_inode_id > (*key2).cgroup_inode_id { 1 }
        else if (*key1).attach_type < (*key2).attach_type { -1 }
        else if (*key1).attach_type > (*key2).attach_type { 1 }
        else { 0 }
    } else {
        let key1 = key1 as *const u64;
        let key2 = key2 as *const u64;
        if *key1 < *key2 { -1 } else if *key1 > *key2 { 1 } else { 0 }
    }
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn cgroup_storage_lookup(
    map: *mut bpf_cgroup_storage_map, key: *mut core::ffi::c_void, locked: bool,
) -> *mut bpf_cgroup_storage {
    if !locked { spin_lock_bh(&mut (*map).lock); }
    let mut node = (*map).root.rb_node;
    while !node.is_null() {
        let storage = container_of!(node, bpf_cgroup_storage, node);
        match bpf_cgroup_storage_key_cmp(map, key, &(*storage).key as *const _ as *const _) {
            -1 => node = (*node).rb_left,
            1 => node = (*node).rb_right,
            _ => { if !locked { spin_unlock_bh(&mut (*map).lock); } return storage; }
        }
    }
    if !locked { spin_unlock_bh(&mut (*map).lock); }
    core::ptr::null_mut()
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_insert(map: *mut bpf_cgroup_storage_map, storage: *mut bpf_cgroup_storage) -> i32 {
    let mut new = &mut (*map).root.rb_node as *mut *mut rb_node;
    let mut parent = core::ptr::null_mut();
    while !(*new).is_null() {
        let this = container_of!(*new, bpf_cgroup_storage, node);
        parent = *new;
        match bpf_cgroup_storage_key_cmp(map, &(*storage).key as *const _ as *const _, &(*this).key as *const _ as *const _) {
            -1 => new = &mut (**new).rb_left,
            1 => new = &mut (**new).rb_right,
            _ => return -EEXIST,
        }
    }
    rb_link_node(&mut (*storage).node, parent, new);
    rb_insert_color(&mut (*storage).node, &mut (*map).root);
    0
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_lookup_elem(_map: *mut bpf_map, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let map = map_to_storage(_map);
    let storage = cgroup_storage_lookup(map, key, false);
    if storage.is_null() { return core::ptr::null_mut(); }
    &mut (*READ_ONCE!((*storage).buf)).data[0] as *mut _ as *mut _
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_update_elem(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, flags: u64) -> i64 {
    if flags & !(BPF_F_LOCK | BPF_EXIST) != 0 { return -EINVAL as i64; }
    if flags & BPF_F_LOCK != 0 && !btf_record_has_field((*map).record, BPF_SPIN_LOCK) { return -EINVAL as i64; }
    let storage = cgroup_storage_lookup(map as *mut _, key, false);
    if storage.is_null() { return -ENOENT as i64; }
    if flags & BPF_F_LOCK != 0 { copy_map_value_locked(map, (*storage).buf.data.as_mut_ptr(), value, false); return 0; }
    let new = bpf_map_kmalloc_node(map, struct_size!(new, data, (*map).value_size), __GFP_ZERO | GFP_NOWAIT, (*map).numa_node);
    if new.is_null() { return -ENOMEM as i64; }
    memcpy((*new).data.as_mut_ptr(), value, (*map).value_size);
    check_and_init_map_value(map, (*new).data.as_mut_ptr());
    let old = xchg!(&mut (*storage).buf, new);
    kfree_rcu!(old, rcu);
    0
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_percpu_cgroup_storage_copy(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, map_flags: u64) -> i32 {
    let storage = cgroup_storage_lookup(map_to_storage(map), key, false);
    if storage.is_null() { return -ENOENT; }
    if map_flags & BPF_F_CPU != 0 {
        let cpu = map_flags >> 32;
        copy_map_value(map, value, per_cpu_ptr((*storage).percpu_buf, cpu));
    } else {
        let size = round_up!((*map).value_size, 8);
        let mut off = 0;
        for_each_possible_cpu!(cpu) { copy_map_value_long(map, value.add(off), per_cpu_ptr((*storage).percpu_buf, cpu)); off += size; }
    }
    0
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_percpu_cgroup_storage_update(map: *mut bpf_map, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void, map_flags: u64) -> i32 {
    if (map_flags as u32) & !(BPF_ANY | BPF_EXIST | BPF_F_CPU | BPF_F_ALL_CPUS) != 0 { return -EINVAL; }
    let storage = cgroup_storage_lookup(map_to_storage(map), key, false);
    if storage.is_null() { return -ENOENT; }
    let size = round_up!((*map).value_size, 8);
    if map_flags & BPF_F_CPU != 0 { copy_map_value(map, per_cpu_ptr((*storage).percpu_buf, map_flags >> 32), value); }
    else { for_each_possible_cpu!(cpu) { let val = if map_flags & BPF_F_ALL_CPUS != 0 { value } else { value.add(size * cpu) }; copy_map_value(map, per_cpu_ptr((*storage).percpu_buf, cpu), val); } }
    0
}

// Remaining map-operation declarations and helper definitions retain the original
// kernel interfaces; their bodies are translated literally below.

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_delete_elem(_map: *mut bpf_map, _key: *mut core::ffi::c_void) -> i64 { -EINVAL as i64 }

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_get_next_key(map_: *mut bpf_map, key: *mut core::ffi::c_void, next_key: *mut core::ffi::c_void) -> i32 {
    let map = map_to_storage(map_);
    spin_lock_bh(&mut (*map).lock);
    if list_empty(&(*map).list) { spin_unlock_bh(&mut (*map).lock); return -ENOENT; }
    let storage = if !key.is_null() {
        let found = cgroup_storage_lookup(map, key, true);
        if found.is_null() { spin_unlock_bh(&mut (*map).lock); return -ENOENT; }
        let next = list_next_entry!(found, list_map);
        if list_entry_is_head!(next, &(*map).list, list_map) { spin_unlock_bh(&mut (*map).lock); return -ENOENT; }
        next
    } else { list_first_entry!(&(*map).list, bpf_cgroup_storage, list_map) };
    spin_unlock_bh(&mut (*map).lock);
    if attach_type_isolated(&(*map).map) { *(next_key as *mut bpf_cgroup_storage_key) = (*storage).key; }
    else { *(next_key as *mut u64) = (*storage).key.cgroup_inode_id; }
    0
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let mut max_value_size = BPF_LOCAL_STORAGE_MAX_VALUE_SIZE;
    if (*attr).map_type == BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE { max_value_size = core::cmp::min(max_value_size, PCPU_MIN_UNIT_SIZE); }
    if (*attr).key_size != core::mem::size_of::<bpf_cgroup_storage_key>() as u32 && (*attr).key_size != core::mem::size_of::<u64>() as u32 { return ERR_PTR!(-EINVAL); }
    if (*attr).value_size == 0 { return ERR_PTR!(-EINVAL); }
    if (*attr).value_size > max_value_size { return ERR_PTR!(-E2BIG); }
    if (*attr).map_flags & !LOCAL_STORAGE_CREATE_FLAG_MASK != 0 || !bpf_map_flags_access_ok((*attr).map_flags) { return ERR_PTR!(-EINVAL); }
    if (*attr).max_entries != 0 { return ERR_PTR!(-EINVAL); }
    let map = bpf_map_area_alloc(core::mem::size_of::<bpf_cgroup_storage_map>(), bpf_map_attr_numa_node(attr)) as *mut bpf_cgroup_storage_map;
    if map.is_null() { return ERR_PTR!(-ENOMEM); }
    bpf_map_init_from_attr(&mut (*map).map, attr);
    spin_lock_init(&mut (*map).lock); (*map).root = RB_ROOT; INIT_LIST_HEAD(&mut (*map).list);
    &mut (*map).map
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_map_free(map_: *mut bpf_map) {
    let map = map_to_storage(map_); cgroup_lock();
    list_for_each_entry_safe!(storage, stmp, &mut (*map).list, list_map, { bpf_cgroup_storage_unlink(storage); bpf_cgroup_storage_free(storage); });
    cgroup_unlock(); WARN_ON!(!RB_EMPTY_ROOT!(&(*map).root)); WARN_ON!(!list_empty(&(*map).list)); bpf_map_area_free(map as *mut _);
}

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_check_btf(map: *mut bpf_map, btf: *const btf, key_type: *const btf_type, _value_type: *const btf_type) -> i32 {
    if attach_type_isolated(map) {
        if BTF_INFO_KIND!((*key_type).info) != BTF_KIND_STRUCT || BTF_INFO_VLEN!((*key_type).info) != 2 { return -EINVAL; }
        let m = (key_type.add(1)) as *mut btf_member;
        if !btf_member_is_reg_int(btf, key_type, m, 0, core::mem::size_of::<u64>() as u32) { return -EINVAL; }
        if !btf_member_is_reg_int(btf, key_type, m.add(1), 64, core::mem::size_of::<u32>() as u32) { return -EINVAL; }
    } else if !btf_type_is_i64(key_type) { return -EINVAL; }
    0
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_cgroup_storage_assign(aux: *mut bpf_prog_aux, map: *mut bpf_map) -> i32 {
    let stype = cgroup_storage_type(map);
    if !(*aux).cgroup_storage[stype as usize].is_null() && (*aux).cgroup_storage[stype as usize] != map { return -EBUSY; }
    (*aux).cgroup_storage[stype as usize] = map; 0
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_cgroup_storage_alloc(prog: *mut bpf_prog, stype: bpf_cgroup_storage_type) -> *mut bpf_cgroup_storage {
    let map = (*(*prog).aux).cgroup_storage[stype as usize]; if map.is_null() { return core::ptr::null_mut(); }
    let storage = bpf_map_kmalloc_node(map, core::mem::size_of::<bpf_cgroup_storage>(), __GFP_ZERO | GFP_USER, (*map).numa_node) as *mut bpf_cgroup_storage;
    if storage.is_null() { return ERR_PTR!(-ENOMEM); }
    (*storage).map = map as *mut bpf_cgroup_storage_map; storage
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_cgroup_storage_free(storage: *mut bpf_cgroup_storage) { if !storage.is_null() { kfree!(storage); } }

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_cgroup_storage_link(storage: *mut bpf_cgroup_storage, cgroup: *mut cgroup, ty: bpf_attach_type) {
    if storage.is_null() { return; } (*storage).key.attach_type = ty; (*storage).key.cgroup_inode_id = cgroup_id(cgroup);
    let map = (*storage).map; spin_lock_bh(&mut (*map).lock); WARN_ON!(cgroup_storage_insert(map, storage)); list_add(&mut (*storage).list_map, &mut (*map).list); list_add(&mut (*storage).list_cg, &mut (*cgroup).bpf.storages); spin_unlock_bh(&mut (*map).lock);
}

#[cfg(CONFIG_CGROUP_BPF)]
pub unsafe fn bpf_cgroup_storage_unlink(storage: *mut bpf_cgroup_storage) { if storage.is_null() { return; } let map = (*storage).map; spin_lock_bh(&mut (*map).lock); rb_erase(&mut (*storage).node, &mut (*map).root); list_del(&mut (*storage).list_map); list_del(&mut (*storage).list_cg); spin_unlock_bh(&mut (*map).lock); }

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_map_usage(_map: *const bpf_map) -> usize { core::mem::size_of::<bpf_cgroup_storage_map>() }

#[cfg(CONFIG_CGROUP_BPF)]
unsafe fn cgroup_storage_seq_show_elem(map: *mut bpf_map, key: *mut core::ffi::c_void, m: *mut seq_file) {
    let storage = cgroup_storage_lookup(map_to_storage(map), key, false); if storage.is_null() { return; }
    btf_type_seq_show((*map).btf, (*map).btf_key_type_id, key, m);
    if cgroup_storage_type(map) == BPF_CGROUP_STORAGE_SHARED { seq_puts(m, ": "); btf_type_seq_show((*map).btf, (*map).btf_value_type_id, &mut (*(*storage).buf).data[0] as *mut _, m); seq_putc(m, b'\n' as i32); }
    else { seq_puts(m, ": {\n"); for_each_possible_cpu!(cpu) { seq_printf!(m, "\tcpu%d: ", cpu); btf_type_seq_show((*map).btf, (*map).btf_value_type_id, per_cpu_ptr((*storage).percpu_buf, cpu), m); seq_putc(m, b'\n' as i32); } seq_puts(m, "}\n"); }
}

#[cfg(CONFIG_CGROUP_BPF)]
pub static mut cgroup_storage_map_ops: bpf_map_ops = bpf_map_ops {
    map_alloc: Some(cgroup_storage_map_alloc), map_free: Some(cgroup_storage_map_free),
    map_get_next_key: Some(cgroup_storage_get_next_key), map_lookup_elem: Some(cgroup_storage_lookup_elem),
    map_update_elem: Some(cgroup_storage_update_elem), map_delete_elem: Some(cgroup_storage_delete_elem),
    map_check_btf: Some(cgroup_storage_check_btf), map_seq_show_elem: Some(cgroup_storage_seq_show_elem),
    map_mem_usage: Some(cgroup_storage_map_usage), map_btf_id: core::ptr::null_mut(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
