// SPDX-License-Identifier: GPL-2.0+
/* NILFS module and super block management.  Linux dependencies are supplied
 * by the surrounding translation unit. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct super_block { pub s_fs_info: *mut the_nilfs, pub s_flags: u64, pub s_magic: u32, pub s_blocksize: u32, pub s_bdev: *mut block_device, pub s_root: *mut dentry, pub s_op: *const super_operations, pub s_export_op: *const c_void, pub s_time_gran: u32, pub s_max_links: u32, pub s_id: [c_char; 32] }
#[repr(C)] pub struct block_device { pub bd_dev: u64 }
#[repr(C)] pub struct buffer_head { pub b_data: *mut u8, pub b_size: usize, pub b_blocknr: u64 }
#[repr(C)] pub struct inode { pub i_mode: u16, pub i_blocks: u64, pub i_size: i64 }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct kstatfs { pub f_type:u64,pub f_bsize:u64,pub f_blocks:u64,pub f_bfree:u64,pub f_bavail:u64,pub f_files:u64,pub f_ffree:u64,pub f_namelen:u32,pub f_fsid:u64 }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct fs_context { pub fs_private:*mut c_void,pub root:*mut dentry,pub sb_flags:u64,pub purpose:u32,pub source:*const c_char,pub ops:*const fs_context_operations }
#[repr(C)] pub struct fs_parameter { pub key:*const c_char,pub string:*const c_char }
#[repr(C)] pub struct fs_parse_result { pub negated:bool,pub uint_32:u32,pub uint_64:u64 }
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct nilfs_root { pub ifile:*mut inode,pub nilfs:*mut the_nilfs,pub cno:u64,pub inodes_count:u64 }
#[repr(C)] pub struct nilfs_inode_info { pub vfs_inode:inode,pub i_bh:*mut buffer_head,pub i_state:u64,pub i_type:u32,pub i_cno:u64,pub i_assoc_inode:*mut inode,pub i_bmap:*mut c_void,pub i_bmap_data:c_void,pub i_root:*mut nilfs_root }
#[repr(C)] pub struct nilfs_super_block { pub s_magic:u16,pub s_state:u16,pub s_max_mnt_count:u16,pub s_mnt_count:u16,pub s_wtime:u64,pub s_sum:u32,pub s_last_cno:u64,pub s_last_seq:u64,pub s_last_pseg:u64,pub s_free_blocks_count:u64,pub s_dev_size:u64,pub s_nsegments:u64,pub s_feature_incompat:u64,pub s_feature_compat_ro:u64,pub s_uuid:[u8;16],pub s_def_resuid:u16,pub s_def_resgid:u16,pub s_c_interval:u32,pub s_c_block_max:u32 }
#[repr(C)] pub struct the_nilfs { pub ns_sem:c_void,pub ns_segctor_sem:c_void,pub ns_snapshot_mount_mutex:c_void,pub ns_sbp:*mut *mut nilfs_super_block,pub ns_sbh:*mut *mut buffer_head,pub ns_sbsize:usize,pub ns_crc_seed:u32,pub ns_sbwtime:i64,pub ns_sbwcount:u64,pub ns_mount_state:u16,pub ns_flushed_device:i32,pub ns_first_data_block:u64,pub ns_blocksize:u64,pub ns_blocksize_bits:u32,pub ns_blocks_per_segment:u64,pub ns_nsegments:u64,pub ns_nrsvsegs:u64,pub ns_last_seq:u64,pub ns_last_pseg:u64,pub ns_last_cno:u64,pub ns_prot_seq:u64,pub ns_sufile:*mut inode,pub ns_cpfile:*mut inode,pub ns_dat:*mut inode,pub ns_inode_size:u32,pub ns_cno:u64,pub ns_mount_opt:usize,pub ns_resuid:u16,pub ns_resgid:u16,pub ns_interval:u32,pub ns_watermark:u32 }
#[repr(C)] pub struct super_operations { pub alloc_inode:Option<unsafe extern "C" fn(*mut super_block)->*mut inode>,pub free_inode:Option<unsafe extern "C" fn(*mut inode)>,pub put_super:Option<unsafe extern "C" fn(*mut super_block)>,pub sync_fs:Option<unsafe extern "C" fn(*mut super_block,c_int)->c_int>,pub statfs:Option<unsafe extern "C" fn(*mut dentry,*mut kstatfs)->c_int> }
#[repr(C)] pub struct fs_context_operations { pub parse_param:Option<unsafe extern "C" fn(*mut fs_context,*mut fs_parameter)->c_int>,pub get_tree:Option<unsafe extern "C" fn(*mut fs_context)->c_int>,pub reconfigure:Option<unsafe extern "C" fn(*mut fs_context)->c_int>,pub free:Option<unsafe extern "C" fn(*mut fs_context)> }

pub const NILFS_SUPER_MAGIC:u16=0x3434; pub const NILFS_VALID_FS:u16=1; pub const NILFS_ERROR_FS:u16=2; pub const NILFS_SB_COMMIT:c_int=0; pub const NILFS_SB_COMMIT_ALL:c_int=1; pub const SB_RDONLY:u64=1; pub const NILFS_CPTREE_CURRENT_CNO:u64=!0; pub const NILFS_ROOT_INO:u64=2; pub const NILFS_NAME_LEN:u32=255; pub const NILFS_MOUNT_ERRORS_RO:usize=1; pub const NILFS_MOUNT_BARRIER:usize=2; pub const NILFS_MOUNT_ERROR_MODE:usize=0xff; pub const BARRIER:usize=1; pub const STRICT_ORDER:usize=2; pub const NORECOVERY:usize=4; pub const DISCARD:usize=8;

extern "C" { fn printk(fmt:*const c_char,...); fn nilfs_prepare_super(*mut super_block,c_int)->*mut *mut nilfs_super_block; fn nilfs_commit_super(*mut super_block,c_int)->c_int; fn nilfs_sync_super(*mut super_block,c_int)->c_int; fn nilfs_count_free_blocks(*mut the_nilfs,*mut u64)->c_int; fn nilfs_set_log_cursor(*mut nilfs_super_block,*mut the_nilfs); fn nilfs_test_opt(*mut the_nilfs,usize)->bool; fn nilfs_cleanup_super(*mut super_block)->c_int; fn nilfs_construct_segment(*mut super_block)->c_int; fn nilfs_flush_device(*mut the_nilfs)->c_int; fn nilfs_attach_checkpoint(*mut super_block,u64,c_int,*mut *mut nilfs_root)->c_int; fn nilfs_put_root(*mut nilfs_root); fn nilfs_find_or_create_root(*mut the_nilfs,u64)->*mut nilfs_root; fn nilfs_ifile_read(*mut super_block,*mut nilfs_root,u64,u32)->c_int; fn nilfs_iget(*mut super_block,*mut nilfs_root,u64)->*mut inode; fn nilfs_ilookup(*mut super_block,*mut nilfs_root,u64)->*mut inode; fn nilfs_lookup_root(*mut the_nilfs,u64)->*mut nilfs_root; fn nilfs_last_cno(*mut the_nilfs)->u64; fn nilfs_alloc_inode(*mut super_block)->*mut inode; fn alloc_inode_sb(*mut super_block,*mut kmem_cache,usize)->*mut nilfs_inode_info; fn kmem_cache_free(*mut kmem_cache,*mut nilfs_inode_info); }
static mut nilfs_inode_cachep:*mut kmem_cache=core::ptr::null_mut(); pub static mut nilfs_transaction_cachep:*mut kmem_cache=core::ptr::null_mut(); pub static mut nilfs_segbuf_cachep:*mut kmem_cache=core::ptr::null_mut(); pub static mut nilfs_btree_path_cache:*mut kmem_cache=core::ptr::null_mut();

pub unsafe extern "C" fn nilfs_alloc_inode_impl(sb:*mut super_block)->*mut inode { let ii=alloc_inode_sb(sb,nilfs_inode_cachep,0); if ii.is_null(){return core::ptr::null_mut()} (*ii).i_bh=core::ptr::null_mut(); (*ii).i_state=0; (*ii).i_type=0; (*ii).i_cno=0; (*ii).i_assoc_inode=core::ptr::null_mut(); &mut (*ii).vfs_inode }
pub unsafe extern "C" fn nilfs_set_log_cursor_impl(sbp:*mut nilfs_super_block,nilfs:*mut the_nilfs) { let mut n=0; nilfs_count_free_blocks(nilfs,&mut n); (*sbp).s_free_blocks_count=n; (*sbp).s_last_seq=(*nilfs).ns_last_seq; (*sbp).s_last_pseg=(*nilfs).ns_last_pseg; (*sbp).s_last_cno=(*nilfs).ns_last_cno; }
pub unsafe extern "C" fn nilfs_prepare_super_impl(sb:*mut super_block,flip:c_int)->*mut *mut nilfs_super_block { let n=(*sb).s_fs_info; let p=(*n).ns_sbp; if p.is_null(){return core::ptr::null_mut()} let a=*p; if a.is_null() || (*a).s_magic!=NILFS_SUPER_MAGIC{return core::ptr::null_mut()} let _=flip; p }
pub unsafe extern "C" fn nilfs_cleanup_super_impl(sb:*mut super_block)->c_int { let n=(*sb).s_fs_info; let p=nilfs_prepare_super_impl(sb,0); if p.is_null(){return -5} (*(*p)).s_state=(*n).ns_mount_state; nilfs_set_log_cursor_impl(*p,n); nilfs_commit_super(sb,NILFS_SB_COMMIT) }
pub unsafe extern "C" fn nilfs_checkpoint_is_mounted(sb:*mut super_block,cno:u64)->bool { let n=(*sb).s_fs_info; if cno>(*n).ns_cno{return false} if cno>=nilfs_last_cno(n){return true} let r=nilfs_lookup_root(n,cno); if r.is_null(){return false} let i=nilfs_ilookup(sb,r,NILFS_ROOT_INO); let ret=!i.is_null(); nilfs_put_root(r); ret }

#[repr(C)] pub struct nilfs_fs_context { pub ns_mount_opt:usize,pub cno:u64 }
pub unsafe extern "C" fn nilfs_free_fc(fc:*mut fs_context){ let _=fc; }
pub static mut nilfs_sops:super_operations=super_operations{alloc_inode:Some(nilfs_alloc_inode_impl),free_inode:None,put_super:None,sync_fs:None,statfs:None};
pub static mut nilfs_context_ops:fs_context_operations=fs_context_operations{parse_param:None,get_tree:None,reconfigure:None,free:Some(nilfs_free_fc)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
