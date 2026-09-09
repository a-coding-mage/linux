/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __LINUX_INITRD_H

// starting block # of image
extern "C" {
    pub static mut rd_image_start: core::ffi::c_int;

    // size of a single RAM disk
    pub static mut rd_size: core::ffi::c_ulong;

    // 1 if it is not an error if initrd_start < memory_start
    pub static mut initrd_below_start_ok: core::ffi::c_int;

    // free_initrd_mem always gets called with the next two as arguments..
    pub static mut initrd_start: core::ffi::c_ulong;
    pub static mut initrd_end: core::ffi::c_ulong;
    pub fn free_initrd_mem(start: core::ffi::c_ulong, end: core::ffi::c_ulong);

    pub static mut phys_initrd_start: phys_addr_t;
    pub static mut phys_initrd_size: core::ffi::c_ulong;

    pub static mut __initramfs_start: [core::ffi::c_char; 0];
    pub static mut __initramfs_size: core::ffi::c_ulong;

    pub fn console_on_rootfs();
}

// __init is a kernel-specific declaration attribute supplied by the
// surrounding build environment.

// CONFIG_BLK_DEV_INITRD controls whether these functions are external
// declarations or empty inline functions in the C header.
#[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
extern "C" {
    pub fn reserve_initrd_mem();
    pub fn wait_for_initramfs();
}

#[cfg(not(feature = "CONFIG_BLK_DEV_INITRD"))]
#[inline]
pub unsafe fn reserve_initrd_mem() {}

#[cfg(not(feature = "CONFIG_BLK_DEV_INITRD"))]
#[inline]
pub unsafe fn wait_for_initramfs() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
