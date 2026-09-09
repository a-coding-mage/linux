/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of fuse_i.h. External kernel types/functions are supplied elsewhere. */

pub const FUSE_DEFAULT_MAX_PAGES_PER_REQ: u32 = 32;
pub const FUSE_NOWRITE: i32 = i32::MIN;
pub const FUSE_NAME_LOW_MAX: usize = 1024;
pub const FUSE_CTL_NUM_DENTRIES: usize = 5;

extern "C" {
    pub static mut inval_wq: ::core::ffi::c_uint;
    pub static mut fuse_max_pages_limit: ::core::ffi::c_uint;
    pub static mut fuse_conn_list: list_head;
    pub static mut fuse_mutex: mutex;
    pub static mut max_user_bgreq: ::core::ffi::c_uint;
    pub static mut max_user_congthresh: ::core::ffi::c_uint;
}

#[repr(C)] pub struct fuse_submount_lookup { pub count: refcount_t, pub nodeid: u64, pub forget: *mut fuse_forget_link }
#[repr(C)] pub struct fuse_backing { pub file: *mut file, pub cred: *const cred, pub count: refcount_t, pub rcu: rcu_head }

#[repr(C)]
pub struct fuse_inode {
    pub inode: inode, pub nodeid: u64, pub nlookup: u64, pub forget: *mut fuse_forget_link,
    pub i_time: u64, pub inval_mask: u32, pub orig_i_mode: umode_t, pub i_btime: timespec64,
    pub orig_ino: u64, pub attr_version: u64,
    pub data: fuse_inode_data, pub state: ::core::ffi::c_ulong, pub mutex: mutex, pub lock: spinlock_t,
    #[cfg(CONFIG_FUSE_DAX)] pub dax: *mut fuse_inode_dax,
    pub submount_lookup: *mut fuse_submount_lookup,
    #[cfg(CONFIG_FUSE_PASSTHROUGH)] pub fb: *mut fuse_backing,
    pub cached_i_blkbits: u8,
}
#[repr(C)] pub union fuse_inode_data { pub rw: fuse_inode_rw, pub rdc: fuse_inode_rdc }
#[repr(C)] pub struct fuse_inode_rw { pub write_files: list_head, pub queued_writes: list_head, pub writectr: i32, pub iocachectr: i32, pub page_waitq: wait_queue_head_t, pub direct_io_waitq: wait_queue_head_t }
#[repr(C)] pub struct fuse_inode_rdc { pub cached: bool, pub size: loff_t, pub pos: loff_t, pub version: u64, pub mtime: timespec64, pub epoch: i32, pub iversion: u64, pub lock: spinlock_t }

pub const FUSE_I_ADVISE_RDPLUS: u32 = 0; pub const FUSE_I_INIT_RDPLUS: u32 = 1;
pub const FUSE_I_SIZE_UNSTABLE: u32 = 2; pub const FUSE_I_BAD: u32 = 3;
pub const FUSE_I_BTIME: u32 = 4; pub const FUSE_I_CACHE_IO_MODE: u32 = 5; pub const FUSE_I_EXCLUSIVE: u32 = 6;

#[repr(C)] pub struct fuse_file { pub fm: *mut fuse_mount, pub args: *mut fuse_file_args, pub kh: u64, pub fh: u64, pub nodeid: u64, pub count: refcount_t, pub open_flags: u32, pub write_entry: list_head, pub readdir: fuse_readdir_state, pub polled_node: rb_node, pub poll_wait: wait_queue_head_t, pub iomode: i32, #[cfg(CONFIG_FUSE_PASSTHROUGH)] pub passthrough: *mut file, #[cfg(CONFIG_FUSE_PASSTHROUGH)] pub cred: *const cred, pub flock: bool }
#[repr(C)] pub struct fuse_readdir_state { pub pos: loff_t, pub cache_off: loff_t, pub version: u64 }
#[repr(C)] pub struct fuse_release_args { pub args: fuse_args, pub inarg: fuse_release_in, pub inode: *mut inode }
#[repr(C)] pub union fuse_file_args { pub open_outarg: fuse_open_out, pub release_args: fuse_release_args }
#[repr(C)] pub struct fuse_io_priv { pub refcnt: kref, pub work: work_struct, pub async_: i32, pub lock: spinlock_t, pub reqs: u32, pub bytes: ssize_t, pub size: usize, pub offset: u64, pub write: bool, pub should_dirty: bool, pub err: i32, pub iocb: *mut kiocb, pub done: *mut completion, pub blocking: bool }

