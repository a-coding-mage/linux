/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/mm.h, linux/mm_types.h, vdso/datapage.h, and asm/barrier.h.

/*
 * struct loongarch_vdso_info - Details of a VDSO image.
 * @vdso: Pointer to VDSO image (page-aligned).
 * @size: Size of the VDSO image (page-aligned).
 * @off_rt_sigreturn: Offset of the rt_sigreturn() trampoline.
 * @code_mapping: Special mapping structure for vdso code.
 * @code_mapping: Special mapping structure for vdso data.
 *
 * This structure contains details of a VDSO image, including the image data
 * and offsets of certain symbols required by the kernel. It is generated as
 * part of the VDSO build process, aside from the mapping page array, which is
 * populated at runtime.
 */
#[repr(C)]
pub struct loongarch_vdso_info {
    pub vdso: *mut core::ffi::c_void,
    pub size: core::ffi::c_ulong,
    pub offset_sigreturn: core::ffi::c_ulong,
    pub code_mapping: vm_special_mapping,
}

unsafe extern "C" {
    pub static mut vdso_info: loongarch_vdso_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
