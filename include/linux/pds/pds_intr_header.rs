/* SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause */
/* Copyright(c) 2023 Advanced Micro Devices, Inc. */

#[repr(C)]
pub struct pds_core_intr {
    pub coal_init: u32,
    pub mask: u32,
    pub credits: u16,
    pub flags: u16,
    pub mask_on_assert: u32,
    pub coalescing_curr: u32,
    pub rsvd6: [u32; 3],
}

pub const PDS_CORE_INTR_F_UNMASK: u16 = 0x0001;
pub const PDS_CORE_INTR_F_TIMER_RESET: u16 = 0x0002;

pub const PDS_CORE_INTR_CTRL_REGS_MAX: u32 = 2048;
pub const PDS_CORE_INTR_CTRL_COAL_MAX: u32 = 0x3F;
pub const PDS_CORE_INTR_INDEX_NOT_ASSIGNED: i32 = -1;

#[repr(C)]
pub struct pds_core_intr_status {
    pub status: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_intr_mask_vals {
    PDS_CORE_INTR_MASK_CLEAR = 0,
    PDS_CORE_INTR_MASK_SET = 1,
}

pub const PDS_CORE_INTR_CRED_COUNT: u32 = 0x7fffu32;
pub const PDS_CORE_INTR_CRED_COUNT_SIGNED: u32 = 0xffffu32;
pub const PDS_CORE_INTR_CRED_UNMASK: u32 = 0x10000u32;
pub const PDS_CORE_INTR_CRED_RESET_COALESCE: u32 = 0x20000u32;
pub const PDS_CORE_INTR_CRED_REARM: u32 =
    PDS_CORE_INTR_CRED_UNMASK | PDS_CORE_INTR_CRED_RESET_COALESCE;

// These MMIO and warning interfaces are supplied by other dependencies.
extern "C" {
    fn ioread32(addr: *const u32) -> u32;
    fn iowrite32(value: u32, addr: *mut u32);
    fn warn_on_once(condition: bool) -> bool;
}

#[inline]
pub unsafe fn pds_core_intr_coal_init(intr_ctrl: *mut pds_core_intr, coal: u32) {
    iowrite32(coal, &mut (*intr_ctrl).coal_init);
}

#[inline]
pub unsafe fn pds_core_intr_mask(intr_ctrl: *mut pds_core_intr, mask: u32) {
    iowrite32(mask, &mut (*intr_ctrl).mask);
}

#[inline]
pub unsafe fn pds_core_intr_credits(
    intr_ctrl: *mut pds_core_intr,
    mut cred: u32,
    flags: u32,
) {
    if warn_on_once(cred > PDS_CORE_INTR_CRED_COUNT) {
        cred = ioread32(&(*intr_ctrl).credits as *const u16 as *const u32);
        cred &= PDS_CORE_INTR_CRED_COUNT_SIGNED;
    }

    iowrite32(cred | flags, &mut (*intr_ctrl).credits as *mut u16 as *mut u32);
}

#[inline]
pub unsafe fn pds_core_intr_clean_flags(intr_ctrl: *mut pds_core_intr, flags: u32) {
    let mut cred: u32 = ioread32(&(*intr_ctrl).credits as *const u16 as *const u32);
    cred &= PDS_CORE_INTR_CRED_COUNT_SIGNED;
    cred |= flags;
    iowrite32(cred, &mut (*intr_ctrl).credits as *mut u16 as *mut u32);
}

#[inline]
pub unsafe fn pds_core_intr_clean(intr_ctrl: *mut pds_core_intr) {
    pds_core_intr_clean_flags(intr_ctrl, PDS_CORE_INTR_CRED_RESET_COALESCE);
}

#[inline]
pub unsafe fn pds_core_intr_mask_assert(intr_ctrl: *mut pds_core_intr, mask: u32) {
    iowrite32(mask, &mut (*intr_ctrl).mask_on_assert);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
