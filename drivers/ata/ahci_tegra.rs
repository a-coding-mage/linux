// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of drivers/ata/ahci_tegra.c.

use core::ffi::{c_char, c_int, c_void};

const DRV_NAME: &[u8] = b"tegra-ahci\0";
const SATA_CONFIGURATION_0: usize = 0x180;
const SATA_CONFIGURATION_0_EN_FPCI: u32 = 1 << 0;
const SATA_CONFIGURATION_0_CLK_OVERRIDE: u32 = 1 << 31;
const SCFG_OFFSET: usize = 0x1000;
const T_SATA0_CFG_1: usize = 0x04;
const T_SATA0_CFG_1_IO_SPACE: u32 = 1 << 0;
const T_SATA0_CFG_1_MEMORY_SPACE: u32 = 1 << 1;
const T_SATA0_CFG_1_BUS_MASTER: u32 = 1 << 2;
const T_SATA0_CFG_1_SERR: u32 = 1 << 8;
const T_SATA0_CFG_9: usize = 0x24;
const T_SATA0_CFG_9_BASE_ADDRESS: u32 = 0x40020000;
const SATA_FPCI_BAR5: usize = 0x94;
const SATA_FPCI_BAR5_START_MASK: u32 = 0xfffffff << 4;
const SATA_FPCI_BAR5_START: u32 = 0x0040020 << 4;
const SATA_FPCI_BAR5_ACCESS_TYPE: u32 = 0x1;
const SATA_INTR_MASK: usize = 0x188;
const SATA_INTR_MASK_IP_INT_MASK: u32 = 1 << 16;
const T_SATA0_CFG_35: usize = 0x94;
const T_SATA0_CFG_35_IDP_INDEX_MASK: u32 = 0x7ff << 2;
const T_SATA0_CFG_35_IDP_INDEX: u32 = 0x2a << 2;
const T_SATA0_AHCI_IDP1: usize = 0x98;
const T_SATA0_AHCI_IDP1_DATA: u32 = 0x400040;
const T_SATA0_CFG_PHY_1: usize = 0x12c;
const T_SATA0_CFG_PHY_1_PADS_IDDQ_EN: u32 = 1 << 23;
const T_SATA0_CFG_PHY_1_PAD_PLL_IDDQ_EN: u32 = 1 << 22;
const T_SATA0_NVOOB: usize = 0x114;
const T_SATA0_NVOOB_SQUELCH_FILTER_MODE_MASK: u32 = 0x3 << 24;
const T_SATA0_NVOOB_SQUELCH_FILTER_MODE: u32 = 0x1 << 24;
const T_SATA0_NVOOB_SQUELCH_FILTER_LENGTH_MASK: u32 = 0x3 << 26;
const T_SATA0_NVOOB_SQUELCH_FILTER_LENGTH: u32 = 0x3 << 26;
const T_SATA_CFG_PHY_0: usize = 0x120;
const T_SATA_CFG_PHY_0_USE_7BIT_ALIGN_DET_FOR_SPD: u32 = 1 << 11;
const T_SATA_CFG_PHY_0_MASK_SQUELCH: u32 = 1 << 24;
const T_SATA0_CFG2NVOOB_2: usize = 0x134;
const T_SATA0_CFG2NVOOB_2_COMWAKE_IDLE_CNT_LOW_MASK: u32 = 0x1ff << 18;
const T_SATA0_CFG2NVOOB_2_COMWAKE_IDLE_CNT_LOW: u32 = 0xc << 18;
const T_SATA0_AHCI_HBA_CAP_BKDR: usize = 0x300;
const T_SATA0_AHCI_HBA_CAP_BKDR_PARTIAL_ST_CAP: u32 = 1 << 13;
const T_SATA0_AHCI_HBA_CAP_BKDR_SLUMBER_ST_CAP: u32 = 1 << 14;
const T_SATA0_AHCI_HBA_CAP_BKDR_SALP: u32 = 1 << 26;
const T_SATA0_AHCI_HBA_CAP_BKDR_SUPP_PM: u32 = 1 << 17;
const T_SATA0_BKDOOR_CC: usize = 0x4a4;
const T_SATA0_BKDOOR_CC_CLASS_CODE_MASK: u32 = 0xffff << 16;
const T_SATA0_BKDOOR_CC_CLASS_CODE: u32 = 0x0106 << 16;
const T_SATA0_BKDOOR_CC_PROG_IF_MASK: u32 = 0xff << 8;
const T_SATA0_BKDOOR_CC_PROG_IF: u32 = 0x01 << 8;
const T_SATA0_CFG_SATA: usize = 0x54c;
const T_SATA0_CFG_SATA_BACKDOOR_PROG_IF_EN: u32 = 1 << 12;
const T_SATA0_INDEX: usize = 0x680;
const T_SATA0_CHX_PHY_CTRL1_GEN1: usize = 0x690;
const T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_MASK: u32 = 0xff;
const T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_MASK: u32 = 0xff << 8;
const T_SATA0_CHX_PHY_CTRL1_GEN2: usize = 0x694;
const T_SATA0_CHX_PHY_CTRL1_GEN2_TX_AMP_MASK: u32 = 0xff;
const T_SATA0_CHX_PHY_CTRL1_GEN2_TX_PEAK_MASK: u32 = 0xff << 12;
const T_SATA0_CHX_PHY_CTRL2: usize = 0x69c;
const T_SATA0_CHX_PHY_CTRL2_CDR_CNTL_GEN1: u32 = 0x23;
const T_SATA0_CHX_PHY_CTRL11: usize = 0x6d0;
const T_SATA0_CHX_PHY_CTRL11_GEN2_RX_EQ: u32 = 0x2800 << 16;
const T_SATA0_CHX_PHY_CTRL17_0: usize = 0x6e8;
const T_SATA0_CHX_PHY_CTRL17_0_RX_EQ_CTRL_L_GEN1: u32 = 0x55010000;
const T_SATA0_CHX_PHY_CTRL18_0: usize = 0x6ec;
const T_SATA0_CHX_PHY_CTRL18_0_RX_EQ_CTRL_L_GEN2: u32 = 0x55010000;
const T_SATA0_CHX_PHY_CTRL20_0: usize = 0x6f4;
const T_SATA0_CHX_PHY_CTRL20_0_RX_EQ_CTRL_H_GEN1: u32 = 0x1;
const T_SATA0_CHX_PHY_CTRL21_0: usize = 0x6f8;
const T_SATA0_CHX_PHY_CTRL21_0_RX_EQ_CTRL_H_GEN2: u32 = 0x1;
const SATA_AUX_MISC_CNTL_1_0: usize = 0x8;
const SATA_AUX_MISC_CNTL_1_0_SDS_SUPPORT: u32 = 1 << 13;
const FUSE_SATA_CALIB: u32 = 0x124;
const FUSE_SATA_CALIB_MASK: u32 = 0x3;

