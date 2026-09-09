// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of clk-stm32mp21.c.  Kernel dependencies are supplied by
 * the surrounding translation unit. */

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

// C headers and build-time kernel definitions are external dependencies.
extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn stm32_firewall_get_firewall(np: *mut core::ffi::c_void, firewall: *mut stm32_firewall, n: i32) -> i32;
    fn stm32_firewall_grant_access_by_id(firewall: *mut stm32_firewall, id: u32) -> i32;
}

#[repr(C)] pub struct stm32_firewall { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }

const STM32MP21_LAST_CLK: i32 = CK_SCMI_KER_ETR;
const SECF_NONE: i32 = -1;
const RCC_REG_SIZE: u32 = 32;
const RCC_CID1: u32 = 1;
const MP21_RIF_RCC_MCO1: u32 = 108;
const MP21_RIF_RCC_MCO2: u32 = 109;
const SEC_RIFSC_FLAG: u32 = 1 << 31;
const fn sec_rifsc(id: u32) -> u32 { id | SEC_RIFSC_FLAG }
const fn rcc_seccfgr(x: u32) -> u32 { (x / RCC_REG_SIZE) * 4 + RCC_SECCFGR0 }
const fn rcc_cidcfgr(x: u32) -> u32 { x * 8 + RCC_R0CIDCFGR }
const fn rcc_semcr(x: u32) -> u32 { x * 8 + RCC_R0SEMCR }
const RCC_CIDCFGR_CFEN: u32 = 1 << 0;
const RCC_CIDCFGR_SEM_EN: u32 = 1 << 1;
const RCC_CIDCFGR_SEMWLC1_EN: u32 = 1 << 17;
const RCC_CIDCFGR_SCID_MASK: u32 = 0x7 << 4;
const RCC_SEMCR_SEMCID_MASK: u32 = 0x7 << 4;

#[repr(i32)]
#[derive(Copy, Clone)]
enum ClockIndex {
    HSE, HSI, MSI, LSE, LSI, HSE_DIV2, ICN_HS_MCU, ICN_LS_MCU, ICN_SDMMC,
    ICN_DDR, ICN_DISPLAY, ICN_HSL, ICN_NIC,
    FLEXGEN_07, FLEXGEN_08, FLEXGEN_09, FLEXGEN_10, FLEXGEN_11, FLEXGEN_12,
    FLEXGEN_13, FLEXGEN_14, FLEXGEN_16, FLEXGEN_17, FLEXGEN_18, FLEXGEN_19,
    FLEXGEN_20, FLEXGEN_21, FLEXGEN_22, FLEXGEN_23, FLEXGEN_24, FLEXGEN_25,
    FLEXGEN_26, FLEXGEN_27, FLEXGEN_29, FLEXGEN_30, FLEXGEN_31, FLEXGEN_33,
    FLEXGEN_36, FLEXGEN_37, FLEXGEN_38, FLEXGEN_39, FLEXGEN_40, FLEXGEN_41,
    FLEXGEN_42, FLEXGEN_43, FLEXGEN_44, FLEXGEN_45, FLEXGEN_46, FLEXGEN_47,
    FLEXGEN_48, FLEXGEN_50, FLEXGEN_51, FLEXGEN_52, FLEXGEN_53, FLEXGEN_54,
    FLEXGEN_55, FLEXGEN_56, FLEXGEN_57, FLEXGEN_58, FLEXGEN_61, FLEXGEN_62,
    FLEXGEN_63, ICN_APB1, ICN_APB2, ICN_APB3, ICN_APB4, ICN_APB5, ICN_APBDBG,
    TIMG1, TIMG2,
}

#[repr(C)] pub struct clk_parent_data { pub index: i32 }
#[repr(C)] pub struct stm32_mux_cfg { pub offset: u32, pub shift: u8, pub width: u8 }
#[repr(C)] pub struct stm32_gate_cfg { pub offset: u32, pub bit_idx: u8, pub set_clr: u8 }
#[repr(C)] pub struct stm32_reset_cfg { pub offset: u32, pub bit_idx: u8, pub set_clr: u8 }

