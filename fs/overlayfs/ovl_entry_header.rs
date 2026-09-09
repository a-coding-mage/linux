/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2011 Novell Inc.
 * Copyright (C) 2016 Red Hat, Inc.
 */

#[repr(C)]
pub struct ovl_config {
    pub upperdir: *mut core::ffi::c_char,
    pub workdir: *mut core::ffi::c_char,
    pub lowerdirs: *mut *mut core::ffi::c_char,
    pub default_permissions: bool,
    pub redirect_mode: core::ffi::c_int,
    pub verity_mode: core::ffi::c_int,
    pub index: bool,
    pub uuid: core::ffi::c_int,
    pub nfs_export: bool,
    pub xino: core::ffi::c_int,
    pub metacopy: bool,
    pub userxattr: bool,
    pub fsync_mode: core::ffi::c_int,
}

#[repr(C)]
pub struct ovl_sb {
    pub sb: *mut super_block,
    pub pseudo_dev: dev_t,
    /* Unusable (conflicting) uuid */
    pub bad_uuid: bool,
    /* Used as a lower layer (but maybe also as upper) */
    pub is_lower: bool,
}

#[repr(C)]
pub struct ovl_layer {
    /* ovl_free_fs() relies on @mnt being the first member! */
    pub mnt: *mut vfsmount,
    /* Trap in ovl inode cache */
    pub trap: *mut inode,
    pub fs: *mut ovl_sb,
    /* Index of this layer in fs root (upper idx == 0) */
    pub idx: core::ffi::c_int,
    /* One fsid per unique underlying sb (upper fsid == 0) */
    pub fsid: core::ffi::c_int,
    /* xwhiteouts were found on this layer */
    pub has_xwhiteouts: bool,
}

#[repr(C)]
pub struct ovl_path {
    pub layer: *const ovl_layer,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct ovl_entry {
    pub __numlower: core::ffi::c_uint,
    pub __lowerstack: [ovl_path; 0],
}

/* private information held for overlayfs's superblock */
#[repr(C)]
pub struct ovl_fs {
    pub numlayer: core::ffi::c_uint,
    /* Number of unique fs among layers including upper fs */
    pub numfs: core::ffi::c_uint,
    /* Number of data-only lower layers */
    pub numdatalayer: core::ffi::c_uint,
    pub layers: *mut ovl_layer,
    pub fs: *mut ovl_sb,
    /* workbasedir is the path at workdir= mount option */
    pub workbasedir: *mut dentry,
    /* workdir is the 'work' or 'index' directory under workbasedir */
    pub workdir: *mut dentry,
    pub namelen: core::ffi::c_long,
    /* pathnames of lower and upper dirs, for show_options */
    pub config: ovl_config,
    /* creds of process who forced instantiation of super block */
    pub creator_cred: *const cred,
    pub tmpfile: bool,
    pub noxattr: bool,
    pub nofh: bool,
    /* Did we take the inuse lock? */
    pub upperdir_locked: bool,
    pub workdir_locked: bool,
    /* Traps in ovl inode cache */
    pub workbasedir_trap: *mut inode,
    pub workdir_trap: *mut inode,
    /* -1: disabled, 0: same fs, 1..32: number of unused ino bits */
    pub xino_mode: core::ffi::c_int,
    /* For allocation of non-persistent inode numbers */
    pub last_ino: atomic_long_t,
    /* Shared whiteout cache */
    pub whiteout: *mut dentry,
    pub no_shared_whiteout: bool,
    pub whiteout_lock: mutex,
    /* r/o snapshot of upperdir sb's only taken on volatile mounts */
    pub errseq: errseq_t,
    pub casefold: bool,
}

/* Number of lower layers, not including data-only layers */
#[inline]
pub unsafe fn ovl_numlowerlayer(ofs: *mut ovl_fs) -> core::ffi::c_uint {
    (*ofs).numlayer - (*ofs).numdatalayer - 1
}

#[inline]
pub unsafe fn ovl_upper_mnt(ofs: *mut ovl_fs) -> *mut vfsmount {
    (*ofs).layers[0].mnt
}

#[inline]
pub unsafe fn ovl_upper_mnt_idmap(ofs: *mut ovl_fs) -> *mut mnt_idmap {
    mnt_idmap(ovl_upper_mnt(ofs))
}

extern "C" {
    pub static mut ovl_fs_type: file_system_type;
}

#[inline]
pub unsafe fn OVL_FS(sb: *mut super_block) -> *mut ovl_fs {
    if IS_ENABLED(CONFIG_OVERLAY_FS_DEBUG) {
        WARN_ON_ONCE((*sb).s_type != &mut ovl_fs_type);
    }
    (*sb).s_fs_info as *mut ovl_fs
}

#[inline]
pub unsafe fn ovl_numlower(oe: *mut ovl_entry) -> core::ffi::c_uint {
    if !oe.is_null() { (*oe).__numlower } else { 0 }
}

#[inline]
pub unsafe fn ovl_lowerstack(oe: *mut ovl_entry) -> *mut ovl_path {
    if ovl_numlower(oe) != 0 { (*oe).__lowerstack.as_mut_ptr() } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn ovl_lowerpath(oe: *mut ovl_entry) -> *mut ovl_path {
    ovl_lowerstack(oe)
}

#[inline]
pub unsafe fn ovl_lowerdata(oe: *mut ovl_entry) -> *mut ovl_path {
    let lowerstack = ovl_lowerstack(oe);
    if !lowerstack.is_null() { lowerstack.add((*oe).__numlower as usize - 1) } else { core::ptr::null_mut() }
}

/* May return NULL if lazy lookup of lowerdata is needed */
#[inline]
pub unsafe fn ovl_lowerdata_dentry(oe: *mut ovl_entry) -> *mut dentry {
    let lowerdata = ovl_lowerdata(oe);
    if !lowerdata.is_null() { core::ptr::read_volatile(&(*lowerdata).dentry) } else { core::ptr::null_mut() }
}

/* private information held for every overlayfs dentry */
#[inline]
pub unsafe fn OVL_E_FLAGS(dentry: *mut dentry) -> *mut core::ffi::c_ulong {
    &mut (*dentry).d_fsdata as *mut _
}

#[repr(C)]
pub union ovl_inode_union {
    pub cache: *mut ovl_dir_cache,
    pub lowerdata_redirect: *const core::ffi::c_char,
}

#[repr(C)]
pub struct ovl_inode {
    pub anon: ovl_inode_union,
    pub redirect: *const core::ffi::c_char,
    pub version: u64,
    pub flags: core::ffi::c_ulong,
    pub vfs_inode: inode,
    pub __upperdentry: *mut dentry,
    pub oe: *mut ovl_entry,
    /* synchronize copy up and more */
    pub lock: mutex,
}

#[inline]
pub unsafe fn OVL_I(inode: *mut inode) -> *mut ovl_inode {
    (inode as *mut u8).sub(core::mem::offset_of!(ovl_inode, vfs_inode)) as *mut ovl_inode
}

#[inline]
pub unsafe fn OVL_I_E(inode: *mut inode) -> *mut ovl_entry {
    if !inode.is_null() { (*OVL_I(inode)).oe } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn OVL_E(dentry: *mut dentry) -> *mut ovl_entry {
    OVL_I_E(d_inode(dentry))
}

#[inline]
pub unsafe fn ovl_upperdentry_dereference(oi: *mut ovl_inode) -> *mut dentry {
    core::ptr::read_volatile(&(*oi).__upperdentry)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
