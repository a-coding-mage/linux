/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Derived from arch/arm/kvm/coproc.h
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Authors: Christoffer Dall <c.dall@virtualopensystems.com>
 */

#[repr(C)]
pub struct sys_reg_params {
    pub Op0: u8, pub Op1: u8, pub CRn: u8, pub CRm: u8, pub Op2: u8,
    pub regval: u64, pub is_write: bool,
}

#[repr(C)]
pub struct sys_reg_desc {
    pub name: *const core::ffi::c_char,
    pub aarch32_map: Aarch32Map,
    pub Op0: u8, pub Op1: u8, pub CRn: u8, pub CRm: u8, pub Op2: u8,
    pub access: Option<unsafe extern "C" fn(*mut kvm_vcpu, *mut sys_reg_params, *const sys_reg_desc) -> bool>,
    pub reset: Option<unsafe extern "C" fn(*mut kvm_vcpu, *const sys_reg_desc) -> u64>,
    pub reg: i32,
    pub val: u64,
    pub get_user: Option<unsafe extern "C" fn(*mut kvm_vcpu, *const sys_reg_desc, *mut u64) -> i32>,
    pub set_user: Option<unsafe extern "C" fn(*mut kvm_vcpu, *const sys_reg_desc, u64) -> i32>,
    pub visibility: Option<unsafe extern "C" fn(*const kvm_vcpu, *const sys_reg_desc) -> u32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Aarch32Map { AA32_DIRECT, AA32_LO, AA32_HI }

pub const REG_HIDDEN: u32 = 1 << 0;
pub const REG_RAZ: u32 = 1 << 1;
pub const REG_USER_WI: u32 = 1 << 2;

extern "C" {
    pub fn sys_reg(op0: u32, op1: u32, crn: u32, crm: u32, op2: u32) -> u32;
    pub fn sys_reg_Op0(reg: u64) -> u32; pub fn sys_reg_Op1(reg: u64) -> u32;
    pub fn sys_reg_CRn(reg: u64) -> u32; pub fn sys_reg_CRm(reg: u64) -> u32;
    pub fn sys_reg_Op2(reg: u64) -> u32;
    pub fn __vcpu_assign_sys_reg(vcpu: *mut kvm_vcpu, reg: i32, val: u64);
    pub fn __vcpu_sys_reg(vcpu: *mut kvm_vcpu, reg: i32) -> u64;
    pub fn MPIDR_LEVEL_SHIFT(level: u32) -> u32;
    pub fn __inline_bsearch(key: *const core::ffi::c_void, base: *const sys_reg_desc, num: usize, size: usize,
                            cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32) -> *const sys_reg_desc;
}

pub const AA32_DIRECT: Aarch32Map = Aarch32Map::AA32_DIRECT;
pub const AA32_LO: Aarch32Map = Aarch32Map::AA32_LO;
pub const AA32_HI: Aarch32Map = Aarch32Map::AA32_HI;

#[inline] pub unsafe fn reg_to_encoding(x: *const sys_reg_desc) -> u32 {
    sys_reg((*x).Op0 as u32, (*x).Op1 as u32, (*x).CRn as u32, (*x).CRm as u32, (*x).Op2 as u32)
}

#[inline] pub unsafe fn encoding_to_params(reg: u64) -> sys_reg_params { sys_reg_params {
    Op0: sys_reg_Op0(reg) as u8, Op1: sys_reg_Op1(reg) as u8, CRn: sys_reg_CRn(reg) as u8,
    CRm: sys_reg_CRm(reg) as u8, Op2: sys_reg_Op2(reg) as u8, regval: 0, is_write: false,
} }

#[inline] pub fn esr_sys64_to_params(esr: u64) -> sys_reg_params { sys_reg_params {
    Op0: ((esr >> 20) & 3) as u8, Op1: ((esr >> 14) & 7) as u8, CRn: ((esr >> 10) & 15) as u8,
    CRm: ((esr >> 1) & 15) as u8, Op2: ((esr >> 17) & 7) as u8, regval: 0, is_write: esr & 1 == 0,
} }
#[inline] pub fn esr_cp1x_32_to_params(esr: u64) -> sys_reg_params { let mut p = esr_sys64_to_params(esr); p.Op0 = 0; p }

#[inline] pub unsafe fn in_feat_id_space(p: *mut sys_reg_params) -> bool {
    (*p).Op0 == 3 && (*p).Op1 & 4 == 0 && (*p).Op1 != 2 && (*p).CRn == 0 && (*p).CRm & 8 == 0
}

#[inline] pub unsafe fn ignore_write(_: *mut kvm_vcpu, _: *const sys_reg_params) -> bool { true }
#[inline] pub unsafe fn read_zero(_: *mut kvm_vcpu, p: *mut sys_reg_params) -> bool { (*p).regval = 0; true }

