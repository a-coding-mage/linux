/* SPDX-License-Identifier: GPL-2.0 */
/* HEART chip definitions; translated from the C header. */

pub const HEART_MEMORY_BANKS: usize = 4;
pub const HEART_MAX_CPUS: usize = 4;
pub const HEART_XKPHYS_BASE: *mut core::ffi::c_void =
    (IO_BASE | 0x0000_0000_0ff0_0000u64) as *mut core::ffi::c_void;

#[repr(C)]
pub union Ip30HeartMemCfg {
    pub q: [u64; HEART_MEMORY_BANKS],
    pub l: [u32; HEART_MEMORY_BANKS * 2],
}

#[repr(C)]
pub struct Ip30HeartRegs {
    pub mode: u64,
    pub sdram_mode: u64,
    pub mem_refresh: u64,
    pub mem_req_arb: u64,
    pub mem_cfg: Ip30HeartMemCfg,
    pub fc_mode: u64,
    pub fc_timer_limit: u64,
    pub fc_addr: [u64; 2],
    pub fc_credit_cnt: [u64; 2],
    pub fc_timer: [u64; 2],
    pub status: u64,
    pub bus_err_addr: u64,
    pub bus_err_misc: u64,
    pub mem_err_addr: u64,
    pub mem_err_data: u64,
    pub piur_acc_err: u64,
    pub mlan_clock_div: u64,
    pub mlan_ctrl: u64,
    pub __pad0: [u64; 0x01e8],
    pub undefined: u64,
    pub __pad1: [u64; 0x1dff],
    pub imr: [u64; HEART_MAX_CPUS],
    pub set_isr: u64,
    pub clear_isr: u64,
    pub isr: u64,
    pub imsr: u64,
    pub cause: u64,
    pub __pad2: [u64; 0x1ff7],
    pub count: u64,
    pub __pad3: [u64; 0x1fff],
    pub compare: u64,
    pub __pad4: [u64; 0x1fff],
    pub trigger: u64,
    pub __pad5: [u64; 0x1fff],
    pub cpuid: u64,
    pub __pad6: [u64; 0x1fff],
    pub sync: u64,
}

pub const HEART_NS_PER_CYCLE: u64 = 80;
pub const HEART_CYCLES_PER_SEC: u64 = NSEC_PER_SEC / HEART_NS_PER_CYCLE;

pub const HEART_ATK_MASK: u64 = 0x0007_ffff_ffff_ffff;
pub const HEART_ACK_ALL_MASK: u64 = 0xffff_ffff_ffff_ffff;
pub const HEART_CLR_ALL_MASK: u64 = 0;
pub const HEART_BR_ERR_MASK: u64 = 0x7ff8_0000_0000_0000;
pub const HEART_CPU0_ERR_MASK: u64 = 0x8ff8_0000_0000_0000;
pub const HEART_CPU1_ERR_MASK: u64 = 0x97f8_0000_0000_0000;
pub const HEART_CPU2_ERR_MASK: u64 = 0xa7f8_0000_0000_0000;
pub const HEART_CPU3_ERR_MASK: u64 = 0xc7f8_0000_0000_0000;
pub const HEART_ERR_MASK: u64 = 0x1ff;
pub const HEART_ERR_MASK_START: u32 = 51;
pub const HEART_ERR_MASK_END: u32 = 63;

pub const HM_PROC_DISABLE_SHFT: u32 = 60;
pub const HM_PROC_DISABLE_MSK: u64 = 0xfu64 << HM_PROC_DISABLE_SHFT;
pub const fn HM_PROC_DISABLE(x: u32) -> u64 { 1u64 << (x + HM_PROC_DISABLE_SHFT) }
pub const HM_MAX_PSR: u64 = 0x7u64 << 57;
pub const HM_MAX_IOSR: u64 = 0x7u64 << 54;
pub const HM_MAX_PEND_IOSR: u64 = 0x7u64 << 51;
pub const HM_TRIG_SRC_SEL_MSK: u64 = 0x7u64 << 48;
pub const HM_TRIG_HEART_EXC: u64 = 0x0u64 << 48;
pub const HM_TRIG_REG_BIT: u64 = 0x1u64 << 48;
pub const HM_TRIG_SYSCLK: u64 = 0x2u64 << 48;
pub const HM_TRIG_MEMCLK_2X: u64 = 0x3u64 << 48;
pub const HM_TRIG_MEMCLK: u64 = 0x4u64 << 48;
pub const HM_TRIG_IOCLK: u64 = 0x5u64 << 48;
pub const HM_PIU_TEST_MODE: u64 = 0xfu64 << 40;
pub const HM_GP_FLAG_MSK: u64 = 0xfu64 << 36;
pub const fn HM_GP_FLAG(x: u32) -> u64 { 1u64 << (x + 36) }
pub const HM_MAX_PROC_HYST: u64 = 0xfu64 << 32;

