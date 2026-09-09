/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 ARM Limited, All Rights Reserved. */
/* Translated from arm-gic-v5.h. */

const fn bit(n: u32) -> u64 { 1u64 << n }
const fn genmask(h: u32, l: u32) -> u64 { ((1u64 << (h - l + 1)) - 1) << l }

pub const GICV5_IPIS_PER_CPU: u64 = MAX_IPI as u64;
pub const GICV5_HWIRQ_ID: u64 = genmask(23, 0);
pub const GICV5_HWIRQ_TYPE: u64 = genmask(31, 29);
pub const GICV5_HWIRQ_INTID: u64 = genmask(31, 0);
pub const GICV5_HWIRQ_TYPE_PPI: u64 = 0x1;
pub const GICV5_HWIRQ_TYPE_LPI: u64 = 0x2;
pub const GICV5_HWIRQ_TYPE_SPI: u64 = 0x3;

pub const GICV5_ARCH_PPI_S_DB_PPI: u64 = 0x0;
pub const GICV5_ARCH_PPI_RL_DB_PPI: u64 = 0x1;
pub const GICV5_ARCH_PPI_NS_DB_PPI: u64 = 0x2;
pub const GICV5_ARCH_PPI_SW_PPI: u64 = 0x3;
pub const GICV5_ARCH_PPI_HACDBSIRQ: u64 = 0xf;
pub const GICV5_ARCH_PPI_CNTHVS: u64 = 0x13;
pub const GICV5_ARCH_PPI_CNTHPS: u64 = 0x14;
pub const GICV5_ARCH_PPI_PMBIRQ: u64 = 0x15;
pub const GICV5_ARCH_PPI_COMMIRQ: u64 = 0x16;
pub const GICV5_ARCH_PPI_PMUIRQ: u64 = 0x17;
pub const GICV5_ARCH_PPI_CTIIRQ: u64 = 0x18;
pub const GICV5_ARCH_PPI_GICMNT: u64 = 0x19;
pub const GICV5_ARCH_PPI_CNTHP: u64 = 0x1a;
pub const GICV5_ARCH_PPI_CNTV: u64 = 0x1b;
pub const GICV5_ARCH_PPI_CNTHV: u64 = 0x1c;
pub const GICV5_ARCH_PPI_CNTPS: u64 = 0x1d;
pub const GICV5_ARCH_PPI_CNTP: u64 = 0x1e;
pub const GICV5_ARCH_PPI_TRBIRQ: u64 = 0x1f;

pub const GICV5_NO_READ_ALLOC: u64 = 0b0;
pub const GICV5_READ_ALLOC: u64 = 0b1;
pub const GICV5_NO_WRITE_ALLOC: u64 = 0b0;
pub const GICV5_WRITE_ALLOC: u64 = 0b1;
pub const GICV5_NON_CACHE: u64 = 0b00;
pub const GICV5_WB_CACHE: u64 = 0b01;
pub const GICV5_WT_CACHE: u64 = 0b10;
pub const GICV5_NON_SHARE: u64 = 0b00;
pub const GICV5_OUTER_SHARE: u64 = 0b10;
pub const GICV5_INNER_SHARE: u64 = 0b11;

/* Register offsets and bit fields retain the source names and values. */

macro_rules! gicv5_reg_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u64 = $v;)* }; }
gicv5_reg_consts! {
 GICV5_IRS_IDR0=0x0000, GICV5_IRS_IDR1=0x0004, GICV5_IRS_IDR2=0x0008,
 GICV5_IRS_IDR5=0x0014, GICV5_IRS_IDR6=0x0018, GICV5_IRS_IDR7=0x001c,
 GICV5_IRS_CR0=0x0080, GICV5_IRS_CR1=0x0084, GICV5_IRS_SYNCR=0x00c0,
 GICV5_IRS_SYNC_STATUSR=0x00c4, GICV5_IRS_SPI_SELR=0x0108, GICV5_IRS_SPI_CFGR=0x0114,
 GICV5_IRS_SPI_STATUSR=0x0118, GICV5_IRS_PE_SELR=0x0140, GICV5_IRS_PE_STATUSR=0x0144,
 GICV5_IRS_PE_CR0=0x0148, GICV5_IRS_IST_BASER=0x0180, GICV5_IRS_IST_CFGR=0x0190,
 GICV5_IRS_IST_STATUSR=0x0194, GICV5_IRS_MAP_L2_ISTR=0x01c0,
 GICV5_ITS_IDR1=0x0004, GICV5_ITS_IDR2=0x0008, GICV5_ITS_CR0=0x0080, GICV5_ITS_CR1=0x0084,
 GICV5_ITS_DT_BASER=0x00c0, GICV5_ITS_DT_CFGR=0x00d0, GICV5_ITS_DIDR=0x0100,
 GICV5_ITS_EIDR=0x0108, GICV5_ITS_INV_EVENTR=0x010c, GICV5_ITS_INV_DEVICER=0x0110,
 GICV5_ITS_STATUSR=0x0120, GICV5_ITS_SYNCR=0x0140, GICV5_ITS_SYNC_STATUSR=0x0148,
 GICV5_IWB_IDR0=0x0000, GICV5_IWB_CR0=0x0080, GICV5_IWB_WENABLE_STATUSR=0x00c0,
 GICV5_IWB_WENABLER=0x2000, GICV5_IWB_WTMR=0x4000
}

