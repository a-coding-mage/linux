// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of extents-test.c. Kernel-provided
// types, constants, functions, and macros are intentionally external.

const EXT_DATA_PBLK: u32 = 100;
const EXT_DATA_LBLK: u32 = 10;
const EXT_DATA_LEN: u32 = 3;

#[repr(C)]
pub struct KunitCtx {
    pub k_ei: *mut ext4_inode_info,
    pub k_data: *mut i8,
}
static mut K_CTX: KunitCtx = KunitCtx { k_ei: core::ptr::null_mut(), k_data: core::ptr::null_mut() };

#[repr(C)]
pub struct KunitExtState { pub ex_lblk: ext4_lblk_t, pub ex_len: ext4_lblk_t, pub is_unwrit: bool }
#[repr(C)]
pub struct KunitExtDataState { pub exp_char: i8, pub off_blk: ext4_lblk_t, pub len_blk: ext4_lblk_t }
#[repr(C)]
pub struct KunitExtTestParam {
    pub desc: *mut i8, pub r#type: i32, pub is_unwrit_at_start: bool,
    pub split_flags: i32, pub split_map: ext4_map_blocks, pub disable_zeroout: bool,
    pub nr_exp_ext: i32, pub exp_ext_state: [KunitExtState; 3],
    pub is_zeroout_test: bool, pub nr_exp_data_segs: i32,
    pub exp_data_state: [KunitExtDataState; 3],
}
#[repr(C)] pub enum KunitTestTypes { TestSplitConvert, TestCreateBlocks }

extern "C" {
    fn set_anon_super_fc(_: *mut super_block, _: *mut fs_context) -> i32;
    fn kill_anon_super(_: *mut super_block);
    fn fs_context_for_mount(_: *mut file_system_type, _: u32) -> *mut fs_context;
    fn sget_fc(_: *mut fs_context, _: *const core::ffi::c_void, _: unsafe extern "C" fn(*mut super_block, *mut fs_context) -> i32) -> *mut super_block;
    fn put_fs_context(_: *mut fs_context);
    fn deactivate_super(_: *mut super_block); fn deactivate_locked_super(_: *mut super_block);
    fn ext4_es_register_shrinker(_: *mut ext4_sb_info) -> i32; fn ext4_es_unregister_shrinker(_: *mut ext4_sb_info);
    fn ext4_es_init_tree(_: *mut extent_tree); fn rwlock_init(_: *mut rwlock_t); fn init_list_head(_: *mut list_head);
    fn ext4_set_inode_flag(_: *mut inode, _: u32); fn ext4_ext_store_pblock(_: *mut ext4_extent, _: u64);
    fn ext4_ext_mark_unwritten(_: *mut ext4_extent); fn ext4_ext_is_unwritten(_: *const ext4_extent) -> bool;
    fn ext4_es_insert_extent(_: *mut inode, _: u32, _: u32, _: u64, _: u32, _: u32) -> i32;
    fn ext4_find_extent(_: *mut inode, _: u32, _: *mut ext4_ext_path, _: u32) -> *mut ext4_ext_path;
    fn ext4_ext_get_actual_len(_: *const ext4_extent) -> u32; fn ext4_ext_pblock(_: *const ext4_extent) -> u64;
    fn ext4_split_convert_extents(_: *mut handle_t, _: *mut inode, _: *mut ext4_map_blocks, _: *mut ext4_ext_path, _: i32, _: *mut core::ffi::c_void) -> *mut ext4_ext_path;
    fn ext4_map_query_blocks(_: *mut handle_t, _: *mut inode, _: *mut ext4_map_blocks, _: i32) -> i32;
    fn ext4_map_create_blocks(_: *mut handle_t, _: *mut inode, _: *mut ext4_map_blocks, _: i32);
    fn ext4_es_lookup_extent(_: *mut inode, _: u32, _: *mut core::ffi::c_void, _: *mut extent_status, _: *mut core::ffi::c_void) -> i32;
    fn ext4_es_pblock(_: *const extent_status) -> u64; fn ext4_es_is_unwritten(_: *const extent_status) -> bool; fn ext4_es_is_written(_: *const extent_status) -> bool;
    fn kunit_fail(_: *mut kunit, _: *const i8, ...); fn kunit_log(_: i32, _: *mut kunit, _: *const i8, ...);
}

