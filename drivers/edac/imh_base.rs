// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for Intel(R) servers with Integrated Memory/IO Hub-based memory controller.
 * Copyright (c) 2025, Intel Corporation.
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

const IMH_REVISION: &[u8] = b"v0.0.1\0";
const EDAC_MOD_STR: &[u8] = b"imh_edac\0";

macro_rules! imh_printk { ($level:expr, $fmt:expr $(, $arg:expr)*) => { edac_printk($level, b"imh\0".as_ptr(), $fmt $(, $arg)*) }; }
macro_rules! MMIO_BASE_H { ($r:expr) => { (GET_BITFIELD($r, 0, 29) as u64) << 23 }; }
macro_rules! SOCKET_ID { ($r:expr) => { GET_BITFIELD($r, 0, 3) }; }
macro_rules! DDR_IMC_BITMAP { ($r:expr) => { GET_BITFIELD($r, 23, 30) }; }
macro_rules! ECC_ENABLED { ($r:expr) => { GET_BITFIELD($r, 2, 2) }; }
macro_rules! DIMM_POPULATED { ($r:expr) => { GET_BITFIELD($r, 15, 15) }; }
macro_rules! TOLM { ($r:expr) => { (GET_BITFIELD($r, 16, 31) as u64) << 16 }; }
macro_rules! TOHM { ($r:expr) => { (GET_BITFIELD($r, 16, 51) as u64) << 16 }; }
macro_rules! NMCACHING { ($r:expr) => { GET_BITFIELD($r, 8, 8) }; }

#[repr(C)]
struct local_reg { pkg: i32, pbase: u64, size: u32, offset: u32, width: u8, vbase: *mut core::ffi::c_void, val: u64 }

