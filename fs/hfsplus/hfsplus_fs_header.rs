/* SPDX-License-Identifier: GPL-2.0 */
// Translation of linux/include/linux/hfsplus_fs.h. External kernel types are
// intentionally referenced but not defined here.

pub const HFSPLUS_DEF_CR_TYPE: u32 = 0x3F3F3F3F;
pub const HFSPLUS_TYPE_DATA: u8 = 0x00;
pub const HFSPLUS_TYPE_RSRC: u8 = 0xFF;
pub const NODE_HASH_SIZE: usize = 256;

pub type BtreeKeycmp = unsafe extern "C" fn(*const hfsplus_btree_key, *const hfsplus_btree_key) -> core::ffi::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hfsplus_btree_mutex_classes { CATALOG_BTREE_MUTEX, EXTENTS_BTREE_MUTEX, ATTR_BTREE_MUTEX }

#[repr(C)]
pub struct hfs_btree {
    pub sb: *mut super_block, pub inode: *mut inode, pub keycmp: Option<BtreeKeycmp>,
    pub cnid: u32, pub root: u32, pub leaf_count: u32, pub leaf_head: u32, pub leaf_tail: u32,
    pub node_count: u32, pub free_nodes: u32, pub attributes: u32,
    pub node_size: u32, pub node_size_shift: u32, pub max_key_len: u32, pub depth: u32,
    pub tree_lock: mutex, pub pages_per_bnode: u32, pub hash_lock: spinlock_t,
    pub node_hash: [*mut hfs_bnode; NODE_HASH_SIZE], pub node_hash_cnt: core::ffi::c_int,
}

#[repr(C)]
pub struct hfs_bnode {
    pub tree: *mut hfs_btree, pub prev: u32, pub this: u32, pub next: u32, pub parent: u32,
    pub num_recs: u16, pub type_: u8, pub height: u8, pub next_hash: *mut hfs_bnode,
    pub flags: usize, pub lock_wq: wait_queue_head_t, pub refcnt: atomic_t,
    pub page_offset: u32, pub page: *mut *mut page,
}
pub const HFS_BNODE_LOCK: usize = 0; pub const HFS_BNODE_ERROR: usize = 1;
pub const HFS_BNODE_NEW: usize = 2; pub const HFS_BNODE_DIRTY: usize = 3; pub const HFS_BNODE_DELETED: usize = 4;
pub const HFSPLUS_EMPTY_ATTR_TREE: usize = 0; pub const HFSPLUS_CREATING_ATTR_TREE: usize = 1;
pub const HFSPLUS_VALID_ATTR_TREE: usize = 2; pub const HFSPLUS_FAILED_ATTR_TREE: usize = 3;

#[repr(C)]
pub struct hfsplus_sb_info {
    pub s_vhdr_buf: *mut core::ffi::c_void, pub s_vhdr: *mut hfsplus_vh,
    pub s_backup_vhdr_buf: *mut core::ffi::c_void, pub s_backup_vhdr: *mut hfsplus_vh,
    pub ext_tree: *mut hfs_btree, pub cat_tree: *mut hfs_btree, pub attr_tree: *mut hfs_btree,
    pub attr_tree_state: atomic_t, pub alloc_file: *mut inode, pub hidden_dir: *mut inode, pub nls: *mut nls_table,
    pub blockoffset: u32, pub min_io_size: u32, pub part_start: sector_t, pub sect_count: sector_t, pub fs_shift: i32,
    pub alloc_blksz: u32, pub alloc_blksz_shift: i32, pub total_blocks: u32, pub data_clump_blocks: u32, pub rsrc_clump_blocks: u32,
    pub free_blocks: u32, pub alloc_mutex: mutex, pub next_cnid: u32, pub file_count: u32, pub folder_count: u32, pub vh_mutex: mutex,
    pub creator: u32, pub type_: u32, pub umask: umode_t, pub uid: kuid_t, pub gid: kgid_t,
    pub part: i32, pub session: i32, pub flags: usize, pub work_queued: i32, pub sync_work: delayed_work,
    pub work_lock: spinlock_t, pub rcu: rcu_head,
}
pub const HFSPLUS_SB_WRITEBACKUP: usize=0; pub const HFSPLUS_SB_NODECOMPOSE: usize=1; pub const HFSPLUS_SB_FORCE: usize=2;
pub const HFSPLUS_SB_HFSX: usize=3; pub const HFSPLUS_SB_CASEFOLD: usize=4; pub const HFSPLUS_SB_NOBARRIER: usize=5;
pub const HFSPLUS_SB_UID: usize=6; pub const HFSPLUS_SB_GID: usize=7;