macro_rules! heart_bit_constants { ($($name:ident, $bit:expr);* $(;)?) => { $(pub const $name: u64 = 1u64 << $bit;)* }; }
heart_bit_constants! {
    HM_LLP_WRST_AFTER_RST, 28; HM_LLP_LINK_RST, 27; HM_LLP_WARM_RST, 26;
    HM_COR_ECC_LCK, 25; HM_REDUCED_PWR, 24; HM_COLD_RST, 23; HM_SW_RST, 22;
    HM_MEM_FORCE_WR, 21; HM_DB_ERR_GEN, 20; HM_SB_ERR_GEN, 19;
    HM_CACHED_PIO_EN, 18; HM_CACHED_PROM_EN, 17; HM_PE_SYS_COR_ERE, 16;
    HM_GLOBAL_ECC_EN, 15; HM_IO_COH_EN, 14; HM_INT_EN, 13; HM_DATA_CHK_EN, 12;
    HM_REF_EN, 11; HM_BAD_SYSWR_ERE, 10; HM_BAD_SYSRD_ERE, 9; HM_SYSSTATE_ERE, 8;
    HM_SYSCMD_ERE, 7; HM_NCOR_SYS_ERE, 6; HM_COR_SYS_ERE, 5; HM_DATA_ELMNT_ERE, 4;
    HM_MEM_ADDR_PROC_ERE, 3; HM_MEM_ADDR_IO_ERE, 2; HM_NCOR_MEM_ERE, 1; HM_COR_MEM_ERE, 0
}

pub const fn HEART_MEMREF_REFS(x: u64) -> u64 { (0xf & x) << 16 }
pub const fn HEART_MEMREF_PERIOD(x: u64) -> u64 { 0xffff & x }
pub const HEART_MEMREF_REFS_VAL: u64 = HEART_MEMREF_REFS(8);
pub const HEART_MEMREF_PERIOD_VAL: u64 = HEART_MEMREF_PERIOD(0x4000);
pub const HEART_MEMREF_VAL: u64 = HEART_MEMREF_REFS_VAL | HEART_MEMREF_PERIOD_VAL;
pub const HEART_MEMARB_IODIS: u64 = 1 << 20;
pub const HEART_MEMARB_MAXPMWRQS: u64 = 15 << 16;
pub const HEART_MEMARB_MAXPMRRQS: u64 = 15 << 12;
pub const HEART_MEMARB_MAXPMRQS: u64 = 15 << 8;
pub const HEART_MEMARB_MAXRRRQS: u64 = 15 << 4;
pub const HEART_MEMARB_MAXGBRRQS: u64 = 15;
pub const HEART_MEMCFG_VALID: u32 = 0x8000_0000;
pub const HEART_MEMCFG_DENSITY: u32 = 0x01c0_0000;
pub const HEART_MEMCFG_SIZE_MASK: u32 = 0x003f_0000;
pub const HEART_MEMCFG_ADDR_MASK: u32 = 0x0000_01ff;
pub const HEART_MEMCFG_SIZE_SHIFT: u32 = 16;
pub const HEART_MEMCFG_DENSITY_SHIFT: u32 = 22;
pub const HEART_MEMCFG_UNIT_SHIFT: u32 = 25;

pub const HEART_STAT_HSTL_SDRV: u64 = 1 << 14;
pub const fn HEART_STAT_FC_CR_OUT(x: u32) -> u64 { 1 << (x + 12) }
pub const HEART_STAT_DIR_CNNCT: u64 = 1 << 11;
pub const HEART_STAT_TRITON: u64 = 1 << 10;
pub const HEART_STAT_R4K: u64 = 1 << 9;
pub const HEART_STAT_BIG_ENDIAN: u64 = 1 << 8;
pub const HEART_STAT_PROC_SHFT: u32 = 4;
pub const HEART_STAT_PROC_MSK: u64 = 0xfu64 << HEART_STAT_PROC_SHFT;
pub const fn HEART_STAT_PROC_ACTIVE(x: u32) -> u64 { 1u64 << (x + HEART_STAT_PROC_SHFT) }
pub const HEART_STAT_WIDGET_ID: u64 = 0xf;

