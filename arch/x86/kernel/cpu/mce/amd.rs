// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level AMD MCA implementation translated from amd.c.  The surrounding
// kernel translation supplies the C-compatible types, globals, and helpers.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const NR_BLOCKS: usize = 5;
pub const THRESHOLD_MAX: u32 = 0x0fff;
pub const INT_TYPE_APIC: u32 = 0x0002_0000;
pub const MASK_VALID_HI: u32 = 0x8000_0000;
pub const MASK_CNTP_HI: u32 = 0x4000_0000;
pub const MASK_LOCKED_HI: u32 = 0x2000_0000;
pub const MASK_LVTOFF_HI: u32 = 0x00f0_0000;
pub const MASK_COUNT_EN_HI: u32 = 0x0008_0000;
pub const MASK_INT_TYPE_HI: u32 = 0x0006_0000;
pub const MASK_OVERFLOW_HI: u32 = 0x0001_0000;
pub const MASK_ERR_COUNT_HI: u32 = 0x0000_0fff;
pub const MASK_BLKPTR_LO: u32 = 0xff00_0000;
pub const MCG_XBLK_ADDR: u32 = 0xc000_0400;
pub const MSR_CU_DEF_ERR: u32 = 0xc000_0410;
pub const MASK_DEF_LVTOFF: u32 = 0x0000_00f0;
pub const SMCA_THR_LVT_OFF: u32 = 0xf000;

#[repr(C)]
pub struct mce_amd_cpu_data {
    pub thr_intr_banks: mce_banks_t,
    pub dfr_intr_banks: mce_banks_t,
    pub thr_intr_en: u32,
    pub dfr_intr_en: u32,
    pub __resv: u32,
}

#[repr(C)]
pub struct smca_hwid {
    pub bank_type: u32,
    pub hwid_mcatype: u32,
}

#[repr(C)]
pub struct smca_bank {
    pub hwid: *const smca_hwid,
    pub id: u32,
    pub sysfs_id: u8,
    pub paddrv: u64,
}

#[repr(C)]
pub struct threshold_block {
    pub block: u32,
    pub bank: u32,
    pub cpu: u32,
    pub address: u32,
    pub interrupt_enable: bool,
    pub interrupt_capable: bool,
    pub threshold_limit: u16,
    pub kobj: kobject,
    pub miscj: list_head,
}

#[repr(C)]
pub struct threshold_bank {
    pub kobj: *mut kobject,
    pub miscj: list_head,
}

extern "C" {
    pub static mut thresholding_irq_en: bool;
    pub static mut deferred_error_int_vector: Option<unsafe extern "C" fn()>;
    pub fn smca_get_bank_type(cpu: u32, bank: u32) -> u32;
}

// External kernel types are intentionally left as declarations supplied by
// the other translated kernel units.
pub type mce_banks_t = [u64; 1];
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

pub unsafe extern "C" fn default_deferred_error_interrupt() {
    // pr_err("Unexpected deferred interrupt at vector %x\\n", DEFERRED_ERROR_VECTOR);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
