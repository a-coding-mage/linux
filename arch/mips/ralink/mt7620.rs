// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

/* C dependencies supplied by the surrounding kernel translation unit. */

/* analog */
const PMU0_CFG: u32 = 0x88;
const PMU_SW_SET: u32 = 1 << 28;
const A_DCDC_EN: u32 = 1 << 24;
const A_SSC_PERI: u32 = 1 << 19;
const A_SSC_GEN: u32 = 1 << 18;
const A_SSC_M: u32 = 0x3;
const A_SSC_S: u32 = 16;
const A_DLY_M: u32 = 0x7;
const A_DLY_S: u32 = 8;
const A_VTUNE_M: u32 = 0xff;

/* digital */
const PMU1_CFG: u32 = 0x8c;
const DIG_SW_SEL: u32 = 1 << 25;

/* EFUSE bits */
const EFUSE_MT7688: u32 = 0x100000;

/* DRAM type bit */
const DRAM_TYPE_MT7628_MASK: u32 = 0x1;

/* does the board have sdram or ddram */
static mut dram_type: i32 = 0;

static mut soc_info_ptr: *mut ralink_soc_info = core::ptr::null_mut();

unsafe fn mt7620_dram_init(soc_info: *mut ralink_soc_info) {
    match dram_type {
        SYSCFG0_DRAM_TYPE_SDRAM => {
            pr_info!("Board has SDRAM\n");
            (*soc_info).mem_size_min = MT7620_SDRAM_SIZE_MIN;
            (*soc_info).mem_size_max = MT7620_SDRAM_SIZE_MAX;
        }
        SYSCFG0_DRAM_TYPE_DDR1 => {
            pr_info!("Board has DDR1\n");
            (*soc_info).mem_size_min = MT7620_DDR1_SIZE_MIN;
            (*soc_info).mem_size_max = MT7620_DDR1_SIZE_MAX;
        }
        SYSCFG0_DRAM_TYPE_DDR2 => {
            pr_info!("Board has DDR2\n");
            (*soc_info).mem_size_min = MT7620_DDR2_SIZE_MIN;
            (*soc_info).mem_size_max = MT7620_DDR2_SIZE_MAX;
        }
        _ => BUG!(),
    }
}

unsafe fn mt7628_dram_init(soc_info: *mut ralink_soc_info) {
    match dram_type {
        SYSCFG0_DRAM_TYPE_DDR1_MT7628 => {
            pr_info!("Board has DDR1\n");
            (*soc_info).mem_size_min = MT7620_DDR1_SIZE_MIN;
            (*soc_info).mem_size_max = MT7620_DDR1_SIZE_MAX;
        }
        SYSCFG0_DRAM_TYPE_DDR2_MT7628 => {
            pr_info!("Board has DDR2\n");
            (*soc_info).mem_size_min = MT7620_DDR2_SIZE_MIN;
            (*soc_info).mem_size_max = MT7620_DDR2_SIZE_MAX;
        }
        _ => BUG!(),
    }
}

unsafe fn mt7620_get_soc_name0() -> u32 {
    __raw_readl(MT7620_SYSC_BASE + SYSC_REG_CHIP_NAME0)
}

unsafe fn mt7620_get_soc_name1() -> u32 {
    __raw_readl(MT7620_SYSC_BASE + SYSC_REG_CHIP_NAME1)
}

unsafe fn mt7620_soc_valid() -> bool {
    mt7620_get_soc_name0() == MT7620_CHIP_NAME0 &&
        mt7620_get_soc_name1() == MT7620_CHIP_NAME1
}

unsafe fn mt7628_soc_valid() -> bool {
    mt7620_get_soc_name0() == MT7620_CHIP_NAME0 &&
        mt7620_get_soc_name1() == MT7628_CHIP_NAME1
}

unsafe fn mt7620_get_rev() -> u32 {
    __raw_readl(MT7620_SYSC_BASE + SYSC_REG_CHIP_REV)
}

unsafe fn mt7620_get_bga() -> u32 {
    (mt7620_get_rev() >> CHIP_REV_PKG_SHIFT) & CHIP_REV_PKG_MASK
}

unsafe fn mt7620_get_efuse() -> u32 {
    __raw_readl(MT7620_SYSC_BASE + SYSC_REG_EFUSE_CFG)
}

unsafe fn mt7620_get_soc_ver() -> u32 {
    (mt7620_get_rev() >> CHIP_REV_VER_SHIFT) & CHIP_REV_VER_MASK
}

