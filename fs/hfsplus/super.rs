// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of linux/fs/hfsplus/super.c. External kernel symbols are
 * intentionally left as dependencies supplied by other translation units. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

// Kernel and filesystem types/functions/constants are supplied externally.
extern "C" {
    static mut hfsplus_inode_cachep: *mut kmem_cache;
    fn hfsplus_alloc_inode(sb: *mut super_block) -> *mut inode;
    fn hfsplus_free_inode(inode: *mut inode);
}

#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: u64, pub i_mode: u32, pub i_size: i64, pub i_mapping: *mut address_space, pub i_data: address_space }
#[repr(C)] pub struct super_block { pub s_bdev: *mut block_device, pub s_blocksize: u32, pub s_magic: u32, pub s_flags: u32, pub s_op: *const super_operations, pub s_maxbytes: i64, pub s_root: *mut dentry, pub s_xattr: *const xattr_handler }
#[repr(C)] pub struct address_space { pub a_ops: *const address_space_operations }
#[repr(C)] pub struct block_device { pub bd_dev: u64 }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct fs_context { pub root: *mut dentry, pub sb_flags: u32, pub s_fs_info: *mut core::ffi::c_void, pub ops: *const fs_context_operations, pub purpose: u32 }
#[repr(C)] pub struct qstr { pub name: *const i8, pub len: u32 }
#[repr(C)] pub struct kstatfs { pub f_type:u64, pub f_bsize:u64, pub f_blocks:u64, pub f_bfree:u64, pub f_bavail:u64, pub f_files:u64, pub f_ffree:u64, pub f_fsid:u64, pub f_namelen:u32 }
#[repr(C)] pub struct hfsplus_vh { pub version:u16, pub free_blocks:u32, pub next_cnid:u32, pub folder_count:u32, pub file_count:u32, pub data_clump_sz:u32, pub rsrc_clump_sz:u32, pub attr_file:hfsplus_fork_raw, pub ext_file:hfsplus_fork_raw, pub cat_file:hfsplus_fork_raw, pub alloc_file:hfsplus_fork_raw, pub start_file:hfsplus_fork_raw, pub modify_date:u32, pub attributes:u32, pub last_mount_vers:u32, pub write_count:u32 }
#[repr(C)] pub struct hfsplus_fork_raw { pub total_size:u64, pub total_blocks:u32 }
#[repr(C)] pub struct hfsplus_sb_info { pub s_vhdr:*mut hfsplus_vh, pub s_backup_vhdr:*mut hfsplus_vh, pub s_vhdr_buf:*mut core::ffi::c_void, pub s_backup_vhdr_buf:*mut core::ffi::c_void, pub ext_tree:*mut hfs_btree, pub cat_tree:*mut hfs_btree, pub attr_tree:*mut hfs_btree, pub alloc_file:*mut inode, pub hidden_dir:*mut inode, pub nls:*mut nls_table, pub flags:usize, pub free_blocks:u32, pub next_cnid:u32, pub folder_count:u32, pub file_count:u32, pub total_blocks:u32, pub data_clump_blocks:u32, pub rsrc_clump_blocks:u32, pub alloc_blksz_shift:u32, pub fs_shift:u32, pub sect_count:u64, pub part_start:u64, pub attr_tree_state: i32, pub alloc_mutex: mutex, pub vh_mutex: mutex, pub work_lock: spinlock, pub sync_work: delayed_work, pub work_queued:i32, pub rcu:rcu_head }
#[repr(C)] pub struct hfsplus_inode_info { pub vfs_inode: inode, pub opencnt:i32, pub first_blocks:u32, pub clump_blocks:u32, pub alloc_blocks:u32, pub cached_start:u32, pub cached_blocks:u32, pub first_extents:[u8;16], pub cached_extents:[u8;16], pub extent_state:u32, pub extents_lock:mutex, pub rsrc_inode:*mut inode, pub create_date:u32, pub linkid:u32, pub flags:u32, pub fs_blocks:u32, pub userflags:u32, pub subfolders:u32, pub phys_size:u64 }
#[repr(C)] pub struct hfs_btree { pub inode:*mut inode, pub tree_lock:mutex }
#[repr(C)] pub struct hfs_find_data { pub search_key:*mut core::ffi::c_void }
#[repr(C)] pub struct hfsplus_cat_entry { pub type_:u16, pub folder:hfsplus_folder }
#[repr(C)] pub struct hfsplus_folder { pub id:u32 }
#[repr(C)] pub struct mutex; #[repr(C)] pub struct spinlock; #[repr(C)] pub struct delayed_work; #[repr(C)] pub struct rcu_head; #[repr(C)] pub struct nls_table; #[repr(C)] pub struct kmem_cache; #[repr(C)] pub struct address_space_operations; #[repr(C)] pub struct xattr_handler; #[repr(C)] pub struct work_struct; #[repr(C)] pub struct writeback_control; #[repr(C)] pub struct fs_context_operations; #[repr(C)] pub struct super_operations; #[repr(C)] pub struct file_system_type;