/* Remaining source macros are represented directly as Rust constants. */
pub const GICV5_IRS_IDR0_VIRT:u64=bit(6); pub const GICV5_IRS_CR0_IDLE:u64=bit(1); pub const GICV5_IRS_CR0_IRSEN:u64=bit(0);
pub const GICV5_IRS_SYNCR_SYNC:u64=bit(31); pub const GICV5_IRS_SYNC_STATUSR_IDLE:u64=bit(0);
pub const GICV5_IRS_SPI_STATUSR_V:u64=bit(1); pub const GICV5_IRS_SPI_STATUSR_IDLE:u64=bit(0);
pub const GICV5_IRS_SPI_SELR_ID:u64=genmask(23,0); pub const GICV5_IRS_SPI_CFGR_TM:u64=bit(0);
pub const GICV5_IRS_PE_SELR_IAFFID:u64=genmask(15,0); pub const GICV5_IRS_PE_STATUSR_V:u64=bit(1); pub const GICV5_IRS_PE_STATUSR_IDLE:u64=bit(0); pub const GICV5_IRS_PE_CR0_DPS:u64=bit(0);
pub const GICV5_IRS_IST_STATUSR_IDLE:u64=bit(0); pub const GICV5_IRS_IST_BASER_VALID:u64=bit(0); pub const GICV5_ISTL1E_VALID:u64=bit(0);
pub const GICV5_ITS_CR0_IDLE:u64=bit(1); pub const GICV5_ITS_CR0_ITSEN:u64=bit(0); pub const GICV5_ITS_STATUSR_IDLE:u64=bit(0);
pub const GICV5_ITS_SYNCR_SYNC:u64=bit(63); pub const GICV5_ITS_SYNCR_SYNCALL:u64=bit(32); pub const GICV5_ITS_SYNCR_DEVICEID:u64=genmask(31,0);
pub const GICV5_GSI_IC_TYPE:u64=genmask(31,29); pub const GICV5_GSI_IWB_TYPE:u64=7; pub const GICV5_GSI_IWB_FRAME_ID:u64=genmask(28,16); pub const GICV5_GSI_IWB_WIRE:u64=genmask(15,0);
pub const GICV5_IRS_IDR1_PRIORITY_BITS:u64=genmask(22,20); pub const GICV5_IRS_IDR1_IAFFID_BITS:u64=genmask(19,16);
pub const GICV5_IRS_IDR2_ISTMD_SZ:u64=genmask(19,15); pub const GICV5_IRS_IDR2_ISTMD:u64=bit(14); pub const GICV5_IRS_IDR2_IST_L2SZ:u64=genmask(13,11); pub const GICV5_IRS_IDR2_IST_LEVELS:u64=bit(10); pub const GICV5_IRS_IDR2_MIN_LPI_ID_BITS:u64=genmask(9,6); pub const GICV5_IRS_IDR2_LPI:u64=bit(5); pub const GICV5_IRS_IDR2_ID_BITS:u64=genmask(4,0);
pub const GICV5_IRS_IDR5_SPI_RANGE:u64=genmask(24,0); pub const GICV5_IRS_IDR6_SPI_IRS_RANGE:u64=genmask(24,0); pub const GICV5_IRS_IDR7_SPI_BASE:u64=genmask(23,0);
pub const GICV5_IRS_CR1_VPED_WA:u64=bit(15); pub const GICV5_IRS_CR1_VPED_RA:u64=bit(14); pub const GICV5_IRS_CR1_VMD_WA:u64=bit(13); pub const GICV5_IRS_CR1_VMD_RA:u64=bit(12); pub const GICV5_IRS_CR1_VPET_WA:u64=bit(11); pub const GICV5_IRS_CR1_VPET_RA:u64=bit(10); pub const GICV5_IRS_CR1_VMT_WA:u64=bit(9); pub const GICV5_IRS_CR1_VMT_RA:u64=bit(8); pub const GICV5_IRS_CR1_IST_WA:u64=bit(7); pub const GICV5_IRS_CR1_IST_RA:u64=bit(6); pub const GICV5_IRS_CR1_IC:u64=genmask(5,4); pub const GICV5_IRS_CR1_OC:u64=genmask(3,2); pub const GICV5_IRS_CR1_SH:u64=genmask(1,0);
pub const GICV5_IRS_IST_CFGR_STRUCTURE:u64=bit(16); pub const GICV5_IRS_IST_CFGR_ISTSZ:u64=genmask(8,7); pub const GICV5_IRS_IST_CFGR_L2SZ:u64=genmask(6,5); pub const GICV5_IRS_IST_CFGR_LPI_ID_BITS:u64=genmask(4,0); pub const GICV5_IRS_IST_BASER_ADDR_MASK:u64=genmask(55,6); pub const GICV5_IRS_MAP_L2_ISTR_ID:u64=genmask(23,0); pub const GICV5_ISTL1E_L2_ADDR_MASK:u64=genmask(55,12);
pub const GICV5_ITS_IDR1_L2SZ:u64=genmask(10,8); pub const GICV5_ITS_IDR1_ITT_LEVELS:u64=bit(7); pub const GICV5_ITS_IDR1_DT_LEVELS:u64=bit(6); pub const GICV5_ITS_IDR1_DEVICEID_BITS:u64=genmask(5,0); pub const GICV5_ITS_IDR2_XDMN_EVENTs:u64=genmask(6,5); pub const GICV5_ITS_IDR2_EVENTID_BITS:u64=genmask(4,0); pub const GICV5_ITS_CR1_ITT_RA:u64=bit(7); pub const GICV5_ITS_CR1_DT_RA:u64=bit(6); pub const GICV5_ITS_CR1_IC:u64=genmask(5,4); pub const GICV5_ITS_CR1_OC:u64=genmask(3,2); pub const GICV5_ITS_CR1_SH:u64=genmask(1,0);
pub const GICV5_ITS_DT_CFGR_STRUCTURE:u64=bit(16); pub const GICV5_ITS_DT_CFGR_L2SZ:u64=genmask(7,6); pub const GICV5_ITS_DT_CFGR_DEVICEID_BITS:u64=genmask(5,0); pub const GICV5_ITS_DT_BASER_ADDR_MASK:u64=genmask(55,3); pub const GICV5_ITS_INV_DEVICER_I:u64=bit(31); pub const GICV5_ITS_INV_DEVICER_EVENTID_BITS:u64=genmask(5,1); pub const GICV5_ITS_INV_DEVICER_L1:u64=bit(0); pub const GICV5_ITS_DIDR_DEVICEID:u64=genmask(31,0); pub const GICV5_ITS_EIDR_EVENTID:u64=genmask(15,0); pub const GICV5_ITS_INV_EVENTR_I:u64=bit(31); pub const GICV5_ITS_INV_EVENTR_ITT_L2SZ:u64=genmask(2,1); pub const GICV5_ITS_INV_EVENTR_L1:u64=bit(0); pub const GICV5_IWB_IDR0_INT_DOMS:u64=genmask(14,11); pub const GICV5_IWB_IDR0_IW_RANGE:u64=genmask(10,0); pub const GICV5_IWB_CR0_IDLE:u64=bit(1); pub const GICV5_IWB_CR0_IWBEN:u64=bit(0); pub const GICV5_IWB_WENABLE_STATUSR_IDLE:u64=bit(0);

