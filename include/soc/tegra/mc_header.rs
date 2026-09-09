/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2014-2026 NVIDIA Corporation */

// Linux dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct tegra_bpmp { _private: [u8; 0] }
#[repr(C)] pub struct icc_node { _private: [u8; 0] }
#[repr(C)] pub struct icc_node_data { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { _private: [u8; 0] }
#[repr(C)] pub struct reset_controller_dev { _private: [u8; 0] }
#[repr(C)] pub struct icc_provider { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
pub type spinlock_t = c_void;
pub type irq_handler_t = Option<unsafe extern "C" fn() -> c_int>;
pub type phys_addr_t = u64;
pub type tegra_icc_client_type = c_int;

#[repr(C)] pub struct tegra_mc_timing { pub rate: c_ulong, pub emem_data: *mut u32 }
pub type c_ulong = usize;

#[repr(C)] pub union tegra_mc_client_swgroup_sid { pub swgroup: u32, pub sid: u32 }
#[repr(C)] pub struct tegra_mc_client_smmu { pub reg: u32, pub bit: u32 }
#[repr(C)] pub struct tegra_mc_client_la { pub reg: u32, pub shift: u32, pub mask: u32, pub def: u32 }
#[repr(C)] pub struct tegra_mc_client_sid { pub override_: u32, pub security: u32 }
#[repr(C)] pub struct tegra_mc_client_regs { pub smmu: tegra_mc_client_smmu, pub la: tegra_mc_client_la, pub sid: tegra_mc_client_sid }
#[repr(C)] pub struct tegra_mc_client { pub id: u32, pub bpmp_id: u32, pub type_: tegra_icc_client_type, pub name: *const c_char, pub swgroup_sid: tegra_mc_client_swgroup_sid, pub fifo_size: u32, pub regs: tegra_mc_client_regs }

#[repr(C)] pub struct tegra_smmu_swgroup { pub name: *const c_char, pub swgroup: u32, pub reg: u32 }
#[repr(C)] pub struct tegra_smmu_group_soc { pub name: *const c_char, pub swgroups: *const u32, pub num_swgroups: u32 }
#[repr(C)] pub struct tegra_smmu_soc {
    pub clients: *const tegra_mc_client, pub num_clients: u32,
    pub swgroups: *const tegra_smmu_swgroup, pub num_swgroups: u32,
    pub groups: *const tegra_smmu_group_soc, pub num_groups: u32,
    pub supports_round_robin_arbitration: bool, pub supports_request_limit: bool,
    pub num_tlb_lines: u32, pub num_asids: u32,
}

#[repr(C)] pub struct tegra_smmu { _private: [u8; 0] }

