/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of overlayfs.h. Kernel dependencies are external. */

#[repr(u32)]
pub enum ovl_path_type { __OVL_PATH_UPPER = 1 << 0, __OVL_PATH_MERGE = 1 << 1, __OVL_PATH_ORIGIN = 1 << 2 }
pub const OVL_XATTR_NAMESPACE: &[u8] = b"overlay.\0";

#[repr(u32)]
pub enum ovl_xattr { OVL_XATTR_OPAQUE, OVL_XATTR_REDIRECT, OVL_XATTR_ORIGIN, OVL_XATTR_IMPURE, OVL_XATTR_NLINK, OVL_XATTR_UPPER, OVL_XATTR_UUID, OVL_XATTR_METACOPY, OVL_XATTR_PROTATTR, OVL_XATTR_XWHITEOUT }
#[repr(u32)]
pub enum ovl_inode_flag { OVL_IMPURE, OVL_WHITEOUTS, OVL_INDEX, OVL_UPPERDATA, OVL_CONST_INO, OVL_HAS_DIGEST, OVL_VERIFIED_DIGEST }
#[repr(u32)]
pub enum ovl_entry_flag { OVL_E_UPPER_ALIAS, OVL_E_OPAQUE, OVL_E_CONNECTED, OVL_E_XWHITEOUTS }

pub const OVL_REDIRECT_OFF: u32 = 0; pub const OVL_REDIRECT_FOLLOW: u32 = 1; pub const OVL_REDIRECT_NOFOLLOW: u32 = 2; pub const OVL_REDIRECT_ON: u32 = 3;
pub const OVL_UUID_OFF: u32 = 0; pub const OVL_UUID_NULL: u32 = 1; pub const OVL_UUID_AUTO: u32 = 2; pub const OVL_UUID_ON: u32 = 3;
pub const OVL_XINO_OFF: u32 = 0; pub const OVL_XINO_AUTO: u32 = 1; pub const OVL_XINO_ON: u32 = 2;
pub const OVL_VERITY_OFF: u32 = 0; pub const OVL_VERITY_ON: u32 = 1; pub const OVL_VERITY_REQUIRE: u32 = 2;
pub const OVL_FSYNC_VOLATILE: u32 = 0; pub const OVL_FSYNC_AUTO: u32 = 1; pub const OVL_FSYNC_STRICT: u32 = 2;
pub const OVL_FH_VERSION: u8 = 0; pub const OVL_FH_MAGIC: u8 = 0xfb;
pub const OVL_FH_FLAG_BIG_ENDIAN: u8 = 1 << 0; pub const OVL_FH_FLAG_ANY_ENDIAN: u8 = 1 << 1; pub const OVL_FH_FLAG_PATH_UPPER: u8 = 1 << 2;
pub const OVL_FILEID_V0: u8 = 0xfb; pub const OVL_FILEID_V1: u8 = 0xf8;

