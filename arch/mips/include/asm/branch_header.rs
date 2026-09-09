/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1998, 2001 by Ralf Baechle
 */

/* Dependencies supplied by the corresponding architecture headers. */
#[repr(C)]
pub struct pt_regs {
    pub cp0_cause: ::core::ffi::c_ulong,
    pub cp0_epc: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct mm_decoded_insn {
    _opaque: [u8; 0],
}

#[repr(C)]
pub union mips_instruction {
    _opaque: u8,
}

#[repr(C)]
pub union mips16e_instruction {
    pub ri: mips16e_instruction_ri,
    _opaque: u8,
}

#[repr(C)]
pub struct mips16e_instruction_ri {
    pub opcode: ::core::ffi::c_uint,
}

extern "C" {
    pub fn __isa_exception_epc(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn __compute_return_epc(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn __compute_return_epc_for_insn(
        regs: *mut pt_regs,
        insn: mips_instruction,
    ) -> ::core::ffi::c_int;
    pub fn __microMIPS_compute_return_epc(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn __MIPS16e_compute_return_epc(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn __mm_isBranchInstr(
        regs: *mut pt_regs,
        dec_insn: mm_decoded_insn,
        contpc: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    pub static cpu_has_mmips: ::core::ffi::c_int;
    pub static cpu_has_mips16: ::core::ffi::c_int;
}

pub const MM_POOL32A_MINOR_MASK: u32 = 0x3f;
pub const MM_POOL32A_MINOR_SHIFT: u32 = 0x6;
pub const MM_MIPS32_COND_FC: u32 = 0x30;

pub const BRANCH_LIKELY_TAKEN: u32 = 0x0001;

pub const CAUSEF_BD: ::core::ffi::c_ulong = 0x80000000;
pub const MIPS16e_extend_op: ::core::ffi::c_uint = 0;

extern "C" {
    pub fn isBranchInstr(
        regs: *mut pt_regs,
        dec_insn: mm_decoded_insn,
        contpc: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn get_isa16_mode(epc: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn mm_isBranchInstr(
    regs: *mut pt_regs,
    dec_insn: mm_decoded_insn,
    contpc: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    if cpu_has_mmips == 0 {
        return 0;
    }

    __mm_isBranchInstr(regs, dec_insn, contpc)
}

#[inline]
pub unsafe fn delay_slot(regs: *mut pt_regs) -> ::core::ffi::c_int {
    ((*regs).cp0_cause & CAUSEF_BD) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn clear_delay_slot(regs: *mut pt_regs) {
    (*regs).cp0_cause &= !CAUSEF_BD;
}

#[inline]
pub unsafe fn set_delay_slot(regs: *mut pt_regs) {
    (*regs).cp0_cause |= CAUSEF_BD;
}

#[inline]
pub unsafe fn exception_epc(regs: *mut pt_regs) -> ::core::ffi::c_ulong {
    if delay_slot(regs) == 0 {
        return (*regs).cp0_epc;
    }

    if get_isa16_mode((*regs).cp0_epc) != 0 {
        return __isa_exception_epc(regs) as ::core::ffi::c_ulong;
    }

    (*regs).cp0_epc.wrapping_add(4)
}

#[inline]
pub unsafe fn compute_return_epc(regs: *mut pt_regs) -> ::core::ffi::c_int {
    if get_isa16_mode((*regs).cp0_epc) != 0 {
        if cpu_has_mmips != 0 {
            return __microMIPS_compute_return_epc(regs);
        }
        if cpu_has_mips16 != 0 {
            return __MIPS16e_compute_return_epc(regs);
        }
    } else if delay_slot(regs) == 0 {
        (*regs).cp0_epc = (*regs).cp0_epc.wrapping_add(4);
        return 0;
    }

    __compute_return_epc(regs)
}

#[inline]
pub unsafe fn MIPS16e_compute_return_epc(
    regs: *mut pt_regs,
    inst: *mut mips16e_instruction,
) -> ::core::ffi::c_int {
    if delay_slot(regs) == 0 {
        if (*inst).ri.opcode == MIPS16e_extend_op {
            (*regs).cp0_epc = (*regs).cp0_epc.wrapping_add(4);
            return 0;
        }
        (*regs).cp0_epc = (*regs).cp0_epc.wrapping_add(2);
        return 0;
    }

    __MIPS16e_compute_return_epc(regs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