#[repr(C)] pub enum fuse_dax_mode { FUSE_DAX_INODE_DEFAULT, FUSE_DAX_ALWAYS, FUSE_DAX_NEVER, FUSE_DAX_INODE_USER }
#[inline] pub unsafe fn fuse_is_inode_dax_mode(mode: fuse_dax_mode) -> bool { matches!(mode, fuse_dax_mode::FUSE_DAX_INODE_DEFAULT | fuse_dax_mode::FUSE_DAX_INODE_USER) }
#[repr(C)] pub struct fuse_fs_context { pub fud: *mut fuse_dev, pub rootmode: u32, pub user_id: kuid_t, pub group_id: kgid_t, pub is_bdev: bool, pub rootmode_present: bool, pub user_id_present: bool, pub group_id_present: bool, pub default_permissions: bool, pub allow_other: bool, pub destroy: bool, pub no_control: bool, pub no_force_umount: bool, pub legacy_opts_show: bool, pub dax_mode: fuse_dax_mode, pub max_read: u32, pub blksize: u32, pub subtype: *const ::core::ffi::c_char, pub dax_dev: *mut dax_device }
#[repr(C)] pub struct fuse_sync_bucket { pub count: atomic_t, pub waitq: wait_queue_head_t, pub rcu: rcu_head }

#[repr(C)] pub struct fuse_conn {
 pub lock: spinlock_t, pub count: refcount_t, pub epoch: atomic_t, pub epoch_work: work_struct, pub rcu: rcu_head,
 pub user_id: kuid_t, pub group_id: kgid_t, pub pid_ns: *mut pid_namespace, pub user_ns: *mut user_namespace,
 pub max_read: ::core::ffi::c_uint, pub max_write: ::core::ffi::c_uint, pub max_pages: u32, pub max_pages_limit: u32, pub chan: *mut fuse_chan, pub khctr: atomic64_t, pub polled_files: rb_root, pub congestion_threshold: ::core::ffi::c_uint,
 pub flags: u64, pub max_stack_depth: i32, pub minor: ::core::ffi::c_uint, pub entry: list_head, pub dev: dev_t, pub scramble_key: [u32;4], pub attr_version: atomic64_t, pub evict_ctr: atomic64_t, pub name_max: u32, pub release: Option<unsafe extern "C" fn(*mut fuse_conn)>, pub killsb: rw_semaphore,
 #[cfg(CONFIG_FUSE_DAX)] pub dax_mode: fuse_dax_mode, #[cfg(CONFIG_FUSE_DAX)] pub dax: *mut fuse_conn_dax,
 pub mounts: list_head, pub curr_bucket: *mut fuse_sync_bucket, #[cfg(CONFIG_FUSE_PASSTHROUGH)] pub backing_files_map: idr,
}
#[repr(C)] pub struct fuse_mount { pub fc: *mut fuse_conn, pub sb: *mut super_block, pub fc_entry: list_head, pub rcu: rcu_head }
#[repr(C)] pub struct fuse_zero_header;

