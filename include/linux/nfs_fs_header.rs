/* SPDX-License-Identifier: GPL-2.0 */
/* Rust source-level translation of linux/nfs_fs.h. */

/* Included kernel types and symbols are supplied by the surrounding translation. */

pub const NFS_MAX_TRANSPORTS: u32 = 16;
pub const NFS_DIR_VERIFIER_SIZE: usize = 2;

#[repr(C)]
pub struct nfs_access_entry { pub rb_node: rb_node, pub lru: list_head, pub fsuid: kuid_t, pub fsgid: kgid_t, pub group_info: *mut group_info, pub timestamp: u64, pub mask: __u32, pub rcu_head: rcu_head }
#[repr(C)]
pub struct nfs_lock_context { pub count: refcount_t, pub list: list_head, pub open_context: *mut nfs_open_context, pub lockowner: fl_owner_t, pub io_count: atomic_t, pub rcu_head: rcu_head }
#[repr(C)]
pub struct nfs_file_localio { pub ro_file: *mut nfsd_file, pub rw_file: *mut nfsd_file, pub list: list_head, pub nfs_uuid: *mut c_void }

#[inline]
pub unsafe fn nfs_localio_file_init(nfl: *mut nfs_file_localio) {
    /* IS_ENABLED(CONFIG_NFS_LOCALIO) is retained as a build-time dependency. */
    (*nfl).ro_file = core::ptr::null_mut(); (*nfl).rw_file = core::ptr::null_mut();
    INIT_LIST_HEAD(&mut (*nfl).list); (*nfl).nfs_uuid = core::ptr::null_mut();
}

pub struct nfs4_state;
#[repr(C)]
pub struct nfs_open_context {
    pub lock_context: nfs_lock_context, pub flock_owner: fl_owner_t, pub dentry: *mut dentry,
    pub cred: *const cred, pub ll_cred: *mut rpc_cred, pub state: *mut nfs4_state, pub mode: fmode_t,
    pub error: c_int, pub flags: c_ulong, pub mdsthreshold: *mut nfs4_threshold, pub list: list_head,
    pub rcu_head: rcu_head, pub nfl: nfs_file_localio,
}
pub const NFS_CONTEXT_BAD: u32 = 2; pub const NFS_CONTEXT_UNLOCK: u32 = 3;
pub const NFS_CONTEXT_FILE_OPEN: u32 = 4; pub const NFS_CONTEXT_WRITE_SYNC: u32 = 5; pub const NFS_CONTEXT_O_DIRECT: u32 = 6;
#[repr(C)]
pub struct nfs_open_dir_context { pub list: list_head, pub cache_hits: atomic_t, pub cache_misses: atomic_t, pub attr_gencount: c_ulong, pub verf: [__be32; NFS_DIR_VERIFIER_SIZE], pub dir_cookie: __u64, pub last_cookie: __u64, pub page_index: pgoff_t, pub dtsize: c_uint, pub force_clear: bool, pub eof: bool, pub rcu_head: rcu_head }
pub struct nfs_delegation; pub struct posix_acl; pub struct nfs4_xattr_cache;

#[repr(C)]
pub struct nfs_inode {
    pub fh: nfs_fh, pub flags: c_ulong, pub cache_validity: c_ulong, pub btime: timespec64,
    pub uncacheable_file_data: bool, pub read_cache_jiffies: c_ulong, pub attrtimeo: c_ulong,
    pub attrtimeo_timestamp: c_ulong, pub attr_gencount: c_ulong, pub access_cache: rb_root,
    pub access_cache_entry_lru: list_head, pub access_cache_inode_lru: list_head,
    pub directory: nfs_inode_directory, pub regular: nfs_inode_regular, pub open_files: list_head,
    pub ooo: *mut nfs_ooo, pub nfs4_acl: *mut nfs4_cached_acl, pub open_states: list_head,
    pub delegation: *mut nfs_delegation, pub rwsem: rw_semaphore, pub layout: *mut pnfs_layout_hdr,
    pub write_io: __u64, pub read_io: __u64, pub xattr_cache: *mut nfs4_xattr_cache, pub vfs_inode: inode,
}
#[repr(C)] pub struct nfs_inode_directory { pub cache_change_attribute: c_ulong, pub cookieverf: [__be32; NFS_DIR_VERIFIER_SIZE], pub rmdir_sem: rw_semaphore }
#[repr(C)] pub struct nfs_inode_regular { pub nrequests: atomic_long_t, pub redirtied_pages: atomic_long_t, pub commit_info: nfs_mds_commit_info, pub commit_mutex: mutex }
#[repr(C)] pub struct nfs_ooo { pub cnt: c_int, pub gap: [nfs_ooo_gap; 16] }
#[repr(C)] pub struct nfs_ooo_gap { pub start: u64, pub end: u64 }
#[repr(C)] pub struct nfs4_copy_state { pub copies: list_head, pub src_copies: list_head, pub stateid: nfs4_stateid, pub completion: completion, pub count: u64, pub verf: nfs_writeverf, pub error: c_int, pub flags: c_int, pub parent_src_state: *mut nfs4_state, pub parent_dst_state: *mut nfs4_state }