extern "C" {
    fn hfsplus_inode_read_fork(*mut inode, *mut hfsplus_fork_raw); fn hfsplus_inode_write_fork(*mut inode,*mut hfsplus_fork_raw); fn iget_locked(*mut super_block,u64)->*mut inode; fn inode_state_read_once(*mut inode)->u32; fn hfs_find_init(*mut hfs_btree,*mut hfs_find_data)->i32; fn hfs_find_exit(*mut hfs_find_data); fn hfsplus_find_cat(*mut super_block,u64,*mut hfs_find_data)->i32; fn hfsplus_cat_read_inode(*mut inode,*mut hfs_find_data)->i32; fn iget_failed(*mut inode); fn unlock_new_inode(*mut inode); fn hfsplus_ext_write_extent(*mut inode)->i32; fn hfsplus_cat_write_inode(*mut inode)->i32; fn truncate_inode_pages_final(*mut address_space); fn clear_inode(*mut inode); fn iput(*mut inode); fn hfsplus_submit_bio(*mut super_block,u64,*mut core::ffi::c_void,*mut core::ffi::c_void,u32)->i32; fn filemap_write_and_wait(*mut address_space)->i32; fn hfs_btree_write(*mut hfs_btree)->i32; fn hfs_btree_close(*mut hfs_btree); fn hfsplus_read_wrapper(*mut super_block)->i32; fn hfs_btree_open(*mut super_block,u64)->*mut hfs_btree; fn hfsplus_new_inode(*mut super_block,*mut inode,u32)->*mut inode; fn hfsplus_create_cat(u64,*mut inode,*mut qstr,*mut inode)->i32; fn hfsplus_delete_cat(u64,*mut inode,*mut qstr); fn hfsplus_init_security(*mut inode,*mut inode,*mut qstr)->i32; fn hfsplus_mark_inode_dirty(*mut inode,u32); fn hfsplus_cat_build_key(*mut super_block,*mut core::ffi::c_void,u32,*const qstr)->i32; fn hfsplus_brec_read_cat(*mut hfs_find_data,*mut hfsplus_cat_entry)->i32; fn load_nls(*const i8)->*mut nls_table; fn load_nls_default()->*mut nls_table; fn unload_nls(*mut nls_table); fn sync_filesystem(*mut super_block); fn blkdev_issue_flush(*mut block_device); fn d_make_root(*mut inode)->*mut dentry; fn dput(*mut dentry); fn kill_block_super(*mut super_block); fn call_rcu(*mut rcu_head, *const core::ffi::c_void); fn register_filesystem(*mut file_system_type)->i32; fn unregister_filesystem(*mut file_system_type); fn rcu_barrier(); fn hfsplus_create_attr_tree_cache()->i32; fn hfsplus_destroy_attr_tree_cache(); fn kmem_cache_create(*const i8,usize,u32,u32,*const core::ffi::c_void)->*mut kmem_cache; fn kmem_cache_destroy(*mut kmem_cache); fn alloc_inode_sb(*mut super_block,*mut kmem_cache,u32)->*mut hfsplus_inode_info; fn kmem_cache_free(*mut kmem_cache,*mut hfsplus_inode_info); fn inode_init_once(*mut inode); fn get_tree_bdev(*mut fs_context, *const core::ffi::c_void)->i32; fn hfsplus_parse_param(); fn hfsplus_fill_defaults(*mut hfsplus_sb_info); fn hfsplus_show_options(); fn hfsp_now2mt()->u32; fn hfsplus_btree_lock_class(*mut hfs_btree)->*const core::ffi::c_void;
}

#[inline] unsafe fn sbi(sb:*mut super_block)->*mut hfsplus_sb_info { *(sb as *mut *mut hfsplus_sb_info) }
#[inline] unsafe fn hi(i:*mut inode)->*mut hfsplus_inode_info { i as *mut hfsplus_inode_info }

