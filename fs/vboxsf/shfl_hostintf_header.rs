/* SPDX-License-Identifier: MIT */
/* VirtualBox Shared Folders: host interface definition. */

// Dependency supplied by the surrounding translation unit:
// linux/vbox_vmmdev_types.h

pub const SHFL_MAX_RW_COUNT: u32 = 16 * (1 << 20);

pub const SHFL_FN_QUERY_MAPPINGS: i32 = 1;
pub const SHFL_FN_QUERY_MAP_NAME: i32 = 2;
pub const SHFL_FN_CREATE: i32 = 3;
pub const SHFL_FN_CLOSE: i32 = 4;
pub const SHFL_FN_READ: i32 = 5;
pub const SHFL_FN_WRITE: i32 = 6;
pub const SHFL_FN_LOCK: i32 = 7;
pub const SHFL_FN_LIST: i32 = 8;
pub const SHFL_FN_INFORMATION: i32 = 9;
pub const SHFL_FN_REMOVE: i32 = 11;
pub const SHFL_FN_MAP_FOLDER_OLD: i32 = 12;
pub const SHFL_FN_UNMAP_FOLDER: i32 = 13;
pub const SHFL_FN_RENAME: i32 = 14;
pub const SHFL_FN_FLUSH: i32 = 15;
pub const SHFL_FN_SET_UTF8: i32 = 16;
pub const SHFL_FN_MAP_FOLDER: i32 = 17;
pub const SHFL_FN_READLINK: i32 = 18;
pub const SHFL_FN_SYMLINK: i32 = 19;
pub const SHFL_FN_SET_SYMLINKS: i32 = 20;

pub const SHFL_ROOT_NIL: u32 = u32::MAX;
pub const SHFL_HANDLE_NIL: u64 = u64::MAX;
pub const SHFL_MAX_LEN: u32 = 256;
pub const SHFL_MAX_MAPPINGS: u32 = 64;

#[repr(C)]
pub union shfl_string_string {
    pub legacy_padding: [u8; 2],
    pub utf8: [u8; 0],
    pub utf16: [u16; 0],
}

#[repr(C)]
pub struct shfl_string {
    pub size: u16,
    pub length: u16,
    pub string: shfl_string_string,
}

pub const SHFLSTRING_HEADER_SIZE: u32 = 4;

#[inline]
pub unsafe fn shfl_string_buf_size(string: *const shfl_string) -> u32 {
    if !string.is_null() { SHFLSTRING_HEADER_SIZE + (*string).size as u32 } else { 0 }
}

pub const SHFL_UNIX_ISUID: u32 = 0o4000;
pub const SHFL_UNIX_ISGID: u32 = 0o2000;
pub const SHFL_UNIX_ISTXT: u32 = 0o1000;
pub const SHFL_UNIX_IRUSR: u32 = 0o400;
pub const SHFL_UNIX_IWUSR: u32 = 0o200;
pub const SHFL_UNIX_IXUSR: u32 = 0o100;
pub const SHFL_UNIX_IRGRP: u32 = 0o40;
pub const SHFL_UNIX_IWGRP: u32 = 0o20;
pub const SHFL_UNIX_IXGRP: u32 = 0o10;
pub const SHFL_UNIX_IROTH: u32 = 0o4;
pub const SHFL_UNIX_IWOTH: u32 = 0o2;
pub const SHFL_UNIX_IXOTH: u32 = 0o1;
pub const SHFL_TYPE_FIFO: u32 = 0o10000;
pub const SHFL_TYPE_DEV_CHAR: u32 = 0o20000;
pub const SHFL_TYPE_DIRECTORY: u32 = 0o40000;
pub const SHFL_TYPE_DEV_BLOCK: u32 = 0o60000;
pub const SHFL_TYPE_FILE: u32 = 0o100000;
pub const SHFL_TYPE_SYMLINK: u32 = 0o120000;
pub const SHFL_TYPE_SOCKET: u32 = 0o140000;
pub const SHFL_TYPE_WHITEOUT: u32 = 0o160000;
pub const SHFL_TYPE_MASK: u32 = 0o170000;

#[inline]
pub const fn SHFL_IS_DIRECTORY(m: u32) -> bool { (m & SHFL_TYPE_MASK) == SHFL_TYPE_DIRECTORY }
#[inline]
pub const fn SHFL_IS_SYMLINK(m: u32) -> bool { (m & SHFL_TYPE_MASK) == SHFL_TYPE_SYMLINK }

