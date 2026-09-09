/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm_types.h, vdso/datapage.h, and asm/barrier.h.

use core::ffi::c_void;
use core::ffi::c_ulong;

/**
 * struct mips_vdso_image - Details of a VDSO image.
 * @data: Pointer to VDSO image data (page-aligned).
 * @size: Size of the VDSO image data (page-aligned).
 * @off_sigreturn: Offset of the sigreturn() trampoline.
 * @off_rt_sigreturn: Offset of the rt_sigreturn() trampoline.
 * @mapping: Special mapping structure.
 *
 * This structure contains details of a VDSO image, including the image data
 * and offsets of certain symbols required by the kernel. It is generated as
 * part of the VDSO build process, aside from the mapping page array, which is
 * populated at runtime.
 */
#[repr(C)]
pub struct mips_vdso_image {
    pub data: *mut c_void,
    pub size: c_ulong,

    pub off_sigreturn: c_ulong,
    pub off_rt_sigreturn: c_ulong,

    pub mapping: vm_special_mapping,
}

/*
 * The following structures are auto-generated as part of the build for each
 * ABI by genvdso, see arch/mips/vdso/Makefile.
 */

extern "C" {
    pub static mut vdso_image: mips_vdso_image;

    // Preserved from #ifdef CONFIG_MIPS32_O32.
    #[cfg(CONFIG_MIPS32_O32)]
    pub static mut vdso_image_o32: mips_vdso_image;

    // Preserved from #ifdef CONFIG_MIPS32_N32.
    #[cfg(CONFIG_MIPS32_N32)]
    pub static mut vdso_image_n32: mips_vdso_image;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
