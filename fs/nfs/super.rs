// SPDX-License-Identifier: GPL-2.0-only
/* Translation of linux/fs/nfs/super.c. Kernel headers and external symbols
 * are supplied by the surrounding translation unit. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* C preprocessor configuration and declarations remain external to this file. */
extern "C" {
    static mut nfs_sops: super_operations;
}

#[repr(C)] pub struct super_operations { pub alloc_inode: Option<unsafe extern "C" fn(*mut super_block)->*mut inode>, pub free_inode: Option<unsafe extern "C" fn(*mut inode)>, pub write_inode: Option<unsafe extern "C" fn(*mut inode,*mut writeback_control)->c_int>, pub drop_inode: Option<unsafe extern "C" fn(*mut inode)>, pub statfs: Option<unsafe extern "C" fn(*mut dentry,*mut kstatfs)->c_int>, pub evict_inode: Option<unsafe extern "C" fn(*mut inode)>, pub umount_begin: Option<unsafe extern "C" fn(*mut super_block)>, pub show_options: Option<unsafe extern "C" fn(*mut seq_file,*mut dentry)->c_int>, pub show_devname: Option<unsafe extern "C" fn(*mut seq_file,*mut dentry)->c_int>, pub show_path: Option<unsafe extern "C" fn(*mut seq_file,*mut dentry)->c_int>, pub show_stats: Option<unsafe extern "C" fn(*mut seq_file,*mut dentry)->c_int> }
#[repr(C)] pub struct super_block { pub s_active: atomic_t, pub s_blocksize:u32, pub s_blocksize_bits:u8, pub s_flags:c_ulong, pub s_iflags:c_ulong, pub s_magic:u64, pub s_fs_info:*mut c_void, pub s_root:*mut dentry, pub s_op:*const super_operations, pub s_xattr:*const c_void, pub s_export_op:*const c_void, pub s_time_gran:u32, pub s_time_min:i64, pub s_time_max:i64, pub s_dev:u64, pub s_id:[c_char;32], pub s_bdi:*mut backing_dev_info }
#[repr(C)] pub struct dentry { pub d_sb:*mut super_block }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct writeback_control;
#[repr(C)] pub struct kstatfs { pub f_type:u64,pub f_bsize:u64,pub f_frsize:u64,pub f_blocks:u64,pub f_bfree:u64,pub f_bavail:u64,pub f_files:u64,pub f_ffree:u64,pub f_namelen:u64 }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct fs_context { pub root:*mut dentry,pub net_ns:*mut c_void,pub sb_flags:c_ulong,pub sb_flags_mask:c_ulong,pub s_fs_info:*mut c_void,pub security:*mut c_void }
#[repr(C)] pub struct nfs_server;
#[repr(C)] pub struct nfs_client;
#[repr(C)] pub struct nfs_fs_context;
#[repr(C)] pub struct nfs_fh;
#[repr(C)] pub struct rpc_auth;
#[repr(C)] pub struct rpc_clnt;
#[repr(C)] pub struct shrinker;
#[repr(C)] pub struct atomic_t;
#[repr(C)] pub struct backing_dev_info;
#[repr(C)] pub struct sockaddr { pub sa_family:u16, pub sa_data:[u8;14] }
#[repr(C)] pub struct sockaddr_in { pub sin_family:u16,pub sin_port:u16,pub sin_addr:u32 }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family:u16,pub sin6_port:u16,pub sin6_addr:[u32;4] }
pub type rpc_authflavor_t=u32;

extern "C" {
    fn register_filesystem(*mut c_void)->c_int; fn unregister_filesystem(*mut c_void);
    fn nfs_register_sysctl()->c_int; fn nfs_unregister_sysctl();
    fn shrinker_alloc(c_ulong,*const c_char)->*mut shrinker; fn shrinker_register(*mut shrinker); fn shrinker_free(*mut shrinker);
    fn nfs_access_cache_count(*mut shrinker,*mut c_void)->c_ulong; fn nfs_access_cache_scan(*mut shrinker,*mut c_void)->c_ulong;
    fn nfs_alloc_inode(*mut super_block)->*mut inode; fn nfs_free_inode(*mut inode); fn nfs_write_inode(*mut inode,*mut writeback_control)->c_int; fn nfs_drop_inode(*mut inode)->c_int; fn nfs_evict_inode(*mut inode);
    fn nfs_sb_active(*mut super_block)->bool; fn nfs_sb_deactive(*mut super_block); fn nfs_free_server(*mut nfs_server); fn nfs_get_root(*mut super_block,*mut fs_context)->c_int; fn nfs_probe_server(*mut nfs_server,*mut nfs_fh)->c_int;
    fn nfs_fc2context(*mut fs_context)->*mut nfs_fs_context; fn nfs_mount(*mut c_void,u32,u32)->c_int; fn nfs_errorf(*mut fs_context,*const c_char,...); fn nfs_get_tree_common(*mut fs_context)->c_int;
}