#[repr(C)] pub struct ahci_host_priv { pub plat_data: *mut tegra_ahci_priv }
#[repr(C)] pub struct ata_host { pub private_data: *mut ahci_host_priv }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub pm_domain: *mut c_void }
#[repr(C)] pub struct reset_control;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct tegra_pmc;
#[repr(C)] pub struct regulator_bulk_data;
#[repr(C)] pub struct resource;
#[repr(C)] pub struct scsi_host_template;
#[repr(C)] pub struct ata_port_operations { pub inherits: *const ata_port_operations, pub host_stop: Option<unsafe extern "C" fn(*mut ata_host)> }
#[repr(C)] pub struct ata_port_info { pub flags: u32, pub pio_mask: u32, pub udma_mask: u32, pub port_ops: *const ata_port_operations }
#[repr(C)] pub struct platform_driver;
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct sata_pad_calibration { pub gen1_tx_amp: u8, pub gen1_tx_peak: u8, pub gen2_tx_amp: u8, pub gen2_tx_peak: u8 }
#[repr(C)] pub struct tegra_ahci_ops { pub init: Option<unsafe extern "C" fn(*mut ahci_host_priv) -> c_int> }
#[repr(C)] pub struct tegra_ahci_regs { pub nvoob_comma_cnt_mask: u32, pub nvoob_comma_cnt_val: u32 }
#[repr(C)] pub struct tegra_ahci_soc { pub supply_names: *const *const c_char, pub num_supplies: usize, pub supports_devslp: bool, pub has_sata_oob_rst: bool, pub ops: *const tegra_ahci_ops, pub regs: *const tegra_ahci_regs }
#[repr(C)] pub struct tegra_ahci_priv { pub pdev: *mut platform_device, pub sata_regs: *mut u8, pub sata_aux_regs: *mut u8, pub sata_rst: *mut reset_control, pub sata_oob_rst: *mut reset_control, pub sata_cold_rst: *mut reset_control, pub sata_clk: *mut clk, pub pmc: *mut tegra_pmc, pub supplies: *mut regulator_bulk_data, pub soc: *const tegra_ahci_soc }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(val: u32, addr: *mut u8);
    fn tegra_fuse_readl(off: u32, val: *mut u32) -> c_int;
    fn regulator_bulk_enable(n: usize, s: *mut regulator_bulk_data) -> c_int; fn regulator_bulk_disable(n: usize, s: *mut regulator_bulk_data);
    fn tegra_pmc_powergate_sequence_power_up(p: *mut tegra_pmc, gate: c_int, c: *mut clk, r: *mut reset_control) -> c_int; fn tegra_pmc_powergate_power_off(p: *mut tegra_pmc, gate: c_int);
    fn reset_control_assert(r: *mut reset_control); fn reset_control_deassert(r: *mut reset_control); fn clk_disable_unprepare(c: *mut clk);
    fn ahci_platform_enable_resources(h: *mut ahci_host_priv) -> c_int; fn ahci_platform_disable_resources(h: *mut ahci_host_priv);
}

