/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

/* Preserved build-time condition: CONFIG_SOC_TYPE_XWAY. */

/* Dependency supplied by lantiq.h is intentionally not implemented here. */

/* Chip IDs */
pub const SOC_ID_DANUBE1: u32 = 0x129;
pub const SOC_ID_DANUBE2: u32 = 0x12B;
pub const SOC_ID_TWINPASS: u32 = 0x12D;
pub const SOC_ID_AMAZON_SE_1: u32 = 0x152; /* 50601 */
pub const SOC_ID_AMAZON_SE_2: u32 = 0x153; /* 50600 */
pub const SOC_ID_ARX188: u32 = 0x16C;
pub const SOC_ID_ARX168_1: u32 = 0x16D;
pub const SOC_ID_ARX168_2: u32 = 0x16E;
pub const SOC_ID_ARX182: u32 = 0x16F;
pub const SOC_ID_GRX188: u32 = 0x170;
pub const SOC_ID_GRX168: u32 = 0x171;

pub const SOC_ID_VRX288: u32 = 0x1C0; /* v1.1 */
pub const SOC_ID_VRX282: u32 = 0x1C1; /* v1.1 */
pub const SOC_ID_VRX268: u32 = 0x1C2; /* v1.1 */
pub const SOC_ID_GRX268: u32 = 0x1C8; /* v1.1 */
pub const SOC_ID_GRX288: u32 = 0x1C9; /* v1.1 */
pub const SOC_ID_VRX288_2: u32 = 0x00B; /* v1.2 */
pub const SOC_ID_VRX268_2: u32 = 0x00C; /* v1.2 */
pub const SOC_ID_GRX288_2: u32 = 0x00D; /* v1.2 */
pub const SOC_ID_GRX282_2: u32 = 0x00E; /* v1.2 */
pub const SOC_ID_VRX220: u32 = 0x000;

pub const SOC_ID_ARX362: u32 = 0x004;
pub const SOC_ID_ARX368: u32 = 0x005;
pub const SOC_ID_ARX382: u32 = 0x007;
pub const SOC_ID_ARX388: u32 = 0x008;
pub const SOC_ID_URX388: u32 = 0x009;
pub const SOC_ID_GRX383: u32 = 0x010;
pub const SOC_ID_GRX369: u32 = 0x011;
pub const SOC_ID_GRX387: u32 = 0x00F;
pub const SOC_ID_GRX389: u32 = 0x012;

/* SoC Types */
pub const SOC_TYPE_DANUBE: u32 = 0x01;
pub const SOC_TYPE_TWINPASS: u32 = 0x02;
pub const SOC_TYPE_AR9: u32 = 0x03;
pub const SOC_TYPE_VR9: u32 = 0x04; /* v1.1 */
pub const SOC_TYPE_VR9_2: u32 = 0x05; /* v1.2 */
pub const SOC_TYPE_AMAZON_SE: u32 = 0x06;
pub const SOC_TYPE_AR10: u32 = 0x07;
pub const SOC_TYPE_GRX390: u32 = 0x08;
pub const SOC_TYPE_VRX220: u32 = 0x09;

/* BOOT_SEL - find what boot media we have */
pub const BS_EXT_ROM: u32 = 0x0;
pub const BS_FLASH: u32 = 0x1;
pub const BS_MII0: u32 = 0x2;
pub const BS_PCI: u32 = 0x3;
pub const BS_UART1: u32 = 0x4;
pub const BS_SPI: u32 = 0x5;
pub const BS_NAND: u32 = 0x6;
pub const BS_RMII0: u32 = 0x7;

/* helpers used to access the cgu */
#[macro_export]
macro_rules! ltq_cgu_w32 {
    ($x:expr, $y:expr) => {
        ltq_w32(($x), unsafe { (ltq_cgu_membase as *mut u8).offset($y as isize) })
    };
}

#[macro_export]
macro_rules! ltq_cgu_r32 {
    ($x:expr) => {
        ltq_r32(unsafe { (ltq_cgu_membase as *mut u8).offset($x as isize) })
    };
}

extern "C" {
    pub static mut ltq_cgu_membase: *mut core::ffi::c_void;
}

/*
 * during early_printk no ioremap is possible
 * let's use KSEG1 instead
 */
pub const LTQ_ASC1_BASE_ADDR: usize = 0x1E100C00;
pub const LTQ_EARLY_ASC: usize = KSEG1ADDR(LTQ_ASC1_BASE_ADDR);

/* EBU - external bus unit */
pub const LTQ_EBU_BUSCON0: u32 = 0x0060;
pub const LTQ_EBU_PCC_CON: u32 = 0x0090;
pub const LTQ_EBU_PCC_IEN: u32 = 0x00A4;
pub const LTQ_EBU_PCC_ISTAT: u32 = 0x00A0;
pub const LTQ_EBU_BUSCON1: u32 = 0x0064;
pub const LTQ_EBU_ADDRSEL1: u32 = 0x0024;
pub const EBU_WRDIS: u32 = 0x80000000;

/* WDT */
pub const LTQ_RST_CAUSE_WDTRST: u32 = 0x20;

/* MPS - multi processor unit (voice) */
pub const LTQ_MPS_BASE_ADDR: usize = KSEG1 + 0x1F107000;
pub const LTQ_MPS_CHIPID: *mut u32 = (LTQ_MPS_BASE_ADDR + 0x0344) as *mut u32;

/* request a non-gpio and set the PIO config */
pub const PMU_PPE: u32 = BIT(13);

extern "C" {
    pub fn ltq_pmu_enable(module: u32);
    pub fn ltq_pmu_disable(module: u32);
    pub fn ltq_get_cp1_base() -> *mut u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
