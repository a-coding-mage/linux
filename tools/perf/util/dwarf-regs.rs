// SPDX-License-Identifier: GPL-2.0
/*
 * dwarf-regs.c : Mapping of DWARF debug register numbers into register names.
 *
 * Written by: Masami Hiramatsu <mhiramat@kernel.org>
 */

use core::ffi::{c_char, c_int, c_uint};

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;

/*
 * C dependencies removed from executable Rust:
 * <stdlib.h>, <string.h>, <debug.h>, <dwarf-regs.h>, <elf.h>,
 * <errno.h>, <linux/kernel.h>.
 *
 * The C source defines const char * {arch}_register_tbl[] by including
 * arch-specific dwarf-regs-table.h files with DEFINE_DWARF_REGSTR_TABLE set.
 * Those tables and arch-specific helper functions are external dependencies
 * in this translation.
 */

unsafe extern "C" {
    static x86_32_regstr_tbl: [*const c_char; 0];
    static x86_64_regstr_tbl: [*const c_char; 0];
    static arm_regstr_tbl: [*const c_char; 0];
    static aarch64_regstr_tbl: [*const c_char; 0];
    static sh_regstr_tbl: [*const c_char; 0];
    static s390_regstr_tbl: [*const c_char; 0];
    static powerpc_regstr_tbl: [*const c_char; 0];
    static riscv_regstr_tbl: [*const c_char; 0];
    static sparc_regstr_tbl: [*const c_char; 0];
    static xtensa_regstr_tbl: [*const c_char; 0];
    static mips_regstr_tbl: [*const c_char; 0];
    static loongarch_regstr_tbl: [*const c_char; 0];

    static EM_HOST: c_uint;

    fn __get_csky_regstr(n: c_uint, flags: c_uint) -> *const c_char;
    fn __get_dwarf_regnum_x86_64(name: *const c_char) -> c_int;
    fn __get_dwarf_regnum_i386(name: *const c_char) -> c_int;
    fn __get_csky_regnum(name: *const c_char, flags: c_uint) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_csky(perf_regnum: c_int, flags: c_uint) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum: c_int) -> c_int;
    fn __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum: c_int) -> c_int;

    fn strdup(s: *const c_char) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut core::ffi::c_void);
    fn pr_err(fmt: *const c_char, ...);
}

const EM_NONE: c_uint = 0;
const EM_386: c_uint = 3;
const EM_MIPS: c_uint = 8;
const EM_PPC: c_uint = 20;
const EM_PPC64: c_uint = 21;
const EM_S390: c_uint = 22;
const EM_SH: c_uint = 42;
const EM_SPARC: c_uint = 2;
const EM_SPARCV9: c_uint = 43;
const EM_ARM: c_uint = 40;
const EM_X86_64: c_uint = 62;
const EM_AARCH64: c_uint = 183;
const EM_RISCV: c_uint = 243;
const EM_CSKY: c_uint = 252;
const EM_LOONGARCH: c_uint = 258;
const EM_XTENSA: c_uint = 94;

#[inline]
unsafe fn get_dwarf_regstr_from_tbl(
    tbl: *const *const c_char,
    num_regstr: usize,
    n: c_uint,
) -> *const c_char {
    if (n as usize) < num_regstr {
        unsafe { *tbl.add(n as usize) }
    } else {
        core::ptr::null()
    }
}

/* Return architecture dependent register string (for kprobe-tracer) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_dwarf_regstr(
    n: c_uint,
    mut machine: c_uint,
    flags: c_uint,
) -> *const c_char {
    if machine == EM_NONE {
        /* Generic arch - use host arch */
        machine = unsafe { EM_HOST };
    }
    match machine {
        EM_386 => unsafe {
            get_dwarf_regstr_from_tbl(x86_32_regstr_tbl.as_ptr(), x86_32_regstr_tbl.len(), n)
        },
        EM_X86_64 => unsafe {
            get_dwarf_regstr_from_tbl(x86_64_regstr_tbl.as_ptr(), x86_64_regstr_tbl.len(), n)
        },
        EM_ARM => unsafe {
            get_dwarf_regstr_from_tbl(arm_regstr_tbl.as_ptr(), arm_regstr_tbl.len(), n)
        },
        EM_AARCH64 => unsafe {
            get_dwarf_regstr_from_tbl(aarch64_regstr_tbl.as_ptr(), aarch64_regstr_tbl.len(), n)
        },
        EM_CSKY => unsafe { __get_csky_regstr(n, flags) },
        EM_SH => unsafe {
            get_dwarf_regstr_from_tbl(sh_regstr_tbl.as_ptr(), sh_regstr_tbl.len(), n)
        },
        EM_S390 => unsafe {
            get_dwarf_regstr_from_tbl(s390_regstr_tbl.as_ptr(), s390_regstr_tbl.len(), n)
        },
        EM_PPC | EM_PPC64 => unsafe {
            get_dwarf_regstr_from_tbl(powerpc_regstr_tbl.as_ptr(), powerpc_regstr_tbl.len(), n)
        },
        EM_RISCV => unsafe {
            get_dwarf_regstr_from_tbl(riscv_regstr_tbl.as_ptr(), riscv_regstr_tbl.len(), n)
        },
        EM_SPARC | EM_SPARCV9 => unsafe {
            get_dwarf_regstr_from_tbl(sparc_regstr_tbl.as_ptr(), sparc_regstr_tbl.len(), n)
        },
        EM_XTENSA => unsafe {
            get_dwarf_regstr_from_tbl(xtensa_regstr_tbl.as_ptr(), xtensa_regstr_tbl.len(), n)
        },
        EM_MIPS => unsafe {
            get_dwarf_regstr_from_tbl(mips_regstr_tbl.as_ptr(), mips_regstr_tbl.len(), n)
        },
        EM_LOONGARCH => unsafe {
            get_dwarf_regstr_from_tbl(loongarch_regstr_tbl.as_ptr(), loongarch_regstr_tbl.len(), n)
        },
        _ => unsafe {
            pr_err(c"ELF MACHINE %x is not supported.\n".as_ptr(), machine);
            core::ptr::null()
        },
    }
}