#[inline] pub unsafe fn reset_unknown(vcpu: *mut kvm_vcpu, r: *const sys_reg_desc) -> u64 {
    assert!((*r).reg != 0 && (*r).reg < NR_SYS_REGS as i32); __vcpu_assign_sys_reg(vcpu, (*r).reg, 0x1de7ec7edbadc0de); __vcpu_sys_reg(vcpu, (*r).reg)
}
#[inline] pub unsafe fn reset_val(vcpu: *mut kvm_vcpu, r: *const sys_reg_desc) -> u64 {
    assert!((*r).reg != 0 && (*r).reg < NR_SYS_REGS as i32); __vcpu_assign_sys_reg(vcpu, (*r).reg, (*r).val); __vcpu_sys_reg(vcpu, (*r).reg)
}
#[inline] pub unsafe fn sysreg_visibility(vcpu: *const kvm_vcpu, r: *const sys_reg_desc) -> u32 { r.as_ref().and_then(|x| x.visibility).map_or(0, |f| f(vcpu, r)) }
#[inline] pub unsafe fn sysreg_hidden(v: *const kvm_vcpu, r: *const sys_reg_desc) -> bool { sysreg_visibility(v,r) & REG_HIDDEN != 0 }
#[inline] pub unsafe fn sysreg_visible_as_raz(v: *const kvm_vcpu, r: *const sys_reg_desc) -> bool { sysreg_visibility(v,r) & REG_RAZ != 0 }
#[inline] pub unsafe fn sysreg_user_write_ignore(v: *const kvm_vcpu, r: *const sys_reg_desc) -> bool { sysreg_visibility(v,r) & REG_USER_WI != 0 }

pub const NR_SYS_REGS: usize = 0; // supplied by the surrounding translation unit
#[repr(C)] pub struct kvm_vcpu { pub vcpu_id: u64 }
#[repr(C)] pub struct kvm_one_reg { _private: [u8; 0] }

extern "C" {
    pub fn get_reg_by_id(id: u64, table: *const sys_reg_desc, num: u32) -> *const sys_reg_desc;
    pub fn kvm_arm_sys_reg_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    pub fn kvm_arm_sys_reg_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    pub fn kvm_sys_reg_get_user(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg, table: *const sys_reg_desc, num: u32) -> i32;
    pub fn kvm_sys_reg_set_user(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg, table: *const sys_reg_desc, num: u32) -> i32;
    pub fn triage_sysreg_trap(vcpu: *mut kvm_vcpu, sr_index: *mut i32) -> bool;
    pub fn kvm_finalize_sys_regs(vcpu: *mut kvm_vcpu) -> i32;
}

#[inline] pub unsafe fn kvm_calculate_mpidr(vcpu: *const kvm_vcpu) -> u64 {
    let id = (*vcpu).vcpu_id; (id & 0x0f) << MPIDR_LEVEL_SHIFT(0) | ((id >> 4) & 0xff) << MPIDR_LEVEL_SHIFT(1) | ((id >> 12) & 0xff) << MPIDR_LEVEL_SHIFT(2) | (1u64 << 31)
}

// C initializer and encoding helpers, retained as Rust macros for table declarations.
#[macro_export] macro_rules! AA32 { ($x:ident) => { aarch32_map: Aarch32Map::AA32_$x }; }
#[macro_export] macro_rules! Op0 { ($x:expr) => { Op0: $x }; }
#[macro_export] macro_rules! Op1 { ($x:expr) => { Op1: $x }; }
#[macro_export] macro_rules! CRn { ($x:expr) => { CRn: $x }; }
#[macro_export] macro_rules! CRm { ($x:expr) => { CRm: $x }; }
#[macro_export] macro_rules! Op2 { ($x:expr) => { Op2: $x }; }

#[inline] pub unsafe fn cmp_sys_reg(i1: *const sys_reg_desc, i2: *const sys_reg_desc) -> i32 {
    assert!(i1 != i2); if i1.is_null() { return 1 } if i2.is_null() { return -1 }
    ((*i1).Op0 as i32 - (*i2).Op0 as i32).then((*i1).Op1 as i32 - (*i2).Op1 as i32)
        .then((*i1).CRn as i32 - (*i2).CRn as i32).then((*i1).CRm as i32 - (*i2).CRm as i32)
        .then((*i1).Op2 as i32 - (*i2).Op2 as i32)
}
#[inline] pub unsafe fn match_sys_reg(key: *const core::ffi::c_void, elt: *const core::ffi::c_void) -> i32 {
    (key as usize as u64).wrapping_sub(reg_to_encoding(elt as *const sys_reg_desc) as u64) as i32
}
#[macro_export] macro_rules! SYS_DESC { ($reg:expr) => {
    name: concat!(stringify!($reg), "\0").as_ptr() as *const core::ffi::c_char,
    Op0: unsafe { sys_reg_Op0($reg) as u8 }, Op1: unsafe { sys_reg_Op1($reg) as u8 },
    CRn: unsafe { sys_reg_CRn($reg) as u8 }, CRm: unsafe { sys_reg_CRm($reg) as u8 }, Op2: unsafe { sys_reg_Op2($reg) as u8 }
} }
#[macro_export] macro_rules! CP15_SYS_DESC { ($reg:expr) => {
    name: concat!(stringify!($reg), "\0").as_ptr() as *const core::ffi::c_char,
    aarch32_map: Aarch32Map::AA32_DIRECT, Op0: 0, Op1: unsafe { sys_reg_Op1($reg) as u8 },
    CRn: unsafe { sys_reg_CRn($reg) as u8 }, CRm: unsafe { sys_reg_CRm($reg) as u8 }, Op2: unsafe { sys_reg_Op2($reg) as u8 }
} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