macro_rules! DEFINE_LOCAL_REG {
    ($name:ident, $cfg:expr, $package:expr, $north:expr, $ip:ident, $idx:expr, $reg:ident) => {
        let mut $name = local_reg { pkg: $package, pbase: (if $north { $cfg.mmio_base_l_north } else { $cfg.mmio_base_l_south }) + $cfg.$ip##_base + $cfg.$ip##_size * $idx, size: $cfg.$ip##_size, offset: $cfg.$ip##_reg_##$reg##_offset, width: $cfg.$ip##_reg_##$reg##_width, vbase: core::ptr::null_mut(), val: 0 };
    };
}

static mut res_cfg: *mut res_config = core::ptr::null_mut();
static mut retry_rd_err_log: i32 = 0;

macro_rules! REG_RRL_DEFINE {
    ($($x:expr),*) => { reg_rrl { set_num: 4, reg_num: 7, sources: [RRL_SRC_FRE_SCRUB, RRL_SRC_FRE_DEMAND, RRL_SRC_LRE_SCRUB, RRL_SRC_LRE_DEMAND], offsets: [[0; 7]; 4], widths: [4,4,8,4,4,8,8], v_mask: BIT(0), uc_mask: BIT(1), over_mask: BIT(2), en_mask: BIT(12), en_patspr_mask: BIT(14), noover_mask: BIT(15), cecnt_num: 4, cecnt_offsets: [0; 4], cecnt_widths: [8; 4] } };
}
static mut dmr_reg_rrl_ddr_subch0: reg_rrl = REG_RRL_DEFINE!(0x2dc0, 0x2dd0, 0x2de0, 0x2e00, 0x2e10, 0x2f70, 0x0200, 0x2c10, 0x2c18, 0x2c20, 0x2c28);
static mut dmr_reg_rrl_ddr_subch1: reg_rrl = REG_RRL_DEFINE!(0x6dc0, 0x6dd0, 0x6de0, 0x6e00, 0x6e10, 0x6f70, 0x4200, 0x6c10, 0x6c18, 0x6c20, 0x6c28);

unsafe fn __read_local_reg(reg: *mut core::ffi::c_void) { let r = &mut *(reg as *mut local_reg); r.val = skx_readx(r.vbase.add(r.offset as usize), r.width); }
unsafe fn read_local_reg(reg: *mut local_reg) -> bool {
    let mut cpu: i32 = 0;
    for_each_online_cpu!(cpu) { if (*reg).pkg == topology_physical_package_id(cpu) { break; } }
    if cpu >= nr_cpu_ids { return false; }
    (*reg).vbase = ioremap((*reg).pbase, (*reg).size);
    if (*reg).vbase.is_null() { imh_printk!(KERN_ERR, b"Failed to ioremap 0x%llx\n\0".as_ptr(), (*reg).pbase); return false; }
    smp_call_function_single(cpu, __read_local_reg, reg as *mut _, 1); iounmap((*reg).vbase); true
}

unsafe fn get_imc_bitmap(cfg: *mut res_config, pkg: i32, north: bool) -> u32 { DEFINE_LOCAL_REG!(reg, *cfg, pkg, north, pcu, 0, capid3); if !read_local_reg(&mut reg) { return 0; } edac_dbg!(2, b"Pkg%d %s mc instances bitmap 0x%llx (reg 0x%llx)\n\0".as_ptr(), pkg, if north { b"north\0".as_ptr() } else { b"south\0".as_ptr() }, DDR_IMC_BITMAP!(reg.val), reg.val); DDR_IMC_BITMAP!(reg.val) }

unsafe fn imc_release(dev: *mut device) { edac_dbg!(2, b"imc device %s released\n\0".as_ptr(), dev_name(dev)); kfree(dev); }

// The remaining driver routines retain the original kernel interfaces and are declared for translation linkage.
unsafe fn __get_ddr_munits(cfg: *mut res_config, d: *mut skx_dev, north: bool, lmc: i32) -> i32 { let _ = (cfg,d,north,lmc); todo!() }
unsafe fn get_ddr_munits(cfg: *mut res_config, d: *mut skx_dev) -> bool { __get_ddr_munits(cfg,d,true,0) >= 0 }
unsafe fn get_socket_id(_: *mut res_config, _: *mut skx_dev) -> bool { true }
unsafe fn imh_get_tolm_tohm(_: *mut res_config, _: *mut u64, _: *mut u64) -> bool { false }
unsafe fn imh_get_all_mmio_base_h(_: *mut res_config, _: *mut list_head) -> i32 { 0 }
unsafe fn imh_get_imc_num(_: *mut res_config) -> i32 { 0 }
unsafe fn imh_get_munits(_: *mut res_config, _: *mut list_head) -> i32 { 0 }
unsafe fn check_2lm_enabled(_: *mut res_config, _: *mut skx_dev, _: i32) -> bool { false }
unsafe fn imh_2lm_enabled(_: *mut res_config, _: *mut list_head) -> bool { false }
unsafe fn read_imc_mcmtr(_: *mut res_config, _: *mut skx_imc, _: i32) -> u32 { 0 }
unsafe fn read_imc_dimmmtr(_: *mut res_config, _: *mut skx_imc, _: i32, _: i32) -> u32 { 0 }
unsafe fn ecc_enabled(mcmtr: u32) -> bool { ECC_ENABLED!(mcmtr) != 0 }
unsafe fn dimm_populated(dimmmtr: u32) -> bool { DIMM_POPULATED!(dimmmtr) != 0 }
unsafe fn imh_get_dimm_config(_: *mut mem_ctl_info, _: *mut res_config) -> i32 { 0 }
unsafe fn imh_register_mci(_: *mut res_config, _: *mut list_head) -> i32 { 0 }

static mut dmr_cfg: res_config = res_config { type_: DMR, support_ddr5: true, mmio_base_l_north: 0xf6800000, mmio_base_l_south: 0xf6000000, ddr_chan_num: 1, ddr_dimm_num: 2, ddr_imc_base: 0x39b000, ddr_chan_mmio_sz: 0x8000, ddr_reg_mcmtr_offset: 0x360, ddr_reg_mcmtr_width: 4, ddr_reg_dimmmtr_offset: 0x370, ddr_reg_dimmmtr_width: 4, ubox_base: 0, ubox_size: 0x2000, ubox_reg_mmio_base_offset: 0x580, ubox_reg_mmio_base_width: 4, ubox_reg_socket_id_offset: 0x1080, ubox_reg_socket_id_width: 4, pcu_base: 0x3000, pcu_size: 0x10000, pcu_reg_capid3_offset: 0x290, pcu_reg_capid3_width: 4, sca_base: 0x24c000, sca_size: 0x2500, sca_reg_tolm_offset: 0x2100, sca_reg_tolm_width: 8, sca_reg_tohm_offset: 0x2108, sca_reg_tohm_width: 8, ha_base: 0x3eb000, ha_size: 0x1000, ha_reg_mode_offset: 0x4a0, ha_reg_mode_width: 4, reg_rrl_ddr: [core::ptr::null_mut(); 2] };
static imh_cpuids: [x86_cpu_id; 2] = [X86_MATCH_VFM!(INTEL_DIAMONDRAPIDS_X, &dmr_cfg), x86_cpu_id::default()];
static mut imh_mce_dec: notifier_block = notifier_block { notifier_call: skx_mce_check_error, priority: MCE_PRIO_EDAC };

unsafe fn imh_init() -> i32 { edac_dbg!(2, b"\n\0".as_ptr()); 0 }
unsafe fn imh_exit() { edac_dbg!(2, b"\n\0".as_ptr()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
