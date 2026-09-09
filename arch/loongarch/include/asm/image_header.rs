/* SPDX-License-Identifier: GPL-2.0 */
/*
 * LoongArch binary image header for EFI(PE/COFF) format.
 *
 * Author: Youling Tang <tangyouling@kylinos.cn>
 * Copyright (C) 2025 KylinSoft Corporation.
 */

/**
 * struct loongarch_image_header
 *
 * @dos_sig: Optional PE format 'MZ' signature.
 * @padding_1: Reserved.
 * @kernel_entry: Kernel image entry pointer.
 * @kernel_asize: An estimated size of the memory image size in LSB byte order.
 * @text_offset: The image load offset in LSB byte order.
 * @padding_2: Reserved.
 * @pe_header: Optional offset to a PE format header.
 **/
#[repr(C)]
pub struct loongarch_image_header {
    pub dos_sig: [u8; 2],
    pub padding_1: [u16; 3],
    pub kernel_entry: u64,
    pub kernel_asize: u64,
    pub text_offset: u64,
    pub padding_2: [u32; 7],
    pub pe_header: u32,
}

/*
 * loongarch_header_check_dos_sig - Helper to check the header
 *
 * Returns true (non-zero) if 'MZ' signature is found.
 */
#[inline]
pub unsafe fn loongarch_header_check_dos_sig(
    h: *const loongarch_image_header,
) -> i32 {
    if h.is_null() {
        return 0;
    }

    unsafe { ((*h).dos_sig[0] == b'M' && (*h).dos_sig[1] == b'Z') as i32 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