#[repr(i32)]
pub enum shfl_fsobjattr_add {
    SHFLFSOBJATTRADD_NOTHING = 1,
    SHFLFSOBJATTRADD_UNIX,
    SHFLFSOBJATTRADD_EASIZE,
    SHFLFSOBJATTRADD_LAST = 3,
    SHFLFSOBJATTRADD_32BIT_SIZE_HACK = 0x7fffffff,
}

#[repr(C, packed)]
pub struct shfl_fsobjattr_unix {
    pub uid: u32, pub gid: u32, pub hardlinks: u32, pub inode_id_device: u32,
    pub inode_id: u64, pub flags: u32, pub generation_id: u32, pub device: u32,
}
#[repr(C, packed)] pub struct shfl_fsobjattr_easize { pub cb: i64; }
#[repr(C, packed)] pub union shfl_fsobjattr_u { pub unix_attr: shfl_fsobjattr_unix, pub size: shfl_fsobjattr_easize }
#[repr(C, packed)] pub struct shfl_fsobjattr { pub mode: u32, pub additional: shfl_fsobjattr_add, pub u: shfl_fsobjattr_u }

#[repr(C)] pub struct shfl_timespec { pub ns_relative_to_unix_epoch: i64 }
#[repr(C, packed)] pub struct shfl_fsobjinfo {
    pub size: i64, pub allocated: i64, pub access_time: shfl_timespec,
    pub modification_time: shfl_timespec, pub change_time: shfl_timespec,
    pub birth_time: shfl_timespec, pub attr: shfl_fsobjattr,
}

#[repr(i32)] pub enum shfl_create_result { SHFL_NO_RESULT, SHFL_PATH_NOT_FOUND, SHFL_FILE_NOT_FOUND, SHFL_FILE_EXISTS, SHFL_FILE_CREATED, SHFL_FILE_REPLACED }

pub const SHFL_CF_NONE: u32 = 0x00000000;
pub const SHFL_CF_LOOKUP: u32 = 0x00000001;
pub const SHFL_CF_OPEN_TARGET_DIRECTORY: u32 = 0x00000002;
pub const SHFL_CF_DIRECTORY: u32 = 0x00000004;
pub const SHFL_CF_ACT_MASK_IF_EXISTS: u32 = 0x000000f0;
pub const SHFL_CF_ACT_MASK_IF_NEW: u32 = 0x00000f00;
pub const SHFL_CF_ACT_OPEN_IF_EXISTS: u32 = 0x00000000;
pub const SHFL_CF_ACT_FAIL_IF_EXISTS: u32 = 0x00000010;
pub const SHFL_CF_ACT_REPLACE_IF_EXISTS: u32 = 0x00000020;
pub const SHFL_CF_ACT_OVERWRITE_IF_EXISTS: u32 = 0x00000030;
pub const SHFL_CF_ACT_CREATE_IF_NEW: u32 = 0x00000000;
pub const SHFL_CF_ACT_FAIL_IF_NEW: u32 = 0x00000100;
pub const SHFL_CF_ACCESS_MASK_RW: u32 = 0x00003000;
pub const SHFL_CF_ACCESS_NONE: u32 = 0x00000000;
pub const SHFL_CF_ACCESS_READ: u32 = 0x00001000;
pub const SHFL_CF_ACCESS_WRITE: u32 = 0x00002000;
pub const SHFL_CF_ACCESS_READWRITE: u32 = 0x00003000;
pub const SHFL_CF_ACCESS_MASK_DENY: u32 = 0x0000c000;
pub const SHFL_CF_ACCESS_DENYNONE: u32 = 0x00000000;
pub const SHFL_CF_ACCESS_DENYREAD: u32 = 0x00004000;
pub const SHFL_CF_ACCESS_DENYWRITE: u32 = 0x00008000;
pub const SHFL_CF_ACCESS_DENYALL: u32 = 0x0000c000;
pub const SHFL_CF_ACCESS_MASK_ATTR: u32 = 0x00030000;
pub const SHFL_CF_ACCESS_ATTR_NONE: u32 = 0x00000000;
pub const SHFL_CF_ACCESS_ATTR_READ: u32 = 0x00010000;
pub const SHFL_CF_ACCESS_ATTR_WRITE: u32 = 0x00020000;
pub const SHFL_CF_ACCESS_ATTR_READWRITE: u32 = 0x00030000;
pub const SHFL_CF_ACCESS_APPEND: u32 = 0x00040000;