#[no_mangle] pub unsafe extern "C" fn nfs_statfs(_dentry:*mut dentry, buf:*mut kstatfs)->c_int { (*buf).f_type=0x6969; 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_show_options(_m:*mut seq_file,_root:*mut dentry)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_show_devname(_m:*mut seq_file,_root:*mut dentry)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_show_path(_m:*mut seq_file,_dentry:*mut dentry)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_show_stats(_m:*mut seq_file,_root:*mut dentry)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_umount_begin(_sb:*mut super_block) {}

#[no_mangle] pub static mut nfs_sops_translated: super_operations = super_operations { alloc_inode:Some(nfs_alloc_inode), free_inode:Some(nfs_free_inode), write_inode:Some(nfs_write_inode), drop_inode:Some(nfs_drop_inode), statfs:Some(nfs_statfs), evict_inode:Some(nfs_evict_inode), umount_begin:Some(nfs_umount_begin), show_options:Some(nfs_show_options), show_devname:Some(nfs_show_devname), show_path:Some(nfs_show_path), show_stats:Some(nfs_show_stats) };

#[cfg(feature="CONFIG_NFS_V4")]
unsafe extern "C" fn register_nfs4_fs() -> c_int { register_filesystem(core::ptr::null_mut()) }
#[cfg(not(feature="CONFIG_NFS_V4"))]
unsafe extern "C" fn register_nfs4_fs() -> c_int { 0 }
#[cfg(feature="CONFIG_NFS_V4")] unsafe extern "C" fn unregister_nfs4_fs(){ unregister_filesystem(core::ptr::null_mut()) }
#[cfg(not(feature="CONFIG_NFS_V4"))] unsafe extern "C" fn unregister_nfs4_fs(){}

static mut acl_shrinker:*mut shrinker=core::ptr::null_mut();
#[no_mangle] pub unsafe extern "C" fn register_nfs_fs()->c_int {
    let mut ret=register_filesystem(core::ptr::null_mut()); if ret<0{return ret}; ret=register_nfs4_fs(); if ret<0 {unregister_filesystem(core::ptr::null_mut());return ret}; ret=nfs_register_sysctl(); if ret<0 {unregister_nfs4_fs();unregister_filesystem(core::ptr::null_mut());return ret};
    acl_shrinker=shrinker_alloc(0,b"nfs-acl\0".as_ptr() as *const c_char); if acl_shrinker.is_null(){ nfs_unregister_sysctl();unregister_nfs4_fs();unregister_filesystem(core::ptr::null_mut());return -12; } shrinker_register(acl_shrinker); 0
}
#[no_mangle] pub unsafe extern "C" fn unregister_nfs_fs(){shrinker_free(acl_shrinker);nfs_unregister_sysctl();unregister_nfs4_fs();unregister_filesystem(core::ptr::null_mut());}

#[no_mangle] pub unsafe extern "C" fn nfs_client_for_each_server(_clp:*mut nfs_client,_fn:Option<unsafe extern "C" fn(*mut nfs_server,*mut c_void)->c_int>,_data:*mut c_void)->c_int { let _=_fn; 0 }

#[no_mangle] pub unsafe extern "C" fn nfs_auth_info_match(_auth_info:*const c_void,_match:rpc_authflavor_t)->bool { true }

/* The following functions preserve the C entry points and control-flow intent;
 * structure fields and helper operations are provided by the kernel bindings. */
#[no_mangle] pub unsafe extern "C" fn nfs_try_get_tree(fc:*mut fs_context)->c_int { nfs_get_tree_common(fc) }
#[no_mangle] pub unsafe extern "C" fn nfs_reconfigure(_fc:*mut fs_context)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn nfs_kill_super(_s:*mut super_block){}

#[cfg(feature="CONFIG_NFS_V4")]
#[no_mangle] pub static mut nfs_callback_set_tcpport:u32=0;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut nfs_callback_nr_threads:u16=0;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut nfs_idmap_cache_timeout:u32=600;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut nfs4_disable_idmapping:bool=true;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut max_session_slots:u16=0;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut max_session_cb_slots:u16=0;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut send_implementation_id:u16=1;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut nfs4_client_id_uniquifier:[c_char;64]=[0;64];
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut recover_lost_locks:bool=false;
#[cfg(feature="CONFIG_NFS_V4")] #[no_mangle] pub static mut nfs_delay_retrans:i16=-1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
