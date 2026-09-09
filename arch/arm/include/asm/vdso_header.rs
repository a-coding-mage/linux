/* SPDX-License-Identifier: GPL-2.0 */

/* This header is active only when building the kernel. */

pub const __VDSO_PAGES: usize = 4;

/* Opaque declaration supplied by the kernel memory-management code. */
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

/* CONFIG_VDSO: the build-time condition from the C header is represented by
 * the corresponding Rust feature condition. */
#[cfg(feature = "CONFIG_VDSO")]
extern "C" {
    pub fn arm_install_vdso(mm: *mut mm_struct, addr: usize);

    pub static mut vdso_total_pages: u32;
}

/* CONFIG_VDSO disabled. */
#[cfg(not(feature = "CONFIG_VDSO"))]
pub unsafe extern "C" fn arm_install_vdso(_mm: *mut mm_struct, _addr: usize) {}

#[cfg(not(feature = "CONFIG_VDSO"))]
pub const vdso_total_pages: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
