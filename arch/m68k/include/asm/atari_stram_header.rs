/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Functions for Atari ST-RAM management
 */

/* public interface */
extern "C" {
    pub fn atari_stram_alloc(size: libc::c_ulong, owner: *const libc::c_char) -> *mut libc::c_void;
    pub fn atari_stram_free(ptr: *mut libc::c_void);
    pub fn atari_stram_to_virt(phys: libc::c_ulong) -> *mut libc::c_void;
    pub fn atari_stram_to_phys(ptr: *mut libc::c_void) -> libc::c_ulong;

    /* functions called internally by other parts of the kernel */
    pub fn atari_stram_init();
    pub fn atari_stram_reserve_pages(start_mem: *mut libc::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
