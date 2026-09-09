// SPDX-License-Identifier: GPL-2.0
/*
 * Allwinner CPUFreq nvmem based driver
 *
 * The sun50i-cpufreq-nvmem driver reads the efuse value from the SoC to
 * provide the OPP framework with required information.
 *
 * Copyright (C) 2019 Yangtao Li <tiny.windzz@gmail.com>
 */

// Kernel dependencies supplied by other translation units.

const NVMEM_MASK: u32 = 0x7;
const NVMEM_SHIFT: u32 = 5;
const SUN50I_A100_NVMEM_MASK: u32 = 0xf;
const SUN50I_A100_NVMEM_SHIFT: u32 = 12;

static mut CPUFREQ_DT_PDEV: *mut platform_device = core::ptr::null_mut();
static mut SUN50I_CPUFREQ_PDEV: *mut platform_device = core::ptr::null_mut();

#[repr(C)]
struct sunxi_cpufreq_data {
    efuse_xlate: unsafe extern "C" fn(speedbin: u32) -> u32,
}

unsafe extern "C" fn sun50i_h6_efuse_xlate(speedbin: u32) -> u32 {
    let efuse_value = (speedbin >> NVMEM_SHIFT) & NVMEM_MASK;
    /*
     * We treat unexpected efuse values as if the SoC was from
     * the slowest bin. Expected efuse values are 1-3, slowest
     * to fastest.
     */
    if efuse_value >= 1 && efuse_value <= 3 {
        efuse_value - 1
    } else {
        0
    }
}

unsafe extern "C" fn sun50i_a100_efuse_xlate(speedbin: u32) -> u32 {
    let efuse_value = (speedbin >> SUN50I_A100_NVMEM_SHIFT) & SUN50I_A100_NVMEM_MASK;
    match efuse_value {
        0b100 => 2,
        0b010 => 1,
        _ => 0,
    }
}

unsafe extern "C" fn get_soc_id_revision() -> i32 {
    // CONFIG_HAVE_ARM_SMCCC_DISCOVERY selects the external implementation.
    arm_smccc_get_soc_id_revision()
}

/*
 * Judging by the OPP tables in the vendor BSP, the quality order of the
 * returned speedbin index is 4 -> 0/2 -> 3 -> 1, from worst to best.
 * 0 and 2 seem identical from the OPP tables' point of view.
 */
unsafe extern "C" fn sun50i_h616_efuse_xlate(speedbin: u32) -> u32 {
    let ver_bits = get_soc_id_revision();
    let mut value = 0;
    match speedbin & 0xffff {
        0x2000 => value = 0,
        0x2400 | 0x7400 | 0x2c00 | 0x7c00 => {
            if ver_bits != SMCCC_RET_NOT_SUPPORTED && ver_bits <= 1 {
                /* ic version A/B */
                value = 1;
            } else {
                /* ic version C and later version */
                value = 2;
            }
        }
        0x5000 | 0x5400 | 0x6000 => value = 3,
        0x5c00 => value = 4,
        0x5d00 => value = 0,
        0x6c00 => value = 5,
        _ => {
            pr_warn("sun50i-cpufreq-nvmem: unknown speed bin 0x{:x}, using default bin 0\n", speedbin & 0xffff);
            value = 0;
        }
    }
    value
}

static mut SUN50I_H6_CPUFREQ_DATA: sunxi_cpufreq_data = sunxi_cpufreq_data {
    efuse_xlate: sun50i_h6_efuse_xlate,
};
static mut SUN50I_A100_CPUFREQ_DATA: sunxi_cpufreq_data = sunxi_cpufreq_data {
    efuse_xlate: sun50i_a100_efuse_xlate,
};
static mut SUN50I_H616_CPUFREQ_DATA: sunxi_cpufreq_data = sunxi_cpufreq_data {
    efuse_xlate: sun50i_h616_efuse_xlate,
};

#[repr(C)]
static CPU_OPP_MATCH_LIST: [of_device_id; 4] = [
    of_device_id { compatible: "allwinner,sun50i-h6-operating-points", data: unsafe { &SUN50I_H6_CPUFREQ_DATA } },
    of_device_id { compatible: "allwinner,sun50i-a100-operating-points", data: unsafe { &SUN50I_A100_CPUFREQ_DATA } },
    of_device_id { compatible: "allwinner,sun50i-h616-operating-points", data: unsafe { &SUN50I_H616_CPUFREQ_DATA } },
    of_device_id { compatible: "", data: core::ptr::null() },
];

unsafe extern "C" fn dt_has_supported_hw() -> bool {
    let cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() { return false; }
    let np = dev_pm_opp_of_get_opp_desc_node(cpu_dev);
    if np.is_null() { return false; }
    let mut has_opp_supported_hw = false;
    for opp in for_each_child_of_node(np) {
        if of_property_present(opp, "opp-supported-hw") {
            has_opp_supported_hw = true;
            break;
        }
    }
    of_node_put(np);
    has_opp_supported_hw
}