unsafe fn __get_dwarf_regnum(
    regstr: *const *const c_char,
    num_regstr: usize,
    name: *const c_char,
) -> c_int {
    for i in 0..num_regstr {
        let entry = unsafe { *regstr.add(i) };
        if !entry.is_null() && unsafe { strcmp(entry, name) } == 0 {
            return i as c_int;
        }
    }
    -ENOENT
}

/* Return DWARF register number from architecture register name */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_dwarf_regnum(
    name: *const c_char,
    mut machine: c_uint,
    flags: c_uint,
) -> c_int {
    let regname = unsafe { strdup(name) };
    let mut reg: c_int = -1;
    let p: *mut c_char;

    if regname.is_null() {
        return -EINVAL;
    }

    /* For convenience, remove trailing characters */
    p = unsafe { strpbrk(regname, c" ,)".as_ptr()) };
    if !p.is_null() {
        unsafe {
            *p = b'\0' as c_char;
        }
    }

    if machine == EM_NONE {
        /* Generic arch - use host arch */
        machine = unsafe { EM_HOST };
    }
    match machine {
        EM_X86_64 => {
            reg = unsafe { __get_dwarf_regnum_x86_64(name) };
        }
        EM_386 => {
            reg = unsafe { __get_dwarf_regnum_i386(name) };
        }
        EM_ARM => {
            reg = unsafe { __get_dwarf_regnum(arm_regstr_tbl.as_ptr(), arm_regstr_tbl.len(), name) };
        }
        EM_AARCH64 => {
            reg = unsafe {
                __get_dwarf_regnum(aarch64_regstr_tbl.as_ptr(), aarch64_regstr_tbl.len(), name)
            };
        }
        EM_CSKY => {
            reg = unsafe { __get_csky_regnum(name, flags) };
        }
        EM_SH => {
            reg = unsafe { __get_dwarf_regnum(sh_regstr_tbl.as_ptr(), sh_regstr_tbl.len(), name) };
        }
        EM_S390 => {
            reg = unsafe { __get_dwarf_regnum(s390_regstr_tbl.as_ptr(), s390_regstr_tbl.len(), name) };
        }
        EM_PPC | EM_PPC64 => {
            reg = unsafe {
                __get_dwarf_regnum(powerpc_regstr_tbl.as_ptr(), powerpc_regstr_tbl.len(), name)
            };
        }
        EM_RISCV => {
            reg = unsafe {
                __get_dwarf_regnum(riscv_regstr_tbl.as_ptr(), riscv_regstr_tbl.len(), name)
            };
        }
        EM_SPARC | EM_SPARCV9 => {
            reg = unsafe {
                __get_dwarf_regnum(sparc_regstr_tbl.as_ptr(), sparc_regstr_tbl.len(), name)
            };
        }
        EM_XTENSA => {
            reg = unsafe {
                __get_dwarf_regnum(xtensa_regstr_tbl.as_ptr(), xtensa_regstr_tbl.len(), name)
            };
        }
        EM_MIPS => {
            reg = unsafe {
                __get_dwarf_regnum(mips_regstr_tbl.as_ptr(), mips_regstr_tbl.len(), name)
            };
        }
        EM_LOONGARCH => {
            reg = unsafe {
                __get_dwarf_regnum(loongarch_regstr_tbl.as_ptr(), loongarch_regstr_tbl.len(), name)
            };
        }
        _ => unsafe {
            pr_err(c"ELF MACHINE %x is not supported.\n".as_ptr(), machine);
        },
    }
    unsafe {
        free(regname.cast());
    }
    reg
}

unsafe fn get_libdw_frame_nregs(machine: c_uint, flags: c_uint) -> c_int {
    let _ = flags;

    match machine {
        EM_X86_64 => 17,
        EM_386 => 9,
        EM_ARM => 16,
        EM_AARCH64 => 97,
        EM_CSKY => 38,
        EM_S390 => 32,
        EM_PPC | EM_PPC64 => 145,
        EM_RISCV => 66,
        EM_SPARC | EM_SPARCV9 => 103,
        EM_LOONGARCH => 74,
        EM_MIPS => 71,
        _ => 0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_dwarf_regnum_for_perf_regnum(
    perf_regnum: c_int,
    machine: c_uint,
    flags: c_uint,
    only_libdw_supported: bool,
) -> c_int {
    let mut reg: c_int;

    match machine {
        EM_X86_64 => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum) };
        }
        EM_386 => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum) };
        }
        EM_ARM => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum) };
        }
        EM_AARCH64 => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum) };
        }
        EM_CSKY => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_csky(perf_regnum, flags) };
        }
        EM_PPC | EM_PPC64 => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum) };
        }
        EM_RISCV => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum) };
        }
        EM_S390 => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum) };
        }
        EM_LOONGARCH => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum) };
        }
        EM_MIPS => {
            reg = unsafe { __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum) };
        }
        _ => unsafe {
            pr_err(c"ELF MACHINE %x is not supported.\n".as_ptr(), machine);
            return -ENOENT;
        },
    }
    if reg >= 0 && only_libdw_supported {
        let nregs = unsafe { get_libdw_frame_nregs(machine, flags) };

        if reg >= nregs {
            reg = -ENOENT;
        }
    }
    reg
}
