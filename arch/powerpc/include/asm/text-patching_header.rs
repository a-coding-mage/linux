/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from the PowerPC code-patching header. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const BRANCH_SET_LINK: i32 = 0x1;
pub const BRANCH_ABSOLUTE: i32 = 0x2;

#[inline]
pub unsafe fn is_offset_in_branch_range(offset: kernel::ffi::c_long) -> bool {
    offset >= -0x2000000 && offset <= 0x1fffffc && (offset & 0x3) == 0
}

#[inline]
pub unsafe fn is_offset_in_cond_branch_range(offset: kernel::ffi::c_long) -> bool {
    offset >= -0x8000 && offset <= 0x7fff && (offset & 0x3) == 0
}

#[inline]
pub unsafe fn create_branch(
    instr: *mut ppc_inst_t,
    addr: *const u32,
    target: kernel::ffi::c_ulong,
    flags: kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    let mut offset: kernel::ffi::c_long;
    *instr = ppc_inst(0);
    offset = target as kernel::ffi::c_long;
    if (flags & BRANCH_ABSOLUTE) == 0 {
        offset = offset.wrapping_sub(addr as kernel::ffi::c_ulong as kernel::ffi::c_long);
    }
    if !is_offset_in_branch_range(offset) {
        return 1;
    }
    *instr = ppc_inst(0x48000000 | ((flags & 0x3) as u32) | ((offset as u32) & 0x03fffffc));
    0
}

