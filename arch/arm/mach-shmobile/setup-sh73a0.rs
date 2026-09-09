// SPDX-License-Identifier: GPL-2.0
/*
 * sh73a0 processor support
 *
 * Copyright (C) 2010  Takashi Yoshii
 * Copyright (C) 2010  Magnus Damm
 * Copyright (C) 2008  Yoshihiro Shimoda
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

extern "C" {
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn l2x0_init(base: *mut core::ffi::c_void, aux_val: u32, aux_mask: u32);
}

const PAGE_SIZE: usize = 4096;

// The C __init annotation is a linker/initialization-section attribute.
unsafe fn sh73a0_generic_init() {
    // CONFIG_CACHE_L2X0 is a build-time condition in the original source.
    #[cfg(CONFIG_CACHE_L2X0)]
    {
        /* Shared attribute override enable, 64K*8way */
        let base = unsafe { ioremap(0xf010_0000, PAGE_SIZE) };
        unsafe { l2x0_init(base, 0x0040_0000, 0xc20f_0fff) };
    }
}

// __initconst
static SH73A0_BOARDS_COMPAT_DT: [&'static core::ffi::c_char; 2] = [
    b"renesas,sh73a0\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

extern "C" {
    fn shmobile_init_late();
}

// DT_MACHINE_START(SH73A0_DT, "Generic SH73A0 (Flattened Device Tree)")
//     .smp        = smp_ops(sh73a0_smp_ops),
//     .init_machine = sh73a0_generic_init,
//     .init_late   = shmobile_init_late,
//     .dt_compat   = sh73a0_boards_compat_dt,
// MACHINE_END
// The machine-description macro and its surrounding architecture-specific
// type are provided by the kernel environment and are preserved above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
