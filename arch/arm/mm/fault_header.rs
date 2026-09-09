/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Fault status register encodings. We steal bit 31 for our own purposes.
 */
pub const FSR_LNX_PF: u32 = 1u32 << 31;
pub const FSR_CM: u32 = 1u32 << 13;
pub const FSR_WRITE: u32 = 1u32 << 11;

#[cfg(CONFIG_ARM_LPAE)]
pub const FSR_FS_AEA: u32 = 17;
#[cfg(CONFIG_ARM_LPAE)]
pub const FS_TRANS_NOLL: u32 = 0x4;
#[cfg(CONFIG_ARM_LPAE)]
pub const FS_PERM_NOLL: u32 = 0xC;
#[cfg(CONFIG_ARM_LPAE)]
pub const FS_MMU_NOLL_MASK: u32 = 0x3C;
#[cfg(CONFIG_ARM_LPAE)]
pub const FSR_FS5_0: u32 = 0x3F;

#[cfg(CONFIG_ARM_LPAE)]
#[inline]
pub fn fsr_fs(fsr: u32) -> i32 {
    (fsr & FSR_FS5_0) as i32
}

#[cfg(CONFIG_ARM_LPAE)]
#[inline]
pub fn is_translation_fault(fsr: u32) -> bool {
    let fs = fsr_fs(fsr);
    ((fs as u32) & FS_MMU_NOLL_MASK) == FS_TRANS_NOLL
}

#[cfg(CONFIG_ARM_LPAE)]
#[inline]
pub fn is_permission_fault(fsr: u32) -> bool {
    let fs = fsr_fs(fsr);
    ((fs as u32) & FS_MMU_NOLL_MASK) == FS_PERM_NOLL
}

#[cfg(not(CONFIG_ARM_LPAE))]
pub const FSR_FS_AEA: u32 = 22;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FS_L1_TRANS: u32 = 0x5;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FS_L2_TRANS: u32 = 0x7;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FS_L1_PERM: u32 = 0xD;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FS_L2_PERM: u32 = 0xF;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FSR_FS4: u32 = 1u32 << 10;
#[cfg(not(CONFIG_ARM_LPAE))]
pub const FSR_FS3_0: u32 = 0xF;

#[cfg(not(CONFIG_ARM_LPAE))]
#[inline]
pub fn fsr_fs(fsr: u32) -> i32 {
    ((fsr & FSR_FS3_0) | ((fsr & FSR_FS4) >> 6)) as i32
}

#[cfg(not(CONFIG_ARM_LPAE))]
#[inline]
pub fn is_translation_fault(fsr: u32) -> bool {
    let fs = fsr_fs(fsr) as u32;
    fs == FS_L1_TRANS || fs == FS_L2_TRANS
}

#[cfg(not(CONFIG_ARM_LPAE))]
#[inline]
pub fn is_permission_fault(fsr: u32) -> bool {
    let fs = fsr_fs(fsr) as u32;
    fs == FS_L1_PERM || fs == FS_L2_PERM
}

extern "C" {
    pub fn do_bad_area(addr: usize, fsr: u32, regs: *mut pt_regs);
    pub fn early_abt_enable();
    /* asmlinkage */
    pub fn do_DataAbort(addr: usize, fsr: u32, regs: *mut pt_regs);
    /* asmlinkage */
    pub fn do_PrefetchAbort(addr: usize, ifsr: u32, regs: *mut pt_regs);
}

pub enum pt_regs {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