extern "C" {
    #[cfg(CONFIG_PPC64)]
    pub fn patch_uint(addr: *mut kernel::ffi::c_void, val: kernel::ffi::c_uint) -> kernel::ffi::c_int;
    #[cfg(CONFIG_PPC64)]
    pub fn patch_ulong(addr: *mut kernel::ffi::c_void, val: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
    pub fn create_cond_branch(instr: *mut ppc_inst_t, addr: *const u32, target: kernel::ffi::c_ulong, flags: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn patch_branch(addr: *mut u32, target: kernel::ffi::c_ulong, flags: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> kernel::ffi::c_int;
    pub fn raw_patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> kernel::ffi::c_int;
    pub fn patch_instructions(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> kernel::ffi::c_int;
    pub fn instr_is_relative_branch(instr: ppc_inst_t) -> kernel::ffi::c_int;
    pub fn instr_is_relative_link_branch(instr: ppc_inst_t) -> kernel::ffi::c_int;
    pub fn branch_target(instr: *const u32) -> kernel::ffi::c_ulong;
    pub fn translate_branch(instr: *mut ppc_inst_t, dest: *const u32, src: *const u32) -> kernel::ffi::c_int;
    pub fn is_conditional_branch(instr: ppc_inst_t) -> bool;
    pub fn kallsyms_lookup_name(name: *const kernel::ffi::c_char) -> kernel::ffi::c_ulong;
}

#[cfg(CONFIG_PPC64)]
pub use patch_ulong as patch_u64;
pub use patch_uint as patch_u32;

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub unsafe fn patch_uint(addr: *mut kernel::ffi::c_void, val: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    if !is_aligned(addr as kernel::ffi::c_ulong, core::mem::size_of::<kernel::ffi::c_uint>()) { return -EINVAL; }
    patch_instruction(addr as *mut u32, ppc_inst(val))
}

#[cfg(not(CONFIG_PPC64))]
#[inline]
pub unsafe fn patch_ulong(addr: *mut kernel::ffi::c_void, val: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    if !is_aligned(addr as kernel::ffi::c_ulong, core::mem::size_of::<kernel::ffi::c_ulong>()) { return -EINVAL; }
    patch_instruction(addr as *mut u32, ppc_inst(val))
}

#[inline]
pub unsafe fn patch_site_addr(site: *mut s32) -> kernel::ffi::c_ulong {
    (site as kernel::ffi::c_ulong).wrapping_add(*site as kernel::ffi::c_ulong)
}

#[inline]
pub unsafe fn patch_instruction_site(site: *mut s32, instr: ppc_inst_t) -> kernel::ffi::c_int {
    patch_instruction(patch_site_addr(site) as *mut u32, instr)
}

#[inline]
pub unsafe fn patch_branch_site(site: *mut s32, target: kernel::ffi::c_ulong, flags: kernel::ffi::c_int) -> kernel::ffi::c_int {
    patch_branch(patch_site_addr(site) as *mut u32, target, flags)
}

#[inline]
pub unsafe fn modify_instruction(addr: *mut kernel::ffi::c_uint, clr: kernel::ffi::c_uint, set: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    patch_instruction(addr as *mut u32, ppc_inst((*addr & !clr) | set))
}

#[inline]
pub unsafe fn modify_instruction_site(site: *mut s32, clr: kernel::ffi::c_uint, set: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    modify_instruction(patch_site_addr(site) as *mut kernel::ffi::c_uint, clr, set)
}

#[inline]
pub unsafe fn branch_opcode(instr: ppc_inst_t) -> kernel::ffi::c_uint { ppc_inst_primary_opcode(instr) & 0x3f }
#[inline]
pub unsafe fn instr_is_branch_iform(instr: ppc_inst_t) -> kernel::ffi::c_int { (branch_opcode(instr) == 18) as kernel::ffi::c_int }
#[inline]
pub unsafe fn instr_is_branch_bform(instr: ppc_inst_t) -> kernel::ffi::c_int { (branch_opcode(instr) == 16) as kernel::ffi::c_int }

pub const OP_RT_RA_MASK: kernel::ffi::c_ulong = 0xffff0000;
pub const LIS_R2: u32 = PPC_RAW_LIS(_R2, 0);
pub const ADDIS_R2_R12: u32 = PPC_RAW_ADDIS(_R2, _R12, 0);
pub const ADDI_R2_R2: u32 = PPC_RAW_ADDI(_R2, _R2, 0);
pub const R2_STACK_OFFSET: kernel::ffi::c_uint = if cfg!(CONFIG_PPC64_ELF_ABI_V2) { 24 } else { 40 };

#[inline]
pub unsafe fn ppc_function_entry(func: *mut kernel::ffi::c_void) -> kernel::ffi::c_ulong {
    #[cfg(CONFIG_PPC64_ELF_ABI_V2)]
    {
        let insn = func as *mut u32;
        if (((*insn as kernel::ffi::c_ulong & OP_RT_RA_MASK) == ADDIS_R2_R12) ||
            ((*insn as kernel::ffi::c_ulong & OP_RT_RA_MASK) == LIS_R2)) &&
            (*insn.add(1) as kernel::ffi::c_ulong & OP_RT_RA_MASK) == ADDI_R2_R2 {
            return insn.add(2) as kernel::ffi::c_ulong;
        }
    }
    #[cfg(CONFIG_PPC64_ELF_ABI_V1)]
    { return (*(func as *mut func_desc)).addr; }
    func as kernel::ffi::c_ulong
}

#[inline]
pub unsafe fn ppc_global_function_entry(func: *mut kernel::ffi::c_void) -> kernel::ffi::c_ulong {
    #[cfg(CONFIG_PPC64_ELF_ABI_V2)]
    { func as kernel::ffi::c_ulong }
    #[cfg(not(CONFIG_PPC64_ELF_ABI_V2))]
    { ppc_function_entry(func) }
}

#[inline]
pub unsafe fn ppc_kallsyms_lookup_name(name: *const kernel::ffi::c_char) -> kernel::ffi::c_ulong {
    kallsyms_lookup_name(name)
}

pub const PPC_INST_LD_TOC: u32 = PPC_RAW_LD(_R2, _R1, R2_STACK_OFFSET);
pub const PPC_INST_STD_LR: u32 = PPC_RAW_STD(_R0, _R1, PPC_LR_STKOFF);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
