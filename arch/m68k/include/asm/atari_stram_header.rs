/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Functions for Atari ST-RAM management
 */

/* public interface */
extern "C" {
    pub fn atari_stram_alloc(size: kernel::ffi::c_ulong, owner: *const kernel::ffi::c_char) -> *mut kernel::ffi::c_void;
    pub fn atari_stram_free(ptr: *mut kernel::ffi::c_void);
    pub fn atari_stram_to_virt(phys: kernel::ffi::c_ulong) -> *mut kernel::ffi::c_void;
    pub fn atari_stram_to_phys(ptr: *mut kernel::ffi::c_void) -> kernel::ffi::c_ulong;

    /* functions called internally by other parts of the kernel */
    pub fn atari_stram_init();
    pub fn atari_stram_reserve_pages(start_mem: *mut kernel::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
