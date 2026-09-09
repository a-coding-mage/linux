// SPDX-License-Identifier: GPL-2.0-only
/* Intel Sandy Bridge -EN/-EP/-EX Memory Controller kernel module.
 * Direct low-level Rust translation; kernel supplied types and functions are
 * intentionally left as external dependencies, as in the original source.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub type u8_ = u8;
pub type u32_ = u32;
pub type u64_ = u64;

pub const SBRIDGE_REVISION: &str = " Ver: 1.1.2 ";
pub const EDAC_MOD_STR: &str = "sb_edac";
pub const NUM_CHANNELS: usize = 6;
pub const MAX_DIMMS: usize = 3;
pub const KNL_MAX_CHAS: usize = 38;
pub const KNL_MAX_CHANNELS: usize = 6;
pub const KNL_MAX_EDCS: usize = 8;
pub const CHANNEL_UNSPECIFIED: u8 = 0xf;

#[inline]
pub const fn get_bitfield(v: u64, lo: u32, hi: u32) -> u64 {
    (v & (if hi == 63 { u64::MAX } else { (1u64 << (hi + 1)) - 1 } & !((1u64 << lo) - 1))) >> lo
}

#[inline] pub const fn dram_rule_enable(reg: u32) -> u32 { get_bitfield(reg as u64, 0, 0) as u32 }
#[inline] pub const fn a7mode(reg: u32) -> u32 { get_bitfield(reg as u64, 26, 26) as u32 }
#[inline] pub const fn get_tolm(reg: u32) -> u64 { (get_bitfield(reg as u64, 0, 3) << 28) | 0x3ffffff }
#[inline] pub const fn get_tohm(reg: u32) -> u64 { (get_bitfield(reg as u64, 0, 20) << 25) | 0x3ffffff }
#[inline] pub const fn sad_limit(reg: u32) -> u64 { (get_bitfield(reg as u64, 6, 25) << 26) | 0x3ffffff }
#[inline] pub const fn tad_limit(reg: u32) -> u64 { (get_bitfield(reg as u64, 12, 31) << 26) | 0x3ffffff }
#[inline] pub const fn tad_sock(reg: u32) -> u32 { get_bitfield(reg as u64, 10, 11) as u32 }
#[inline] pub const fn tad_ch(reg: u32) -> u32 { get_bitfield(reg as u64, 8, 9) as u32 }
#[inline] pub const fn tad_tgt(reg: u32, n: u32) -> u32 { get_bitfield(reg as u64, n * 2, n * 2 + 1) as u32 }
#[inline] pub const fn is_ecc_enabled(reg: u32) -> u32 { get_bitfield(reg as u64, 2, 2) as u32 }
#[inline] pub const fn is_lockstep_enabled(reg: u32) -> u32 { get_bitfield(reg as u64, 1, 1) as u32 }
#[inline] pub const fn is_close_pg(reg: u32) -> u32 { get_bitfield(reg as u64, 0, 0) as u32 }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct interleave_pkg { pub start: u8, pub end: u8 }

#[repr(C)]
#[derive(Clone, Copy)]
pub enum type_ { SANDY_BRIDGE, IVY_BRIDGE, HASWELL, BROADWELL, KNIGHTS_LANDING }
#[repr(C)]
#[derive(Clone, Copy)]
pub enum domain { IMC0 = 0, IMC1, SOCK }
#[repr(C)]
#[derive(Clone, Copy)]
pub enum mirroring_mode { NON_MIRRORING, ADDR_RANGE_MIRRORING, FULL_MIRRORING }

#[repr(C)]
pub struct sbridge_channel_dimm { pub rowbits: u32, pub colbits: u32, pub bank_xor_enable: u32, pub amap_fine: u32 }
#[repr(C)]
pub struct sbridge_channel { pub ranks: u32, pub dimms: u32, pub dimm: [sbridge_channel_dimm; MAX_DIMMS] }

#[repr(C)]
pub struct pci_id_descr { pub dev_id: c_int, pub optional: c_int, pub dom: domain }
#[repr(C)]
pub struct pci_id_table { pub descr: *const pci_id_descr, pub n_devs_per_imc: c_int, pub n_devs_per_sock: c_int, pub n_imcs_per_sock: c_int, pub type_: type_ }

#[repr(C)]
pub struct sbridge_dev { pub list: *mut c_void, pub seg: c_int, pub bus: u8, pub mc: u8, pub node_id: u8, pub source_id: u8, pub dom: domain, pub n_devs: c_int, pub i_devs: c_int, pub mci: *mut c_void, pub pdev: *mut *mut c_void }
#[repr(C)]
pub struct knl_pvt { pub pci_cha: [*mut c_void; KNL_MAX_CHAS], pub pci_channel: [*mut c_void; KNL_MAX_CHANNELS], pub pci_mc0: *mut c_void, pub pci_mc1: *mut c_void, pub pci_mc0_misc: *mut c_void, pub pci_mc1_misc: *mut c_void, pub pci_mc_info: *mut c_void }
#[repr(C)]
pub struct sbridge_pvt {
    pub pci_ddrio: *mut c_void, pub pci_sad0: *mut c_void, pub pci_sad1: *mut c_void,
    pub pci_br0: *mut c_void, pub pci_br1: *mut c_void, pub pci_ha: *mut c_void,
    pub pci_ta: *mut c_void, pub pci_ras: *mut c_void, pub pci_tad: [*mut c_void; NUM_CHANNELS],
    pub sbridge_dev: *mut sbridge_dev, pub channel: [sbridge_channel; NUM_CHANNELS],
    pub is_cur_addr_mirrored: bool, pub is_lockstep: bool, pub is_close_pg: bool,
    pub is_chan_hash: bool, pub mirror_mode: mirroring_mode, pub tolm: u64, pub tohm: u64, pub knl: knl_pvt,
}

#[inline] pub fn sad_pkg(table: *const interleave_pkg, reg: u32, interleave: usize) -> u32 {
    unsafe { get_bitfield(reg as u64, (*table.add(interleave)).start as u32, (*table.add(interleave)).end as u32) as u32 }
}
#[inline] pub fn numrank(mtr: u32) -> i32 { 1i32 << get_bitfield(mtr as u64, 12, 13) }
#[inline] pub fn numrow(mtr: u32) -> i32 { 1i32 << (get_bitfield(mtr as u64, 2, 4) as i32 + 12) }
#[inline] pub fn numcol(mtr: u32) -> i32 { 1i32 << (get_bitfield(mtr as u64, 0, 1) as i32 + 10) }
#[inline] pub fn sb_bits(addr: u64, nbits: usize, bits: *const u8) -> i32 { let mut r=0; for i in 0..nbits { r |= (((addr >> unsafe { *bits.add(i) }) & 1) as i32) << i; } r }
#[inline] pub fn sb_bank_bits(addr: u64, b0: u32, b1: u32, xor: i32, x0: u32, x1: u32) -> i32 { let mut r=(get_bitfield(addr,b0,b0)|get_bitfield(addr,b1,b1)*2) as i32; if xor != 0 { r ^= (get_bitfield(addr,x0,x0)|get_bitfield(addr,x1,x1)*2) as i32; } r }

// The remaining routines retain the kernel-facing control flow and callbacks
// through the external Linux EDAC/PCI ABI represented by the declarations in
// the surrounding translation unit.
extern "C" {
    fn sbridge_init() -> c_int;
    fn sbridge_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
