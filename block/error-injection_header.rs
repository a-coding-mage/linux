/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub struct gendisk {
    pub state: *mut core::ffi::c_ulong,
}

#[repr(C)]
pub struct block_device {
    pub bd_disk: *mut gendisk,
}

#[repr(C)]
pub struct bio {
    pub bi_bdev: *mut block_device,
}

// DECLARE_STATIC_KEY_FALSE(blk_error_injection_enabled)
extern "C" {
    pub static blk_error_injection_enabled: static_key_false;

    pub fn blk_error_injection_init(disk: *mut gendisk);
    pub fn blk_error_injection_exit(disk: *mut gendisk);
    pub fn __blk_error_inject(bio: *mut bio) -> bool;

    pub fn static_branch_unlikely(key: *const static_key_false) -> bool;
    pub fn test_bit(nr: core::ffi::c_ulong, addr: *const core::ffi::c_ulong) -> bool;
}

// Opaque dependency type provided by linux/jump_label.h.
#[repr(C)]
pub struct static_key_false {
    _private: [u8; 0],
}

// Dependency supplied by the surrounding kernel translation.
pub const GD_ERROR_INJECT: core::ffi::c_ulong = 0;

#[inline]
pub unsafe fn blk_error_inject(bio: *mut bio) -> bool {
    // CONFIG_BLK_ERROR_INJECTION is a build-time configuration condition.
    if static_branch_unlikely(&blk_error_injection_enabled)
        && test_bit(
            GD_ERROR_INJECT,
            (*(*bio).bi_bdev).bd_disk.as_ref().unwrap().state,
        )
    {
        return __blk_error_inject(bio);
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