pub const NFS_ACCESS_READ: u32=0x0001; pub const NFS_ACCESS_LOOKUP:u32=0x0002; pub const NFS_ACCESS_MODIFY:u32=0x0004; pub const NFS_ACCESS_EXTEND:u32=0x0008; pub const NFS_ACCESS_DELETE:u32=0x0010; pub const NFS_ACCESS_EXECUTE:u32=0x0020; pub const NFS_ACCESS_XAREAD:u32=0x0040; pub const NFS_ACCESS_XAWRITE:u32=0x0080; pub const NFS_ACCESS_XALIST:u32=0x0100;
pub const NFS_INO_STALE: u32=1; pub const NFS_INO_ACL_LRU_SET:u32=2; pub const NFS_INO_INVALIDATING:u32=3; pub const NFS_INO_PRESERVE_UNLINKED:u32=4; pub const NFS_INO_LAYOUTCOMMIT:u32=9; pub const NFS_INO_LAYOUTCOMMITTING:u32=10; pub const NFS_INO_LAYOUTSTATS:u32=11; pub const NFS_INO_ODIRECT:u32=12; pub const NFS_INO_REQ_DIR_DELEG:u32=13;
pub const NFS_INO_INVALID_DATA:u64=1<<1; pub const NFS_INO_INVALID_ATIME:u64=1<<2; pub const NFS_INO_INVALID_ACCESS:u64=1<<3; pub const NFS_INO_INVALID_ACL:u64=1<<4; pub const NFS_INO_REVAL_FORCED:u64=1<<6; pub const NFS_INO_INVALID_LABEL:u64=1<<7; pub const NFS_INO_INVALID_CHANGE:u64=1<<8; pub const NFS_INO_INVALID_CTIME:u64=1<<9; pub const NFS_INO_INVALID_MTIME:u64=1<<10; pub const NFS_INO_INVALID_SIZE:u64=1<<11; pub const NFS_INO_INVALID_OTHER:u64=1<<12; pub const NFS_INO_DATA_INVAL_DEFER:u64=1<<13; pub const NFS_INO_INVALID_BLOCKS:u64=1<<14; pub const NFS_INO_INVALID_XATTR:u64=1<<15; pub const NFS_INO_INVALID_NLINK:u64=1<<16; pub const NFS_INO_INVALID_MODE:u64=1<<17; pub const NFS_INO_INVALID_BTIME:u64=1<<18; pub const NFS_INO_INVALID_UNCACHEABLE_FILE_DATA:u64=1<<19;
pub const NFS_INO_INVALID_ATTR:u64=NFS_INO_INVALID_CHANGE|NFS_INO_INVALID_CTIME|NFS_INO_INVALID_MTIME|NFS_INO_INVALID_BTIME|NFS_INO_INVALID_SIZE|NFS_INO_INVALID_NLINK|NFS_INO_INVALID_MODE|NFS_INO_INVALID_OTHER;

extern "C" { pub fn nfs_sync_mapping(mapping:*mut address_space)->c_int; pub fn nfs_zap_mapping(inode:*mut inode,mapping:*mut address_space); pub fn nfs_zap_caches(inode:*mut inode); pub fn nfs_set_inode_stale(inode:*mut inode); pub fn nfs_invalidate_atime(inode:*mut inode); pub fn nfs_fhget(sb:*mut super_block,fh:*mut nfs_fh,fattr:*mut nfs_fattr)->*mut inode; pub fn nfs_ilookup(sb:*mut super_block,fattr:*mut nfs_fattr,fh:*mut nfs_fh)->*mut inode; pub fn nfs_refresh_inode(inode:*mut inode,fattr:*mut nfs_fattr)->c_int; pub fn nfs_post_op_update_inode(inode:*mut inode,fattr:*mut nfs_fattr)->c_int; pub fn nfs_open(inode:*mut inode,file:*mut file)->c_int; pub fn nfs_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr)->c_int; pub fn nfs_alloc_fattr()->*mut nfs_fattr; pub fn nfs_alloc_fhandle()->*mut nfs_fh; }

#[inline] pub unsafe fn NFS_I(inode:*const inode)->*mut nfs_inode { container_of(inode, core::mem::offset_of!(nfs_inode,vfs_inode)) }
#[inline] pub unsafe fn NFS_FH(inode:*const inode)->*mut nfs_fh { &mut (*NFS_I(inode)).fh }
#[inline] pub unsafe fn nfs_size_to_loff_t(size:__u64)->loff_t { core::cmp::min(size, OFFSET_MAX as u64) as loff_t }
#[inline] pub unsafe fn nfs_ooo_clear(nfsi:*mut nfs_inode) { (*nfsi).cache_validity &= !NFS_INO_DATA_INVAL_DEFER; kfree((*nfsi).ooo as *mut c_void); (*nfsi).ooo=core::ptr::null_mut(); }
#[inline] pub unsafe fn nfs_ooo_test(nfsi:*mut nfs_inode)->bool { ((*nfsi).cache_validity&NFS_INO_DATA_INVAL_DEFER)!=0 || (!(*nfsi).ooo.is_null() && (*(*nfsi).ooo).cnt>0) }
pub const NFS_JUKEBOX_RETRY_TIME: u64 = 5 * HZ as u64;
pub const NFS_FSDATA_BLOCKED: *mut c_void = 1 as *mut c_void;

/* Remaining declarations are intentionally external: their definitions live in the
 * corresponding kernel translation units and are not part of this header. */
extern "C" {
    pub fn nfs_free_fattr(fattr: *const nfs_fattr);
    pub fn nfs_free_fhandle(fh: *const nfs_fh);
    pub fn nfs_register_sysctl() -> c_int;
    pub fn nfs_unregister_sysctl();
    pub fn nfs_file_direct_read(iocb: *mut kiocb, iter: *mut iov_iter, swap: bool) -> ssize_t;
    pub fn nfs_file_direct_write(iocb: *mut kiocb, iter: *mut iov_iter, swap: bool) -> ssize_t;
    pub fn nfs_sync_inode(inode: *mut inode) -> c_int;
    pub fn nfs_wb_all(inode: *mut inode) -> c_int;
    pub fn nfs_read_folio(file: *mut file, folio: *mut folio) -> c_int;
    pub fn nfs_readahead(rac: *mut readahead_control);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