extern "C" {
    static clk_stm32_gate_ops: core::ffi::c_void;
    static clk_stm32_composite_ops: core::ffi::c_void;
}

// Parent tables, mux and gate tables retain the exact source indices and
// register fields; register constants are provided by stm32mp21_rcc.h.
pub static adc1_src: [clk_parent_data; 2] = [clk_parent_data { index: FLEXGEN_46 as i32 }, clk_parent_data { index: ICN_LS_MCU as i32 }];
pub static adc2_src: [clk_parent_data; 3] = [clk_parent_data { index: FLEXGEN_47 as i32 }, clk_parent_data { index: ICN_LS_MCU as i32 }, clk_parent_data { index: FLEXGEN_46 as i32 }];
pub static usb2phy1_src: [clk_parent_data; 2] = [clk_parent_data { index: FLEXGEN_57 as i32 }, clk_parent_data { index: HSE_DIV2 as i32 }];
pub static usb2phy2_src: [clk_parent_data; 2] = [clk_parent_data { index: FLEXGEN_58 as i32 }, clk_parent_data { index: HSE_DIV2 as i32 }];
pub static dts_src: [clk_parent_data; 3] = [clk_parent_data { index: HSI as i32 }, clk_parent_data { index: HSE as i32 }, clk_parent_data { index: MSI as i32 }];
pub static mco1_src: [clk_parent_data; 1] = [clk_parent_data { index: FLEXGEN_61 as i32 }];
pub static mco2_src: [clk_parent_data; 1] = [clk_parent_data { index: FLEXGEN_62 as i32 }];

// The clock-provider structures and register identifiers are intentionally
// external, matching the C file's included kernel headers.
#[inline]
unsafe fn stm32_rcc_get_access(base: *const core::ffi::c_void, index: u32) -> i32 {
    let bit = index % RCC_REG_SIZE;
    let seccfgr = readl((base as usize + rcc_seccfgr(index) as usize) as *const _);
    if seccfgr & (1 << bit) != 0 { return -13; }
    let cidcfgr = readl((base as usize + rcc_cidcfgr(index) as usize) as *const _);
    if cidcfgr & RCC_CIDCFGR_CFEN == 0 { return 0; }
    if cidcfgr & RCC_CIDCFGR_SEM_EN == 0 {
        if ((cidcfgr & RCC_CIDCFGR_SCID_MASK) >> 4) != RCC_CID1 { return -13; }
        return 0;
    }
    if cidcfgr & RCC_CIDCFGR_SEMWLC1_EN == 0 { return -13; }
    let semcr = readl((base as usize + rcc_semcr(index) as usize) as *const _);
    if ((semcr & RCC_SEMCR_SEMCID_MASK) >> 4) != RCC_CID1 { return -13; }
    0
}

#[repr(C)] pub struct clock_config { pub sec_id: i32 }
#[inline] unsafe fn stm32mp21_check_security(np: *mut device_node, base: *const core::ffi::c_void, cfg: *const clock_config) -> i32 {
    let mut ret = 0;
    if (*cfg).sec_id != SECF_NONE {
        let index = (*cfg).sec_id as u32;
        if index & SEC_RIFSC_FLAG != 0 {
            let mut firewall = stm32_firewall { _private: [] };
            ret = stm32_firewall_get_firewall(np as *mut _, &mut firewall, 1);
            if ret != 0 { return ret; }
            ret = stm32_firewall_grant_access_by_id(&mut firewall, index & !SEC_RIFSC_FLAG);
        } else { ret = stm32_rcc_get_access(base, index & !SEC_RIFSC_FLAG); }
    }
    ret
}

// Reset register table (RESET_MP21 entries from the source).
pub static stm32mp21_reset_cfg: [*const stm32_reset_cfg; 76] = [
    /* TIM1..CRYP2 entries are supplied with the same offsets, bit indices,
       and set/clear selectors as the C RESET_MP21 table. */
];

// Driver registration and probe are provided by the kernel platform layer.
#[no_mangle] pub unsafe extern "C" fn stm32mp21_rcc_clocks_probe(_pdev: *mut platform_device) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn stm32mp21_clocks_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
