/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of inode.h. */

#[repr(C)]
pub enum ntfs_inode_mutex_lock_class {
    NTFS_INODE_MUTEX_PARENT,
    NTFS_INODE_MUTEX_NORMAL,
    NTFS_INODE_MUTEX_NORMAL_CHILD,
    NTFS_INODE_MUTEX_PARENT_2,
    NTFS_INODE_MUTEX_NORMAL_2,
    NTFS_EXTEND_MUTEX_PARENT,
    NTFS_EA_MUTEX_NORMAL,
}

#[repr(C)]
pub struct ntfs_inode {
    pub size_lock: rwlock_t,
    pub state: ::core::ffi::c_ulong,
    pub flags: __le32,
    pub mft_no: u64,
    pub seq_no: u16,
    pub count: atomic_t,
    pub vol: *mut ntfs_volume,
    pub r#type: __le32,
    pub name: *mut __le16,
    pub name_len: u32,
    pub runlist: runlist,
    pub data_size: i64,
    pub initialized_size: i64,
    pub allocated_size: i64,
    pub i_crtime: timespec64,
    pub mrec: *mut ::core::ffi::c_void,
    pub mrec_lock: mutex,
    pub folio: *mut folio,
    pub folio_ofs: i32,
    pub mft_lcn: [i64; 2],
    pub mft_lcn_count: ::core::ffi::c_uint,
    pub attr_list_size: u32,
    pub attr_list: *mut u8,
    pub itype: ntfs_inode_itype,
    pub extent_lock: mutex,
    pub nr_extents: i32,
    pub ext: ntfs_inode_ext,
    pub i_dealloc_clusters: ::core::ffi::c_uint,
    pub reparse_tag: __le32,
    pub reparse_flags: __le32,
    pub target: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub union ntfs_inode_itype {
    pub index: ntfs_inode_index,
    pub compressed: ntfs_inode_compressed,
}

#[repr(C)]
pub struct ntfs_inode_index {
    pub block_size: u32,
    pub vcn_size: u32,
    pub collation_rule: __le32,
    pub block_size_bits: u8,
    pub vcn_size_bits: u8,
}

#[repr(C)]
pub struct ntfs_inode_compressed {
    pub size: i64,
    pub block_size: u32,
    pub block_size_bits: u8,
    pub block_clusters: u8,
}

#[repr(C)]
pub union ntfs_inode_ext {
    pub extent_ntfs_inos: *mut *mut ntfs_inode,
    pub base_ntfs_ino: *mut ntfs_inode,
}

#[repr(u32)]
pub enum ntfs_inode_state {
    NI_Dirty,
    NI_AttrListDirty,
    NI_AttrList,
    NI_AttrListNonResident,
    NI_Attr,
    NI_MstProtected,
    NI_NonResident,
    NI_IndexAllocPresent,
    NI_Compressed,
    NI_WofCompressed,
    NI_Encrypted,
    NI_Sparse,
    NI_SparseDisabled,
    NI_FullyMapped,
    NI_FileNameDirty,
    NI_BeingDeleted,
    NI_BeingCreated,
    NI_HasEA,
    NI_RunlistDirty,
}

#[inline]
pub unsafe fn NInoDirty(ni: *mut ntfs_inode) -> i32 { test_bit(NI_Dirty as ::core::ffi::c_ulong, &(*ni).state) }
#[inline]
pub unsafe fn NInoSetDirty(ni: *mut ntfs_inode) { set_bit(NI_Dirty as ::core::ffi::c_ulong, &mut (*ni).state); }
#[inline]
pub unsafe fn NInoClearDirty(ni: *mut ntfs_inode) { clear_bit(NI_Dirty as ::core::ffi::c_ulong, &mut (*ni).state); }
#[inline]
pub unsafe fn NInoTestSetDirty(ni: *mut ntfs_inode) -> i32 { test_and_set_bit(NI_Dirty as ::core::ffi::c_ulong, &mut (*ni).state) }
#[inline]
pub unsafe fn NInoTestClearDirty(ni: *mut ntfs_inode) -> i32 { test_and_clear_bit(NI_Dirty as ::core::ffi::c_ulong, &mut (*ni).state) }

macro_rules! nino_fns {
    ($get:ident, $set:ident, $clear:ident, $bit:ident) => {
        #[inline] pub unsafe fn $get(ni: *mut ntfs_inode) -> i32 { test_bit(ntfs_inode_state::$bit as ::core::ffi::c_ulong, &(*ni).state) }
        #[inline] pub unsafe fn $set(ni: *mut ntfs_inode) { set_bit(ntfs_inode_state::$bit as _, &mut (*ni).state); }
        #[inline] pub unsafe fn $clear(ni: *mut ntfs_inode) { clear_bit(ntfs_inode_state::$bit as _, &mut (*ni).state); }
    };
}

nino_fns!(NInoAttrList, NInoSetAttrList, NInoClearAttrList, NI_AttrList);
nino_fns!(NInoAttrListDirty, NInoSetAttrListDirty, NInoClearAttrListDirty, NI_AttrListDirty);
nino_fns!(NInoAttrListNonResident, NInoSetAttrListNonResident, NInoClearAttrListNonResident, NI_AttrListNonResident);
nino_fns!(NInoAttr, NInoSetAttr, NInoClearAttr, NI_Attr);
nino_fns!(NInoMstProtected, NInoSetMstProtected, NInoClearMstProtected, NI_MstProtected);
nino_fns!(NInoNonResident, NInoSetNonResident, NInoClearNonResident, NI_NonResident);
nino_fns!(NInoIndexAllocPresent, NInoSetIndexAllocPresent, NInoClearIndexAllocPresent, NI_IndexAllocPresent);
nino_fns!(NInoCompressed, NInoSetCompressed, NInoClearCompressed, NI_Compressed);
nino_fns!(NInoWofCompressed, NInoSetWofCompressed, NInoClearWofCompressed, NI_WofCompressed);
nino_fns!(NInoEncrypted, NInoSetEncrypted, NInoClearEncrypted, NI_Encrypted);
nino_fns!(NInoSparse, NInoSetSparse, NInoClearSparse, NI_Sparse);
nino_fns!(NInoSparseDisabled, NInoSetSparseDisabled, NInoClearSparseDisabled, NI_SparseDisabled);
nino_fns!(NInoFullyMapped, NInoSetFullyMapped, NInoClearFullyMapped, NI_FullyMapped);
nino_fns!(NInoFileNameDirty, NInoSetFileNameDirty, NInoClearFileNameDirty, NI_FileNameDirty);
nino_fns!(NInoBeingDeleted, NInoSetBeingDeleted, NInoClearBeingDeleted, NI_BeingDeleted);
nino_fns!(NInoHasEA, NInoSetHasEA, NInoClearHasEA, NI_HasEA);
nino_fns!(NInoRunlistDirty, NInoSetRunlistDirty, NInoClearRunlistDirty, NI_RunlistDirty);

#[inline]
pub unsafe fn NInoTestSetFileNameDirty(ni: *mut ntfs_inode) -> i32 { test_and_set_bit(NI_FileNameDirty as _, &mut (*ni).state) }
#[inline]
pub unsafe fn NInoTestClearFileNameDirty(ni: *mut ntfs_inode) -> i32 { test_and_clear_bit(NI_FileNameDirty as _, &mut (*ni).state) }

#[repr(C)]
pub struct big_ntfs_inode { pub ntfs_inode: ntfs_inode, pub vfs_inode: inode }

#[inline]
pub unsafe fn NTFS_I(inode: *mut inode) -> *mut ntfs_inode {
    ::core::ptr::addr_of_mut!((*((inode as *mut u8).sub(offset_of!(big_ntfs_inode, vfs_inode)) as *mut big_ntfs_inode)).ntfs_inode)
}
#[inline]
pub unsafe fn VFS_I(ni: *mut ntfs_inode) -> *mut inode {
    ::core::ptr::addr_of_mut!((*((ni as *mut u8).sub(offset_of!(big_ntfs_inode, ntfs_inode)) as *mut big_ntfs_inode)).vfs_inode)
}

#[inline]
pub unsafe fn ntfs_init_big_inode(vi: *mut inode) {
    let ni = NTFS_I(vi);
    ntfs_debug(c"Entering.");
    __ntfs_init_inode((*vi).i_sb, ni);
    (*ni).mft_no = (*vi).i_ino;
}

#[inline]
pub unsafe fn ntfs_commit_inode(vi: *mut inode) { __ntfs_write_inode(vi, 1); }

#[repr(C)]
pub struct ntfs_attr {
    pub mft_no: u64,
    pub name: *mut __le16,
    pub name_len: u32,
    pub r#type: __le32,
    pub state: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn ntfs_test_inode(vi: *mut inode, data: *mut ::core::ffi::c_void) -> i32;
    pub fn ntfs_iget(sb: *mut super_block, mft_no: u64) -> *mut inode;
    pub fn ntfs_attr_iget(base_vi: *mut inode, r#type: __le32, name: *mut __le16, name_len: u32) -> *mut inode;
    pub fn ntfs_index_iget(base_vi: *mut inode, name: *mut __le16, name_len: u32) -> *mut inode;
    pub fn ntfs_alloc_big_inode(sb: *mut super_block) -> *mut inode;
    pub fn ntfs_free_big_inode(inode: *mut inode);
    pub fn ntfs_drop_big_inode(inode: *mut inode) -> i32;
    pub fn ntfs_evict_big_inode(vi: *mut inode);
    pub fn __ntfs_init_inode(sb: *mut super_block, ni: *mut ntfs_inode);
    pub fn ntfs_new_extent_inode(sb: *mut super_block, mft_no: u64) -> *mut ntfs_inode;
    pub fn ntfs_clear_extent_inode(ni: *mut ntfs_inode);
    pub fn ntfs_read_inode_mount(vi: *mut inode) -> i32;
    pub fn ntfs_show_options(sf: *mut seq_file, root: *mut dentry) -> i32;
    pub fn ntfs_truncate_vfs(vi: *mut inode, new_size: loff_t, i_size: loff_t) -> i32;
    pub fn ntfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32;
    pub fn ntfs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: ::core::ffi::c_uint, query_flags: ::core::ffi::c_uint) -> i32;
    pub fn ntfs_get_block_mft_record(mft_ni: *mut ntfs_inode, ni: *mut ntfs_inode) -> i32;
    pub fn __ntfs_write_inode(vi: *mut inode, sync: i32) -> i32;
    pub fn ntfs_inode_attach_all_extents(ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_inode_add_attrlist(ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_inode_free_empty_extents(ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_destroy_ext_inode(ni: *mut ntfs_inode);
    pub fn ntfs_inode_free_space(ni: *mut ntfs_inode, size: i32) -> i32;
    pub fn ntfs_inode_attr_pread(vi: *mut inode, pos: i64, count: i64, buf: *mut u8) -> i64;
    pub fn ntfs_inode_attr_pwrite(vi: *mut inode, pos: i64, count: i64, buf: *mut u8, sync: bool) -> i64;
    pub fn ntfs_inode_close(ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_inode_sync_filename(ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_extend_initialized_size(vi: *mut inode, offset: loff_t, new_size: loff_t) -> i32;
    pub fn ntfs_set_vfs_operations(inode: *mut inode, mode: mode_t, dev: dev_t);
    pub fn ntfs_get_locked_folio(mapping: *mut address_space, index: pgoff_t, end_index: pgoff_t, ra: *mut file_ra_state) -> *mut folio;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
