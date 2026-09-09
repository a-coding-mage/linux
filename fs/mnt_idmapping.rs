// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Christian Brauner <brauner@kernel.org> */

// Dependencies supplied by the surrounding kernel translation unit.

/*
 * Outside of this file vfs{g,u}id_t are always created from k{g,u}id_t,
 * never from raw values. These are just internal helpers.
 */
macro_rules! VFSUIDT_INIT_RAW { ($val:expr) => { vfsuid_t { val: $val } }; }
macro_rules! VFSGIDT_INIT_RAW { ($val:expr) => { vfsgid_t { val: $val } }; }

#[repr(C)]
pub struct mnt_idmap {
    pub uid_map: uid_gid_map,
    pub gid_map: uid_gid_map,
    pub count: refcount_t,
}

/* Carries the initial idmapping of 0:0:4294967295, which is an identity mapping. */
#[no_mangle]
pub static mut nop_mnt_idmap: mnt_idmap = mnt_idmap {
    uid_map: uid_gid_map::default(),
    gid_map: uid_gid_map::default(),
    count: REFCOUNT_INIT(1),
};

/* Carries the invalid idmapping of a full 0-4294967295 {g,u}id range. */
#[no_mangle]
pub static mut invalid_mnt_idmap: mnt_idmap = mnt_idmap {
    uid_map: uid_gid_map::default(),
    gid_map: uid_gid_map::default(),
    count: REFCOUNT_INIT(1),
};

#[inline]
unsafe fn initial_idmapping(ns: *const user_namespace) -> bool {
    ns == core::ptr::addr_of!(init_user_ns)
}

pub unsafe fn make_vfsuid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, kuid: kuid_t) -> vfsuid_t {
    let uid: uid_t;
    if idmap == core::ptr::addr_of_mut!(nop_mnt_idmap) { return VFSUIDT_INIT(kuid); }
    if idmap == core::ptr::addr_of_mut!(invalid_mnt_idmap) { return INVALID_VFSUID; }
    if initial_idmapping(fs_userns) { uid = __kuid_val(kuid); } else { uid = from_kuid(fs_userns, kuid); }
    if uid == (-1i32 as uid_t) { return INVALID_VFSUID; }
    VFSUIDT_INIT_RAW!(map_id_down(&(*idmap).uid_map, uid))
}

pub unsafe fn make_vfsgid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, kgid: kgid_t) -> vfsgid_t {
    let gid: gid_t;
    if idmap == core::ptr::addr_of_mut!(nop_mnt_idmap) { return VFSGIDT_INIT(kgid); }
    if idmap == core::ptr::addr_of_mut!(invalid_mnt_idmap) { return INVALID_VFSGID; }
    if initial_idmapping(fs_userns) { gid = __kgid_val(kgid); } else { gid = from_kgid(fs_userns, kgid); }
    if gid == (-1i32 as gid_t) { return INVALID_VFSGID; }
    VFSGIDT_INIT_RAW!(map_id_down(&(*idmap).gid_map, gid))
}

pub unsafe fn from_vfsuid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsuid: vfsuid_t) -> kuid_t {
    if idmap == core::ptr::addr_of_mut!(nop_mnt_idmap) { return AS_KUIDT(vfsuid); }
    if idmap == core::ptr::addr_of_mut!(invalid_mnt_idmap) { return INVALID_UID; }
    let uid = map_id_up(&(*idmap).uid_map, __vfsuid_val(vfsuid));
    if uid == (-1i32 as uid_t) { return INVALID_UID; }
    if initial_idmapping(fs_userns) { KUIDT_INIT(uid) } else { make_kuid(fs_userns, uid) }
}

pub unsafe fn from_vfsgid(idmap: *mut mnt_idmap, fs_userns: *mut user_namespace, vfsgid: vfsgid_t) -> kgid_t {
    if idmap == core::ptr::addr_of_mut!(nop_mnt_idmap) { return AS_KGIDT(vfsgid); }
    if idmap == core::ptr::addr_of_mut!(invalid_mnt_idmap) { return INVALID_GID; }
    let gid = map_id_up(&(*idmap).gid_map, __vfsgid_val(vfsgid));
    if gid == (-1i32 as gid_t) { return INVALID_GID; }
    if initial_idmapping(fs_userns) { KGIDT_INIT(gid) } else { make_kgid(fs_userns, gid) }
}

