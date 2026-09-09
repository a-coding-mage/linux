/* SPDX-License-Identifier: GPL-2.0 */
/* written by Philipp Rumpf, Copyright (C) 1999 SuSE GmbH Nuernberg
** Copyright (C) 2000 Grant Grundler, Hewlett-Packard
*/

// Dependencies supplied by the corresponding assembly and uapi headers:
// `TASK_REGS`, `PtRegs`, and `PRIV_KERNEL`.

#[inline]
pub unsafe fn task_regs(task: *mut core::ffi::c_void) -> *mut PtRegs {
    (task as *mut u8).add(TASK_REGS) as *mut PtRegs
}

#[inline]
pub const fn arch_has_single_step() -> i32 {
    1
}

#[inline]
pub const fn arch_has_block_step() -> i32 {
    1
}

/* XXX should we use iaoq[1] or iaoq[0] ? */
#[inline]
pub unsafe fn user_mode(regs: *const PtRegs) -> bool {
    ((*regs).iaoq[0] & 3) != PRIV_KERNEL
}

#[inline]
pub unsafe fn user_space(regs: *const PtRegs) -> bool {
    (*regs).iasq[1] != PRIV_KERNEL
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const PtRegs) -> u64 {
    (*regs).iaoq[0] & !3
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const PtRegs) -> u64 {
    (*regs).gr[30]
}

unsafe extern "C" {
    pub fn profile_pc(regs: *mut PtRegs) -> u64;
}

#[inline]
pub unsafe fn regs_return_value(regs: *mut PtRegs) -> u64 {
    (*regs).gr[28]
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut PtRegs, val: u64) {
    (*regs).iaoq[0] = val;
    (*regs).iaoq[1] = val.wrapping_add(4);
}

/* Query offset/name of register from its name/offset */
unsafe extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
    pub fn regs_query_register_name(offset: u32) -> *const core::ffi::c_char;
}

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(PtRegs, ipsw);

#[inline]
pub unsafe fn kernel_stack_pointer(regs: *const PtRegs) -> u64 {
    (*regs).gr[30]
}

#[inline]
pub unsafe fn regs_get_register(regs: *mut PtRegs, offset: u32) -> u64 {
    if offset as usize > MAX_REG_OFFSET {
        return 0;
    }
    *((regs as *mut u8).add(offset as usize) as *const u64)
}

unsafe extern "C" {
    pub fn regs_get_kernel_stack_nth(regs: *mut PtRegs, n: u32) -> u64;
    pub fn regs_within_kernel_stack(regs: *mut PtRegs, addr: u64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