unsafe extern "C" fn sun50i_cpufreq_get_efuse() -> i32 {
    let cpu_dev = get_cpu_device(0);
    if cpu_dev.is_null() { return -ENODEV; }
    let np = dev_pm_opp_of_get_opp_desc_node(cpu_dev);
    if np.is_null() { return -ENOENT; }
    let match_entry = of_match_node(CPU_OPP_MATCH_LIST.as_ptr(), np);
    if match_entry.is_null() { return -ENOENT; }
    let opp_data = (*match_entry).data as *const sunxi_cpufreq_data;
    let speedbin_nvmem = of_nvmem_cell_get(np, core::ptr::null());
    if IS_ERR(speedbin_nvmem) { return dev_err_probe(cpu_dev, PTR_ERR(speedbin_nvmem), "Could not get nvmem cell\n"); }
    let mut len: usize = 0;
    let speedbin_ptr = nvmem_cell_read(speedbin_nvmem, &mut len);
    nvmem_cell_put(speedbin_nvmem);
    if IS_ERR(speedbin_ptr) { return PTR_ERR(speedbin_ptr); }
    let mut speedbin: u32 = 0;
    if len <= 4 { core::ptr::copy_nonoverlapping(speedbin_ptr as *const u8, &mut speedbin as *mut u32 as *mut u8, len); }
    speedbin = u32::from_le(speedbin);
    let ret = ((*opp_data).efuse_xlate)(speedbin) as i32;
    kfree(speedbin_ptr);
    ret
}

// The remaining platform-driver registration and teardown declarations retain
// the source-level interface; kernel types and helpers are external.
unsafe extern "C" fn sun50i_cpufreq_nvmem_probe(pdev: *mut platform_device) -> i32 {
    let opp_tokens = kzalloc_objs::<i32>(num_possible_cpus());
    if opp_tokens.is_null() { return -ENOMEM; }
    let speed = sun50i_cpufreq_get_efuse();
    if speed < 0 { kfree(opp_tokens as *mut _); return speed; }
    let mut supported_hw: u32 = 0;
    let mut config: dev_pm_opp_config = core::mem::zeroed();
    if dt_has_supported_hw() { supported_hw = 1u32 << speed; config.supported_hw = &mut supported_hw; config.supported_hw_count = 1; }
    let mut name = [0u8; 16];
    snprintf(name.as_mut_ptr(), name.len(), "speed%d", speed);
    config.prop_name = name.as_ptr();
    for cpu in for_each_present_cpu() {
        let cpu_dev = get_cpu_device(cpu);
        if cpu_dev.is_null() { kfree(opp_tokens as *mut _); return -ENODEV; }
        let ret = dev_pm_opp_set_config(cpu_dev, &config);
        if ret < 0 { kfree(opp_tokens as *mut _); return ret; }
        *opp_tokens.add(cpu as usize) = ret;
    }
    CPUFREQ_DT_PDEV = platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    if !IS_ERR(CPUFREQ_DT_PDEV) { platform_set_drvdata(pdev, opp_tokens); return 0; }
    let ret = PTR_ERR(CPUFREQ_DT_PDEV);
    pr_err("Failed to register platform device\n");
    for cpu in for_each_present_cpu() { dev_pm_opp_clear_config(*opp_tokens.add(cpu as usize)); }
    kfree(opp_tokens as *mut _);
    ret
}

unsafe extern "C" fn sun50i_cpufreq_nvmem_remove(pdev: *mut platform_device) {
    let opp_tokens = platform_get_drvdata(pdev) as *mut i32;
    platform_device_unregister(CPUFREQ_DT_PDEV);
    for cpu in for_each_present_cpu() { dev_pm_opp_clear_config(*opp_tokens.add(cpu as usize)); }
    kfree(opp_tokens as *mut _);
}

static SUN50I_CPUFREQ_DRIVER: platform_driver = platform_driver {
    probe: sun50i_cpufreq_nvmem_probe,
    remove: sun50i_cpufreq_nvmem_remove,
    driver: driver { name: "sun50i-cpufreq-nvmem" },
};

static SUN50I_CPUFREQ_MATCH_LIST: [of_device_id; 6] = [
    of_device_id { compatible: "allwinner,sun50i-h6", data: core::ptr::null() },
    of_device_id { compatible: "allwinner,sun50i-a100", data: core::ptr::null() },
    of_device_id { compatible: "allwinner,sun50i-h616", data: core::ptr::null() },
    of_device_id { compatible: "allwinner,sun50i-h618", data: core::ptr::null() },
    of_device_id { compatible: "allwinner,sun50i-h700", data: core::ptr::null() },
    of_device_id { compatible: "", data: core::ptr::null() },
];

// Since the driver depends on nvmem drivers, all real activity is done in probe.
unsafe extern "C" fn sun50i_cpufreq_init() -> i32 {
    if !of_machine_device_match(SUN50I_CPUFREQ_MATCH_LIST.as_ptr()) { return -ENODEV; }
    let ret = platform_driver_register(&SUN50I_CPUFREQ_DRIVER);
    if ret < 0 { return ret; }
    SUN50I_CPUFREQ_PDEV = platform_device_register_simple("sun50i-cpufreq-nvmem", -1, core::ptr::null_mut(), 0);
    let ret = PTR_ERR_OR_ZERO(SUN50I_CPUFREQ_PDEV);
    if ret == 0 { return 0; }
    platform_driver_unregister(&SUN50I_CPUFREQ_DRIVER);
    ret
}

unsafe extern "C" fn sun50i_cpufreq_exit() {
    platform_device_unregister(SUN50I_CPUFREQ_PDEV);
    platform_driver_unregister(&SUN50I_CPUFREQ_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
