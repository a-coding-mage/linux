/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

use core::arch::asm;

#[repr(C)]
pub struct gs_cb {
    pub reserved: u64,
    pub gsd: u64,
    pub gssm: u64,
    pub gs_epl_a: u64,
}

#[repr(C)]
pub struct gs_epl_flags_eam {
    pub value: u8,
}

#[repr(C)]
pub union gs_epl_eam {
    pub gs_eam: u8,
    pub flags: gs_epl_flags_eam,
}

#[repr(C)]
pub struct gs_epl_flags_eci {
    pub value: u8,
}

#[repr(C)]
pub union gs_epl_eci {
    pub gs_eci: u8,
    pub flags: gs_epl_flags_eci,
}

#[repr(C)]
pub struct gs_epl_flags_eai {
    pub value: u8,
}

#[repr(C)]
pub union gs_epl_eai {
    pub gs_eai: u8,
    pub flags: gs_epl_flags_eai,
}

#[repr(C)]
pub struct gs_epl {
    pub pad1: u8,
    pub eam: gs_epl_eam,
    pub eci: gs_epl_eci,
    pub eai: gs_epl_eai,
    pub pad2: u32,
    pub gs_eha: u64,
    pub gs_eia: u64,
    pub gs_eoa: u64,
    pub gs_eir: u64,
    pub gs_era: u64,
}

pub const GS_ENABLE: i32 = 0;
pub const GS_DISABLE: i32 = 1;
pub const GS_SET_BC_CB: i32 = 2;
pub const GS_CLEAR_BC_CB: i32 = 3;
pub const GS_BROADCAST: i32 = 4;

#[inline]
pub unsafe fn load_gs_cb(gs_cb: *mut gs_cb) {
    // Corresponds to the s390 .insn rxy instruction with a memory operand.
    asm!(".insn rxy,0xe3000000004d,0,{0}", in(reg) gs_cb, options(nostack));
}

#[inline]
pub unsafe fn store_gs_cb(gs_cb: *mut gs_cb) {
    // Corresponds to the s390 .insn rxy instruction with a memory operand.
    asm!(".insn rxy,0xe30000000049,0,{0}", in(reg) gs_cb, options(nostack));
}

#[inline]
pub unsafe fn save_gs_cb(gs_cb: *mut gs_cb) {
    if !gs_cb.is_null() {
        store_gs_cb(gs_cb);
    }
}

#[inline]
pub unsafe fn restore_gs_cb(gs_cb: *mut gs_cb) {
    if !gs_cb.is_null() {
        load_gs_cb(gs_cb);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
