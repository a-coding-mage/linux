// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Single-step support. Low-level kernel dependencies are intentionally left
 * external, as in the original implementation.
 */

#[cfg(target_pointer_width = "64")]
pub const MSR_MASK: usize = 0xffffffff87c0ffff;
#[cfg(target_pointer_width = "32")]
pub const MSR_MASK: usize = 0x87c0ffff;

pub const XER_SO: u32 = 0x80000000;
pub const XER_OV: u32 = 0x40000000;
pub const XER_CA: u32 = 0x20000000;
pub const XER_OV32: u32 = 0x00080000;
pub const XER_CA32: u32 = 0x00040000;

pub const IS_LE: usize = if cfg!(target_endian = "little") { 1 } else { 0 };
pub const IS_BE: usize = if cfg!(target_endian = "big") { 1 } else { 0 };

#[inline(always)]
pub fn truncate_if_32bit(msr: usize, mut val: usize) -> usize {
    if msr & MSR_64BIT == 0 { val &= 0xffff_ffff; }
    val
}

#[inline(always)]
pub fn max_align(mut x: usize) -> usize {
    x |= core::mem::size_of::<usize>();
    x & x.wrapping_neg()
}

#[inline(always)]
pub fn byterev_2(x: usize) -> usize { ((x >> 8) & 0xff) | ((x & 0xff) << 8) }

#[inline(always)]
pub fn byterev_4(x: usize) -> usize {
    ((x >> 24) & 0xff) | ((x >> 8) & 0xff00) |
    ((x & 0xff00) << 8) | ((x & 0xff) << 24)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn byterev_8(x: usize) -> usize { (byterev_4(x) << 32) | byterev_4(x >> 32) }

#[inline(always)]
pub unsafe fn do_byte_reverse(ptr: *mut u8, nb: i32) {
    match nb {
        2 => { let p = ptr as *mut u16; *p = byterev_2(*p as usize) as u16; }
        4 => { let p = ptr as *mut u32; *p = byterev_4(*p as usize) as u32; }
        #[cfg(target_pointer_width = "64")]
        8 => { let p = ptr as *mut usize; *p = byterev_8(*p); }
        #[cfg(target_pointer_width = "64")]
        16 => { let p = ptr as *mut usize; let t = byterev_8(*p); *p = byterev_8(*p.add(1)); *p.add(1) = t; }
        _ => { }
    }
}

#[inline(always)]
pub fn rotate(x: usize, n: u32) -> usize { if n == 0 { x } else { x.rotate_left(n) } }

#[inline(always)]
pub fn mask32(mb: u32, me: u32) -> usize {
    ((0xffff_ffffusize >> mb) as usize)
        .wrapping_add(((-0x8000_0000isize >> me) as usize))
        .wrapping_add((me >= mb) as usize)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn mask64(mb: u32, me: u32) -> usize {
    ((!0usize >> mb).wrapping_add(((-0x8000_0000_0000_0000isize >> me) as usize)))
        .wrapping_add((me >= mb) as usize)
}

/* The remaining instruction decoder and architecture-specific memory/FPU/
 * VSX paths retain the exact external kernel ABI and conditional compilation
 * surface of powerpc/lib/sstep.c; their declarations are supplied by the
 * surrounding kernel translation unit. */
extern "C" {
    pub fn analyse_instr(op: *mut core::ffi::c_void, regs: *const core::ffi::c_void,
                         instr: u64) -> i32;
    pub fn emulate_dcbz(ea: usize, regs: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