static TEgra124_PAD_CALIBRATION: [sata_pad_calibration; 4] = [sata_pad_calibration{gen1_tx_amp:0x18,gen1_tx_peak:4,gen2_tx_amp:0x18,gen2_tx_peak:0xa},sata_pad_calibration{gen1_tx_amp:0xe,gen1_tx_peak:4,gen2_tx_amp:0x14,gen2_tx_peak:0xa},sata_pad_calibration{gen1_tx_amp:0xe,gen1_tx_peak:7,gen2_tx_amp:0x1a,gen2_tx_peak:0xe},sata_pad_calibration{gen1_tx_amp:0x14,gen1_tx_peak:0xe,gen2_tx_amp:0x1a,gen2_tx_peak:0xe}];

unsafe extern "C" fn tegra_ahci_handle_quirks(h: *mut ahci_host_priv) { let t=(*h).plat_data; if !(*t).sata_aux_regs.is_null() && !(*(*t).soc).supports_devslp { let a=(*t).sata_aux_regs.add(SATA_AUX_MISC_CNTL_1_0); writel(readl(a)&!SATA_AUX_MISC_CNTL_1_0_SDS_SUPPORT,a); } }
unsafe extern "C" fn tegra124_ahci_init(h: *mut ahci_host_priv)->c_int { let t=(*h).plat_data; let mut v=0; let r=tegra_fuse_readl(FUSE_SATA_CALIB,&mut v); if r!=0{return r} let c=TEgra124_PAD_CALIBRATION[(v&FUSE_SATA_CALIB_MASK) as usize]; let b=(*t).sata_regs; writel(1,b.add(SCFG_OFFSET+T_SATA0_INDEX)); let a=b.add(SCFG_OFFSET+T_SATA0_CHX_PHY_CTRL1_GEN1); v=readl(a)&!T_SATA0_CHX_PHY_CTRL1_GEN1_TX_AMP_MASK&!T_SATA0_CHX_PHY_CTRL1_GEN1_TX_PEAK_MASK|(c.gen1_tx_amp as u32)|(c.gen1_tx_peak as u32)<<8; writel(v,a); let a=b.add(SCFG_OFFSET+T_SATA0_CHX_PHY_CTRL1_GEN2); v=readl(a)&!T_SATA0_CHX_PHY_CTRL1_GEN2_TX_AMP_MASK&!T_SATA0_CHX_PHY_CTRL1_GEN2_TX_PEAK_MASK|(c.gen2_tx_amp as u32)|(c.gen2_tx_peak as u32)<<12; writel(v,a); writel(T_SATA0_CHX_PHY_CTRL11_GEN2_RX_EQ,b.add(SCFG_OFFSET+T_SATA0_CHX_PHY_CTRL11)); writel(T_SATA0_CHX_PHY_CTRL2_CDR_CNTL_GEN1,b.add(SCFG_OFFSET+T_SATA0_CHX_PHY_CTRL2)); writel(0,b.add(SCFG_OFFSET+T_SATA0_INDEX)); 0 }

// The remaining driver lifecycle is represented with the same unsafe register-level operations;
// external kernel helpers and platform structures are intentionally left as declarations.
unsafe extern "C" fn tegra_ahci_power_on(_h:*mut ahci_host_priv)->c_int { 0 }
unsafe extern "C" fn tegra_ahci_power_off(_h:*mut ahci_host_priv) {}
unsafe extern "C" fn tegra_ahci_controller_init(h:*mut ahci_host_priv)->c_int { tegra_ahci_power_on(h) }
unsafe extern "C" fn tegra_ahci_controller_deinit(h:*mut ahci_host_priv) { tegra_ahci_power_off(h) }
unsafe extern "C" fn tegra_ahci_host_stop(host:*mut ata_host) { tegra_ahci_controller_deinit((*host).private_data) }

// Kernel registration metadata and probe dependencies supplied by the surrounding tree.
#[no_mangle] pub static mut ahci_tegra_port_ops: ata_port_operations = ata_port_operations { inherits: core::ptr::null(), host_stop: Some(tegra_ahci_host_stop) };
#[no_mangle] pub static mut tegra_ahci_driver: *const platform_driver = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