#[repr(C, packed)] pub struct shfl_createparms { pub handle: u64, pub result: shfl_create_result, pub create_flags: u32, pub info: shfl_fsobjinfo }
#[repr(C)] pub struct shfl_dirinfo { pub info: shfl_fsobjinfo, pub short_name_len: u16, pub short_name: [u16; 14], pub name: shfl_string }
#[repr(C)] pub struct shfl_fsproperties { pub max_component_len: u32, pub remote: bool, pub case_sensitive: bool, pub read_only: bool, pub supports_unicode: bool, pub compressed: bool, pub file_compression: bool }
#[repr(C)] pub struct shfl_volinfo { pub total_allocation_bytes: i64, pub available_allocation_bytes: i64, pub bytes_per_allocation_unit: u32, pub bytes_per_sector: u32, pub serial: u32, pub properties: shfl_fsproperties }

#[repr(C)] pub struct shfl_map_folder { pub path: vmmdev_hgcm_function_parameter, pub root: vmmdev_hgcm_function_parameter, pub delimiter: vmmdev_hgcm_function_parameter, pub case_sensitive: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_MAP_FOLDER: usize = 4;
#[repr(C)] pub struct shfl_unmap_folder { pub root: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_UNMAP_FOLDER: usize = 1;
#[repr(C)] pub struct shfl_create { pub root: vmmdev_hgcm_function_parameter, pub path: vmmdev_hgcm_function_parameter, pub parms: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_CREATE: usize = 3;
#[repr(C)] pub struct shfl_close { pub root: vmmdev_hgcm_function_parameter, pub handle: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_CLOSE: usize = 2;
#[repr(C)] pub struct shfl_read { pub root: vmmdev_hgcm_function_parameter, pub handle: vmmdev_hgcm_function_parameter, pub offset: vmmdev_hgcm_function_parameter, pub cb: vmmdev_hgcm_function_parameter, pub buffer: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_READ: usize = 5;
#[repr(C)] pub struct shfl_write { pub root: vmmdev_hgcm_function_parameter, pub handle: vmmdev_hgcm_function_parameter, pub offset: vmmdev_hgcm_function_parameter, pub cb: vmmdev_hgcm_function_parameter, pub buffer: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_WRITE: usize = 5;

pub const SHFL_LIST_NONE: u32 = 0;
pub const SHFL_LIST_RETURN_ONE: u32 = 1;
#[repr(C)] pub struct shfl_list { pub root: vmmdev_hgcm_function_parameter, pub handle: vmmdev_hgcm_function_parameter, pub flags: vmmdev_hgcm_function_parameter, pub cb: vmmdev_hgcm_function_parameter, pub path: vmmdev_hgcm_function_parameter, pub buffer: vmmdev_hgcm_function_parameter, pub resume_point: vmmdev_hgcm_function_parameter, pub file_count: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_LIST: usize = 8;
#[repr(C)] pub struct shfl_readLink { pub root: vmmdev_hgcm_function_parameter, pub path: vmmdev_hgcm_function_parameter, pub buffer: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_READLINK: usize = 3;

pub const SHFL_INFO_MODE_MASK: u32 = 0x1;
pub const SHFL_INFO_GET: u32 = 0x0;
pub const SHFL_INFO_SET: u32 = 0x1;
pub const SHFL_INFO_NAME: u32 = 0x2;
pub const SHFL_INFO_SIZE: u32 = 0x4;
pub const SHFL_INFO_FILE: u32 = 0x8;
pub const SHFL_INFO_VOLUME: u32 = 0x10;
#[repr(C)] pub struct shfl_information { pub root: vmmdev_hgcm_function_parameter, pub handle: vmmdev_hgcm_function_parameter, pub flags: vmmdev_hgcm_function_parameter, pub cb: vmmdev_hgcm_function_parameter, pub info: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_INFORMATION: usize = 5;

pub const SHFL_REMOVE_FILE: u32 = 0x1;
pub const SHFL_REMOVE_DIR: u32 = 0x2;
pub const SHFL_REMOVE_SYMLINK: u32 = 0x4;
#[repr(C)] pub struct shfl_remove { pub root: vmmdev_hgcm_function_parameter, pub path: vmmdev_hgcm_function_parameter, pub flags: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_REMOVE: usize = 3;

pub const SHFL_RENAME_FILE: u32 = 0x1;
pub const SHFL_RENAME_DIR: u32 = 0x2;
pub const SHFL_RENAME_REPLACE_IF_EXISTS: u32 = 0x4;
#[repr(C)] pub struct shfl_rename { pub root: vmmdev_hgcm_function_parameter, pub src: vmmdev_hgcm_function_parameter, pub dest: vmmdev_hgcm_function_parameter, pub flags: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_RENAME: usize = 4;

#[repr(C)] pub struct shfl_symlink { pub root: vmmdev_hgcm_function_parameter, pub new_path: vmmdev_hgcm_function_parameter, pub old_path: vmmdev_hgcm_function_parameter, pub info: vmmdev_hgcm_function_parameter }
pub const SHFL_CPARMS_SYMLINK: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
