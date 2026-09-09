// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// C dependencies supplied by other translation units are intentionally left external.

static mut soc_info_ptr: *mut ralink_soc_info = core::ptr::null_mut();

unsafe fn rt5350_get_mem_size() -> ::core::ffi::c_ulong {
    let mut ret: ::core::ffi::c_ulong;
    let mut t: u32;

    t = __raw_readl(RT305X_SYSC_BASE + SYSC_REG_SYSTEM_CONFIG);
    t = (t >> RT5350_SYSCFG0_DRAM_SIZE_SHIFT) & RT5350_SYSCFG0_DRAM_SIZE_MASK;

    match t {
        RT5350_SYSCFG0_DRAM_SIZE_2M => ret = 2,
        RT5350_SYSCFG0_DRAM_SIZE_8M => ret = 8,
        RT5350_SYSCFG0_DRAM_SIZE_16M => ret = 16,
        RT5350_SYSCFG0_DRAM_SIZE_32M => ret = 32,
        RT5350_SYSCFG0_DRAM_SIZE_64M => ret = 64,
        _ => panic!("rt5350: invalid DRAM size: {}", t),
    }

    ret
}

unsafe fn rt305x_get_soc_name0() -> u32 {
    __raw_readl(RT305X_SYSC_BASE + SYSC_REG_CHIP_NAME0)
}

unsafe fn rt305x_get_soc_name1() -> u32 {
    __raw_readl(RT305X_SYSC_BASE + SYSC_REG_CHIP_NAME1)
}

unsafe fn rt3052_soc_valid() -> bool {
    rt305x_get_soc_name0() == RT3052_CHIP_NAME0 && rt305x_get_soc_name1() == RT3052_CHIP_NAME1
}

unsafe fn rt3350_soc_valid() -> bool {
    rt305x_get_soc_name0() == RT3350_CHIP_NAME0 && rt305x_get_soc_name1() == RT3350_CHIP_NAME1
}

unsafe fn rt3352_soc_valid() -> bool {
    rt305x_get_soc_name0() == RT3352_CHIP_NAME0 && rt305x_get_soc_name1() == RT3352_CHIP_NAME1
}

unsafe fn rt5350_soc_valid() -> bool {
    rt305x_get_soc_name0() == RT5350_CHIP_NAME0 && rt305x_get_soc_name1() == RT5350_CHIP_NAME1
}

unsafe fn rt305x_get_soc_name(soc_info: *mut ralink_soc_info) -> &'static str {
    if rt3052_soc_valid() {
        let icache_sets = (read_c0_config1() >> 22) & 7;
        if icache_sets == 1 {
            ralink_soc = RT305X_SOC_RT3050;
            (*soc_info).compatible = "ralink,rt3050-soc";
            "RT3050"
        } else {
            ralink_soc = RT305X_SOC_RT3052;
            (*soc_info).compatible = "ralink,rt3052-soc";
            "RT3052"
        }
    } else if rt3350_soc_valid() {
        ralink_soc = RT305X_SOC_RT3350;
        (*soc_info).compatible = "ralink,rt3350-soc";
        "RT3350"
    } else if rt3352_soc_valid() {
        ralink_soc = RT305X_SOC_RT3352;
        (*soc_info).compatible = "ralink,rt3352-soc";
        "RT3352"
    } else if rt5350_soc_valid() {
        ralink_soc = RT305X_SOC_RT5350;
        (*soc_info).compatible = "ralink,rt5350-soc";
        "RT5350"
    } else {
        panic!("rt305x: unknown SoC, n0:{:08x} n1:{:08x}", rt305x_get_soc_name0(), rt305x_get_soc_name1())
    }
}

unsafe fn rt305x_get_soc_id() -> u32 {
    __raw_readl(RT305X_SYSC_BASE + SYSC_REG_CHIP_ID)
}

unsafe fn rt305x_get_soc_ver() -> u32 {
    (rt305x_get_soc_id() >> CHIP_ID_ID_SHIFT) & CHIP_ID_ID_MASK
}

unsafe fn rt305x_get_soc_rev() -> u32 {
    rt305x_get_soc_id() & CHIP_ID_REV_MASK
}

unsafe fn rt305x_get_soc_id_name() -> &'static str {
    if soc_is_rt3050() { "rt3050" }
    else if soc_is_rt3052() { "rt3052" }
    else if soc_is_rt3350() { "rt3350" }
    else if soc_is_rt3352() { "rt3352" }
    else if soc_is_rt5350() { "rt5350" }
    else { "invalid" }
}

unsafe fn rt305x_soc_dev_init() -> i32 {
    let mut soc_dev_attr: *mut soc_device_attribute = kzalloc_obj();
    if soc_dev_attr.is_null() { return -ENOMEM; }

    (*soc_dev_attr).family = "Ralink";
    (*soc_dev_attr).soc_id = rt305x_get_soc_id_name();
    (*soc_dev_attr).data = soc_info_ptr;

    let soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) {
        kfree(soc_dev_attr);
        return PTR_ERR(soc_dev);
    }
    0
}

// device_initcall(rt305x_soc_dev_init);

unsafe fn prom_soc_init(soc_info: *mut ralink_soc_info) {
    let name = rt305x_get_soc_name(soc_info);

    snprintf((*soc_info).sys_type.as_mut_ptr(), RAMIPS_SYS_TYPE_LEN,
        "Ralink {} id:{} rev:{}", name, rt305x_get_soc_ver(), rt305x_get_soc_rev());

    (*soc_info).mem_base = RT305X_SDRAM_BASE;
    if soc_is_rt5350() {
        (*soc_info).mem_size = rt5350_get_mem_size();
    } else if soc_is_rt305x() || soc_is_rt3350() {
        (*soc_info).mem_size_min = RT305X_MEM_SIZE_MIN;
        (*soc_info).mem_size_max = RT305X_MEM_SIZE_MAX;
    } else if soc_is_rt3352() {
        (*soc_info).mem_size_min = RT3352_MEM_SIZE_MIN;
        (*soc_info).mem_size_max = RT3352_MEM_SIZE_MAX;
    }

    soc_info_ptr = soc_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