pub unsafe fn vfsgid_in_group_p(vfsgid: vfsgid_t) -> i32 {
    #[cfg(CONFIG_MULTIUSER)]
    { in_group_p(AS_KGIDT(vfsgid)) }
    #[cfg(not(CONFIG_MULTIUSER))]
    { 1 }
}

unsafe fn copy_mnt_idmap(map_from: *mut uid_gid_map, map_to: *mut uid_gid_map) -> i32 {
    let nr_extents = READ_ONCE((*map_from).nr_extents);
    smp_rmb();
    if nr_extents == 0 { return -EINVAL; }
    if nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS {
        *map_to = *map_from;
        return 0;
    }
    let forward = kmemdup_array((*map_from).forward, nr_extents, core::mem::size_of::<uid_gid_extent>(), GFP_KERNEL_ACCOUNT);
    if forward.is_null() { return -ENOMEM; }
    let reverse = kmemdup_array((*map_from).reverse, nr_extents, core::mem::size_of::<uid_gid_extent>(), GFP_KERNEL_ACCOUNT);
    if reverse.is_null() { kfree(forward); return -ENOMEM; }
    (*map_to).forward = forward;
    (*map_to).reverse = reverse;
    (*map_to).nr_extents = nr_extents;
    0
}

unsafe fn free_mnt_idmap(idmap: *mut mnt_idmap) {
    if (*idmap).uid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS { kfree((*idmap).uid_map.forward); kfree((*idmap).uid_map.reverse); }
    if (*idmap).gid_map.nr_extents > UID_GID_MAP_MAX_BASE_EXTENTS { kfree((*idmap).gid_map.forward); kfree((*idmap).gid_map.reverse); }
    kfree(idmap);
}

pub unsafe fn alloc_mnt_idmap(mnt_userns: *mut user_namespace) -> *mut mnt_idmap {
    let idmap = kzalloc_obj::<mnt_idmap>(GFP_KERNEL_ACCOUNT);
    if idmap.is_null() { return ERR_PTR(-ENOMEM); }
    refcount_set(&mut (*idmap).count, 1);
    let mut ret = copy_mnt_idmap(&mut (*mnt_userns).uid_map, &mut (*idmap).uid_map);
    if ret == 0 { ret = copy_mnt_idmap(&mut (*mnt_userns).gid_map, &mut (*idmap).gid_map); }
    if ret != 0 { free_mnt_idmap(idmap); return ERR_PTR(ret); }
    idmap
}

pub unsafe fn mnt_idmap_get(idmap: *mut mnt_idmap) -> *mut mnt_idmap {
    if idmap != core::ptr::addr_of_mut!(nop_mnt_idmap) && idmap != core::ptr::addr_of_mut!(invalid_mnt_idmap) { refcount_inc(&mut (*idmap).count); }
    idmap
}

pub unsafe fn mnt_idmap_put(idmap: *mut mnt_idmap) {
    if idmap != core::ptr::addr_of_mut!(nop_mnt_idmap) && idmap != core::ptr::addr_of_mut!(invalid_mnt_idmap) && refcount_dec_and_test(&mut (*idmap).count) { free_mnt_idmap(idmap); }
}

pub unsafe fn statmount_mnt_idmap(idmap: *mut mnt_idmap, seq: *mut seq_file, uid_map: bool) -> i32 {
    if !is_valid_mnt_idmap(idmap) { return 0; }
    let (map, map_up) = if uid_map { (&mut (*idmap).uid_map, &mut current_user_ns().uid_map) } else { (&mut (*idmap).gid_map, &mut current_user_ns().gid_map) };
    let mut nr_mappings = 0;
    for idx in 0..map.nr_extents {
        let extent = if map.nr_extents <= UID_GID_MAP_MAX_BASE_EXTENTS { &mut map.extent[idx] } else { &mut map.forward[idx] };
        let lower = map_id_range_up(map_up, extent.lower_first, extent.count);
        if lower == (-1i32 as uid_t) { continue; }
        seq_printf(seq, "%u %u %u", extent.first, lower, extent.count);
        if seq_has_overflowed(seq) { return -EAGAIN; }
        (*seq).count += 1;
        if seq_has_overflowed(seq) { return -EAGAIN; }
        nr_mappings += 1;
    }
    nr_mappings
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