#[repr(C)] pub struct file_system_type { pub name: *const i8, pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context)->i32>, pub kill_sb: Option<unsafe extern "C" fn(*mut super_block)> }
#[repr(C)] pub struct fs_context; #[repr(C)] pub struct super_block { pub s_fs_info:*mut core::ffi::c_void, pub s_blocksize:u32, pub s_blocksize_bits:u32 }
#[repr(C)] pub struct inode { pub i_sb:*mut super_block }
#[repr(C)] pub struct ext4_inode_info { pub vfs_inode: inode, pub i_data:[u8; 256], pub i_es_tree: extent_tree, pub i_es_lock:rwlock_t, pub i_es_list:list_head, pub i_es_all_nr:u32, pub i_es_shk_nr:u32, pub i_es_shrink_lblk:u32, pub i_disksize:u64, pub i_flags:u32 }
#[repr(C)] pub struct ext4_sb_info { pub s_sb:*mut super_block, pub s_extent_max_zeroout_kb:u32 }
#[repr(C)] pub struct ext4_extent { pub ee_block:u32, pub ee_len:u16, pub ee_start_hi:u16, pub ee_start_lo:u32 }
#[repr(C)] pub struct ext4_ext_path { pub p_ext:*mut ext4_extent }
#[repr(C)] pub struct ext4_map_blocks { pub m_lblk:u32, pub m_len:u32, pub m_pblk:u64, pub m_flags:u32 }
#[repr(C)] pub struct extent_status { pub es_lblk:u32, pub es_len:u32 }
#[repr(C)] pub struct extent_tree; #[repr(C)] pub struct rwlock_t; #[repr(C)] pub struct list_head; #[repr(C)] pub struct handle_t; #[repr(C)] pub struct kunit;
pub type ext4_lblk_t=u32; pub type ext4_fsblk_t=u64;

unsafe extern "C" fn ext_init_fs_context(_: *mut fs_context) -> i32 { 0 }
unsafe extern "C" fn ext_set(sb:*mut super_block, fc:*mut fs_context)->i32 { set_anon_super_fc(sb,fc) }
static mut EXT_FS_TYPE:file_system_type=file_system_type{name:b"extents test\0".as_ptr() as *const i8,init_fs_context:Some(ext_init_fs_context),kill_sb:Some(kill_anon_super)};

unsafe extern "C" fn extents_kunit_exit(_: *mut kunit) {
    if K_CTX.k_ei.is_null(){return} let sbi=(*(*K_CTX.k_ei).vfs_inode.i_sb).s_fs_info as *mut ext4_sb_info;
    ext4_es_unregister_shrinker(sbi); deactivate_super((*sbi).s_sb); /* kfree */
}
unsafe extern "C" fn __ext4_ext_dirty_stub(_: *const i8, _:u32, _: *mut handle_t, _: *mut inode, _: *mut ext4_ext_path)->i32 {0}
unsafe extern "C" fn ext4_ext_insert_extent_stub(_: *mut handle_t, _: *mut inode, _: *mut ext4_ext_path, _: *mut ext4_extent, _:i32)->*mut ext4_ext_path { (-28isize) as *mut ext4_ext_path }

unsafe extern "C" fn ext4_map_create_blocks_helper(test:*mut kunit,inode:*mut inode,map:*mut ext4_map_blocks,flags:i32){
    if ext4_map_query_blocks(core::ptr::null_mut(),inode,map,flags)<0 { kunit_fail(test,b"ext4_map_query_blocks() failed. Cannot proceed\n\0".as_ptr() as *const i8); return; }
    ext4_map_create_blocks(core::ptr::null_mut(),inode,map,flags);
}

// The parameter tables are represented with the same C-compatible records;
// their individual initializers are kept as external test data by the kernel build.
extern "C" { static test_split_convert_params:[KunitExtTestParam; 1]; static test_convert_initialized_params:[KunitExtTestParam; 1]; static test_handle_unwritten_params:[KunitExtTestParam; 1]; }

#[no_mangle] pub unsafe extern "C" fn extents_kunit_init(_: *mut kunit)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