#[inline] pub unsafe fn HFSPLUS_SB(sb: *mut super_block) -> *mut hfsplus_sb_info { (*sb).s_fs_info as *mut hfsplus_sb_info }

#[repr(C)]
pub struct hfsplus_inode_info {
    pub opencnt: atomic_t, pub first_blocks:u32, pub clump_blocks:u32, pub alloc_blocks:u32, pub cached_start:u32, pub cached_blocks:u32,
    pub first_extents: hfsplus_extent_rec, pub cached_extents: hfsplus_extent_rec, pub extent_state:u32, pub extents_lock: mutex,
    pub rsrc_inode:*mut inode, pub create_date: __be32, pub linkid:u32, pub flags:usize, pub fs_blocks:sector_t,
    pub userflags:u8, pub subfolders:u32, pub phys_size:loff_t, pub vfs_inode: inode,
}
pub const HFSPLUS_EXT_DIRTY:u16=1; pub const HFSPLUS_EXT_NEW:u16=2;
pub const HFSPLUS_I_RSRC:usize=0; pub const HFSPLUS_I_CAT_DIRTY:usize=1; pub const HFSPLUS_I_EXT_DIRTY:usize=2;
pub const HFSPLUS_I_ALLOC_DIRTY:usize=3; pub const HFSPLUS_I_ATTR_DIRTY:usize=4;
#[inline] pub unsafe fn HFSPLUS_I(i:*mut inode)->*mut hfsplus_inode_info { ((i as *mut u8).sub(core::mem::offset_of!(hfsplus_inode_info,vfs_inode))) as *mut hfsplus_inode_info }
#[inline] pub unsafe fn HFSPLUS_IS_RSRC(i:*mut inode)->bool { test_bit(HFSPLUS_I_RSRC, &mut (*HFSPLUS_I(i)).flags) != 0 }
#[inline] pub unsafe fn hfsplus_mark_inode_dirty(i:*mut inode, flag:usize) { set_bit(flag,&mut (*HFSPLUS_I(i)).flags); mark_inode_dirty(i); }

#[repr(C)] pub struct hfs_find_data { pub search_key:*mut hfsplus_btree_key, pub key:*mut hfsplus_btree_key, pub tree:*mut hfs_btree, pub bnode:*mut hfs_bnode, pub record:i32, pub keyoffset:i32, pub keylength:i32, pub entryoffset:i32, pub entrylength:i32 }
#[repr(C)] pub struct hfsplus_readdir_data { pub pos:loff_t, pub key:hfsplus_cat_key }
#[inline] pub unsafe fn hfsplus_min_io_size(sb:*mut super_block)->u16 { core::cmp::max((*HFSPLUS_SB(sb)).min_io_size as u16,HFSPLUS_SECTOR_SIZE as u16) }

pub const HFSPLUS_IOC_BLESS: u32 = _IO(b'h' as u32,0x80);
pub type SearchStrategyT = unsafe extern "C" fn(*mut hfs_bnode,*mut hfs_find_data,*mut i32,*mut i32,*mut i32)->i32;

// C macro compatibility aliases.
pub use hfsplus_btree_open as hfs_btree_open; pub use hfsplus_btree_close as hfs_btree_close;
pub use hfsplus_btree_write as hfs_btree_write; pub use hfsplus_bmap_reserve as hfs_bmap_reserve;
pub use hfsplus_bmap_alloc as hfs_bmap_alloc; pub use hfsplus_bmap_free as hfs_bmap_free;

pub const HFSPLUS_UTC_OFFSET:u32=2082844800;
#[inline] pub unsafe fn __hfsp_mt2ut(mt:__be32)->time64_t { (be32_to_cpu(mt).wrapping_sub(HFSPLUS_UTC_OFFSET)) as time64_t }
#[inline] pub unsafe fn __hfsp_ut2mt(ut:time64_t)->__be32 { cpu_to_be32((ut as u32).wrapping_add(HFSPLUS_UTC_OFFSET)) }
#[inline] pub unsafe fn is_hfs_thread_record_type(t:u16)->bool { t==HFSPLUS_FOLDER_THREAD || t==HFSPLUS_FILE_THREAD }
#[inline] pub unsafe fn hfsplus_cat_thread_size(t:*const hfsplus_cat_thread)->usize { core::mem::offset_of!(hfsplus_cat_thread,nodeName)+core::mem::offset_of!(hfsplus_unistr,unicode)+be16_to_cpu((*t).nodeName.length) as usize*core::mem::size_of::<hfsplus_unichr>() }