/* The source's external types and functions are supplied by other kernel headers/modules. */
#[repr(C)] pub struct Gicv5ChipData { pub fwnode:*mut fwnode_handle, pub ppi_domain:*mut irq_domain, pub spi_domain:*mut irq_domain, pub lpi_domain:*mut irq_domain, pub ipi_domain:*mut irq_domain, pub global_spi_count:u32, pub cpuif_pri_bits:u8, pub cpuif_id_bits:u8, pub irs_pri_bits:u8, pub virt_capable:bool }
extern "C" { pub static mut gicv5_global_data:Gicv5ChipData; pub fn gicv5_init_lpi_domain(); pub fn gicv5_free_lpi_domain(); pub fn gicv5_irs_of_probe(parent:*mut device_node)->i32; pub fn gicv5_irs_acpi_probe()->i32; pub fn gicv5_irs_remove(); pub fn gicv5_irs_enable()->i32; pub fn gicv5_irs_its_probe(); pub fn gicv5_irs_register_cpu(cpuid:i32)->i32; pub fn gicv5_irs_cpu_to_iaffid(cpu_id:i32, iaffid:*mut u16)->i32; pub fn gicv5_irs_syncr(); pub fn gicv5_init_lpis(max:u32); pub fn gicv5_deinit_lpis(); pub fn gicv5_its_of_probe(parent:*mut device_node); pub fn gicv5_its_acpi_probe(); }
#[repr(C)] pub struct Gicv5Vpe { pub resident:bool }
#[repr(C)] pub struct Gicv5ItsDevtabCfg { pub raw:[u64;2], pub cfgr:u32 }
#[repr(C)] pub struct Gicv5ItsIttCfg { pub raw:[u64;4], pub event_id_bits:u8, pub l2itt:bool }

#[allow(non_camel_case_types)] pub enum fwnode_handle {} #[allow(non_camel_case_types)] pub enum irq_domain {} #[allow(non_camel_case_types)] pub enum device_node {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
