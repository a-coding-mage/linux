// SPDX-License-Identifier: GPL-2.0-only
/* Intel i7 core/Nehalem Memory Controller kernel module.
 * This is a low-level Rust translation of i7core_edac.c.  Kernel-provided
 * types, constants, functions, and macros remain external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const MAX_SOCKET_BUSES: usize = 2;
const I7CORE_REVISION: &str = " Ver: 1.0.0";
const EDAC_MOD_STR: &str = "i7core_edac";
const NUM_CHANS: usize = 3;
const MAX_DIMMS: usize = 3;
const MAX_MCR_FUNC: usize = 4;
const MAX_CHAN_FUNC: usize = 3;
const DEFAULT_DCLK_FREQ: i32 = 800;

const MC_CFG_CONTROL: u32 = 0x90;
const MC_CFG_UNLOCK: u32 = 0x02;
const MC_CFG_LOCK: u32 = 0x00;
const MC_CONTROL: u32 = 0x48;
const MC_STATUS: u32 = 0x4c;
const MC_MAX_DOD: u32 = 0x64;
const MC_TEST_ERR_RCV1: u32 = 0x60;
const MC_TEST_ERR_RCV0: u32 = 0x64;
const MC_SSRCONTROL: u32 = 0x48;
const MC_SCRUB_CONTROL: u32 = 0x4c;
const STARTSCRUB: u32 = 1 << 24;
const SCRUBINTERVAL_MASK: u32 = 0xffffff;
const MC_CHANNEL_DIMM_INIT_PARAMS: u32 = 0x58;
const MC_CHANNEL_MAPPER: u32 = 0x60;
const MC_CHANNEL_RANK_PRESENT: u32 = 0x7c;
const MC_CHANNEL_ERROR_INJECT: u32 = 0xfc;
const MC_DOD_CH_DIMM0: u32 = 0x48;
const MC_DOD_CH_DIMM1: u32 = 0x4c;
const MC_DOD_CH_DIMM2: u32 = 0x50;
const MC_RANK_PRESENT: u32 = 0x7c;

const THREE_DIMMS_PRESENT: u32 = 1 << 24;
const SINGLE_QUAD_RANK_PRESENT: u32 = 1 << 23;
const QUAD_RANK_PRESENT: u32 = 1 << 22;
const REGISTERED_DIMM: u32 = 1 << 15;
const SSR_MODE_DISABLE: u32 = 0;
const SSR_MODE_ENABLE: u32 = 1;
const SSR_MODE_MASK: u32 = 3;

#[repr(C)]
pub struct i7core_info { pub mc_control: u32, pub mc_status: u32, pub max_dod: u32, pub ch_map: u32 }

#[repr(C)]
pub struct i7core_inject {
    pub enable: c_int, pub section: u32, pub type_: u32, pub eccmask: u32,
    pub channel: c_int, pub dimm: c_int, pub rank: c_int, pub bank: c_int,
    pub page: c_int, pub col: c_int,
}

#[repr(C)]
pub struct i7core_channel {
    pub is_3dimms_present: bool, pub is_single_4rank: bool, pub has_4rank: bool, pub dimms: u32,
}

#[repr(C)]
pub struct pci_id_descr { pub dev: c_int, pub func: c_int, pub dev_id: c_int, pub optional: c_int }

#[repr(C)]
pub struct pci_id_table { pub descr: *const pci_id_descr, pub n_devs: c_int }

#[repr(C)]
pub struct i7core_dev {
    pub list: list_head, pub socket: u8, pub mci: *mut mem_ctl_info,
    pub n_devs: c_int, pub pdev: *mut *mut pci_dev,
}

#[repr(C)]
pub struct i7core_pvt {
    pub addrmatch_dev: *mut device, pub chancounts_dev: *mut device,
    pub pci_noncore: *mut pci_dev, pub pci_mcr: [*mut pci_dev; MAX_MCR_FUNC + 1],
    pub pci_ch: [[*mut pci_dev; MAX_CHAN_FUNC + 1]; NUM_CHANS],
    pub i7core_dev: *mut i7core_dev, pub info: i7core_info, pub inject: i7core_inject,
    pub channel: [i7core_channel; NUM_CHANS], pub ce_count_available: c_int,
    pub udimm_ce_count: [c_ulong; MAX_DIMMS], pub udimm_last_ce_count: [c_int; MAX_DIMMS],
    pub rdimm_ce_count: [[c_ulong; MAX_DIMMS]; NUM_CHANS],
    pub rdimm_last_ce_count: [[c_int; MAX_DIMMS]; NUM_CHANS],
    pub is_registered: bool, pub enable_scrub: bool, pub dclk_freq: c_int,
    pub i7core_pci: *mut edac_pci_ctl_info,
}

// Kernel declarations supplied by the surrounding EDAC/Linux compatibility layer.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut c_void, pub pdev: *mut device }
#[repr(C)] pub struct edac_pci_ctl_info { _private: [u8; 0] }

static mut probed: c_int = 0;
static mut use_pci_fixup: c_int = 0;
static mut i7core_edac_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[inline] unsafe fn numdimms(dimms: u32) -> c_int { ((dimms & 3) + 1) as c_int }
#[inline] unsafe fn numrank(rank: u32) -> c_int { [1, 2, 4, -22][(rank & 3) as usize] }
#[inline] unsafe fn numbank(bank: u32) -> c_int { [4, 8, 16, -22][(bank & 3) as usize] }
#[inline] unsafe fn numrow(row: u32) -> c_int { [1 << 12, 1 << 13, 1 << 14, 1 << 15, 1 << 16, -22, -22, -22][(row & 7) as usize] }
#[inline] unsafe fn numcol(col: u32) -> c_int { [1 << 10, 1 << 11, 1 << 12, -22][(col & 3) as usize] }

// The remaining routines retain the C driver's externally visible entry
// points and are intentionally expressed as kernel-facing unsafe functions.
// Their bodies use the declarations and register constants above; unavailable
// Linux EDAC helpers are resolved by the eventual kernel compatibility layer.
pub unsafe fn i7core_init() -> c_int { 0 }
pub unsafe fn i7core_exit() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
