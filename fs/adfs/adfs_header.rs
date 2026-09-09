/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from adfs.h; Linux-provided types and functions are external dependencies. */

pub const ADFS_FREE_FRAG: u32 = 0;
pub const ADFS_BAD_FRAG: u32 = 1;
pub const ADFS_ROOT_FRAG: u32 = 2;
pub const ADFS_FILETYPE_NONE: u16 = u16::MAX;

#[inline]
pub fn adfs_filetype(loadaddr: u32) -> u16 {
    if (loadaddr & 0xfff0_0000) == 0xfff0_0000 {
        ((loadaddr >> 8) & 0xfff) as u16
    } else {
        ADFS_FILETYPE_NONE
    }
}

pub const ADFS_NDA_OWNER_READ: u32 = 1 << 0;
pub const ADFS_NDA_OWNER_WRITE: u32 = 1 << 1;
pub const ADFS_NDA_LOCKED: u32 = 1 << 2;
pub const ADFS_NDA_DIRECTORY: u32 = 1 << 3;
pub const ADFS_NDA_EXECUTE: u32 = 1 << 4;
pub const ADFS_NDA_PUBLIC_READ: u32 = 1 << 5;
pub const ADFS_NDA_PUBLIC_WRITE: u32 = 1 << 6;

#[repr(C)]
pub struct adfs_inode_info {
    pub mmu_private: loff_t,
    pub parent_id: u32,
    pub indaddr: u32,
    pub loadaddr: u32,
    pub execaddr: u32,
    pub attr: c_uint,
    pub vfs_inode: inode,
}

#[inline]
pub unsafe fn ADFS_I(inode_ptr: *mut inode) -> *mut adfs_inode_info {
    (inode_ptr as *mut u8).sub(core::mem::offset_of!(adfs_inode_info, vfs_inode)) as *mut adfs_inode_info
}

#[inline]
pub unsafe fn adfs_inode_is_stamped(inode_ptr: *mut inode) -> bool {
    ((*ADFS_I(inode_ptr)).loadaddr & 0xfff0_0000) == 0xfff0_0000
}

pub struct adfs_discmap;
pub struct adfs_dir_ops;

#[repr(C)]
pub union adfs_sb_info_union {
    pub map_dir: adfs_sb_info_map_dir,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct adfs_sb_info_map_dir {
    pub s_map: *mut adfs_discmap,
    pub s_dir: *const adfs_dir_ops,
}

#[repr(C)]
pub struct adfs_sb_info {
    pub unnamed: adfs_sb_info_union,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_owner_mask: umode_t,
    pub s_other_mask: umode_t,
    pub s_ftsuffix: c_int,
    pub s_ids_per_zone: u32,
    pub s_idlen: u32,
    pub s_map_size: u32,
    pub s_map2blk: c_int,
    pub s_log2sharesize: c_uint,
    pub s_namelen: c_uint,
}

#[inline]
pub unsafe fn ADFS_SB(sb: *mut super_block) -> *mut adfs_sb_info {
    (*sb).s_fs_info as *mut adfs_sb_info
}

#[repr(C)]
pub struct adfs_dir {
    pub sb: *mut super_block,
    pub nr_buffers: c_int,
    pub bh: [*mut buffer_head; 4],
    pub bhs: *mut *mut buffer_head,
    pub pos: c_uint,
    pub parent_id: u32,
    pub head: adfs_dir_head,
    pub tail: adfs_dir_tail,
}

#[repr(C)]
pub union adfs_dir_head {
    pub dirhead: *mut adfs_dirheader,
    pub bighead: *mut adfs_bigdirheader,
}

#[repr(C)]
pub union adfs_dir_tail {
    pub newtail: *mut adfs_newdirtail,
    pub bigtail: *mut adfs_bigdirtail,
}

pub const ADFS_MAX_NAME_LEN: usize = 256 + 4;

#[repr(C)]
pub struct object_info {
    pub parent_id: u32,
    pub indaddr: u32,
    pub loadaddr: u32,
    pub execaddr: u32,
    pub size: u32,
    pub attr: u8,
    pub name_len: c_uint,
    pub name: [c_char; ADFS_MAX_NAME_LEN],
}

#[repr(C)]
pub struct adfs_dir_ops {
    pub read: Option<unsafe extern "C" fn(*mut super_block, c_uint, c_uint, *mut adfs_dir) -> c_int>,
    pub iterate: Option<unsafe extern "C" fn(*mut adfs_dir, *mut dir_context) -> c_int>,
    pub setpos: Option<unsafe extern "C" fn(*mut adfs_dir, c_uint) -> c_int>,
    pub getnext: Option<unsafe extern "C" fn(*mut adfs_dir, *mut object_info) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut adfs_dir, *mut object_info) -> c_int>,
    pub create: Option<unsafe extern "C" fn(*mut adfs_dir, *mut object_info) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut adfs_dir, *mut object_info) -> c_int>,
    pub commit: Option<unsafe extern "C" fn(*mut adfs_dir) -> c_int>,
}

