// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2024 Oracle.
 * All rights reserved.
 */
// Dependencies supplied by the surrounding XFS translation.

pub static mut xfs_parent_args_cache: *mut kmem_cache = core::ptr::null_mut();

/*
 * Parent pointer attribute handling.
 *
 * Because the attribute name is a filename component, it will never be longer
 * than 255 bytes and must not contain nulls or slashes.  These are roughly the
 * same constraints that apply to attribute names.
 *
 * The attribute value must always be a struct xfs_parent_rec.  This means the
 * attribute will never be in remote format because 12 bytes is nowhere near
 * xfs_attr_leaf_entsize_local_max() (~75% of block size).
 *
 * Creating a new parent attribute will always create a new attribute - there
 * should never, ever be an existing attribute in the tree for a new inode.
 * ENOSPC behavior is problematic - creating the inode without the parent
 * pointer is effectively a corruption, so we allow parent attribute creation
 * to dip into the reserve block pool to avoid unexpected ENOSPC errors from
 * occurring.
 */

/* Return true if parent pointer attr name is valid. */
pub unsafe fn xfs_parent_namecheck(attr_flags: u32, name: *const core::ffi::c_void, length: usize) -> bool {
    /* Parent pointers always use logged operations, so there should never
     * be incomplete xattrs. */
    if attr_flags & XFS_ATTR_INCOMPLETE != 0 { return false; }
    xfs_dir2_namecheck(name, length)
}

/* Return true if parent pointer attr value is valid. */
pub unsafe fn xfs_parent_valuecheck(mp: *mut xfs_mount, value: *const core::ffi::c_void, valuelen: usize) -> bool {
    let rec = value as *const xfs_parent_rec;
    if !xfs_has_parent(mp) { return false; }
    if valuelen != core::mem::size_of::<xfs_parent_rec>() { return false; }
    if value.is_null() { return false; }
    if !xfs_verify_dir_ino(mp, be64_to_cpu((*rec).p_ino)) { return false; }
    true
}

/* Compute the attribute name hash for a parent pointer. */
pub unsafe fn xfs_parent_hashval(mp: *mut xfs_mount, name: *const u8, namelen: i32, parent_ino: xfs_ino_t) -> xfs_dahash_t {
    let xname = xfs_name { name, len: namelen };
    xfs_dir2_hashname(mp, &xname) ^ upper_32_bits(parent_ino) ^ lower_32_bits(parent_ino)
}

/* Compute the attribute name hash from the xattr components. */
pub unsafe fn xfs_parent_hashattr(mp: *mut xfs_mount, name: *const u8, namelen: i32, value: *const core::ffi::c_void, valuelen: i32) -> xfs_dahash_t {
    let rec = value as *const xfs_parent_rec;
    if valuelen != core::mem::size_of::<xfs_parent_rec>() as i32 {
        ASSERT(valuelen == core::mem::size_of::<xfs_parent_rec>() as i32); return 0;
    }
    if value.is_null() { ASSERT(!value.is_null()); return 0; }
    xfs_parent_hashval(mp, name, namelen, be64_to_cpu((*rec).p_ino))
}

/* Initialize the parent pointer arguments structure. */
unsafe fn xfs_parent_da_args_init(args: *mut xfs_da_args, tp: *mut xfs_trans, rec: *mut xfs_parent_rec, child: *mut xfs_inode, owner: xfs_ino_t, parent_name: *const xfs_name) {
    (*args).geo = (*(*child).i_mount).m_attr_geo;
    (*args).whichfork = XFS_ATTR_FORK;
    (*args).attr_filter = XFS_ATTR_PARENT;
    (*args).op_flags = XFS_DA_OP_LOGGED | XFS_DA_OP_OKNOENT;
    (*args).trans = tp; (*args).dp = child; (*args).owner = owner;
    (*args).name = (*parent_name).name; (*args).namelen = (*parent_name).len;
    (*args).value = rec as *mut core::ffi::c_void;
    (*args).valuelen = core::mem::size_of::<xfs_parent_rec>() as i32;
    xfs_attr_sethash(args);
}

unsafe fn xfs_parent_iread_extents(tp: *mut xfs_trans, child: *mut xfs_inode) -> i32 {
    if XFS_IS_CORRUPT((*child).i_mount, !xfs_inode_has_attr_fork(child)) {
        xfs_inode_mark_sick(child, XFS_SICK_INO_PARENT); return -EFSCORRUPTED;
    }
    xfs_iread_extents(tp, child, XFS_ATTR_FORK)
}

pub unsafe fn xfs_parent_addname(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args, dp: *mut xfs_inode, parent_name: *const xfs_name, child: *mut xfs_inode) -> i32 {
    let error = xfs_parent_iread_extents(tp, child); if error != 0 { return error; }
    xfs_inode_to_parent_rec(&mut (*ppargs).rec, dp);
    xfs_parent_da_args_init(&mut (*ppargs).args, tp, &mut (*ppargs).rec, child, I_INO(child), parent_name);
    xfs_attr_setname(&mut (*ppargs).args, 0)
}