#[inline] pub unsafe fn fuse_set_zero_arg0(args: *mut fuse_args) { (*args).in_args[0].size = core::mem::size_of::<fuse_zero_header>(); (*args).in_args[0].value = core::ptr::null_mut(); }
#[inline] pub unsafe fn get_fuse_mount_super(sb: *mut super_block) -> *mut fuse_mount { (*sb).s_fs_info as *mut fuse_mount }
#[inline] pub unsafe fn get_fuse_conn_super(sb: *mut super_block) -> *mut fuse_conn { (*get_fuse_mount_super(sb)).fc }
#[inline] pub unsafe fn get_fuse_mount(inode: *mut inode) -> *mut fuse_mount { get_fuse_mount_super((*inode).i_sb) }
#[inline] pub unsafe fn get_fuse_conn(inode: *mut inode) -> *mut fuse_conn { (*get_fuse_mount_super((*inode).i_sb)).fc }
#[inline] pub unsafe fn get_fuse_inode(inode: *const inode) -> *mut fuse_inode { container_of(inode as *mut _, core::mem::offset_of!(fuse_inode,inode)) }
#[inline] pub unsafe fn get_node_id(inode: *mut inode) -> u64 { (*get_fuse_inode(inode)).nodeid }
#[inline] pub fn invalid_nodeid(nodeid: u64) -> bool { nodeid == 0 || nodeid == FUSE_ROOT_ID }

pub const FUSE_STATX_MODIFY: u32 = STATX_MTIME | STATX_CTIME | STATX_BLOCKS;
pub const FUSE_STATX_MODSIZE: u32 = FUSE_STATX_MODIFY | STATX_SIZE;
pub const FUSE_STATX_MODDIR: u32 = FUSE_STATX_MODSIZE | STATX_NLINK;
pub const FUSE_DIO_WRITE: i32 = 1; pub const FUSE_DIO_CUSE: i32 = 2;

#[repr(C)] pub struct fuse_io_args { pub data: fuse_io_args_data, pub ap: fuse_args_pages, pub io: *mut fuse_io_priv, pub ff: *mut fuse_file }
#[repr(C)] pub union fuse_io_args_data { pub read: fuse_read_args, pub write: fuse_write_args }
#[repr(C)] pub struct fuse_read_args { pub in_: fuse_read_in, pub attr_ver: u64 }
#[repr(C)] pub struct fuse_write_args { pub in_: fuse_write_in, pub out: fuse_write_out, pub folio_locked: bool }

extern "C" {
    pub fn fuse_iget(sb:*mut super_block,nodeid:u64,generation:i32,attr:*mut fuse_attr,attr_valid:u64,attr_version:u64,evict_ctr:u64)->*mut inode;
    pub fn fuse_lookup_name(sb:*mut super_block,nodeid:u64,name:*const qstr,outarg:*mut fuse_entry_out,inode:*mut *mut inode)->i32;
    pub fn fuse_read_args_fill(ia:*mut fuse_io_args,file:*mut file,pos:loff_t,count:usize,opcode:i32);
    pub fn fuse_file_alloc(fm:*mut fuse_mount,release:bool)->*mut fuse_file; pub fn fuse_file_free(ff:*mut fuse_file); pub fn fuse_finish_open(inode:*mut inode,file:*mut file)->i32;
    pub fn fuse_release_common(file:*mut file,isdir:bool); pub fn fuse_fsync_common(file:*mut file,start:loff_t,end:loff_t,datasync:i32,opcode:i32)->i32;
    pub fn fuse_invalidate_attr(inode:*mut inode); pub fn fuse_invalidate_attr_mask(inode:*mut inode,mask:u32);
    pub fn fuse_time_to_jiffies(sec:u64,nsec:u32)->u64;
    pub fn fuse_conn_init(fc:*mut fuse_conn,fm:*mut fuse_mount,user_ns:*mut user_namespace,fch:*mut fuse_chan);
    pub fn fuse_send_init(fm:*mut fuse_mount)->i32; pub fn fuse_conn_destroy(fm:*mut fuse_mount); pub fn fuse_mount_destroy(fm:*mut fuse_mount);
    pub fn fuse_valid_type(m:i32)->i32; pub fn fuse_invalid_attr(attr:*mut fuse_attr)->bool; pub fn fuse_free_conn(fc:*mut fuse_conn);
    pub fn fuse_direct_io(io:*mut fuse_io_priv,iter:*mut iov_iter,ppos:*mut loff_t,flags:i32)->ssize_t;
    pub fn fuse_passthrough_open(file:*mut file,backing_id:i32)->*mut fuse_backing;
}