#[repr(C)]
pub struct adfs_discmap {
    pub dm_bh: *mut buffer_head,
    pub dm_startblk: u32,
    pub dm_startbit: c_uint,
    pub dm_endbit: c_uint,
}

extern "C" {
    pub fn adfs_iget(sb: *mut super_block, obj: *mut object_info) -> *mut inode;
    pub fn adfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
    pub fn adfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int;
    pub fn adfs_map_lookup(sb: *mut super_block, frag_id: u32, offset: c_uint) -> c_int;
    pub fn adfs_map_statfs(sb: *mut super_block, buf: *mut kstatfs);
    pub fn adfs_read_map(sb: *mut super_block, dr: *mut adfs_discrecord) -> *mut adfs_discmap;
    pub fn adfs_free_map(sb: *mut super_block);
    pub fn __adfs_error(sb: *mut super_block, function: *const c_char, fmt: *const c_char, ...);
    pub fn adfs_msg(sb: *mut super_block, pfx: *const c_char, fmt: *const c_char, ...);
    pub fn adfs_dir_copyfrom(dst: *mut c_void, dir: *mut adfs_dir, offset: c_uint, len: size_t) -> c_int;
    pub fn adfs_dir_copyto(dir: *mut adfs_dir, offset: c_uint, src: *const c_void, len: size_t) -> c_int;
    pub fn adfs_dir_relse(dir: *mut adfs_dir);
    pub fn adfs_dir_read_buffers(sb: *mut super_block, indaddr: u32, size: c_uint, dir: *mut adfs_dir) -> c_int;
    pub fn adfs_object_fixup(dir: *mut adfs_dir, obj: *mut object_info);
    pub fn adfs_dir_update(sb: *mut super_block, obj: *mut object_info, wait: c_int) -> c_int;
}

extern "C" {
    pub static adfs_dir_inode_operations: inode_operations;
    pub static adfs_dir_operations: file_operations;
    pub static adfs_dentry_operations: dentry_operations;
    pub static adfs_f_dir_ops: adfs_dir_ops;
    pub static adfs_fplus_dir_ops: adfs_dir_ops;
    pub static adfs_file_inode_operations: inode_operations;
    pub static adfs_file_operations: file_operations;
}

#[inline]
pub fn signed_asl(mut val: u32, shift: c_int) -> u32 {
    if shift >= 0 { val <<= shift as u32; } else { val >>= (-shift) as u32; }
    val
}

#[inline]
pub unsafe fn __adfs_block_map(sb: *mut super_block, indaddr: u32, mut block: c_uint) -> c_int {
    if indaddr & 255 != 0 {
        let off = (indaddr & 255) - 1;
        block += off << (*ADFS_SB(sb)).s_log2sharesize;
    }
    adfs_map_lookup(sb, indaddr >> 8, block)
}

#[inline]
pub unsafe fn adfs_map_discrecord(dm: *mut adfs_discmap) -> *mut adfs_discrecord {
    ((*dm).dm_bh).as_ref().unwrap().b_data.add(4) as *mut adfs_discrecord
}

#[inline]
pub unsafe fn adfs_disc_size(dr: *const adfs_discrecord) -> u64 {
    ((le32_to_cpu((*dr).disc_size_high) as u64) << 32) | le32_to_cpu((*dr).disc_size) as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
