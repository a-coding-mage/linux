// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/perf_regs.c. C include dependencies are represented
// as external symbols or local C-compatible declarations where this file needs
// field access.

use core::ffi::{c_char, c_int};

pub type u64 = u64;

// From <errno.h>.
pub const EINVAL: c_int = 22;

// From <elf.h>.
pub const EM_386: u16 = 3;
pub const EM_MIPS: u16 = 8;
pub const EM_PPC: u16 = 20;
pub const EM_ARM: u16 = 40;
pub const EM_X86_64: u16 = 62;
pub const EM_S390: u16 = 22;
pub const EM_AARCH64: u16 = 183;
pub const EM_RISCV: u16 = 243;
pub const EM_CSKY: u16 = 252;
pub const EM_LOONGARCH: u16 = 258;

// From perf_regs.h / util/sample.h.
pub const PERF_SAMPLE_REGS_CACHE_SIZE: usize = 64;

unsafe extern "C" {
    static SDT_ARG_SKIP: c_int;

    fn __perf_sdt_arg_parse_op_arm64(old_op: *mut c_char, new_op: *mut *mut c_char) -> c_int;
    fn __perf_sdt_arg_parse_op_powerpc(old_op: *mut c_char, new_op: *mut *mut c_char) -> c_int;
    fn __perf_sdt_arg_parse_op_riscv(old_op: *mut c_char, new_op: *mut *mut c_char) -> c_int;
    fn __perf_sdt_arg_parse_op_x86(old_op: *mut c_char, new_op: *mut *mut c_char) -> c_int;
    fn __perf_sdt_arg_parse_op_s390(old_op: *mut c_char, new_op: *mut *mut c_char) -> c_int;

    fn __perf_reg_mask_arm(intr: bool) -> u64;
    fn __perf_reg_mask_arm64(intr: bool) -> u64;
    fn __perf_reg_mask_csky(intr: bool) -> u64;
    fn __perf_reg_mask_loongarch(intr: bool) -> u64;
    fn __perf_reg_mask_mips(intr: bool) -> u64;
    fn __perf_reg_mask_powerpc(intr: bool) -> u64;
    fn __perf_reg_mask_riscv(intr: bool) -> u64;
    fn __perf_reg_mask_s390(intr: bool) -> u64;
    fn __perf_reg_mask_x86(intr: bool) -> u64;

    fn __perf_reg_name_arm(id: c_int) -> *const c_char;
    fn __perf_reg_name_arm64(id: c_int) -> *const c_char;
    fn __perf_reg_name_csky(id: c_int, e_flags: u32) -> *const c_char;
    fn __perf_reg_name_loongarch(id: c_int) -> *const c_char;
    fn __perf_reg_name_mips(id: c_int) -> *const c_char;
    fn __perf_reg_name_powerpc(id: c_int) -> *const c_char;
    fn __perf_reg_name_riscv(id: c_int) -> *const c_char;
    fn __perf_reg_name_s390(id: c_int) -> *const c_char;
    fn __perf_reg_name_x86(id: c_int) -> *const c_char;

    fn __perf_reg_ip_arm() -> u64;
    fn __perf_reg_ip_arm64() -> u64;
    fn __perf_reg_ip_csky() -> u64;
    fn __perf_reg_ip_loongarch() -> u64;
    fn __perf_reg_ip_mips() -> u64;
    fn __perf_reg_ip_powerpc() -> u64;
    fn __perf_reg_ip_riscv() -> u64;
    fn __perf_reg_ip_s390() -> u64;
    fn __perf_reg_ip_x86() -> u64;

    fn __perf_reg_sp_arm() -> u64;
    fn __perf_reg_sp_arm64() -> u64;
    fn __perf_reg_sp_csky() -> u64;
    fn __perf_reg_sp_loongarch() -> u64;
    fn __perf_reg_sp_mips() -> u64;
    fn __perf_reg_sp_powerpc() -> u64;
    fn __perf_reg_sp_riscv() -> u64;
    fn __perf_reg_sp_s390() -> u64;
    fn __perf_reg_sp_x86() -> u64;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct regs_dump {
    pub abi: u64,
    pub mask: u64,
    pub regs: *mut u64,
    pub cache_regs: [u64; PERF_SAMPLE_REGS_CACHE_SIZE],
    pub cache_mask: u64,
}

const UNKNOWN_REG: &[u8] = b"unknown\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_sdt_arg_parse_op(
    e_machine: u16,
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int = unsafe { SDT_ARG_SKIP };

    match e_machine {
        EM_AARCH64 => {
            ret = unsafe { __perf_sdt_arg_parse_op_arm64(old_op, new_op) };
        }
        EM_PPC | EM_PPC64 => {
            ret = unsafe { __perf_sdt_arg_parse_op_powerpc(old_op, new_op) };
        }
        EM_RISCV => {
            ret = unsafe { __perf_sdt_arg_parse_op_riscv(old_op, new_op) };
        }
        EM_386 | EM_X86_64 => {
            ret = unsafe { __perf_sdt_arg_parse_op_x86(old_op, new_op) };
        }
        EM_S390 => {
            ret = unsafe { __perf_sdt_arg_parse_op_s390(old_op, new_op) };
        }
        _ => {
            unsafe {
                pr_debug(
                    c"Unknown ELF machine %d, standard arguments parse will be skipped.\n"
                        .as_ptr(),
                    e_machine as c_int,
                );
            }
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_intr_reg_mask(e_machine: u16) -> u64 {
    let mut mask: u64 = 0;

    match e_machine {
        EM_ARM => {
            mask = unsafe { __perf_reg_mask_arm(true) };
        }
        EM_AARCH64 => {
            mask = unsafe { __perf_reg_mask_arm64(true) };
        }
        EM_CSKY => {
            mask = unsafe { __perf_reg_mask_csky(true) };
        }
        EM_LOONGARCH => {
            mask = unsafe { __perf_reg_mask_loongarch(true) };
        }
        EM_MIPS => {
            mask = unsafe { __perf_reg_mask_mips(true) };
        }
        EM_PPC | EM_PPC64 => {
            mask = unsafe { __perf_reg_mask_powerpc(true) };
        }
        EM_RISCV => {
            mask = unsafe { __perf_reg_mask_riscv(true) };
        }
        EM_S390 => {
            mask = unsafe { __perf_reg_mask_s390(true) };
        }
        EM_386 | EM_X86_64 => {
            mask = unsafe { __perf_reg_mask_x86(true) };
        }
        _ => {
            unsafe {
                pr_debug(
                    c"Unknown ELF machine %d, interrupt sampling register mask will be empty.\n"
                        .as_ptr(),
                    e_machine as c_int,
                );
            }
        }
    }

    mask
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_user_reg_mask(e_machine: u16) -> u64 {
    let mut mask: u64 = 0;

    match e_machine {
        EM_ARM => {
            mask = unsafe { __perf_reg_mask_arm(false) };
        }
        EM_AARCH64 => {
            mask = unsafe { __perf_reg_mask_arm64(false) };
        }
        EM_CSKY => {
            mask = unsafe { __perf_reg_mask_csky(false) };
        }
        EM_LOONGARCH => {
            mask = unsafe { __perf_reg_mask_loongarch(false) };
        }
        EM_MIPS => {
            mask = unsafe { __perf_reg_mask_mips(false) };
        }
        EM_PPC | EM_PPC64 => {
            mask = unsafe { __perf_reg_mask_powerpc(false) };
        }
        EM_RISCV => {
            mask = unsafe { __perf_reg_mask_riscv(false) };
        }
        EM_S390 => {
            mask = unsafe { __perf_reg_mask_s390(false) };
        }
        EM_386 | EM_X86_64 => {
            mask = unsafe { __perf_reg_mask_x86(false) };
        }
        _ => {
            unsafe {
                pr_debug(
                    c"Unknown ELF machine %d, user sampling register mask will be empty.\n".as_ptr(),
                    e_machine as c_int,
                );
            }
        }
    }

    mask
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_reg_name(
    id: c_int,
    e_machine: u16,
    e_flags: u32,
) -> *const c_char {
    let mut reg_name: *const c_char = core::ptr::null();

    match e_machine {
        EM_ARM => {
            reg_name = unsafe { __perf_reg_name_arm(id) };
        }
        EM_AARCH64 => {
            reg_name = unsafe { __perf_reg_name_arm64(id) };
        }
        EM_CSKY => {
            reg_name = unsafe { __perf_reg_name_csky(id, e_flags) };
        }
        EM_LOONGARCH => {
            reg_name = unsafe { __perf_reg_name_loongarch(id) };
        }
        EM_MIPS => {
            reg_name = unsafe { __perf_reg_name_mips(id) };
        }
        EM_PPC | EM_PPC64 => {
            reg_name = unsafe { __perf_reg_name_powerpc(id) };
        }
        EM_RISCV => {
            reg_name = unsafe { __perf_reg_name_riscv(id) };
        }
        EM_S390 => {
            reg_name = unsafe { __perf_reg_name_s390(id) };
        }
        EM_386 | EM_X86_64 => {
            reg_name = unsafe { __perf_reg_name_x86(id) };
        }
        _ => {}
    }
    if !reg_name.is_null() {
        return reg_name;
    }

    unsafe {
        pr_debug(
            c"Failed to find register %d for ELF machine type %u\n".as_ptr(),
            id,
            e_machine as c_int,
        );
    }
    UNKNOWN_REG.as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_reg_value(
    valp: *mut u64,
    regs: *mut regs_dump,
    id: c_int,
) -> c_int {
    let mut idx: c_int = 0;
    let mask: u64 = unsafe { (*regs).mask };

    if (id as u64) >= PERF_SAMPLE_REGS_CACHE_SIZE as u64 {
        return -EINVAL;
    }

    if unsafe { (*regs).cache_mask } & (1_u64 << id) != 0 {
        unsafe { *valp = (*regs).cache_regs[id as usize] };
        return 0;
    }

    if mask & (1_u64 << id) == 0 {
        return -EINVAL;
    }

    let mut i: c_int = 0;
    while i < id {
        if mask & (1_u64 << i) != 0 {
            idx += 1;
        }
        i += 1;
    }

    unsafe {
        (*regs).cache_mask |= 1_u64 << id;
        (*regs).cache_regs[id as usize] = *(*regs).regs.add(idx as usize);
        *valp = (*regs).cache_regs[id as usize];
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_arch_reg_ip(e_machine: u16) -> u64 {
    match e_machine {
        EM_ARM => unsafe { __perf_reg_ip_arm() },
        EM_AARCH64 => unsafe { __perf_reg_ip_arm64() },
        EM_CSKY => unsafe { __perf_reg_ip_csky() },
        EM_LOONGARCH => unsafe { __perf_reg_ip_loongarch() },
        EM_MIPS => unsafe { __perf_reg_ip_mips() },
        EM_PPC | EM_PPC64 => unsafe { __perf_reg_ip_powerpc() },
        EM_RISCV => unsafe { __perf_reg_ip_riscv() },
        EM_S390 => unsafe { __perf_reg_ip_s390() },
        EM_386 | EM_X86_64 => unsafe { __perf_reg_ip_x86() },
        _ => {
            unsafe {
                pr_err(
                    c"Failed to find IP register for ELF machine type %u\n".as_ptr(),
                    e_machine as c_int,
                );
            }
            0
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_arch_reg_sp(e_machine: u16) -> u64 {
    match e_machine {
        EM_ARM => unsafe { __perf_reg_sp_arm() },
        EM_AARCH64 => unsafe { __perf_reg_sp_arm64() },
        EM_CSKY => unsafe { __perf_reg_sp_csky() },
        EM_LOONGARCH => unsafe { __perf_reg_sp_loongarch() },
        EM_MIPS => unsafe { __perf_reg_sp_mips() },
        EM_PPC | EM_PPC64 => unsafe { __perf_reg_sp_powerpc() },
        EM_RISCV => unsafe { __perf_reg_sp_riscv() },
        EM_S390 => unsafe { __perf_reg_sp_s390() },
        EM_386 | EM_X86_64 => unsafe { __perf_reg_sp_x86() },
        _ => {
            unsafe {
                pr_err(
                    c"Failed to find SP register for ELF machine type %u\n".as_ptr(),
                    e_machine as c_int,
                );
            }
            0
        }
    }
}