#[repr(C, packed)]
pub struct ovl_fb { pub version: u8, pub magic: u8, pub len: u8, pub flags: u8, pub r#type: u8, pub uuid: uuid_t, pub fid: [u32; 0] }
#[repr(C, packed)]
pub union ovl_fh_union { pub fb: ovl_fb, pub buf: [u8; 0] }
#[repr(C, packed)]
pub struct ovl_fh { pub padding: [u8; 3], pub u: ovl_fh_union }
#[repr(C, packed)]
pub struct ovl_metacopy { pub version: u8, pub len: u8, pub flags: u8, pub digest_algo: u8, pub digest: [u8; FS_VERITY_MAX_DIGEST_SIZE] }
pub const OVL_METACOPY_MAX_SIZE: usize = core::mem::size_of::<ovl_metacopy>();
pub const OVL_METACOPY_MIN_SIZE: usize = OVL_METACOPY_MAX_SIZE - FS_VERITY_MAX_DIGEST_SIZE;

pub unsafe fn ovl_metadata_digest_size(m: *const ovl_metacopy) -> i32 { if (*m).len as usize < OVL_METACOPY_MIN_SIZE { 0 } else { (*m).len as i32 - OVL_METACOPY_MIN_SIZE as i32 } }
pub const OVL_OPEN_FLAGS: u32 = O_NOATIME;

extern "C" {
    pub static ovl_xattr_table: [[*const core::ffi::c_char; 2]; 10];
    pub fn ovl_upper_mnt_idmap(ofs: *mut ovl_fs) -> *mut mnt_idmap;
    pub fn ovl_upper_mnt(ofs: *mut ovl_fs) -> *mut vfsmount;
    pub fn ovl_get_dir_xattr_val(ofs: *mut ovl_fs, path: *const path, ox: ovl_xattr) -> u8;
    pub fn ovl_path_is_whiteout(ofs: *mut ovl_fs, path: *const path) -> bool;
    pub fn ovl_path_check_origin_xattr(ofs: *mut ovl_fs, path: *const path) -> bool;
    pub fn ovl_test_flag(flag: usize, inode: *mut inode) -> bool;
    pub fn ovl_set_flag(flag: usize, inode: *mut inode);
}

pub unsafe fn ovl_xattr(ofs: *mut ovl_fs, ox: ovl_xattr) -> *const core::ffi::c_char { ovl_xattr_table[ox as usize][(*ofs).config.userxattr as usize] }
pub unsafe fn ovl_upper_is_whiteout(ofs: *mut ovl_fs, d: *mut dentry) -> bool { let p = path { dentry: d, mnt: ovl_upper_mnt(ofs) }; ovl_path_is_whiteout(ofs, &p) }
pub unsafe fn ovl_check_origin_xattr(ofs: *mut ovl_fs, d: *mut dentry) -> bool { let p = path { dentry: d, mnt: ovl_upper_mnt(ofs) }; ovl_path_check_origin_xattr(ofs, &p) }

#[repr(C)] pub struct ovl_inode_params { pub newinode: *mut inode, pub upperdentry: *mut dentry, pub oe: *mut ovl_entry, pub index: bool, pub redirect: *mut core::ffi::c_char, pub lowerdata_redirect: *mut core::ffi::c_char }
#[repr(C)] pub struct ovl_cattr { pub rdev: dev_t, pub mode: umode_t, pub link: *const core::ffi::c_char, pub hardlink: *mut dentry }

extern "C" {
    pub fn ovl_get_write_access(d: *mut dentry) -> i32; pub fn ovl_put_write_access(d: *mut dentry); pub fn ovl_start_write(d: *mut dentry); pub fn ovl_end_write(d: *mut dentry); pub fn ovl_want_write(d: *mut dentry) -> i32; pub fn ovl_drop_write(d: *mut dentry);
    pub fn ovl_workdir(d: *mut dentry) -> *mut dentry; pub fn ovl_override_creds(sb: *mut super_block) -> *const cred;
    pub fn ovl_path_type(d: *mut dentry) -> ovl_path_type; pub fn ovl_path_upper(d: *mut dentry, p: *mut path); pub fn ovl_path_lower(d: *mut dentry, p: *mut path); pub fn ovl_dentry_upper(d: *mut dentry) -> *mut dentry; pub fn ovl_dentry_lower(d: *mut dentry) -> *mut dentry;
    pub fn ovl_copy_up(d: *mut dentry) -> i32; pub fn ovl_copy_up_with_data(d: *mut dentry) -> i32; pub fn ovl_maybe_copy_up(d: *mut dentry, flags: i32) -> i32;
    pub fn ovl_fill_super(sb: *mut super_block, fc: *mut fs_context) -> i32;
}

/* Remaining header declarations retain their C ABI and are supplied by kernel translation units. */
extern "C" {
    pub fn ovl_check_fb_len(fb: *mut ovl_fb, fb_len: i32) -> i32;
    pub fn ovl_decode_real_fh(ofs: *mut ovl_fs, fh: *mut ovl_fh, mnt: *mut vfsmount, connected: bool) -> *mut dentry;
    pub fn ovl_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    pub fn ovl_dir_cache_free(inode: *mut inode); pub fn ovl_set_nlink_upper(d: *mut dentry) -> i32; pub fn ovl_set_nlink_lower(d: *mut dentry) -> i32;
    pub fn ovl_update_time(inode: *mut inode, r#type: fs_update_time, flags: u32) -> i32;
    pub fn ovl_create_real(ofs: *mut ovl_fs, parent: *mut dentry, newdentry: *mut dentry, qname: *mut qstr, attr: *mut ovl_cattr) -> *mut dentry;
    pub fn ovl_cleanup(ofs: *mut ovl_fs, workdir: *mut dentry, dentry: *mut dentry) -> i32;
    pub fn ovl_tempname(name: *mut core::ffi::c_char);
}

pub const OVL_TEMPNAME_SIZE: usize = 20;
pub const OVL_FH_WIRE_OFFSET: usize = 3;
pub const OVL_FH_FID_OFFSET: usize = OVL_FH_WIRE_OFFSET + 16;
pub const OVL_PROT_I_FLAGS_MASK: u32 = S_APPEND | S_IMMUTABLE;
pub const OVL_COPY_FS_FLAGS_MASK: u32 = FS_SYNC_FL | FS_NOATIME_FL;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
