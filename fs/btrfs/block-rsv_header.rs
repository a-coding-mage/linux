/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/compiler.h, and linux/spinlock.h

#[repr(C)]
pub struct btrfs_trans_handle {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btrfs_root {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btrfs_space_info {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct btrfs_fs_info {
    _unused: [u8; 0],
}

// Supplied by the surrounding translation unit.
pub type spinlock_t = ::core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_reserve_flush_enum {
    _BTRFS_RESERVE_FLUSH_ENUM_UNDEFINED,
}

/*
 * Types of block reserves
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_rsv_type {
    BTRFS_BLOCK_RSV_GLOBAL,
    BTRFS_BLOCK_RSV_DELALLOC,
    BTRFS_BLOCK_RSV_TRANS,
    BTRFS_BLOCK_RSV_CHUNK,
    BTRFS_BLOCK_RSV_REMAP,
    BTRFS_BLOCK_RSV_DELOPS,
    BTRFS_BLOCK_RSV_DELREFS,
    BTRFS_BLOCK_RSV_TREELOG,
    BTRFS_BLOCK_RSV_EMPTY,
    BTRFS_BLOCK_RSV_TEMP,
}

#[repr(C)]
pub struct btrfs_block_rsv {
    pub size: u64,
    pub reserved: u64,
    pub space_info: *mut btrfs_space_info,
    pub lock: spinlock_t,
    pub full: bool,
    pub failfast: bool,
    /* Block reserve type, one of BTRFS_BLOCK_RSV_* */
    pub type_: btrfs_rsv_type,

    /*
     * Qgroup equivalent for @size @reserved
     *
     * Unlike normal @size/@reserved for inode rsv, qgroup doesn't care
     * about things like csum size nor how many tree blocks it will need to
     * reserve.
     *
     * Qgroup cares more about net change of the extent usage.
     *
     * So for one newly inserted file extent, in worst case it will cause
     * leaf split and level increase, nodesize for each file extent is
     * already too much.
     *
     * In short, qgroup_size/reserved is the upper limit of possible needed
     * qgroup metadata reservation.
     */
    pub qgroup_rsv_size: u64,
    pub qgroup_rsv_reserved: u64,
}

unsafe extern "C" {
    pub fn btrfs_init_block_rsv(rsv: *mut btrfs_block_rsv, type_: btrfs_rsv_type);
    pub fn btrfs_init_root_block_rsv(root: *mut btrfs_root);
    pub fn btrfs_alloc_block_rsv(
        fs_info: *mut btrfs_fs_info,
        type_: btrfs_rsv_type,
    ) -> *mut btrfs_block_rsv;
    pub fn btrfs_init_metadata_block_rsv(
        fs_info: *mut btrfs_fs_info,
        rsv: *mut btrfs_block_rsv,
        type_: btrfs_rsv_type,
    );
    pub fn btrfs_free_block_rsv(fs_info: *mut btrfs_fs_info, rsv: *mut btrfs_block_rsv);
    pub fn btrfs_block_rsv_add(
        fs_info: *mut btrfs_fs_info,
        block_rsv: *mut btrfs_block_rsv,
        num_bytes: u64,
        flush: btrfs_reserve_flush_enum,
    ) -> i32;
    pub fn btrfs_block_rsv_check(block_rsv: *mut btrfs_block_rsv, min_percent: i32) -> i32;
    pub fn btrfs_block_rsv_refill(
        fs_info: *mut btrfs_fs_info,
        block_rsv: *mut btrfs_block_rsv,
        num_bytes: u64,
        flush: btrfs_reserve_flush_enum,
    ) -> i32;
    pub fn btrfs_block_rsv_migrate(
        src_rsv: *mut btrfs_block_rsv,
        dst_rsv: *mut btrfs_block_rsv,
        num_bytes: u64,
        update_size: bool,
    ) -> i32;
    pub fn btrfs_block_rsv_use_bytes(block_rsv: *mut btrfs_block_rsv, num_bytes: u64) -> i32;
    pub fn btrfs_block_rsv_add_bytes(
        block_rsv: *mut btrfs_block_rsv,
        num_bytes: u64,
        update_size: bool,
    );
    pub fn btrfs_block_rsv_release(
        fs_info: *mut btrfs_fs_info,
        block_rsv: *mut btrfs_block_rsv,
        num_bytes: u64,
        qgroup_to_release: *mut u64,
    ) -> u64;
    pub fn btrfs_update_global_block_rsv(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_init_global_block_rsv(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_release_global_block_rsv(fs_info: *mut btrfs_fs_info);
    pub fn btrfs_use_block_rsv(
        trans: *mut btrfs_trans_handle,
        root: *mut btrfs_root,
        blocksize: u32,
    ) -> *mut btrfs_block_rsv;
    pub fn btrfs_check_trunc_cache_free_space(
        fs_info: *const btrfs_fs_info,
        rsv: *mut btrfs_block_rsv,
    ) -> i32;
}

pub unsafe fn btrfs_unuse_block_rsv(
    fs_info: *mut btrfs_fs_info,
    block_rsv: *mut btrfs_block_rsv,
    blocksize: u32,
) {
    unsafe {
        btrfs_block_rsv_add_bytes(block_rsv, blocksize as u64, false);
        btrfs_block_rsv_release(fs_info, block_rsv, 0, core::ptr::null_mut());
    }
}

/*
 * Fast path to check if the reserve is full, may be carefully used outside of
 * locks.
 */
pub unsafe fn btrfs_block_rsv_full(rsv: *const btrfs_block_rsv) -> bool {
    unsafe { (*rsv).full }
}

/*
 * Get the reserved mount of a block reserve in a context where getting a stale
 * value is acceptable, instead of accessing it directly and trigger data race
 * warning from KCSAN.
 */
pub unsafe fn btrfs_block_rsv_reserved(rsv: *mut btrfs_block_rsv) -> u64 {
    let ret: u64;
    unsafe {
        spin_lock(&mut (*rsv).lock);
        ret = (*rsv).reserved;
        spin_unlock(&mut (*rsv).lock);
    }
    ret
}

/*
 * Get the size of a block reserve in a context where getting a stale value is
 * acceptable, instead of accessing it directly and trigger data race warning
 * from KCSAN.
 */
pub unsafe fn btrfs_block_rsv_size(rsv: *mut btrfs_block_rsv) -> u64 {
    let ret: u64;
    unsafe {
        spin_lock(&mut (*rsv).lock);
        ret = (*rsv).size;
        spin_unlock(&mut (*rsv).lock);
    }
    ret
}

// Supplied by the surrounding translation unit.
unsafe extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
