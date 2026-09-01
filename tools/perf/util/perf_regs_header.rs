/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/perf_regs.h. */
/* C dependencies removed from executable Rust: <linux/types.h>, <linux/compiler.h>. */

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct regs_dump {
    _unused: [u8; 0],
}

pub const SDT_ARG_VALID: c_int = 0;
pub const SDT_ARG_SKIP: c_int = 1;

extern "C" {
    pub fn perf_sdt_arg_parse_op(
        e_machine: u16,
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;
    pub fn perf_intr_reg_mask(e_machine: u16) -> u64;
    pub fn perf_user_reg_mask(e_machine: u16) -> u64;

    pub fn perf_reg_name(id: c_int, e_machine: u16, e_flags: u32) -> *const c_char;
    pub fn perf_reg_value(valp: *mut u64, regs: *mut regs_dump, id: c_int) -> c_int;
    pub fn perf_arch_reg_ip(e_machine: u16) -> u64;
    pub fn perf_arch_reg_sp(e_machine: u16) -> u64;

    pub fn __perf_sdt_arg_parse_op_arm64(
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;
    pub fn __perf_reg_mask_arm64(intr: bool) -> u64;
    pub fn __perf_reg_name_arm64(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_arm64() -> u64;
    pub fn __perf_reg_sp_arm64() -> u64;

    pub fn __perf_reg_mask_arm(intr: bool) -> u64;
    pub fn __perf_reg_name_arm(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_arm() -> u64;
    pub fn __perf_reg_sp_arm() -> u64;

    pub fn __perf_reg_mask_csky(intr: bool) -> u64;
    pub fn __perf_reg_name_csky(id: c_int, e_flags: u32) -> *const c_char;
    pub fn __perf_reg_ip_csky() -> u64;
    pub fn __perf_reg_sp_csky() -> u64;

    pub fn __perf_reg_mask_loongarch(intr: bool) -> u64;
    pub fn __perf_reg_name_loongarch(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_loongarch() -> u64;
    pub fn __perf_reg_sp_loongarch() -> u64;

    pub fn __perf_reg_mask_mips(intr: bool) -> u64;
    pub fn __perf_reg_name_mips(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_mips() -> u64;
    pub fn __perf_reg_sp_mips() -> u64;

    pub fn __perf_sdt_arg_parse_op_powerpc(
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;
    pub fn __perf_reg_mask_powerpc(intr: bool) -> u64;
    pub fn __perf_reg_name_powerpc(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_powerpc() -> u64;
    pub fn __perf_reg_sp_powerpc() -> u64;

    pub fn __perf_sdt_arg_parse_op_riscv(
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;
    pub fn __perf_reg_mask_riscv(intr: bool) -> u64;
    pub fn __perf_reg_name_riscv(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_riscv() -> u64;
    pub fn __perf_reg_sp_riscv() -> u64;

    pub fn __perf_reg_mask_s390(intr: bool) -> u64;
    pub fn __perf_reg_name_s390(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_s390() -> u64;
    pub fn __perf_reg_sp_s390() -> u64;
    pub fn __perf_sdt_arg_parse_op_s390(
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;

    pub fn __perf_sdt_arg_parse_op_x86(
        old_op: *mut c_char,
        new_op: *mut *mut c_char,
    ) -> c_int;
    pub fn __perf_reg_mask_x86(intr: bool) -> u64;
    pub fn __perf_reg_name_x86(id: c_int) -> *const c_char;
    pub fn __perf_reg_ip_x86() -> u64;
    pub fn __perf_reg_sp_x86() -> u64;
}

#[allow(non_snake_case)]
pub unsafe fn DWARF_MINIMAL_REGS(e_machine: u16) -> u64 {
    (1u64 << perf_arch_reg_ip(e_machine)) | (1u64 << perf_arch_reg_sp(e_machine))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