// Declarations exported by the corresponding C implementation files.
extern "C" {
    pub fn hfsplus_create_attr_tree_cache()->i32; pub fn hfsplus_destroy_attr_tree_cache();
    pub fn hfsplus_block_allocate(*mut super_block,u32,u32,*mut u32)->i32; pub fn hfsplus_block_free(*mut super_block,u32,u32)->i32;
    pub fn hfs_btree_open(*mut super_block,u32)->*mut hfs_btree; pub fn hfs_btree_close(*mut hfs_btree);
    pub fn hfs_btree_write(*mut hfs_btree)->i32; pub fn hfs_bmap_reserve(*mut hfs_btree,u32)->i32;
    pub fn hfs_bmap_alloc(*mut hfs_btree)->*mut hfs_bnode; pub fn hfs_bmap_free(*mut hfs_bnode);
    pub fn hfs_bnode_read(*mut hfs_bnode,*mut core::ffi::c_void,u32,u32); pub fn hfs_bnode_read_u16(*mut hfs_bnode,u32)->u16; pub fn hfs_bnode_read_u8(*mut hfs_bnode,u32)->u8;
    pub fn hfs_bnode_write(*mut hfs_bnode,*mut core::ffi::c_void,u32,u32); pub fn hfs_bnode_write_u16(*mut hfs_bnode,u32,u16);
    pub fn hfs_bnode_clear(*mut hfs_bnode,u32,u32); pub fn hfs_bnode_copy(*mut hfs_bnode,u32,*mut hfs_bnode,u32,u32); pub fn hfs_bnode_move(*mut hfs_bnode,u32,u32,u32);
    pub fn hfs_bnode_dump(*mut hfs_bnode); pub fn hfs_bnode_unlink(*mut hfs_bnode); pub fn hfs_bnode_findhash(*mut hfs_btree,u32)->*mut hfs_bnode;
    pub fn hfs_bnode_unhash(*mut hfs_bnode); pub fn hfs_bnode_find(*mut hfs_btree,u32)->*mut hfs_bnode; pub fn hfs_bnode_free(*mut hfs_bnode);
    pub fn hfs_bnode_create(*mut hfs_btree,u32)->*mut hfs_bnode; pub fn hfs_bnode_get(*mut hfs_bnode); pub fn hfs_bnode_put(*mut hfs_bnode); pub fn hfs_bnode_need_zeroout(*mut hfs_btree)->bool;
    pub fn hfs_brec_lenoff(*mut hfs_bnode,u16,*mut u16)->u16; pub fn hfs_brec_keylen(*mut hfs_bnode,u16)->u16; pub fn hfs_brec_insert(*mut hfs_find_data,*mut core::ffi::c_void,u32)->i32; pub fn hfs_brec_remove(*mut hfs_find_data)->i32;
    pub fn hfs_find_init(*mut hfs_btree,*mut hfs_find_data)->i32; pub fn hfs_find_exit(*mut hfs_find_data); pub fn hfs_brec_find(*mut hfs_find_data,Option<SearchStrategyT>)->i32; pub fn hfs_brec_read(*mut hfs_find_data,*mut core::ffi::c_void,u32)->i32; pub fn hfs_brec_goto(*mut hfs_find_data,i32)->i32;
}

// Remaining external declarations retain the header's public interfaces.
extern "C" {
    pub fn hfsplus_get_block(*mut inode,sector_t,*mut buffer_head,i32)->i32; pub fn hfsplus_file_extend(*mut inode,bool)->i32; pub fn hfsplus_file_truncate(*mut inode);
    pub fn hfsplus_iget(*mut super_block,usize)->*mut inode; pub fn hfsplus_mark_mdb_dirty(*mut super_block); pub fn hfsplus_commit_superblock(*mut super_block)->i32;
    pub fn hfsplus_submit_bio(*mut super_block,sector_t,*mut core::ffi::c_void,*mut *mut core::ffi::c_void,blk_opf_t)->i32; pub fn hfsplus_read_wrapper(*mut super_block)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
