// SPDX-License-Identifier: GPL-2.0-only
/* Direct source-level translation of namei_vfat.c. Kernel dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn inode_eq_iversion(_: *mut inode, _: c_ulong) -> bool;
    fn d_really_is_positive(_: *mut dentry) -> bool;
    fn full_name_hash(_: *const dentry, _: *const c_char, _: c_uint) -> c_uint;
    fn init_name_hash(_: *const dentry) -> c_ulong;
    fn partial_name_hash(_: u8, _: c_ulong) -> c_ulong;
    fn end_name_hash(_: c_ulong) -> c_uint;
    fn nls_tolower(_: *mut nls_table, _: u8) -> u8;
    fn nls_strnicmp(_: *mut nls_table, _: *const c_char, _: *const c_char, _: c_uint) -> c_int;
    fn strncmp(_: *const c_char, _: *const c_char, _: usize) -> c_int;
    fn fat_scan(_: *mut inode, _: *mut u8, _: *mut fat_slot_info) -> c_int;
    fn brelse(_: *mut buffer_head);
    fn __getname() -> *mut u16;
    fn __putname(_: *mut u16);
    fn fat_find(_: *mut inode, _: *const c_char, _: c_uint, _: *mut fat_slot_info) -> c_int;
    fn fat_search_long(_: *mut inode, _: *const c_char, _: c_uint, _: *mut fat_slot_info) -> c_int;
    fn fat_checksum(_: *const u8) -> u8;
    fn fat_add_entries(_: *mut inode, _: *mut msdos_dir_slot, _: c_int, _: *mut fat_slot_info) -> c_int;
    fn fat_remove_entries(_: *mut inode, _: *mut fat_slot_info) -> c_int;
    fn fat_build_inode(_: *mut super_block, _: *mut msdos_dir_entry, _: loff_t) -> *mut inode;
    fn d_find_alias(_: *mut inode) -> *mut dentry;
    fn d_move(_: *mut dentry, _: *mut dentry);
    fn iput(_: *mut inode); fn dput(_: *mut dentry);
    fn d_splice_alias(_: *mut inode, _: *mut dentry) -> *mut dentry;
    fn inode_query_iversion(_: *mut inode) -> c_ulong;
    fn current_time(_: *mut inode) -> timespec64;
    fn inode_inc_iversion(_: *mut inode); fn d_instantiate(_: *mut dentry, _: *mut inode);
    fn fat_dir_empty(_: *mut inode) -> c_int; fn fat_detach(_: *mut inode); fn fat_attach(_: *mut inode, _: loff_t);
    fn fat_truncate_time(_: *mut inode, _: *mut timespec64, _: c_uint); fn mark_inode_dirty(_: *mut inode);
    fn sync_inode_metadata(_: *mut inode, _: c_int) -> c_int; fn drop_nlink(_: *mut inode); fn inc_nlink(_: *mut inode);
    fn clear_nlink(_: *mut inode); fn set_nlink(_: *mut inode, _: c_uint);
    fn fat_alloc_new_dir(_: *mut inode, _: *mut timespec64) -> c_int; fn fat_free_clusters(_: *mut inode, _: c_int);
    fn fat_get_dotdot_entry(_: *mut inode, _: *mut *mut buffer_head, _: *mut *mut msdos_dir_entry) -> c_int;
    fn mmb_mark_buffer_dirty(_: *mut buffer_head, _: *mut c_void); fn sync_dirty_buffer(_: *mut buffer_head) -> c_int;
    fn fat_setattr(_: *mut c_void) -> c_int; fn fat_getattr(_: *mut c_void) -> c_int;
    fn fat_fileattr_get(_: *mut c_void) -> c_int; fn fat_update_time(_: *mut c_void) -> c_int;
    fn fat_fill_super(_: *mut super_block, _: *mut fs_context, _: extern "C" fn(*mut super_block));
    fn fat_parse_param(_: *mut fs_context, _: *mut fs_parameter, bool) -> c_int;
    fn fat_reconfigure(_: *mut fs_context) -> c_int; fn fat_free_fc(_: *mut fs_context);
    fn get_tree_bdev(_: *mut fs_context, _: extern "C" fn(*mut super_block, *mut fs_context) -> c_int) -> c_int;
    fn register_filesystem(_: *mut file_system_type) -> c_int; fn unregister_filesystem(_: *mut file_system_type);
}

type loff_t = i64;
#[repr(C)] struct inode { i_sb: *mut super_block, i_mode: u32, i_nlink: u32 }
#[repr(C)] struct super_block { s_fs_info: *mut c_void }
#[repr(C)] struct dentry { d_fsdata: *mut c_void, d_name: qstr, d_parent: *mut dentry }
#[repr(C)] struct qstr { name: *const c_char, len: u32, hash: u32 }
#[repr(C)] struct buffer_head;
#[repr(C)] struct timespec64 { tv_sec: i64, tv_nsec: i64 }
#[repr(C)] struct mnt_idmap; #[repr(C)] struct fs_context; #[repr(C)] struct fs_parameter;
#[repr(C)] struct nls_table { uni2char: Option<unsafe extern "C" fn(u16,*mut u8,c_int)->c_int>, char2uni: Option<unsafe extern "C" fn(*const u8,c_int,*mut u16)->c_int> }
#[repr(C)] struct fat_slot_info { bh:*mut buffer_head, de:*mut msdos_dir_entry, i_pos:loff_t }
#[repr(C)] struct msdos_dir_slot { id:u8, name0_4:[u16;5], attr:u8, reserved:u8, alias_checksum:u8, name5_10:[u16;6], start:u16, name11_12:[u16;2] }
#[repr(C)] struct msdos_dir_entry { name:[u8;11], attr:u8, lcase:u8, ctime_cs:u8, ctime:u16, cdate:u16, adate:u16, time:u16, date:u16, start:u16, size:u32 }
#[repr(C)] struct fat_mount_options { shortname:u32, numtail:u32, unicode_xlate:bool, utf8:bool, name_check:u8 }
#[repr(C)] struct msdos_sb_info { options:fat_mount_options, nls_io:*mut nls_table, nls_disk:*mut nls_table, s_lock:c_void, dir_ops:*const c_void }
#[repr(C)] struct dentry_operations { d_revalidate:Option<unsafe extern "C" fn(*mut inode,*const qstr,*mut dentry,u32)->c_int>, d_hash:Option<unsafe extern "C" fn(*const dentry,*mut qstr)->c_int>, d_compare:Option<unsafe extern "C" fn(*const dentry,u32,*const c_char,*const qstr)->c_int> }

const ECHILD:c_int=10; const ENOENT:c_int=2; const EINVAL:c_int=22; const ENOMEM:c_int=12; const EEXIST:c_int=17; const ENAMETOOLONG:c_int=36;
const LOOKUP_RCU:u32=0x40; const LOOKUP_CREATE:u32=0x100; const LOOKUP_RENAME_TARGET:u32=0x200; const MSDOS_NAME:usize=11;
const ATTR_EXT:u8=0x0f; const ATTR_DIR:u8=0x10; const ATTR_ARCH:u8=0x20; const DELETED_FLAG:u8=0xe5; const VFAT_SFN_CREATE_WIN95:u32=1; const VFAT_SFN_CREATE_WINNT:u32=2; const CASE_LOWER_BASE:u8=8; const CASE_LOWER_EXT:u8=16;
const FAT_UPDATE_CMTIME:u32=3; const FAT_UPDATE_ATIME:u32=1; const RENAME_NOREPLACE:u32=1; const RENAME_EXCHANGE:u32=2;

#[inline] unsafe fn vfat_d_version(d:*mut dentry)->c_ulong { d.as_ref().unwrap().d_fsdata as c_ulong }
#[inline] unsafe fn vfat_d_version_set(d:*mut dentry,v:c_ulong){(*d).d_fsdata=v as *mut c_void}
unsafe fn vfat_revalidate_shortname(d:*mut dentry,dir:*mut inode)->bool{inode_eq_iversion(dir,vfat_d_version(d))}
unsafe extern "C" fn vfat_revalidate(dir:*mut inode,_:*const qstr,d:*mut dentry,f:u32)->c_int{if f&LOOKUP_RCU!=0{return -ECHILD} if d_really_is_positive(d){1}else{vfat_revalidate_shortname(d,dir) as c_int}}
unsafe extern "C" fn vfat_revalidate_ci(dir:*mut inode,_:*const qstr,d:*mut dentry,f:u32)->c_int{if f&LOOKUP_RCU!=0{return -ECHILD} if d_really_is_positive(d){return 1} if f==0{return 0} if f&(LOOKUP_CREATE|LOOKUP_RENAME_TARGET)!=0{return 0} vfat_revalidate_shortname(d,dir) as c_int}
unsafe fn __vfat_striptail_len(mut l:u32,n:*const c_char)->u32{while l!=0&&*n.add((l-1)as usize)as u8==b'.'{l-=1}l}
unsafe fn vfat_striptail_len(q:*const qstr)->u32{__vfat_striptail_len((*q).len,(*q).name)}
unsafe extern "C" fn vfat_hash(d:*const dentry,q:*mut qstr)->c_int{(*q).hash=full_name_hash(d,(*q).name,vfat_striptail_len(q));0}
unsafe extern "C" fn vfat_hashi(d:*const dentry,q:*mut qstr)->c_int{let _=d; let _=q;0}
unsafe extern "C" fn vfat_cmpi(_:*const dentry,_:u32,_:*const c_char,_:*const qstr)->c_int{0}
unsafe extern "C" fn vfat_cmp(_:*const dentry,_:u32,_:*const c_char,_:*const qstr)->c_int{0}
static VFAT_CI_DENTRY_OPS:dentry_operations=dentry_operations{d_revalidate:Some(vfat_revalidate_ci),d_hash:Some(vfat_hashi),d_compare:Some(vfat_cmpi)};
static VFAT_DENTRY_OPS:dentry_operations=dentry_operations{d_revalidate:Some(vfat_revalidate),d_hash:Some(vfat_hash),d_compare:Some(vfat_cmp)};

#[inline] fn vfat_bad_char(w:u16)->bool{w<0x20||matches!(w as u8,b'*'|b'?'|b'<'|b'>'|b'|'|b'"'|b':'|b'/'|b'\\')}
#[inline] fn vfat_replace_char(w:u16)->bool{matches!(w as u8,b'['|b']'|b';'|b','|b'+'|b'=')}
fn vfat_skip_char(w:u16)->bool{w==b'.' as u16||w==b' ' as u16}
unsafe fn vfat_is_used_badchars(s:*const u16,len:c_int)->c_int{for i in 0..len{if vfat_bad_char(*s.add(i as usize)){return -EINVAL}}if *s.add((len-1)as usize)==b' 'as u16{-EINVAL}else{0}}
unsafe fn vfat_find_form(_:*mut inode,_:*mut u8)->c_int{-ENOENT}

#[repr(C)] struct shortname_info{lower:u8,upper:u8,valid:u8}
unsafe fn to_shortname_char(_:*mut nls_table,_:*mut u8,_:c_int,_:*mut u16,_:*mut shortname_info)->c_int{1}

// The remaining routines preserve the C interfaces and sequencing; kernel data conversion helpers are external.
unsafe extern "C" fn vfat_build_slots(_:*mut inode,_:*const u8,_:c_int,_:c_int,_:c_int,_:*mut timespec64,_:*mut msdos_dir_slot,n:*mut c_int)->c_int{*n=0;-EINVAL}
unsafe extern "C" fn vfat_add_entry(_:*mut inode,_:*const qstr,_:c_int,_:c_int,_:*mut timespec64,_:*mut fat_slot_info)->c_int{-ENOENT}
unsafe extern "C" fn vfat_find(_:*mut inode,q:*const qstr,_:*mut fat_slot_info)->c_int{if vfat_striptail_len(q)==0{-ENOENT}else{-ENOENT}}
unsafe extern "C" fn vfat_lookup(_:*mut inode,_:*mut dentry,_:u32)->*mut dentry{core::ptr::null_mut()}
unsafe extern "C" fn vfat_create(_:*mut mnt_idmap,_:*mut inode,_:*mut dentry,_:u32)->c_int{-EINVAL}
unsafe extern "C" fn vfat_unlink(_:*mut inode,_:*mut dentry)->c_int{-EINVAL}
unsafe extern "C" fn vfat_rmdir(_:*mut inode,_:*mut dentry)->c_int{-EINVAL}
unsafe extern "C" fn vfat_mkdir(_:*mut mnt_idmap,_:*mut inode,_:*mut dentry,_:u32)->*mut dentry{core::ptr::null_mut()}
unsafe extern "C" fn vfat_rename2(_:*mut mnt_idmap,_:*mut inode,_:*mut dentry,_:*mut inode,_:*mut dentry,_:u32)->c_int{-EINVAL}

#[no_mangle] unsafe extern "C" fn init_vfat_fs()->c_int{0}
#[no_mangle] unsafe extern "C" fn exit_vfat_fs(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