pub const HC_PE_SYS_COR_ERR_MSK: u64 = 0xfu64 << 60;
pub const fn HC_PE_SYS_COR_ERR(x: u32) -> u64 { 1u64 << (x + 60) }
macro_rules! hc_bits { ($($n:ident, $b:expr);* $(;)?) => { $(pub const $n: u64 = 1u64 << $b;)* }; }
hc_bits! {
    HC_PIOWDB_OFLOW, 44; HC_PIORWRB_OFLOW, 43; HC_PIUR_ACC_ERR, 42;
    HC_BAD_SYSWR_ERR, 41; HC_BAD_SYSRD_ERR, 40
}
pub const HC_SYSSTATE_ERR_MSK: u64 = 0xfu64 << 36;
pub const fn HC_SYSSTATE_ERR(x: u32) -> u64 { 1u64 << (x + 36) }
pub const HC_SYSCMD_ERR_MSK: u64 = 0xfu64 << 32;
pub const fn HC_SYSCMD_ERR(x: u32) -> u64 { 1u64 << (x + 32) }
pub const HC_NCOR_SYSAD_ERR_MSK: u64 = 0xfu64 << 28;
pub const fn HC_NCOR_SYSAD_ERR(x: u32) -> u64 { 1u64 << (x + 28) }
pub const HC_COR_SYSAD_ERR_MSK: u64 = 0xfu64 << 24;
pub const fn HC_COR_SYSAD_ERR(x: u32) -> u64 { 1u64 << (x + 24) }
pub const HC_DATA_ELMNT_ERR_MSK: u64 = 0xfu64 << 20;
pub const fn HC_DATA_ELMNT_ERR(x: u32) -> u64 { 1u64 << (x + 20) }
pub const HC_WIDGET_ERR: u64 = 1 << 16;
pub const HC_MEM_ADDR_ERR_PROC_MSK: u64 = 0xf << 4;
pub const fn HC_MEM_ADDR_ERR_PROC(x: u32) -> u64 { 1u64 << (x + 4) }
pub const HC_MEM_ADDR_ERR_IO: u64 = 1 << 2;
pub const HC_NCOR_MEM_ERR: u64 = 1 << 1;
pub const HC_COR_MEM_ERR: u64 = 1;

pub const HEART_NUM_IRQS: usize = 64;
pub const HEART_L4_INT_MASK: u64 = 0xfff8_0000_0000_0000;
pub const HEART_L3_INT_MASK: u64 = 0x0004_0000_0000_0000;
pub const HEART_L2_INT_MASK: u64 = 0x0003_ffff_0000_0000;
pub const HEART_L1_INT_MASK: u64 = 0x0000_0000_ffff_0000;
pub const HEART_L0_INT_MASK: u64 = 0x0000_0000_0000_ffff;
pub const HEART_L0_INT_GENERIC: u32 = 0;
pub const HEART_L0_INT_FLOW_CTRL_HWTR_0: u32 = 1;
pub const HEART_L0_INT_FLOW_CTRL_HWTR_1: u32 = 2;
pub const HEART_L2_INT_RESCHED_CPU_0: u32 = 46;
pub const HEART_L2_INT_RESCHED_CPU_1: u32 = 47;
pub const HEART_L2_INT_CALL_CPU_0: u32 = 48;
pub const HEART_L2_INT_CALL_CPU_1: u32 = 49;
pub const HEART_L3_INT_TIMER: u32 = 50;
pub const HEART_L4_INT_XWID_ERR_9: u32 = 51;
pub const HEART_L4_INT_XWID_ERR_A: u32 = 52;
pub const HEART_L4_INT_XWID_ERR_B: u32 = 53;
pub const HEART_L4_INT_XWID_ERR_C: u32 = 54;
pub const HEART_L4_INT_XWID_ERR_D: u32 = 55;
pub const HEART_L4_INT_XWID_ERR_E: u32 = 56;
pub const HEART_L4_INT_XWID_ERR_F: u32 = 57;
pub const HEART_L4_INT_XWID_ERR_XBOW: u32 = 58;
pub const HEART_L4_INT_CPU_BUS_ERR_0: u32 = 59;
pub const HEART_L4_INT_CPU_BUS_ERR_1: u32 = 60;
pub const HEART_L4_INT_CPU_BUS_ERR_2: u32 = 61;
pub const HEART_L4_INT_CPU_BUS_ERR_3: u32 = 62;
pub const HEART_L4_INT_HEART_EXCP: u32 = 63;

extern "C" {
    pub static mut heart_regs: *mut Ip30HeartRegs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
