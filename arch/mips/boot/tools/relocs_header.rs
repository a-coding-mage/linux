/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by other translation units / bindings:
// stdio.h, stdarg.h, stdlib.h, stdint.h, inttypes.h, string.h, errno.h,
// unistd.h, elf.h, byteswap.h, endian.h, and regex.h.

unsafe extern "C" {
    pub fn die(fmt: *mut libc::c_char, ...);
}

/*
 * Introduced for MIPSr6
 */
pub const R_MIPS_PC21_S2: libc::c_int = 60;

pub const R_MIPS_PC26_S2: libc::c_int = 61;

/*
 * GNU extension that available in glibc only since 2023, not available on musl.
 */
pub const R_MIPS_PC32: libc::c_int = 248;

#[inline]
pub const fn array_size<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum symtype {
    S_ABS,
    S_REL,
    S_SEG,
    S_LIN,
    S_NSYMTYPES,
}

unsafe extern "C" {
    pub fn process_32(
        fp: *mut libc::FILE,
        as_text: libc::c_int,
        as_bin: libc::c_int,
        show_reloc_info: libc::c_int,
        keep_relocs: libc::c_int,
    );
    pub fn process_64(
        fp: *mut libc::FILE,
        as_text: libc::c_int,
        as_bin: libc::c_int,
        show_reloc_info: libc::c_int,
        keep_relocs: libc::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
