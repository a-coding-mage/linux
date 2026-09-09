// SPDX-License-Identifier: GPL-2.0
// Translated from Linux ELF and s390 kexec declarations.

pub unsafe fn arch_kexec_do_relocs(
    r_type: i32,
    loc: *mut core::ffi::c_void,
    val: u64,
    addr: u64,
) -> i32 {
    match r_type {
        R_390_NONE => {}
        R_390_8 => {
            // Direct 8 bit.
            *(loc as *mut u8) = val as u8;
        }
        R_390_12 => {
            // Direct 12 bit.
            let p = loc as *mut u16;
            *p &= 0xf000;
            *p |= (val & 0xfff) as u16;
        }
        R_390_16 => {
            // Direct 16 bit.
            *(loc as *mut u16) = val as u16;
        }
        R_390_20 => {
            // Direct 20 bit.
            let p = loc as *mut u32;
            *p &= 0xf00000ff;
            *p |= ((val & 0xfff) << 16) as u32; // DL
            *p |= ((val & 0xff000) >> 4) as u32; // DH
        }
        R_390_32 => {
            // Direct 32 bit.
            *(loc as *mut u32) = val as u32;
        }
        R_390_64 | R_390_GLOB_DAT | R_390_JMP_SLOT => {
            // Direct 64 bit.
            *(loc as *mut u64) = val;
        }
        R_390_PC16 => {
            // PC relative 16 bit.
            *(loc as *mut u16) = val.wrapping_sub(addr) as u16;
        }
        R_390_PC16DBL => {
            // PC relative 16 bit shifted by 1.
            *(loc as *mut u16) = (val.wrapping_sub(addr) >> 1) as u16;
        }
        R_390_PC32DBL => {
            // PC relative 32 bit shifted by 1.
            *(loc as *mut u32) = (val.wrapping_sub(addr) >> 1) as u32;
        }
        R_390_PC32 => {
            // PC relative 32 bit.
            *(loc as *mut u32) = val.wrapping_sub(addr) as u32;
        }
        R_390_PC64 => {
            // PC relative 64 bit.
            *(loc as *mut u64) = val.wrapping_sub(addr);
        }
        R_390_RELATIVE => {
            *(loc as *mut u64) = val;
        }
        _ => return 1,
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