unsafe fn mt7620_get_soc_eco() -> u32 {
    mt7620_get_rev() & CHIP_REV_ECO_MASK
}

unsafe fn mt7620_get_soc_name(soc_info: *mut ralink_soc_info) -> *const u8 {
    if mt7620_soc_valid() {
        let bga = mt7620_get_bga();
        if bga != 0 {
            ralink_soc = MT762X_SOC_MT7620A;
            (*soc_info).compatible = b"ralink,mt7620a-soc\0".as_ptr();
            b"MT7620A\0".as_ptr()
        } else {
            ralink_soc = MT762X_SOC_MT7620N;
            (*soc_info).compatible = b"ralink,mt7620n-soc\0".as_ptr();
            b"MT7620N\0".as_ptr()
        }
    } else if mt7628_soc_valid() {
        let efuse = mt7620_get_efuse();
        let name;
        if efuse & EFUSE_MT7688 != 0 {
            ralink_soc = MT762X_SOC_MT7688;
            name = b"MT7688\0".as_ptr();
        } else {
            ralink_soc = MT762X_SOC_MT7628AN;
            name = b"MT7628AN\0".as_ptr();
        }
        (*soc_info).compatible = b"ralink,mt7628an-soc\0".as_ptr();
        name
    } else {
        panic!("mt762x: unknown SoC, n0:{:08x} n1:{:08x}\n",
               mt7620_get_soc_name0(), mt7620_get_soc_name1());
    }
}

unsafe fn mt7620_get_soc_id_name() -> *const u8 {
    if ralink_soc == MT762X_SOC_MT7620A { b"mt7620a\0".as_ptr() }
    else if ralink_soc == MT762X_SOC_MT7620N { b"mt7620n\0".as_ptr() }
    else if ralink_soc == MT762X_SOC_MT7688 { b"mt7688\0".as_ptr() }
    else if ralink_soc == MT762X_SOC_MT7628AN { b"mt7628n\0".as_ptr() }
    else { b"invalid\0".as_ptr() }
}

unsafe fn mt7620_soc_dev_init() -> i32 {
    let mut soc_dev_attr: *mut soc_device_attribute = kzalloc_obj!();
    if soc_dev_attr.is_null() { return -ENOMEM; }
    (*soc_dev_attr).family = b"Ralink\0".as_ptr();
    (*soc_dev_attr).soc_id = mt7620_get_soc_id_name();
    (*soc_dev_attr).data = soc_info_ptr as *const _;
    let soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) {
        kfree(soc_dev_attr as *mut _);
        return PTR_ERR(soc_dev);
    }
    0
}

device_initcall!(mt7620_soc_dev_init);

pub unsafe fn prom_soc_init(soc_info: *mut ralink_soc_info) {
    let name = mt7620_get_soc_name(soc_info);
    let mut cfg0: u32;
    let mut pmu0: u32;
    let mut pmu1: u32;

    snprintf!((*soc_info).sys_type.as_mut_ptr(), RAMIPS_SYS_TYPE_LEN,
        "MediaTek {} ver:{} eco:{}", name, mt7620_get_soc_ver(), mt7620_get_soc_eco());
    cfg0 = __raw_readl(MT7620_SYSC_BASE + SYSC_REG_SYSTEM_CONFIG0);
    if is_mt76x8() { dram_type = (cfg0 & DRAM_TYPE_MT7628_MASK) as i32; }
    else {
        dram_type = ((cfg0 >> SYSCFG0_DRAM_TYPE_SHIFT) & SYSCFG0_DRAM_TYPE_MASK) as i32;
        if dram_type == SYSCFG0_DRAM_TYPE_UNKNOWN { dram_type = SYSCFG0_DRAM_TYPE_SDRAM; }
    }
    (*soc_info).mem_base = MT7620_DRAM_BASE;
    if is_mt76x8() { mt7628_dram_init(soc_info); } else { mt7620_dram_init(soc_info); }
    pmu0 = __raw_readl(MT7620_SYSC_BASE + PMU0_CFG);
    pmu1 = __raw_readl(MT7620_SYSC_BASE + PMU1_CFG);
    pr_info!("Analog PMU set to {} control\n", if pmu0 & PMU_SW_SET != 0 { "sw" } else { "hw" });
    pr_info!("Digital PMU set to {} control\n", if pmu1 & DIG_SW_SEL != 0 { "sw" } else { "hw" });
    soc_info_ptr = soc_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