#[cfg(feature = "CONFIG_TEGRA_IOMMU_SMMU")]
unsafe extern "C" { pub fn tegra_smmu_probe(dev: *mut device, soc: *const tegra_smmu_soc, mc: *mut tegra_mc) -> *mut tegra_smmu; pub fn tegra_smmu_remove(smmu: *mut tegra_smmu); }
#[cfg(not(feature = "CONFIG_TEGRA_IOMMU_SMMU"))]
pub unsafe fn tegra_smmu_probe(_: *mut device, _: *const tegra_smmu_soc, _: *mut tegra_mc) -> *mut tegra_smmu { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_TEGRA_IOMMU_SMMU"))]
pub unsafe fn tegra_smmu_remove(_: *mut tegra_smmu) {}

#[repr(C)] pub struct tegra_mc_reset { pub name: *const c_char, pub id: c_ulong, pub control: u32, pub status: u32, pub reset: u32, pub bit: u32 }
#[repr(C)] pub struct tegra_mc_reset_ops { pub hotreset_assert: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> c_int>, pub hotreset_deassert: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> c_int>, pub block_dma: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> c_int>, pub dma_idling: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> bool>, pub unblock_dma: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> c_int>, pub reset_status: Option<unsafe extern "C" fn(*mut tegra_mc, *const tegra_mc_reset) -> c_int> }

pub const TEGRA_MC_ICC_TAG_DEFAULT: u32 = 0;
pub const TEGRA_MC_ICC_TAG_ISO: u32 = 1 << 0;
#[repr(C)] pub struct tegra_mc_icc_ops { pub set: Option<unsafe extern "C" fn(*mut icc_node, *mut icc_node) -> c_int>, pub aggregate: Option<unsafe extern "C" fn(*mut icc_node, u32, u32, u32, *mut u32, *mut u32) -> c_int>, pub xlate: Option<unsafe extern "C" fn(*const of_phandle_args, *mut c_void) -> *mut icc_node>, pub xlate_extended: Option<unsafe extern "C" fn(*const of_phandle_args, *mut c_void) -> *mut icc_node_data>, pub get_bw: Option<unsafe extern "C" fn(*mut icc_node, *mut u32, *mut u32) -> c_int> }
unsafe extern "C" { pub fn tegra_mc_icc_xlate(spec: *const of_phandle_args, data: *mut c_void) -> *mut icc_node; pub static tegra_mc_icc_ops: tegra_mc_icc_ops; }

#[repr(C)] pub struct tegra_mc_ops { pub probe: Option<unsafe extern "C" fn(*mut tegra_mc) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut tegra_mc)>, pub resume: Option<unsafe extern "C" fn(*mut tegra_mc)>, pub probe_device: Option<unsafe extern "C" fn(*mut tegra_mc, *mut device) -> c_int> }
#[repr(C)] pub struct tegra_mc_regs { pub cfg_channel_enable: u32, pub err_status: u32, pub err_add: u32, pub err_add_hi: u32, pub err_vpr_status: u32, pub err_vpr_add: u32, pub err_sec_status: u32, pub err_sec_add: u32, pub err_mts_status: u32, pub err_mts_add: u32, pub err_gen_co_status: u32, pub err_gen_co_add: u32, pub err_route_status: u32, pub err_route_add: u32 }
#[repr(C)] pub struct tegra_mc_intmask { pub reg: u32, pub mask: u32 }
#[repr(C)] pub struct tegra_mc_soc { pub clients: *const tegra_mc_client, pub num_clients: u32, pub emem_regs: *const c_ulong, pub num_emem_regs: u32, pub num_address_bits: u32, pub atom_size: u32, pub num_carveouts: u32, pub client_id_mask: u16, pub num_channels: u8, pub smmu: *const tegra_smmu_soc, pub ch_intmask: u32, pub global_intstatus_channel_shift: u32, pub has_addr_hi_reg: bool, pub reset_ops: *const tegra_mc_reset_ops, pub resets: *const tegra_mc_reset, pub num_resets: u32, pub icc_ops: *const tegra_mc_icc_ops, pub ops: *const tegra_mc_ops, pub regs: *const tegra_mc_regs, pub handle_irq: *const irq_handler_t, pub num_interrupts: u32, pub mc_addr_hi_mask: u32, pub mc_err_status_type_mask: u32, pub intmasks: *const tegra_mc_intmask, pub num_intmasks: u32 }
#[repr(C)] pub struct tegra_mc_debugfs { pub root: *mut dentry }
#[repr(C)] pub struct tegra_mc { pub bpmp: *mut tegra_bpmp, pub dev: *mut device, pub smmu: *mut tegra_smmu, pub regs: *mut c_void, pub bcast_ch_regs: *mut c_void, pub ch_regs: *mut *mut c_void, pub clk: *mut clk, pub soc: *const tegra_mc_soc, pub tick: c_ulong, pub timings: *mut tegra_mc_timing, pub num_timings: u32, pub num_channels: u32, pub bwmgr_mrq_supported: bool, pub reset: reset_controller_dev, pub provider: icc_provider, pub lock: spinlock_t, pub debugfs: tegra_mc_debugfs }

unsafe extern "C" { pub fn tegra_mc_write_emem_configuration(mc: *mut tegra_mc, rate: c_ulong) -> c_int; pub fn tegra_mc_get_emem_device_count(mc: *mut tegra_mc) -> u32; pub static tegra20_mc_regs: tegra_mc_regs; }

#[cfg(feature = "CONFIG_TEGRA_MC")]
unsafe extern "C" {
    pub fn devm_tegra_memory_controller_get(dev: *mut device) -> *mut tegra_mc;
    pub fn tegra_mc_probe_device(mc: *mut tegra_mc, dev: *mut device) -> c_int;
    pub fn tegra_mc_get_carveout_info(mc: *mut tegra_mc, id: u32, base: *mut phys_addr_t, size: *mut u64) -> c_int;
}
#[cfg(not(feature = "CONFIG_TEGRA_MC"))]
pub unsafe fn devm_tegra_memory_controller_get(_: *mut device) -> *mut tegra_mc { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_TEGRA_MC"))]
pub unsafe fn tegra_mc_probe_device(_: *mut tegra_mc, _: *mut device) -> c_int { -19 }
#[cfg(not(feature = "CONFIG_TEGRA_MC"))]
pub unsafe fn tegra_mc_get_carveout_info(_: *mut tegra_mc, _: u32, _: *mut phys_addr_t, _: *mut u64) -> c_int { -19 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