// Remaining declarations are intentionally represented as external kernel ABI symbols.
extern "C" {
    pub fn fuse_change_attributes(inode:*mut inode,attr:*mut fuse_attr,sx:*mut fuse_statx,valid:u64,version:u64);
    pub fn fuse_update_attributes(inode:*mut inode,file:*mut file,mask:u32)->i32;
    pub fn fuse_setxattr(inode:*mut inode,name:*const ::core::ffi::c_char,value:*const ::core::ffi::c_void,size:usize,flags:i32,extra_flags:u32)->i32;
    pub fn fuse_getxattr(inode:*mut inode,name:*const ::core::ffi::c_char,value:*mut ::core::ffi::c_void,size:usize)->ssize_t;
    pub fn fuse_listxattr(entry:*mut dentry,list:*mut ::core::ffi::c_char,size:usize)->ssize_t;
    pub fn fuse_removexattr(inode:*mut inode,name:*const ::core::ffi::c_char)->i32;
    pub fn fuse_sync_release(fi:*mut fuse_inode,ff:*mut fuse_file,flags:u32);
    pub fn fuse_notify_poll_wakeup(fc:*mut fuse_conn,outarg:*mut fuse_notify_poll_wakeup_out)->i32;
    pub fn fuse_init_file_inode(inode:*mut inode,flags:u32); pub fn fuse_init_common(inode:*mut inode); pub fn fuse_init_dir(inode:*mut inode); pub fn fuse_init_symlink(inode:*mut inode);
    pub fn fuse_get_cache_mask(inode:*mut inode)->u32; pub fn fuse_ctl_init()->i32; pub fn fuse_ctl_cleanup();
    pub fn __fuse_simple_request(idmap:*mut mnt_idmap,fm:*mut fuse_mount,args:*mut fuse_args)->ssize_t;
    pub fn fuse_simple_background(fm:*mut fuse_mount,args:*mut fuse_args,gfp_flags:gfp_t)->i32;
    pub fn fuse_simple_notify_reply(fm:*mut fuse_mount,args:*mut fuse_args,unique:u64)->i32;
    pub fn fuse_dentry_tree_init(); pub fn fuse_dentry_tree_cleanup(); pub fn fuse_epoch_work(work:*mut work_struct);
    pub fn fuse_invalidate_entry_cache(entry:*mut dentry); pub fn fuse_invalidate_atime(inode:*mut inode); pub fn fuse_change_entry_timeout(entry:*mut dentry,o:*mut fuse_entry_out); pub fn fuse_dentry_set_epoch(dentry:*mut dentry,epoch:u64);
    pub fn fuse_fill_super_common(sb:*mut super_block,ctx:*mut fuse_fs_context)->i32; pub fn fuse_mount_remove(fm:*mut fuse_mount)->bool; pub fn fuse_init_fs_context_submount(fsc:*mut fs_context)->i32;
    pub fn fuse_ctl_add_conn(fc:*mut fuse_conn)->i32; pub fn fuse_ctl_remove_conn(fc:*mut fuse_conn);
    pub fn fuse_allow_current_process(fc:*mut fuse_conn)->bool; pub fn fuse_lock_owner_id(fc:*mut fuse_conn,id:fl_owner_t)->u64;
    pub fn fuse_flush_time_update(inode:*mut inode); pub fn fuse_update_ctime(inode:*mut inode); pub fn fuse_flush_writepages(inode:*mut inode); pub fn fuse_set_nowrite(inode:*mut inode); pub fn fuse_release_nowrite(inode:*mut inode);
    pub fn fuse_ilookup(fc:*mut fuse_conn,nodeid:u64,fm:*mut *mut fuse_mount)->*mut inode;
    pub fn fuse_reverse_inval_inode(fc:*mut fuse_conn,nodeid:u64,offset:loff_t,len:loff_t)->i32;
    pub fn fuse_reverse_inval_entry(fc:*mut fuse_conn,parent:u64,child:u64,name:*mut qstr,flags:u32)->i32;
    pub fn fuse_try_prune_one_inode(fc:*mut fuse_conn,nodeid:u64); pub fn fuse_do_open(fm:*mut fuse_mount,nodeid:u64,file:*mut file,isdir:bool)->i32;
    pub fn fuse_do_ioctl(file:*mut file,cmd:u32,arg:usize,flags:u32)->c_long; pub fn fuse_ioctl_common(file:*mut file,cmd:u32,arg:usize,flags:u32)->c_long;
    pub fn fuse_write_update_attr(inode:*mut inode,pos:loff_t,written:ssize_t)->bool; pub fn fuse_flush_times(inode:*mut inode,ff:*mut fuse_file)->i32; pub fn fuse_write_inode(inode:*mut inode,wbc:*mut writeback_control)->i32;
    pub fn fuse_do_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr,file:*mut file)->i32; pub fn fuse_unlock_inode(inode:*mut inode,locked:bool); pub fn fuse_lock_inode(inode:*mut inode)->bool;
    pub fn fuse_readdir(file:*mut file,ctx:*mut dir_context)->i32;
    pub fn fuse_dax_read_iter(iocb:*mut kiocb,to:*mut iov_iter)->ssize_t; pub fn fuse_dax_write_iter(iocb:*mut kiocb,from:*mut iov_iter)->ssize_t; pub fn fuse_dax_mmap(file:*mut file,vma:*mut vm_area_struct)->i32;
    pub fn fuse_dax_break_layouts(inode:*mut inode,start:u64,end:u64)->i32; pub fn fuse_dax_conn_alloc(fc:*mut fuse_conn,mode:fuse_dax_mode,dev:*mut dax_device)->i32; pub fn fuse_dax_conn_free(fc:*mut fuse_conn);
    pub fn fuse_dax_inode_alloc(sb:*mut super_block,fi:*mut fuse_inode)->bool; pub fn fuse_dax_inode_init(inode:*mut inode,flags:u32); pub fn fuse_dax_inode_cleanup(inode:*mut inode); pub fn fuse_dax_dontcache(inode:*mut inode,flags:u32); pub fn fuse_dax_check_alignment(fc:*mut fuse_conn,map_alignment:u32)->bool; pub fn fuse_dax_cancel_work(fc:*mut fuse_conn);
    pub fn fuse_file_ioctl(file:*mut file,cmd:u32,arg:usize)->c_long; pub fn fuse_file_compat_ioctl(file:*mut file,cmd:u32,arg:usize)->c_long; pub fn fuse_fileattr_get(dentry:*mut dentry,fa:*mut file_kattr)->i32; pub fn fuse_fileattr_set(idmap:*mut mnt_idmap,dentry:*mut dentry,fa:*mut file_kattr)->i32;
    pub fn fuse_file_cached_io_open(inode:*mut inode,ff:*mut fuse_file)->i32; pub fn fuse_inode_uncached_io_start(fi:*mut fuse_inode,fb:*mut fuse_backing)->i32; pub fn fuse_inode_uncached_io_end(fi:*mut fuse_inode);
    pub fn fuse_file_io_open(file:*mut file,inode:*mut inode)->i32; pub fn fuse_file_io_release(ff:*mut fuse_file,inode:*mut inode);
    pub fn fuse_file_open(fm:*mut fuse_mount,nodeid:u64,flags:u32,isdir:bool)->*mut fuse_file; pub fn fuse_file_release(inode:*mut inode,ff:*mut fuse_file,flags:u32,id:fl_owner_t,isdir:bool);
    pub fn fuse_passthrough_release(ff:*mut fuse_file,fb:*mut fuse_backing);
    pub fn fuse_passthrough_read_iter(iocb:*mut kiocb,iter:*mut iov_iter)->ssize_t; pub fn fuse_passthrough_write_iter(iocb:*mut kiocb,iter:*mut iov_iter)->ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