pub unsafe fn xfs_parent_removename(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args, dp: *mut xfs_inode, parent_name: *const xfs_name, child: *mut xfs_inode) -> i32 {
    let error = xfs_parent_iread_extents(tp, child); if error != 0 { return error; }
    xfs_inode_to_parent_rec(&mut (*ppargs).rec, dp);
    xfs_parent_da_args_init(&mut (*ppargs).args, tp, &mut (*ppargs).rec, child, I_INO(child), parent_name);
    xfs_attr_removename(&mut (*ppargs).args)
}

pub unsafe fn xfs_parent_replacename(tp: *mut xfs_trans, ppargs: *mut xfs_parent_args, old_dp: *mut xfs_inode, old_name: *const xfs_name, new_dp: *mut xfs_inode, new_name: *const xfs_name, child: *mut xfs_inode) -> i32 {
    let error = xfs_parent_iread_extents(tp, child); if error != 0 { return error; }
    xfs_inode_to_parent_rec(&mut (*ppargs).rec, old_dp);
    xfs_parent_da_args_init(&mut (*ppargs).args, tp, &mut (*ppargs).rec, child, I_INO(child), old_name);
    xfs_inode_to_parent_rec(&mut (*ppargs).new_rec, new_dp);
    (*ppargs).args.new_name = (*new_name).name; (*ppargs).args.new_namelen = (*new_name).len;
    (*ppargs).args.new_value = &mut (*ppargs).new_rec as *mut _ as *mut core::ffi::c_void;
    (*ppargs).args.new_valuelen = core::mem::size_of::<xfs_parent_rec>() as i32;
    xfs_attr_replacename(&mut (*ppargs).args, 0)
}

pub unsafe fn xfs_parent_from_attr(mp: *mut xfs_mount, attr_flags: u32, name: *const u8, namelen: u32, value: *const core::ffi::c_void, valuelen: u32, parent_ino: *mut xfs_ino_t, parent_gen: *mut u32) -> i32 {
    let rec = value as *const xfs_parent_rec;
    ASSERT(attr_flags & XFS_ATTR_PARENT != 0);
    if !xfs_parent_namecheck(attr_flags, name as *const _, namelen as usize) || !xfs_parent_valuecheck(mp, value, valuelen as usize) { return -EFSCORRUPTED; }
    if !parent_ino.is_null() { *parent_ino = be64_to_cpu((*rec).p_ino); }
    if !parent_gen.is_null() { *parent_gen = be32_to_cpu((*rec).p_gen); }
    0
}

pub unsafe fn xfs_parent_lookup(tp: *mut xfs_trans, ip: *mut xfs_inode, parent_name: *const xfs_name, pptr: *mut xfs_parent_rec, scratch: *mut xfs_da_args) -> i32 {
    core::ptr::write_bytes(scratch, 0, 1);
    xfs_parent_da_args_init(scratch, tp, pptr, ip, I_INO(ip), parent_name);
    xfs_attr_get_ilocked(scratch)
}

unsafe fn xfs_parent_sanity_check(mp: *mut xfs_mount, parent_name: *const xfs_name, pptr: *const xfs_parent_rec) -> bool {
    xfs_parent_namecheck(XFS_ATTR_PARENT, (*parent_name).name as *const _, (*parent_name).len as usize) && xfs_parent_valuecheck(mp, pptr as *const _, core::mem::size_of::<xfs_parent_rec>())
}

pub unsafe fn xfs_parent_set(ip: *mut xfs_inode, owner: xfs_ino_t, parent_name: *const xfs_name, pptr: *mut xfs_parent_rec, scratch: *mut xfs_da_args) -> i32 {
    if !xfs_parent_sanity_check((*ip).i_mount, parent_name, pptr) { ASSERT(0); return -EFSCORRUPTED; }
    core::ptr::write_bytes(scratch, 0, 1);
    xfs_parent_da_args_init(scratch, core::ptr::null_mut(), pptr, ip, owner, parent_name);
    xfs_attr_set(scratch, XFS_ATTRUPDATE_CREATE, false)
}

pub unsafe fn xfs_parent_unset(ip: *mut xfs_inode, owner: xfs_ino_t, parent_name: *const xfs_name, pptr: *mut xfs_parent_rec, scratch: *mut xfs_da_args) -> i32 {
    if !xfs_parent_sanity_check((*ip).i_mount, parent_name, pptr) { ASSERT(0); return -EFSCORRUPTED; }
    core::ptr::write_bytes(scratch, 0, 1);
    xfs_parent_da_args_init(scratch, core::ptr::null_mut(), pptr, ip, owner, parent_name);
    xfs_attr_set(scratch, XFS_ATTRUPDATE_REMOVE, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