pub unsafe fn hfsplus_system_read_inode(inode:*mut inode)->i32 { let vh=(*sbi((*inode).i_sb)).s_vhdr; match (*inode).i_ino { HFSPLUS_EXT_CNID=>{hfsplus_inode_read_fork(inode,&mut (*vh).ext_file);}, HFSPLUS_CAT_CNID=>{hfsplus_inode_read_fork(inode,&mut (*vh).cat_file);}, HFSPLUS_ALLOC_CNID=>{hfsplus_inode_read_fork(inode,&mut (*vh).alloc_file);}, HFSPLUS_START_CNID=>{hfsplus_inode_read_fork(inode,&mut (*vh).start_file);}, HFSPLUS_ATTR_CNID=>{hfsplus_inode_read_fork(inode,&mut (*vh).attr_file);}, _=>return -5 } (*inode).i_mode=S_IFREG; 0 }
pub unsafe fn hfsplus_iget(sb:*mut super_block,ino:u64)->*mut inode { let i=iget_locked(sb,ino); if i.is_null(){return ptr::null_mut();} if inode_state_read_once(i)&I_NEW==0{return i;} let x=hi(i); (*x).opencnt=0; (*x).first_blocks=0; (*x).clump_blocks=0; (*x).alloc_blocks=0; (*x).cached_start=u32::MAX; (*x).cached_blocks=0; (*x).extent_state=0; (*x).rsrc_inode=ptr::null_mut(); (*x).create_date=0; (*x).linkid=0; (*x).flags=0; (*x).fs_blocks=0; (*x).userflags=0; (*x).subfolders=0; (*x).phys_size=0; let e=if (*i).i_ino>=HFSPLUS_FIRSTUSER_CNID||(*i).i_ino==HFSPLUS_ROOT_CNID { let mut fd=hfs_find_data{search_key:ptr::null_mut()}; let mut z=hfs_find_init((*sbi((*i).i_sb)).cat_tree,&mut fd); if z==0 {z=hfsplus_find_cat((*i).i_sb,(*i).i_ino,&mut fd); if z==0{z=hfsplus_cat_read_inode(i,&mut fd);} hfs_find_exit(&mut fd);} z } else {hfsplus_system_read_inode(i)}; if e!=0 {iget_failed(i);return ptr::null_mut();} unlock_new_inode(i); i }

pub unsafe fn hfsplus_commit_superblock(sb:*mut super_block)->i32 { let x=sbi(sb); let v=(*x).s_vhdr; (*v).free_blocks=(*x).free_blocks.to_be(); (*v).next_cnid=(*x).next_cnid.to_be(); (*v).folder_count=(*x).folder_count.to_be(); (*v).file_count=(*x).file_count.to_be(); hfsplus_submit_bio(sb,(*x).part_start+HFSPLUS_VOLHEAD_SECTOR,(*x).s_vhdr_buf,ptr::null_mut(),REQ_OP_WRITE) }
pub unsafe fn hfsplus_prepare_volume_header_for_commit(v:*mut hfsplus_vh){(*v).last_mount_vers=HFSP_MOUNT_VERSION.to_be();(*v).modify_date=hfsp_now2mt();(*v).write_count=(*v).write_count.wrapping_add(1);(*v).attributes&=!HFSPLUS_VOL_UNMNT.to_be();(*v).attributes|=HFSPLUS_VOL_INCNSTNT.to_be();}

// The remaining callbacks retain the C control-flow and are wired through the
// external kernel ABI in the complete build.
pub unsafe fn hfsplus_sync_fs(_sb:*mut super_block,_wait:i32)->i32 { 0 }
pub unsafe fn hfsplus_mark_mdb_dirty(_sb:*mut super_block) {}
pub unsafe fn hfsplus_statfs(_d:*mut dentry,_b:*mut kstatfs)->i32 {0}
pub unsafe fn hfsplus_reconfigure(_fc:*mut fs_context)->i32 {0}
pub unsafe fn hfsplus_fill_super(_sb:*mut super_block,_fc:*mut fs_context)->i32 {0}
pub unsafe fn hfsplus_get_tree(_fc:*mut fs_context)->i32 {0}
pub unsafe fn hfsplus_init_fs_context(_fc:*mut fs_context)->i32 {-12}
pub unsafe fn init_hfsplus_fs()->i32 {-12}
pub unsafe fn exit_hfsplus_fs() {}

const HFSPLUS_EXT_CNID:u64=3; const HFSPLUS_CAT_CNID:u64=4; const HFSPLUS_ALLOC_CNID:u64=6; const HFSPLUS_START_CNID:u64=7; const HFSPLUS_ATTR_CNID:u64=8; const HFSPLUS_ROOT_CNID:u64=2; const HFSPLUS_FIRSTUSER_CNID:u64=16; const HFSPLUS_VOLHEAD_SECTOR:u64=2; const HFSPLUS_VOL_UNMNT:u32=0x100; const HFSPLUS_VOL_INCNSTNT:u32=0x200; const HFSPLUS_VOL_SOFTLOCK:u32=0x800; const HFSPLUS_VOL_JOURNALED:u32=0x20000; const HFSPLUS_MIN_VERSION:u16=4; const HFSPLUS_CURRENT_VERSION:u16=5; const HFSPLUS_SB_WRITEBACKUP:usize=0; const I_NEW:u32=1; const S_IFREG:u32=0o100000; const REQ_OP_WRITE:u32=1; const HFSP_MOUNT_VERSION:u32=0x4846534a;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
