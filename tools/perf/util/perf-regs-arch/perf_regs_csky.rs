// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};
use core::ptr;

// C includes translated as external dependency intent:
// <elf.h>, "../perf_regs.h", and "../../arch/csky/include/perf_regs.h".
// __CSKYABIV2__ was forced to 1 in the C source to select V2 register definitions.

pub const EF_CSKY_ABIMASK: u32 = 0xF0000000;
pub const EF_CSKY_ABIV2: u32 = 0x20000000;

unsafe extern "C" {
    static PERF_REGS_MASK: u64;
    static PERF_REG_CSKY_A0: c_int;
    static PERF_REG_CSKY_A1: c_int;
    static PERF_REG_CSKY_A2: c_int;
    static PERF_REG_CSKY_A3: c_int;
    static PERF_REG_CSKY_REGS0: c_int;
    static PERF_REG_CSKY_REGS1: c_int;
    static PERF_REG_CSKY_REGS2: c_int;
    static PERF_REG_CSKY_REGS3: c_int;
    static PERF_REG_CSKY_REGS4: c_int;
    static PERF_REG_CSKY_REGS5: c_int;
    static PERF_REG_CSKY_REGS6: c_int;
    static PERF_REG_CSKY_REGS7: c_int;
    static PERF_REG_CSKY_REGS8: c_int;
    static PERF_REG_CSKY_REGS9: c_int;
    static PERF_REG_CSKY_SP: c_int;
    static PERF_REG_CSKY_LR: c_int;
    static PERF_REG_CSKY_PC: c_int;
    static PERF_REG_CSKY_EXREGS0: c_int;
    static PERF_REG_CSKY_EXREGS1: c_int;
    static PERF_REG_CSKY_EXREGS2: c_int;
    static PERF_REG_CSKY_EXREGS3: c_int;
    static PERF_REG_CSKY_EXREGS4: c_int;
    static PERF_REG_CSKY_EXREGS5: c_int;
    static PERF_REG_CSKY_EXREGS6: c_int;
    static PERF_REG_CSKY_EXREGS7: c_int;
    static PERF_REG_CSKY_EXREGS8: c_int;
    static PERF_REG_CSKY_EXREGS9: c_int;
    static PERF_REG_CSKY_EXREGS10: c_int;
    static PERF_REG_CSKY_EXREGS11: c_int;
    static PERF_REG_CSKY_EXREGS12: c_int;
    static PERF_REG_CSKY_EXREGS13: c_int;
    static PERF_REG_CSKY_EXREGS14: c_int;
    static PERF_REG_CSKY_TLS: c_int;
    static PERF_REG_CSKY_HI: c_int;
    static PERF_REG_CSKY_LO: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_csky(_intr: bool) -> u64 {
    unsafe { PERF_REGS_MASK }
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_csky(id: c_int, e_flags: u32) -> *const c_char {
    if id >= unsafe { PERF_REG_CSKY_EXREGS0 } && (e_flags & EF_CSKY_ABIMASK) == EF_CSKY_ABIV2 {
        return ptr::null();
    }

    if id == unsafe { PERF_REG_CSKY_A0 } {
        return b"a0\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_A1 } {
        return b"a1\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_A2 } {
        return b"a2\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_A3 } {
        return b"a3\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS0 } {
        return b"regs0\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS1 } {
        return b"regs1\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS2 } {
        return b"regs2\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS3 } {
        return b"regs3\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS4 } {
        return b"regs4\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS5 } {
        return b"regs5\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS6 } {
        return b"regs6\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS7 } {
        return b"regs7\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS8 } {
        return b"regs8\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_REGS9 } {
        return b"regs9\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_SP } {
        return b"sp\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_LR } {
        return b"lr\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_PC } {
        return b"pc\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS0 } {
        return b"exregs0\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS1 } {
        return b"exregs1\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS2 } {
        return b"exregs2\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS3 } {
        return b"exregs3\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS4 } {
        return b"exregs4\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS5 } {
        return b"exregs5\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS6 } {
        return b"exregs6\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS7 } {
        return b"exregs7\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS8 } {
        return b"exregs8\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS9 } {
        return b"exregs9\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS10 } {
        return b"exregs10\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS11 } {
        return b"exregs11\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS12 } {
        return b"exregs12\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS13 } {
        return b"exregs13\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_EXREGS14 } {
        return b"exregs14\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_TLS } {
        return b"tls\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_HI } {
        return b"hi\0".as_ptr() as *const c_char;
    }
    if id == unsafe { PERF_REG_CSKY_LO } {
        return b"lo\0".as_ptr() as *const c_char;
    }

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_csky() -> u64 {
    unsafe { PERF_REG_CSKY_PC as u64 }
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_csky() -> u64 {
    unsafe { PERF_REG_CSKY_SP as u64 }
}
