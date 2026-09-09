// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation:
// linux/minmax.h, linux/string.h, asm/ebcdic.h, asm/ipl.h

/* VM IPL PARM routines */
pub unsafe fn ipl_block_get_ascii_vmparm(
    dest: *mut core::ffi::c_char,
    size: usize,
    ipb: *const ipl_parameter_block,
) -> usize {
    let mut i: i32;
    let mut len: usize;
    let mut has_lowercase: core::ffi::c_char = 0;

    len = 0;
    if ((*ipb).ccw.vm_flags & IPL_PB0_CCW_VM_FLAG_VP) != 0
        && (*ipb).ccw.vm_parm_len > 0
    {
        len = core::cmp::min(size - 1, (*ipb).ccw.vm_parm_len);
        core::ptr::copy_nonoverlapping((*ipb).ccw.vm_parm.as_ptr(), dest as *mut u8, len);
        /* If at least one character is lowercase, we assume mixed
         * case; otherwise we convert everything to lowercase.
         */
        i = 0;
        while (i as usize) < len {
            let ch = *dest.add(i as usize) as u8;
            if (ch > 0x80 && ch < 0x8a) || /* a-i */
               (ch > 0x90 && ch < 0x9a) || /* j-r */
               (ch > 0xa1 && ch < 0xaa) /* s-z */
            {
                has_lowercase = 1;
                break;
            }
            i += 1;
        }
        if has_lowercase == 0 {
            // EBC_TOLOWER(dest, len)
            ebcdic_tolower(dest, len);
        }
        // EBCASC(dest, len)
        ebcdic_to_ascii(dest, len);
    }
    *dest.add(len) = 0;

    len
}

// External equivalents of the EBC_TOLOWER and EBCASC macros.
extern "C" {
    fn ebcdic_tolower(dest: *mut core::ffi::c_char, len: usize);
    fn ebcdic_to_ascii(dest: *mut core::ffi::c_char, len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
